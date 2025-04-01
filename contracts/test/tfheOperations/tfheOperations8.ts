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

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 1 (340282366920938463463371878378473874149, 340282366920938463463371817794816906113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371878378473874149n);
    input.add128(340282366920938463463371817794816906113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 2 (340282366920938463463371817794816906109, 340282366920938463463371817794816906113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371817794816906109n);
    input.add128(340282366920938463463371817794816906113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 3 (340282366920938463463371817794816906113, 340282366920938463463371817794816906113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371817794816906113n);
    input.add128(340282366920938463463371817794816906113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint128) => ebool test 4 (340282366920938463463371817794816906113, 340282366920938463463371817794816906109)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371817794816906113n);
    input.add128(340282366920938463463371817794816906109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 1 (340282366920938463463373466068258350953, 340282366920938463463368725843962081015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373466068258350953n);
    input.add128(340282366920938463463368725843962081015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 2 (340282366920938463463368725843962081011, 340282366920938463463368725843962081015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368725843962081011n);
    input.add128(340282366920938463463368725843962081015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 3 (340282366920938463463368725843962081015, 340282366920938463463368725843962081015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368725843962081015n);
    input.add128(340282366920938463463368725843962081015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint128) => ebool test 4 (340282366920938463463368725843962081015, 340282366920938463463368725843962081011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368725843962081015n);
    input.add128(340282366920938463463368725843962081011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 1 (340282366920938463463366070522534059797, 340282366920938463463369964117073493025)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366070522534059797n);
    input.add128(340282366920938463463369964117073493025n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 2 (340282366920938463463366070522534059793, 340282366920938463463366070522534059797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366070522534059793n);
    input.add128(340282366920938463463366070522534059797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 3 (340282366920938463463366070522534059797, 340282366920938463463366070522534059797)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366070522534059797n);
    input.add128(340282366920938463463366070522534059797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint128) => ebool test 4 (340282366920938463463366070522534059797, 340282366920938463463366070522534059793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366070522534059797n);
    input.add128(340282366920938463463366070522534059793n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 1 (340282366920938463463373178800352805043, 340282366920938463463370558820913344395)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373178800352805043n);
    input.add128(340282366920938463463370558820913344395n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 2 (340282366920938463463370558820913344391, 340282366920938463463370558820913344395)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370558820913344391n);
    input.add128(340282366920938463463370558820913344395n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 3 (340282366920938463463370558820913344395, 340282366920938463463370558820913344395)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370558820913344395n);
    input.add128(340282366920938463463370558820913344395n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint128) => ebool test 4 (340282366920938463463370558820913344395, 340282366920938463463370558820913344391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370558820913344395n);
    input.add128(340282366920938463463370558820913344391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 1 (340282366920938463463371540119009257839, 340282366920938463463369872103194482123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371540119009257839n);
    input.add128(340282366920938463463369872103194482123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369872103194482123n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 2 (340282366920938463463369872103194482119, 340282366920938463463369872103194482123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369872103194482119n);
    input.add128(340282366920938463463369872103194482123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369872103194482119n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 3 (340282366920938463463369872103194482123, 340282366920938463463369872103194482123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369872103194482123n);
    input.add128(340282366920938463463369872103194482123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369872103194482123n);
  });

  it('test operator "min" overload (euint128, euint128) => euint128 test 4 (340282366920938463463369872103194482123, 340282366920938463463369872103194482119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369872103194482123n);
    input.add128(340282366920938463463369872103194482119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369872103194482119n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 1 (340282366920938463463373294351238319455, 340282366920938463463373159732481326341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373294351238319455n);
    input.add128(340282366920938463463373159732481326341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463373294351238319455n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 2 (340282366920938463463373159732481326337, 340282366920938463463373159732481326341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373159732481326337n);
    input.add128(340282366920938463463373159732481326341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463373159732481326341n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 3 (340282366920938463463373159732481326341, 340282366920938463463373159732481326341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373159732481326341n);
    input.add128(340282366920938463463373159732481326341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463373159732481326341n);
  });

  it('test operator "max" overload (euint128, euint128) => euint128 test 4 (340282366920938463463373159732481326341, 340282366920938463463373159732481326337)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373159732481326341n);
    input.add128(340282366920938463463373159732481326337n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463373159732481326341n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 1 (340282366920938463463367431691506925051, 115792089237316195423570985008687907853269984665640564039457578403191444505187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367431691506925051n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578403191444505187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463366726686515167331n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 2 (340282366920938463463367431691506925047, 340282366920938463463367431691506925051)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367431691506925047n);
    input.add256(340282366920938463463367431691506925051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367431691506925043n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 3 (340282366920938463463367431691506925051, 340282366920938463463367431691506925051)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367431691506925051n);
    input.add256(340282366920938463463367431691506925051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367431691506925051n);
  });

  it('test operator "and" overload (euint128, euint256) => euint256 test 4 (340282366920938463463367431691506925051, 340282366920938463463367431691506925047)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367431691506925051n);
    input.add256(340282366920938463463367431691506925047n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367431691506925043n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 1 (340282366920938463463367165254567953509, 115792089237316195423570985008687907853269984665640564039457583848360202399085)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367165254567953509n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583848360202399085n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583990197329063277n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 2 (340282366920938463463367165254567953505, 340282366920938463463367165254567953509)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367165254567953505n);
    input.add256(340282366920938463463367165254567953509n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367165254567953509n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 3 (340282366920938463463367165254567953509, 340282366920938463463367165254567953509)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367165254567953509n);
    input.add256(340282366920938463463367165254567953509n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367165254567953509n);
  });

  it('test operator "or" overload (euint128, euint256) => euint256 test 4 (340282366920938463463367165254567953509, 340282366920938463463367165254567953505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367165254567953509n);
    input.add256(340282366920938463463367165254567953505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463367165254567953509n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 1 (340282366920938463463370554464339412497, 115792089237316195423570985008687907853269984665640564039457580202086892371107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370554464339412497n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580202086892371107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907852929702298719625575994210494804586019506n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 2 (340282366920938463463370554464339412493, 340282366920938463463370554464339412497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370554464339412493n);
    input.add256(340282366920938463463370554464339412497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 3 (340282366920938463463370554464339412497, 340282366920938463463370554464339412497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370554464339412497n);
    input.add256(340282366920938463463370554464339412497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint256) => euint256 test 4 (340282366920938463463370554464339412497, 340282366920938463463370554464339412493)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370554464339412497n);
    input.add256(340282366920938463463370554464339412493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 1 (340282366920938463463372735835714138325, 115792089237316195423570985008687907853269984665640564039457579203676229322547)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463372735835714138325n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579203676229322547n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 2 (340282366920938463463372735835714138321, 340282366920938463463372735835714138325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463372735835714138321n);
    input.add256(340282366920938463463372735835714138325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 3 (340282366920938463463372735835714138325, 340282366920938463463372735835714138325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463372735835714138325n);
    input.add256(340282366920938463463372735835714138325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 4 (340282366920938463463372735835714138325, 340282366920938463463372735835714138321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463372735835714138325n);
    input.add256(340282366920938463463372735835714138321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 1 (340282366920938463463368244124099232299, 115792089237316195423570985008687907853269984665640564039457579499915017327175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368244124099232299n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579499915017327175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 2 (340282366920938463463368244124099232295, 340282366920938463463368244124099232299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368244124099232295n);
    input.add256(340282366920938463463368244124099232299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 3 (340282366920938463463368244124099232299, 340282366920938463463368244124099232299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368244124099232299n);
    input.add256(340282366920938463463368244124099232299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint256) => ebool test 4 (340282366920938463463368244124099232299, 340282366920938463463368244124099232295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368244124099232299n);
    input.add256(340282366920938463463368244124099232295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582515676676611145, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582515676676611145n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 2 (24, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(24n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(24n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 3 (28, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(28n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(28n);
  });

  it('test operator "and" overload (euint256, euint8) => euint256 test 4 (28, 24)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(28n);
    input.add8(24n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(24n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575430358626638405, 126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575430358626638405n);
    input.add8(126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575430358626638463n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 2 (122, 126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(122n);
    input.add8(126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(126n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 3 (126, 126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(126n);
    input.add8(126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(126n);
  });

  it('test operator "or" overload (euint256, euint8) => euint256 test 4 (126, 122)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(126n);
    input.add8(122n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(126n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576872403891001737, 166)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576872403891001737n);
    input.add8(166n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576872403891001647n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 2 (162, 166)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(162n);
    input.add8(166n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 3 (166, 166)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(166n);
    input.add8(166n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint8) => euint256 test 4 (166, 162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(166n);
    input.add8(162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575815567596884125, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575815567596884125n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 2 (88, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(88n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 3 (92, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(92n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint8) => ebool test 4 (92, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(92n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583266347155314121, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583266347155314121n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint8) => ebool test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582336788729157085, 14244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582336788729157085n);
    input.add16(14244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(5508n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 2 (14240, 14244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(14240n);
    input.add16(14244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(14240n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 3 (14244, 14244)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(14244n);
    input.add16(14244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(14244n);
  });

  it('test operator "and" overload (euint256, euint16) => euint256 test 4 (14244, 14240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(14244n);
    input.add16(14240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(14240n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577799389986777331, 7238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577799389986777331n);
    input.add16(7238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577799389986782455n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 2 (7234, 7238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(7234n);
    input.add16(7238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(7238n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 3 (7238, 7238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(7238n);
    input.add16(7238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(7238n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 4 (7238, 7234)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(7238n);
    input.add16(7234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(7238n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583580014534067651, 38303)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583580014534067651n);
    input.add16(38303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583580014534103132n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 2 (38299, 38303)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(38299n);
    input.add16(38303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 3 (38303, 38303)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(38303n);
    input.add16(38303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 4 (38303, 38299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(38303n);
    input.add16(38299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575880528606387215, 62423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575880528606387215n);
    input.add16(62423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 2 (62419, 62423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(62419n);
    input.add16(62423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 3 (62423, 62423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(62423n);
    input.add16(62423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 4 (62423, 62419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(62423n);
    input.add16(62419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577948948788611795, 36597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577948948788611795n);
    input.add16(36597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 2 (36593, 36597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(36593n);
    input.add16(36597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 3 (36597, 36597)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(36597n);
    input.add16(36597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 4 (36597, 36593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(36597n);
    input.add16(36593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578708924703909887, 3686495958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578708924703909887n);
    input.add32(3686495958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(422782678n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 2 (3686495954, 3686495958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3686495954n);
    input.add32(3686495958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(3686495954n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 3 (3686495958, 3686495958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3686495958n);
    input.add32(3686495958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(3686495958n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 4 (3686495958, 3686495954)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3686495958n);
    input.add32(3686495954n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(3686495954n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581531100962164365, 3645239033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581531100962164365n);
    input.add32(3645239033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581531104204418813n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 2 (3645239029, 3645239033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3645239029n);
    input.add32(3645239033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(3645239037n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 3 (3645239033, 3645239033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3645239033n);
    input.add32(3645239033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(3645239033n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 4 (3645239033, 3645239029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(3645239033n);
    input.add32(3645239029n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(3645239037n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579466014308445973, 959192485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579466014308445973n);
    input.add32(959192485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579466015262388912n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 2 (959192481, 959192485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(959192481n);
    input.add32(959192485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 3 (959192485, 959192485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(959192485n);
    input.add32(959192485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 4 (959192485, 959192481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(959192485n);
    input.add32(959192481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583253561181359787, 1148703419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583253561181359787n);
    input.add32(1148703419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 2 (1148703415, 1148703419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(1148703415n);
    input.add32(1148703419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 3 (1148703419, 1148703419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(1148703419n);
    input.add32(1148703419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 4 (1148703419, 1148703415)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(1148703419n);
    input.add32(1148703415n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582536407988353049, 2552849794)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582536407988353049n);
    input.add32(2552849794n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 2 (2552849790, 2552849794)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2552849790n);
    input.add32(2552849794n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 3 (2552849794, 2552849794)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2552849794n);
    input.add32(2552849794n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 4 (2552849794, 2552849790)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(2552849794n);
    input.add32(2552849790n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581873696038066289, 18441256854080844079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581873696038066289n);
    input.add64(18441256854080844079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18439989087057219617n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 2 (18441256854080844075, 18441256854080844079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18441256854080844075n);
    input.add64(18441256854080844079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18441256854080844075n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 3 (18441256854080844079, 18441256854080844079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18441256854080844079n);
    input.add64(18441256854080844079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18441256854080844079n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 4 (18441256854080844079, 18441256854080844075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18441256854080844079n);
    input.add64(18441256854080844075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18441256854080844075n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579985002943061415, 18444611788438915611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579985002943061415n);
    input.add64(18444611788438915611n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582316588294463423n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 2 (18444611788438915607, 18444611788438915611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18444611788438915607n);
    input.add64(18444611788438915611n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18444611788438915615n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 3 (18444611788438915611, 18444611788438915611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18444611788438915611n);
    input.add64(18444611788438915611n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18444611788438915611n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 4 (18444611788438915611, 18444611788438915607)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18444611788438915611n);
    input.add64(18444611788438915607n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(18444611788438915615n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579515515614083701, 18443005938373701431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579515515614083701n);
    input.add64(18443005938373701431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039439138022638721045826n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 2 (18443005938373701427, 18443005938373701431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18443005938373701427n);
    input.add64(18443005938373701431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 3 (18443005938373701431, 18443005938373701431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18443005938373701431n);
    input.add64(18443005938373701431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 4 (18443005938373701431, 18443005938373701427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18443005938373701431n);
    input.add64(18443005938373701427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580157877855380839, 18445861414108352117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580157877855380839n);
    input.add64(18445861414108352117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 2 (18445861414108352113, 18445861414108352117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18445861414108352113n);
    input.add64(18445861414108352117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 3 (18445861414108352117, 18445861414108352117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18445861414108352117n);
    input.add64(18445861414108352117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 4 (18445861414108352117, 18445861414108352113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18445861414108352117n);
    input.add64(18445861414108352113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582767795625604333, 18442517442309497305)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582767795625604333n);
    input.add64(18442517442309497305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 2 (18442517442309497301, 18442517442309497305)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18442517442309497301n);
    input.add64(18442517442309497305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 3 (18442517442309497305, 18442517442309497305)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18442517442309497305n);
    input.add64(18442517442309497305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 4 (18442517442309497305, 18442517442309497301)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(18442517442309497305n);
    input.add64(18442517442309497301n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576570272678427043, 340282366920938463463365977580027748807)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576570272678427043n);
    input.add128(340282366920938463463365977580027748807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463365902598219506051n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 2 (340282366920938463463365977580027748803, 340282366920938463463365977580027748807)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463365977580027748803n);
    input.add128(340282366920938463463365977580027748807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463365977580027748803n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 3 (340282366920938463463365977580027748807, 340282366920938463463365977580027748807)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463365977580027748807n);
    input.add128(340282366920938463463365977580027748807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463365977580027748807n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 4 (340282366920938463463365977580027748807, 340282366920938463463365977580027748803)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463365977580027748807n);
    input.add128(340282366920938463463365977580027748803n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463365977580027748803n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577197813457990369, 340282366920938463463369936236637249441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577197813457990369n);
    input.add128(340282366920938463463369936236637249441n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579486526930157537n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 2 (340282366920938463463369936236637249437, 340282366920938463463369936236637249441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463369936236637249437n);
    input.add128(340282366920938463463369936236637249441n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463369936236637249469n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 3 (340282366920938463463369936236637249441, 340282366920938463463369936236637249441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463369936236637249441n);
    input.add128(340282366920938463463369936236637249441n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463369936236637249441n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 4 (340282366920938463463369936236637249441, 340282366920938463463369936236637249437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463369936236637249441n);
    input.add128(340282366920938463463369936236637249437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(340282366920938463463369936236637249469n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577669991347740401, 340282366920938463463370946277401117143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577669991347740401n);
    input.add128(340282366920938463463370946277401117143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907852929702298719625575994217147101371440934n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 2 (340282366920938463463370946277401117139, 340282366920938463463370946277401117143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463370946277401117139n);
    input.add128(340282366920938463463370946277401117143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 3 (340282366920938463463370946277401117143, 340282366920938463463370946277401117143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463370946277401117143n);
    input.add128(340282366920938463463370946277401117143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 4 (340282366920938463463370946277401117143, 340282366920938463463370946277401117139)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463370946277401117143n);
    input.add128(340282366920938463463370946277401117139n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582483289534633819, 340282366920938463463368175294632218305)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582483289534633819n);
    input.add128(340282366920938463463368175294632218305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 2 (340282366920938463463368175294632218301, 340282366920938463463368175294632218305)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463368175294632218301n);
    input.add128(340282366920938463463368175294632218305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 3 (340282366920938463463368175294632218305, 340282366920938463463368175294632218305)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463368175294632218305n);
    input.add128(340282366920938463463368175294632218305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 4 (340282366920938463463368175294632218305, 340282366920938463463368175294632218301)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463368175294632218305n);
    input.add128(340282366920938463463368175294632218301n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580732478702616499, 340282366920938463463370208690176225647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580732478702616499n);
    input.add128(340282366920938463463370208690176225647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 2 (340282366920938463463370208690176225643, 340282366920938463463370208690176225647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463370208690176225643n);
    input.add128(340282366920938463463370208690176225647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 3 (340282366920938463463370208690176225647, 340282366920938463463370208690176225647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463370208690176225647n);
    input.add128(340282366920938463463370208690176225647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 4 (340282366920938463463370208690176225647, 340282366920938463463370208690176225643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(340282366920938463463370208690176225647n);
    input.add128(340282366920938463463370208690176225643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576731167032528679, 115792089237316195423570985008687907853269984665640564039457578368724569402985)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576731167032528679n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578368724569402985n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575604414494969377n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576731167032528675, 115792089237316195423570985008687907853269984665640564039457576731167032528679)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576731167032528675n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576731167032528679n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576731167032528675n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576731167032528679, 115792089237316195423570985008687907853269984665640564039457576731167032528679)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576731167032528679n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576731167032528679n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576731167032528679n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576731167032528679, 115792089237316195423570985008687907853269984665640564039457576731167032528675)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576731167032528679n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576731167032528675n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576731167032528675n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577608279852617567, 115792089237316195423570985008687907853269984665640564039457576693273518325357)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577608279852617567n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576693273518325357n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579297466868481919n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576693273518325353, 115792089237316195423570985008687907853269984665640564039457576693273518325357)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576693273518325353n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576693273518325357n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576693273518325357n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576693273518325357, 115792089237316195423570985008687907853269984665640564039457576693273518325357)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576693273518325357n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576693273518325357n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576693273518325357n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576693273518325357, 115792089237316195423570985008687907853269984665640564039457576693273518325353)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576693273518325357n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576693273518325353n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576693273518325357n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578299350234902987, 115792089237316195423570985008687907853269984665640564039457579133571653816313)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578299350234902987n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579133571653816313n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(1432493188906546n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578299350234902983, 115792089237316195423570985008687907853269984665640564039457578299350234902987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578299350234902983n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578299350234902987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578299350234902987, 115792089237316195423570985008687907853269984665640564039457578299350234902987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578299350234902987n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578299350234902987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578299350234902987, 115792089237316195423570985008687907853269984665640564039457578299350234902983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578299350234902987n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578299350234902983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract5.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580719248345881805, 115792089237316195423570985008687907853269984665640564039457576623325431097525)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580719248345881805n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576623325431097525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457576623325431097521, 115792089237316195423570985008687907853269984665640564039457576623325431097525)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576623325431097521n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576623325431097525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457576623325431097525, 115792089237316195423570985008687907853269984665640564039457576623325431097525)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576623325431097525n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576623325431097525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457576623325431097525, 115792089237316195423570985008687907853269984665640564039457576623325431097521)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576623325431097525n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576623325431097521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580202639431468021, 115792089237316195423570985008687907853269984665640564039457583280960900509527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580202639431468021n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583280960900509527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457580202639431468017, 115792089237316195423570985008687907853269984665640564039457580202639431468021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580202639431468017n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580202639431468021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457580202639431468021, 115792089237316195423570985008687907853269984665640564039457580202639431468021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580202639431468021n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580202639431468021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457580202639431468021, 115792089237316195423570985008687907853269984665640564039457580202639431468017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580202639431468021n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580202639431468017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 1 (78, 132)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(78n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 132n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(210n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 2 (74, 78)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(74n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 78n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(152n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 3 (78, 78)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(78n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 78n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(156n);
  });

  it('test operator "add" overload (euint8, uint8) => euint8 test 4 (78, 74)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(78n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint8_uint8(encryptedAmount.handles[0], 74n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(152n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 1 (83, 67)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(67n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(83n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(150n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 2 (74, 78)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(78n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(74n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(152n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 3 (78, 78)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(78n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(78n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(156n);
  });

  it('test operator "add" overload (uint8, euint8) => euint8 test 4 (78, 74)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(74n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint8_euint8(78n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(152n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 1 (45, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(45n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint8_uint8(encryptedAmount.handles[0], 45n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, uint8) => euint8 test 2 (45, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(45n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint8_uint8(encryptedAmount.handles[0], 41n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 1 (45, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint8_euint8(45n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint8, euint8) => euint8 test 2 (45, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint8_euint8(45n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 1 (8, 31)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 31n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(248n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 2 (8, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(80n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint8, uint8) => euint8 test 4 (10, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(80n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 1 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(9n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 2 (8, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(80n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(10n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (uint8, euint8) => euint8 test 4 (10, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint8_euint8(10n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(80n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 1 (50, 225)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(50n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 225n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 2 (40, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(40n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 44n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 3 (44, 44)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(44n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 44n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint8, uint8) => euint8 test 4 (44, 40)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(44n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint8_uint8(encryptedAmount.handles[0], 40n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (179, 122)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(179n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 122n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(57n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (17, 21)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(17n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 21n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(17n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (21, 21)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(21n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 21n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (21, 17)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(21n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint8_uint8(encryptedAmount.handles[0], 17n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 1 (5, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 191n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint8_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 1 (155, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(155n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(155n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 2 (1, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(1n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(5n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(5n);
  });

  it('test operator "and" overload (uint8, euint8) => euint8 test 4 (5, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint8_euint8(5n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 1 (244, 106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(244n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 106n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(254n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 2 (157, 161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(157n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 161n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(189n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 3 (161, 161)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(161n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 161n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(161n);
  });

  it('test operator "or" overload (euint8, uint8) => euint8 test 4 (161, 157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(161n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint8_uint8(encryptedAmount.handles[0], 157n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract5.resEuint8());
    expect(res).to.equal(189n);
  });
});
