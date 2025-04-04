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

    const contract8 = await deployTfheTestFixture8();
    this.contract8Address = await contract8.getAddress();
    this.contract8 = contract8;

    const contract9 = await deployTfheTestFixture9();
    this.contract9Address = await contract9.getAddress();
    this.contract9 = contract9;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "min" overload (euint32, euint256) => euint256 test 1 (2105195162, 115792089237316195423570985008687907853269984665640564039457576481269037859923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2105195162n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576481269037859923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2105195162n);
  });

  it('test operator "min" overload (euint32, euint256) => euint256 test 2 (2105195158, 2105195162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2105195158n);
    input.add256(2105195162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2105195158n);
  });

  it('test operator "min" overload (euint32, euint256) => euint256 test 3 (2105195162, 2105195162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2105195162n);
    input.add256(2105195162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2105195162n);
  });

  it('test operator "min" overload (euint32, euint256) => euint256 test 4 (2105195162, 2105195158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2105195162n);
    input.add256(2105195158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2105195158n);
  });

  it('test operator "max" overload (euint32, euint256) => euint256 test 1 (3229896994, 115792089237316195423570985008687907853269984665640564039457582368760806080917)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3229896994n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582368760806080917n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582368760806080917n);
  });

  it('test operator "max" overload (euint32, euint256) => euint256 test 2 (3229896990, 3229896994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3229896990n);
    input.add256(3229896994n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(3229896994n);
  });

  it('test operator "max" overload (euint32, euint256) => euint256 test 3 (3229896994, 3229896994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3229896994n);
    input.add256(3229896994n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(3229896994n);
  });

  it('test operator "max" overload (euint32, euint256) => euint256 test 4 (3229896994, 3229896990)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3229896994n);
    input.add256(3229896990n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(3229896994n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (1473487268, 793253262)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1473487268n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_uint32(
      encryptedAmount.handles[0],
      793253262n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2266740530n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (1473487264, 1473487268)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1473487264n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_uint32(
      encryptedAmount.handles[0],
      1473487268n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2946974532n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (1473487268, 1473487268)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1473487268n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_uint32(
      encryptedAmount.handles[0],
      1473487268n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2946974536n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (1473487268, 1473487264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1473487268n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_uint32(
      encryptedAmount.handles[0],
      1473487264n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2946974532n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (2854466166, 793253262)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(793253262n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint32_euint32(
      2854466166n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(3647719428n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (1473487264, 1473487268)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1473487268n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint32_euint32(
      1473487264n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2946974532n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (1473487268, 1473487268)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1473487268n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint32_euint32(
      1473487268n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2946974536n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (1473487268, 1473487264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1473487264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint32_euint32(
      1473487268n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2946974532n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (2630999733, 2630999733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2630999733n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_uint32(
      encryptedAmount.handles[0],
      2630999733n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (2630999733, 2630999729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2630999733n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_uint32(
      encryptedAmount.handles[0],
      2630999729n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (2630999733, 2630999733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2630999733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_uint32_euint32(
      2630999733n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (2630999733, 2630999729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2630999729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_uint32_euint32(
      2630999733n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (57895, 43053)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(57895n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_uint32(encryptedAmount.handles[0], 43053n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2492553435n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (58844, 58844)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(58844n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_uint32(encryptedAmount.handles[0], 58844n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(3462616336n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (58844, 58844)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(58844n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_uint32(encryptedAmount.handles[0], 58844n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(3462616336n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (58844, 58844)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(58844n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_uint32(encryptedAmount.handles[0], 58844n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(3462616336n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (33125, 43053)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(43053n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint32_euint32(33125n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1426130625n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (58844, 58844)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(58844n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint32_euint32(58844n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(3462616336n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (58844, 58844)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(58844n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint32_euint32(58844n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(3462616336n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (58844, 58844)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(58844n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint32_euint32(58844n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(3462616336n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (2451291967, 2068282460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2451291967n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.div_euint32_uint32(
      encryptedAmount.handles[0],
      2068282460n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (2451291963, 2451291967)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2451291963n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.div_euint32_uint32(
      encryptedAmount.handles[0],
      2451291967n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (2451291967, 2451291967)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2451291967n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.div_euint32_uint32(
      encryptedAmount.handles[0],
      2451291967n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (2451291967, 2451291963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2451291967n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.div_euint32_uint32(
      encryptedAmount.handles[0],
      2451291963n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (642707548, 2357516926)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(642707548n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rem_euint32_uint32(
      encryptedAmount.handles[0],
      2357516926n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(642707548n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (642707544, 642707548)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(642707544n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rem_euint32_uint32(
      encryptedAmount.handles[0],
      642707548n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(642707544n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (642707548, 642707548)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(642707548n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rem_euint32_uint32(
      encryptedAmount.handles[0],
      642707548n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (642707548, 642707544)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(642707548n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rem_euint32_uint32(
      encryptedAmount.handles[0],
      642707544n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 1 (1634775410, 1272475740)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1634775410n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_uint32(
      encryptedAmount.handles[0],
      1272475740n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1095774288n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 2 (117791198, 117791202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(117791198n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_uint32(
      encryptedAmount.handles[0],
      117791202n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(117791170n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 3 (117791202, 117791202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(117791202n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_uint32(
      encryptedAmount.handles[0],
      117791202n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(117791202n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 4 (117791202, 117791198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(117791202n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_uint32(
      encryptedAmount.handles[0],
      117791198n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(117791170n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 1 (665072343, 1272475740)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1272475740n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_uint32_euint32(
      665072343n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(58732628n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 2 (117791198, 117791202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(117791202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_uint32_euint32(
      117791198n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(117791170n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 3 (117791202, 117791202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(117791202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_uint32_euint32(
      117791202n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(117791202n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 4 (117791202, 117791198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(117791198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_uint32_euint32(
      117791202n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(117791170n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 1 (566808747, 2884963541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(566808747n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_uint32(
      encryptedAmount.handles[0],
      2884963541n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2885541119n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 2 (566808743, 566808747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(566808743n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_uint32(
      encryptedAmount.handles[0],
      566808747n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(566808751n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 3 (566808747, 566808747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(566808747n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_uint32(
      encryptedAmount.handles[0],
      566808747n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(566808747n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 4 (566808747, 566808743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(566808747n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_uint32(
      encryptedAmount.handles[0],
      566808743n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(566808751n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 1 (1592326700, 2884963541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2884963541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_uint32_euint32(
      1592326700n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4294835965n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 2 (566808743, 566808747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(566808747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_uint32_euint32(
      566808743n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(566808751n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 3 (566808747, 566808747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(566808747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_uint32_euint32(
      566808747n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(566808747n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 4 (566808747, 566808743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(566808743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_uint32_euint32(
      566808747n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(566808751n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 1 (3546807643, 2217405515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3546807643n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_uint32(
      encryptedAmount.handles[0],
      2217405515n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1464668432n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 2 (1914181665, 1914181669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1914181665n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_uint32(
      encryptedAmount.handles[0],
      1914181669n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 3 (1914181669, 1914181669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1914181669n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_uint32(
      encryptedAmount.handles[0],
      1914181669n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 4 (1914181669, 1914181665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1914181669n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_uint32(
      encryptedAmount.handles[0],
      1914181665n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 1 (2215841508, 2217405515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2217405515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_uint32_euint32(
      2215841508n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(3793583n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 2 (1914181665, 1914181669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1914181669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_uint32_euint32(
      1914181665n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 3 (1914181669, 1914181669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1914181669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_uint32_euint32(
      1914181669n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 4 (1914181669, 1914181665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1914181665n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_uint32_euint32(
      1914181669n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (3405236202, 2622205281)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3405236202n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_uint32(
      encryptedAmount.handles[0],
      2622205281n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (3405236198, 3405236202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3405236198n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_uint32(
      encryptedAmount.handles[0],
      3405236202n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (3405236202, 3405236202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3405236202n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_uint32(
      encryptedAmount.handles[0],
      3405236202n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (3405236202, 3405236198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3405236202n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_uint32(
      encryptedAmount.handles[0],
      3405236198n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (3663864640, 2622205281)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2622205281n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint32_euint32(
      3663864640n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (3405236198, 3405236202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(3405236202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint32_euint32(
      3405236198n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (3405236202, 3405236202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(3405236202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint32_euint32(
      3405236202n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (3405236202, 3405236198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(3405236198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint32_euint32(
      3405236202n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (2863544693, 2083233910)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2863544693n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_uint32(
      encryptedAmount.handles[0],
      2083233910n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (2863544689, 2863544693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2863544689n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_uint32(
      encryptedAmount.handles[0],
      2863544693n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (2863544693, 2863544693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2863544693n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_uint32(
      encryptedAmount.handles[0],
      2863544693n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (2863544693, 2863544689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2863544693n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_uint32(
      encryptedAmount.handles[0],
      2863544689n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (2272178734, 2083233910)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2083233910n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint32_euint32(
      2272178734n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (2863544689, 2863544693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2863544693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint32_euint32(
      2863544689n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (2863544693, 2863544693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2863544693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint32_euint32(
      2863544693n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (2863544693, 2863544689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2863544689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint32_euint32(
      2863544693n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (551100436, 2208736947)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(551100436n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2208736947n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (551100432, 551100436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(551100432n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_uint32(
      encryptedAmount.handles[0],
      551100436n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (551100436, 551100436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(551100436n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_uint32(
      encryptedAmount.handles[0],
      551100436n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (551100436, 551100432)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(551100436n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_uint32(
      encryptedAmount.handles[0],
      551100432n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (316433111, 2208736947)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2208736947n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_uint32_euint32(
      316433111n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (551100432, 551100436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(551100436n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_uint32_euint32(
      551100432n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (551100436, 551100436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(551100436n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_uint32_euint32(
      551100436n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (551100436, 551100432)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(551100432n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_uint32_euint32(
      551100436n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (3680469840, 1793760368)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3680469840n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      1793760368n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (2255802159, 2255802163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2255802159n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      2255802163n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (2255802163, 2255802163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2255802163n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      2255802163n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (2255802163, 2255802159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2255802163n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      2255802159n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (1735110936, 1793760368)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1793760368n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_uint32_euint32(
      1735110936n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (2255802159, 2255802163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2255802163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_uint32_euint32(
      2255802159n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (2255802163, 2255802163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2255802163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_uint32_euint32(
      2255802163n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (2255802163, 2255802159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2255802159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_uint32_euint32(
      2255802163n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (1828796222, 3487277978)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1828796222n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_uint32(
      encryptedAmount.handles[0],
      3487277978n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (1828796218, 1828796222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1828796218n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_uint32(
      encryptedAmount.handles[0],
      1828796222n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (1828796222, 1828796222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1828796222n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_uint32(
      encryptedAmount.handles[0],
      1828796222n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (1828796222, 1828796218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1828796222n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_uint32(
      encryptedAmount.handles[0],
      1828796218n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (900906283, 3487277978)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(3487277978n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint32_euint32(
      900906283n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (1828796218, 1828796222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1828796222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint32_euint32(
      1828796218n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (1828796222, 1828796222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1828796222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint32_euint32(
      1828796222n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (1828796222, 1828796218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1828796218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint32_euint32(
      1828796222n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (1506398496, 1135091247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1506398496n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_uint32(
      encryptedAmount.handles[0],
      1135091247n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (222862390, 222862394)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(222862390n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_uint32(
      encryptedAmount.handles[0],
      222862394n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (222862394, 222862394)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(222862394n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_uint32(
      encryptedAmount.handles[0],
      222862394n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (222862394, 222862390)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(222862394n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_uint32(
      encryptedAmount.handles[0],
      222862390n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (3727625505, 1135091247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1135091247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint32_euint32(
      3727625505n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (222862390, 222862394)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(222862394n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint32_euint32(
      222862390n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (222862394, 222862394)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(222862394n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint32_euint32(
      222862394n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (222862394, 222862390)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(222862390n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint32_euint32(
      222862394n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (719746117, 1013450447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(719746117n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_uint32(
      encryptedAmount.handles[0],
      1013450447n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(719746117n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (719746113, 719746117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(719746113n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_uint32(
      encryptedAmount.handles[0],
      719746117n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(719746113n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (719746117, 719746117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(719746117n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_uint32(
      encryptedAmount.handles[0],
      719746117n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(719746117n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (719746117, 719746113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(719746117n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_uint32(
      encryptedAmount.handles[0],
      719746113n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(719746113n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (616119507, 1013450447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1013450447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint32_euint32(
      616119507n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(616119507n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (719746113, 719746117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(719746117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint32_euint32(
      719746113n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(719746113n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (719746117, 719746117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(719746117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint32_euint32(
      719746117n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(719746117n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (719746117, 719746113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(719746113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint32_euint32(
      719746117n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(719746113n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (335410015, 559102725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(335410015n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_uint32(
      encryptedAmount.handles[0],
      559102725n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(559102725n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (335410011, 335410015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(335410011n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_uint32(
      encryptedAmount.handles[0],
      335410015n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(335410015n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (335410015, 335410015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(335410015n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_uint32(
      encryptedAmount.handles[0],
      335410015n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(335410015n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (335410015, 335410011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(335410015n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_uint32(
      encryptedAmount.handles[0],
      335410011n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(335410015n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (1685727210, 559102725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(559102725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint32_euint32(
      1685727210n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1685727210n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (335410011, 335410015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(335410015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint32_euint32(
      335410011n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(335410015n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (335410015, 335410015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(335410015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint32_euint32(
      335410015n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(335410015n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (335410015, 335410011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(335410011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint32_euint32(
      335410015n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(335410015n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (88, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(88n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(180n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (92, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(92n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(184n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (92, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(92n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(180n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (33, 33)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(33n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (33, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(33n);
    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (11, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(121n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (18443329990502344691, 100)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443329990502344691n);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(96n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (96, 100)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(96n);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(96n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (100, 100)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(100n);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(100n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (100, 96)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(100n);
    input.add8(96n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(96n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (18444709924961113443, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444709924961113443n);
    input.add8(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18444709924961113595n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (214, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(214n);
    input.add8(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(222n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (218, 218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(218n);
    input.add8(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(218n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (218, 214)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(218n);
    input.add8(214n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(222n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (18441230832767135375, 169)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441230832767135375n);
    input.add8(169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18441230832767135270n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (165, 169)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(165n);
    input.add8(169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (169, 169)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(169n);
    input.add8(169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (169, 165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(169n);
    input.add8(165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (18440186929924062425, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440186929924062425n);
    input.add8(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (243, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(243n);
    input.add8(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (247, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(247n);
    input.add8(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (247, 243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(247n);
    input.add8(243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (18439407436968588685, 116)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439407436968588685n);
    input.add8(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (112, 116)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(112n);
    input.add8(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (116, 116)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(116n);
    input.add8(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (116, 112)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(116n);
    input.add8(112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (18437867528697676657, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437867528697676657n);
    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (47, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(47n);
    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (51, 51)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(51n);
    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (51, 47)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(51n);
    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (18443853501726338219, 175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443853501726338219n);
    input.add8(175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (171, 175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(171n);
    input.add8(175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (175, 175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(175n);
    input.add8(175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (175, 171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(175n);
    input.add8(171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (18438431490704252463, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438431490704252463n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (22, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(22n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (26, 26)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(26n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (26, 22)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(26n);
    input.add8(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (18443192031640914529, 54)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443192031640914529n);
    input.add8(54n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (50, 54)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(50n);
    input.add8(54n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (54, 54)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(54n);
    input.add8(54n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (54, 50)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(54n);
    input.add8(50n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (18446045431492416549, 249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446045431492416549n);
    input.add8(249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(249n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (245, 249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(245n);
    input.add8(249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(245n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (249, 249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(249n);
    input.add8(249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(249n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (249, 245)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(249n);
    input.add8(245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(245n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (18438329117164058833, 164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438329117164058833n);
    input.add8(164n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18438329117164058833n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 2 (160, 164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(160n);
    input.add8(164n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(164n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 3 (164, 164)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(164n);
    input.add8(164n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(164n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 4 (164, 160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(164n);
    input.add8(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(164n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (65532, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(65532n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(65534n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (4967, 4971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4967n);
    input.add16(4971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(9938n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (4971, 4971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4971n);
    input.add16(4971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(9942n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (4971, 4967)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4971n);
    input.add16(4967n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(9938n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (63754, 63754)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(63754n);
    input.add16(63754n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (63754, 63750)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(63754n);
    input.add16(63750n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (32759, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(32759n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(65518n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (249, 249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(249n);
    input.add16(249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(62001n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (249, 249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(249n);
    input.add16(249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(62001n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (249, 249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(249n);
    input.add16(249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(62001n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (18438697501767064719, 38184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438697501767064719n);
    input.add16(38184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(5128n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (38180, 38184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(38180n);
    input.add16(38184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(38176n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (38184, 38184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(38184n);
    input.add16(38184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(38184n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (38184, 38180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(38184n);
    input.add16(38180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(38176n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (18443150484798383859, 22674)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18443150484798383859n);
    input.add16(22674n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18443150484798406387n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (22670, 22674)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(22670n);
    input.add16(22674n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(22686n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (22674, 22674)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(22674n);
    input.add16(22674n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(22674n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (22674, 22670)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(22674n);
    input.add16(22670n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(22686n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (18444584014432144839, 38243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18444584014432144839n);
    input.add16(38243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18444584014432114852n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (38239, 38243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(38239n);
    input.add16(38243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(60n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (38243, 38243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(38243n);
    input.add16(38243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (38243, 38239)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(38243n);
    input.add16(38239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(60n);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (18439495513791484549, 60962)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439495513791484549n);
    input.add16(60962n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (60958, 60962)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(60958n);
    input.add16(60962n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (60962, 60962)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(60962n);
    input.add16(60962n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (60962, 60958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(60962n);
    input.add16(60958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (18442543652418695997, 17515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18442543652418695997n);
    input.add16(17515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (17511, 17515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(17511n);
    input.add16(17515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (17515, 17515)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(17515n);
    input.add16(17515n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (17515, 17511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(17515n);
    input.add16(17511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (18440792536886429555, 61879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18440792536886429555n);
    input.add16(61879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (61875, 61879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(61875n);
    input.add16(61879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (61879, 61879)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(61879n);
    input.add16(61879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (61879, 61875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(61879n);
    input.add16(61875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (18445140881164995541, 28344)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18445140881164995541n);
    input.add16(28344n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (28340, 28344)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(28340n);
    input.add16(28344n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (28344, 28344)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(28344n);
    input.add16(28344n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (28344, 28340)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(28344n);
    input.add16(28340n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (18439655938048229863, 64761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439655938048229863n);
    input.add16(64761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (64757, 64761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(64757n);
    input.add16(64761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (64761, 64761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(64761n);
    input.add16(64761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (64761, 64757)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(64761n);
    input.add16(64757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (18439351956644474419, 8039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18439351956644474419n);
    input.add16(8039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (8035, 8039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(8035n);
    input.add16(8039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (8039, 8039)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(8039n);
    input.add16(8039n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (8039, 8035)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(8039n);
    input.add16(8035n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (18441355374577144863, 56859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18441355374577144863n);
    input.add16(56859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(56859n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (56855, 56859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(56855n);
    input.add16(56859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(56855n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (56859, 56859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(56859n);
    input.add16(56859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(56859n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (56859, 56855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(56859n);
    input.add16(56855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(56855n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18438103508695375859, 28036)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(18438103508695375859n);
    input.add16(28036n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18438103508695375859n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (28032, 28036)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(28032n);
    input.add16(28036n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(28036n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (28036, 28036)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(28036n);
    input.add16(28036n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(28036n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (28036, 28032)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(28036n);
    input.add16(28032n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(28036n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (4292948123, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4292948123n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4292948125n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (696185955, 696185959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(696185955n);
    input.add32(696185959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1392371914n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (696185959, 696185959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(696185959n);
    input.add32(696185959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1392371918n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (696185959, 696185955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(696185959n);
    input.add32(696185955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1392371914n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (4204742005, 4204742005)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4204742005n);
    input.add32(4204742005n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (4204742005, 4204742001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add64(4204742005n);
    input.add32(4204742001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });
});
