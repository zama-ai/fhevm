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

    const contract8 = await deployTfheTestFixture8();
    this.contract8Address = await contract8.getAddress();
    this.contract8 = contract8;

    const contract9 = await deployTfheTestFixture9();
    this.contract9Address = await contract9.getAddress();
    this.contract9 = contract9;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "mul" overload (euint16, euint256) => euint256 test 1 (2, 16385)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(2n);
    input.add256(16385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint16, euint256) => euint256 test 2 (232, 232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(232n);
    input.add256(232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(53824n);
  });

  it('test operator "mul" overload (euint16, euint256) => euint256 test 3 (232, 232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(232n);
    input.add256(232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(53824n);
  });

  it('test operator "mul" overload (euint16, euint256) => euint256 test 4 (232, 232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(232n);
    input.add256(232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(53824n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 1 (17914, 115792089237316195423570985008687907853269984665640564039457580580768792303137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(17914n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580580768792303137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(17440n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 2 (17910, 17914)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(17910n);
    input.add256(17914n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(17906n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 3 (17914, 17914)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(17914n);
    input.add256(17914n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(17914n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 4 (17914, 17910)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(17914n);
    input.add256(17910n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(17906n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 1 (46457, 115792089237316195423570985008687907853269984665640564039457576476867546216367)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46457n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576476867546216367n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576476867546249215n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 2 (46453, 46457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46453n);
    input.add256(46457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(46461n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 3 (46457, 46457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46457n);
    input.add256(46457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(46457n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 4 (46457, 46453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46457n);
    input.add256(46453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(46461n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 1 (53590, 115792089237316195423570985008687907853269984665640564039457579165686103561155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(53590n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579165686103561155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579165686103573141n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 2 (53586, 53590)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(53586n);
    input.add256(53590n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 3 (53590, 53590)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(53590n);
    input.add256(53590n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 4 (53590, 53586)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(53590n);
    input.add256(53586n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 1 (9141, 115792089237316195423570985008687907853269984665640564039457580883424502618393)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(9141n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580883424502618393n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 2 (9137, 9141)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(9137n);
    input.add256(9141n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 3 (9141, 9141)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(9141n);
    input.add256(9141n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 4 (9141, 9137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(9141n);
    input.add256(9137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 1 (41880, 115792089237316195423570985008687907853269984665640564039457580305014045563783)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(41880n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580305014045563783n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 2 (41876, 41880)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(41876n);
    input.add256(41880n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 3 (41880, 41880)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(41880n);
    input.add256(41880n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 4 (41880, 41876)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(41880n);
    input.add256(41876n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint256) => ebool test 1 (21630, 115792089237316195423570985008687907853269984665640564039457581351628219124645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(21630n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581351628219124645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint256) => ebool test 2 (21626, 21630)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(21626n);
    input.add256(21630n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint256) => ebool test 3 (21630, 21630)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(21630n);
    input.add256(21630n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint256) => ebool test 4 (21630, 21626)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(21630n);
    input.add256(21626n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint256) => ebool test 1 (30766, 115792089237316195423570985008687907853269984665640564039457579862160036454683)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(30766n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579862160036454683n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint256) => ebool test 2 (30762, 30766)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(30762n);
    input.add256(30766n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint256) => ebool test 3 (30766, 30766)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(30766n);
    input.add256(30766n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint256) => ebool test 4 (30766, 30762)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(30766n);
    input.add256(30762n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint256) => ebool test 1 (1034, 115792089237316195423570985008687907853269984665640564039457579344599069172131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(1034n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579344599069172131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint256) => ebool test 2 (1030, 1034)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(1030n);
    input.add256(1034n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint256) => ebool test 3 (1034, 1034)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(1034n);
    input.add256(1034n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint256) => ebool test 4 (1034, 1030)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(1034n);
    input.add256(1030n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint256) => ebool test 1 (39428, 115792089237316195423570985008687907853269984665640564039457582554616385928663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(39428n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582554616385928663n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint256) => ebool test 2 (39424, 39428)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(39424n);
    input.add256(39428n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint256) => ebool test 3 (39428, 39428)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(39428n);
    input.add256(39428n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint256) => ebool test 4 (39428, 39424)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(39428n);
    input.add256(39424n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint256) => euint256 test 1 (41222, 115792089237316195423570985008687907853269984665640564039457580765985897940815)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(41222n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580765985897940815n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(41222n);
  });

  it('test operator "min" overload (euint16, euint256) => euint256 test 2 (41218, 41222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(41218n);
    input.add256(41222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(41218n);
  });

  it('test operator "min" overload (euint16, euint256) => euint256 test 3 (41222, 41222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(41222n);
    input.add256(41222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(41222n);
  });

  it('test operator "min" overload (euint16, euint256) => euint256 test 4 (41222, 41218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(41222n);
    input.add256(41218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(41218n);
  });

  it('test operator "max" overload (euint16, euint256) => euint256 test 1 (60744, 115792089237316195423570985008687907853269984665640564039457576440963201685139)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(60744n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576440963201685139n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576440963201685139n);
  });

  it('test operator "max" overload (euint16, euint256) => euint256 test 2 (60740, 60744)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(60740n);
    input.add256(60744n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(60744n);
  });

  it('test operator "max" overload (euint16, euint256) => euint256 test 3 (60744, 60744)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(60744n);
    input.add256(60744n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(60744n);
  });

  it('test operator "max" overload (euint16, euint256) => euint256 test 4 (60744, 60740)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(60744n);
    input.add256(60740n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.res256());
    expect(res).to.equal(60744n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (53047, 2161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(53047n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_uint16(encryptedAmount.handles[0], 2161n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(55208n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (7373, 7377)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7373n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_uint16(encryptedAmount.handles[0], 7377n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14750n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (7377, 7377)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7377n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_uint16(encryptedAmount.handles[0], 7377n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14754n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (7377, 7373)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7377n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_uint16(encryptedAmount.handles[0], 7373n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14750n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (1253, 2161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(2161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_uint16_euint16(1253n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(3414n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (7373, 7377)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(7377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_uint16_euint16(7373n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14750n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (7377, 7377)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(7377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_uint16_euint16(7377n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14754n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (7377, 7373)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(7373n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_uint16_euint16(7377n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14750n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (32328, 32328)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(32328n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_uint16(encryptedAmount.handles[0], 32328n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (32328, 32324)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(32328n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_uint16(encryptedAmount.handles[0], 32324n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (32328, 32328)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(32328n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_uint16_euint16(32328n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (32328, 32324)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(32324n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_uint16_euint16(32328n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (850, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(850n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_uint16(encryptedAmount.handles[0], 60n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(51000n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (238, 238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(238n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_uint16(encryptedAmount.handles[0], 238n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(56644n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (238, 238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(238n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_uint16(encryptedAmount.handles[0], 238n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(56644n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (238, 238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(238n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_uint16(encryptedAmount.handles[0], 238n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(56644n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (462, 60)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(60n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_uint16_euint16(462n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(27720n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (238, 238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_uint16_euint16(238n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(56644n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (238, 238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_uint16_euint16(238n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(56644n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (238, 238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_uint16_euint16(238n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(56644n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (34748, 29601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34748n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint16_uint16(encryptedAmount.handles[0], 29601n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (34744, 34748)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34744n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint16_uint16(encryptedAmount.handles[0], 34748n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (34748, 34748)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34748n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint16_uint16(encryptedAmount.handles[0], 34748n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (34748, 34744)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34748n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.div_euint16_uint16(encryptedAmount.handles[0], 34744n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (68, 45087)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(68n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rem_euint16_uint16(encryptedAmount.handles[0], 45087n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(68n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (64, 68)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(64n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rem_euint16_uint16(encryptedAmount.handles[0], 68n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(64n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (68, 68)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(68n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rem_euint16_uint16(encryptedAmount.handles[0], 68n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (68, 64)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(68n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rem_euint16_uint16(encryptedAmount.handles[0], 64n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 1 (39208, 56643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(39208n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_uint16(encryptedAmount.handles[0], 56643n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(39168n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 2 (2221, 2225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(2221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_uint16(encryptedAmount.handles[0], 2225n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(2209n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 3 (2225, 2225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(2225n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_uint16(encryptedAmount.handles[0], 2225n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(2225n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 4 (2225, 2221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(2225n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_uint16(encryptedAmount.handles[0], 2221n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(2209n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 1 (34391, 56643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(56643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_uint16_euint16(34391n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(33859n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 2 (2221, 2225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(2225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_uint16_euint16(2221n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(2209n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 3 (2225, 2225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(2225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_uint16_euint16(2225n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(2225n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 4 (2225, 2221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(2221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_uint16_euint16(2225n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(2209n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 1 (16879, 47023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(16879n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_uint16(encryptedAmount.handles[0], 47023n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(63471n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 2 (6081, 6085)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(6081n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_uint16(encryptedAmount.handles[0], 6085n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(6085n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 3 (6085, 6085)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(6085n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_uint16(encryptedAmount.handles[0], 6085n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(6085n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 4 (6085, 6081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(6085n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_uint16(encryptedAmount.handles[0], 6081n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(6085n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 1 (62003, 47023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(47023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_uint16_euint16(62003n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(63423n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 2 (6081, 6085)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(6085n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_uint16_euint16(6081n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(6085n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 3 (6085, 6085)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(6085n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_uint16_euint16(6085n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(6085n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 4 (6085, 6081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(6081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_uint16_euint16(6085n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(6085n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 1 (11764, 5709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(11764n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_uint16(encryptedAmount.handles[0], 5709n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(15289n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 2 (11760, 11764)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(11760n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_uint16(encryptedAmount.handles[0], 11764n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 3 (11764, 11764)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(11764n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_uint16(encryptedAmount.handles[0], 11764n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 4 (11764, 11760)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(11764n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_uint16(encryptedAmount.handles[0], 11760n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 1 (45522, 5709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(5709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_uint16_euint16(45522n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(42911n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 2 (11760, 11764)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(11764n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_uint16_euint16(11760n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 3 (11764, 11764)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(11764n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_uint16_euint16(11764n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 4 (11764, 11760)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(11760n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_uint16_euint16(11764n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (57149, 1010)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(57149n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_uint16(encryptedAmount.handles[0], 1010n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (21056, 21060)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(21056n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_uint16(encryptedAmount.handles[0], 21060n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (21060, 21060)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(21060n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_uint16(encryptedAmount.handles[0], 21060n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (21060, 21056)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(21060n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_uint16(encryptedAmount.handles[0], 21056n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (29719, 1010)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(1010n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint16_euint16(29719n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (21056, 21060)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(21060n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint16_euint16(21056n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (21060, 21060)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(21060n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint16_euint16(21060n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (21060, 21056)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(21056n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint16_euint16(21060n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (28168, 22647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(28168n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_uint16(encryptedAmount.handles[0], 22647n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (28164, 28168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(28164n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_uint16(encryptedAmount.handles[0], 28168n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (28168, 28168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(28168n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_uint16(encryptedAmount.handles[0], 28168n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (28168, 28164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(28168n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_uint16(encryptedAmount.handles[0], 28164n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (59155, 22647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(22647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_uint16_euint16(59155n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (28164, 28168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(28168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_uint16_euint16(28164n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (28168, 28168)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(28168n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_uint16_euint16(28168n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (28168, 28164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(28164n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_uint16_euint16(28168n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (58159, 46958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(58159n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_uint16(encryptedAmount.handles[0], 46958n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (32898, 32902)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(32898n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_uint16(encryptedAmount.handles[0], 32902n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (32902, 32902)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(32902n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_uint16(encryptedAmount.handles[0], 32902n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (32902, 32898)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(32902n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_uint16(encryptedAmount.handles[0], 32898n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (60041, 46958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(46958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint16_euint16(60041n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (32898, 32902)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(32902n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint16_euint16(32898n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (32902, 32902)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(32902n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint16_euint16(32902n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (32902, 32898)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(32898n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_uint16_euint16(32902n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (13459, 37949)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(13459n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_uint16(encryptedAmount.handles[0], 37949n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (13455, 13459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(13455n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_uint16(encryptedAmount.handles[0], 13459n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (13459, 13459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(13459n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_uint16(encryptedAmount.handles[0], 13459n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (13459, 13455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(13459n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_uint16(encryptedAmount.handles[0], 13455n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (30297, 37949)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(37949n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_uint16_euint16(30297n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (13455, 13459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(13459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_uint16_euint16(13455n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (13459, 13459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(13459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_uint16_euint16(13459n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (13459, 13455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(13455n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_uint16_euint16(13459n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (34869, 1957)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34869n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_uint16(encryptedAmount.handles[0], 1957n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (34865, 34869)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34865n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_uint16(encryptedAmount.handles[0], 34869n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (34869, 34869)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34869n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_uint16(encryptedAmount.handles[0], 34869n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (34869, 34865)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34869n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_uint16(encryptedAmount.handles[0], 34865n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (12278, 1957)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(1957n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(12278n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (34865, 34869)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(34869n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(34865n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (34869, 34869)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(34869n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(34869n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (34869, 34865)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(34865n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(34869n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (22868, 52767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(22868n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_uint16(encryptedAmount.handles[0], 52767n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (22527, 22531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(22527n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_uint16(encryptedAmount.handles[0], 22531n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (22531, 22531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(22531n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_uint16(encryptedAmount.handles[0], 22531n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (22531, 22527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(22531n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_uint16(encryptedAmount.handles[0], 22527n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (50356, 52767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(52767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint16_euint16(50356n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (22527, 22531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(22531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint16_euint16(22527n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (22531, 22531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(22531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint16_euint16(22531n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (22531, 22527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(22527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint16_euint16(22531n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (17672, 42656)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(17672n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_uint16(encryptedAmount.handles[0], 42656n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(17672n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (4572, 4576)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(4572n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_uint16(encryptedAmount.handles[0], 4576n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4572n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (4576, 4576)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(4576n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_uint16(encryptedAmount.handles[0], 4576n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4576n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (4576, 4572)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(4576n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_uint16(encryptedAmount.handles[0], 4572n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4572n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (51926, 42656)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(42656n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_uint16_euint16(51926n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(42656n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (4572, 4576)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(4576n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_uint16_euint16(4572n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4572n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (4576, 4576)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(4576n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_uint16_euint16(4576n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4576n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (4576, 4572)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(4572n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_uint16_euint16(4576n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4572n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (35123, 30969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(35123n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_uint16(encryptedAmount.handles[0], 30969n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(35123n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (7481, 7485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7481n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_uint16(encryptedAmount.handles[0], 7485n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7485n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (7485, 7485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7485n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_uint16(encryptedAmount.handles[0], 7485n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7485n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (7485, 7481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7485n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_uint16(encryptedAmount.handles[0], 7481n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7485n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (11160, 30969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(30969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_uint16_euint16(11160n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(30969n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (7481, 7485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(7485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_uint16_euint16(7481n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7485n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (7485, 7485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(7485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_uint16_euint16(7485n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7485n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (7485, 7481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);

    input.add16(7481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_uint16_euint16(7485n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7485n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (246, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(246n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(248n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (120, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(120n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(244n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (124, 124)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(124n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(248n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (124, 120)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(124n);
    input.add8(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(244n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (121, 121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(121n);
    input.add8(121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (121, 117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(121n);
    input.add8(117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (68, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(68n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(136n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (10, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(10n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(110n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (11, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(11n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(110n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (4089078353, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4089078353n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (40, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(40n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(40n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (44, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(44n);
    input.add8(44n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(44n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (44, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(44n);
    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(40n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (2317076013, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2317076013n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2317076141n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (125, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(125n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(253n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (129, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(129n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(129n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (129, 125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(129n);
    input.add8(125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(253n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (2042354109, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2042354109n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2042354058n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (51, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(51n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (55, 55)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(55n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (55, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(55n);
    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (3706598064, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3706598064n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (110, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(110n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (114, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(114n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (114, 110)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(114n);
    input.add8(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (582553648, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(582553648n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (11, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(11n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (15, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(15n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (15, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(15n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (834524579, 202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(834524579n);
    input.add8(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (198, 202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(198n);
    input.add8(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (202, 202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(202n);
    input.add8(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (202, 198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(202n);
    input.add8(198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (1450395465, 251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1450395465n);
    input.add8(251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (247, 251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(247n);
    input.add8(251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (251, 251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(251n);
    input.add8(251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (251, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(251n);
    input.add8(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (3917310764, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3917310764n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (37, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(37n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (41, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(41n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (41, 37)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(41n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (138760330, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(138760330n);
    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (79, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(79n);
    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (83, 83)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(83n);
    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (83, 79)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(83n);
    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (2118066938, 207)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2118066938n);
    input.add8(207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(207n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (203, 207)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(203n);
    input.add8(207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(203n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (207, 207)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(207n);
    input.add8(207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(207n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (207, 203)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(207n);
    input.add8(203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(203n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (2103349618, 151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2103349618n);
    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2103349618n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (147, 151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(147n);
    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(151n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (151, 151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(151n);
    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(151n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (151, 147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(151n);
    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(151n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (49048, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(49048n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(49050n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (14834, 14838)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(14834n);
    input.add16(14838n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(29672n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (14838, 14838)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(14838n);
    input.add16(14838n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(29676n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (14838, 14834)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(14838n);
    input.add16(14834n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(29672n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (47807, 47807)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(47807n);
    input.add16(47807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (47807, 47803)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(47807n);
    input.add16(47803n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (18914, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(18914n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(37828n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (181, 181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(181n);
    input.add16(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(32761n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (181, 181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(181n);
    input.add16(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(32761n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (181, 181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(181n);
    input.add16(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(32761n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (1329277729, 38650)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1329277729n);
    input.add16(38650n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(544n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (38646, 38650)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(38646n);
    input.add16(38650n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(38642n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (38650, 38650)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(38650n);
    input.add16(38650n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(38650n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (38650, 38646)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(38650n);
    input.add16(38646n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(38642n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (2974165368, 50651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2974165368n);
    input.add16(50651n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2974215675n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (50647, 50651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(50647n);
    input.add16(50651n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(50655n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (50651, 50651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(50651n);
    input.add16(50651n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(50651n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (50651, 50647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(50651n);
    input.add16(50647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(50655n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (3454596358, 45834)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3454596358n);
    input.add16(45834n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(3454551564n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (45830, 45834)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(45830n);
    input.add16(45834n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (45834, 45834)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(45834n);
    input.add16(45834n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (45834, 45830)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(45834n);
    input.add16(45830n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(12n);
  });
});
