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

describe('TFHE operations 10', function () {
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

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (3448348036, 3448348036)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3448348036n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint32_euint32(
      3448348036n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (3448348036, 3448348032)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3448348032n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint32_euint32(
      3448348036n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (102973, 27327)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(102973n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 27327n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2813943171n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (51487, 51487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(51487n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 51487n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2650911169n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (51487, 51487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(51487n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 51487n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2650911169n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (51487, 51487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(51487n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 51487n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2650911169n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (25505, 54652)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(54652n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(25505n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1393899260n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (51487, 51487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(51487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(51487n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2650911169n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (51487, 51487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(51487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(51487n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2650911169n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (51487, 51487)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(51487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(51487n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2650911169n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (3513811526, 1271314557)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3513811526n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      1271314557n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (2764419392, 2764419396)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2764419392n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      2764419396n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (2764419396, 2764419396)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2764419396n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      2764419396n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (2764419396, 2764419392)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2764419396n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      2764419392n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (1526787558, 419815126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1526787558n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      419815126n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(267342180n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (681395771, 681395775)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(681395771n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      681395775n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(681395771n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (681395775, 681395775)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(681395775n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      681395775n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (681395775, 681395771)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(681395775n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      681395771n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 1 (1969251182, 1516664286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1969251182n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      1516664286n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1348493646n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 2 (1969251178, 1969251182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1969251178n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      1969251182n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1969251178n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 3 (1969251182, 1969251182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1969251182n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      1969251182n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1969251182n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 4 (1969251182, 1969251178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1969251182n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      1969251178n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1969251178n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 1 (3115707460, 1516664286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1516664286n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      3115707460n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(405038148n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 2 (1969251178, 1969251182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1969251182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      1969251178n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1969251178n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 3 (1969251182, 1969251182)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1969251182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      1969251182n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1969251182n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 4 (1969251182, 1969251178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1969251178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      1969251182n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1969251178n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 1 (1112287315, 1328423067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1112287315n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      1328423067n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1332619483n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 2 (1079582004, 1079582008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1079582004n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      1079582008n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1079582012n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 3 (1079582008, 1079582008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1079582008n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      1079582008n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1079582008n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 4 (1079582008, 1079582004)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1079582008n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      1079582004n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1079582012n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 1 (582263388, 1328423067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1328423067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      582263388n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1874764511n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 2 (1079582004, 1079582008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1079582008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      1079582004n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1079582012n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 3 (1079582008, 1079582008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1079582008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      1079582008n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1079582008n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 4 (1079582008, 1079582004)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1079582004n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      1079582008n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1079582012n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 1 (3137527105, 244917414)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3137527105n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      244917414n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3046898151n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 2 (3137527101, 3137527105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3137527101n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      3137527105n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 3 (3137527105, 3137527105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3137527105n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      3137527105n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 4 (3137527105, 3137527101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3137527105n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      3137527101n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 1 (1270088516, 244917414)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(244917414n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      1270088516n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1160587234n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 2 (3137527101, 3137527105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3137527105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      3137527101n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 3 (3137527105, 3137527105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3137527105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      3137527105n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 4 (3137527105, 3137527101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3137527101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      3137527105n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(124n);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (4198345697, 4142683001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4198345697n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      4142683001n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (2959962289, 2959962293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2959962289n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      2959962293n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (2959962293, 2959962293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2959962293n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      2959962293n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (2959962293, 2959962289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2959962293n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      2959962289n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (3750326578, 4142683001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(4142683001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      3750326578n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (2959962289, 2959962293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2959962293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      2959962289n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (2959962293, 2959962293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2959962293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      2959962293n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (2959962293, 2959962289)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2959962289n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      2959962293n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (4142346425, 3572762408)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4142346425n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      3572762408n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (3056969935, 3056969939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3056969935n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      3056969939n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (3056969939, 3056969939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3056969939n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      3056969939n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (3056969939, 3056969935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3056969939n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      3056969935n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (3036247468, 3572762408)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3572762408n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      3036247468n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (3056969935, 3056969939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3056969939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      3056969935n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (3056969939, 3056969939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3056969939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      3056969939n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (3056969939, 3056969935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3056969935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      3056969939n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (3417794908, 3182586799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3417794908n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      3182586799n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (2711241119, 2711241123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2711241119n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2711241123n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (2711241123, 2711241123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2711241123n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2711241123n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (2711241123, 2711241119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2711241123n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2711241119n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (805087336, 3182586799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3182586799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      805087336n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (2711241119, 2711241123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2711241123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      2711241119n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (2711241123, 2711241123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2711241123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      2711241123n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (2711241123, 2711241119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2711241119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      2711241123n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (1992276575, 28198471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1992276575n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      28198471n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (1415326397, 1415326401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1415326397n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      1415326401n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (1415326401, 1415326401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1415326401n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      1415326401n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (1415326401, 1415326397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1415326401n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      1415326397n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (2891326451, 28198471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(28198471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      2891326451n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (1415326397, 1415326401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1415326401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      1415326397n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (1415326401, 1415326401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1415326401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      1415326401n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (1415326401, 1415326397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1415326397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      1415326401n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (2411753996, 1462160354)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2411753996n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      1462160354n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (2411753992, 2411753996)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2411753992n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      2411753996n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (2411753996, 2411753996)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2411753996n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      2411753996n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (2411753996, 2411753992)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2411753996n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      2411753992n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (165950222, 1462160354)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1462160354n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      165950222n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (2411753992, 2411753996)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2411753996n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      2411753992n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (2411753996, 2411753996)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2411753996n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      2411753996n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (2411753996, 2411753992)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2411753992n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      2411753996n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (1147819287, 3958232313)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1147819287n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      3958232313n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (1147819283, 1147819287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1147819283n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      1147819287n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (1147819287, 1147819287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1147819287n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      1147819287n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (1147819287, 1147819283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1147819287n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      1147819283n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (962385412, 3958232313)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3958232313n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      962385412n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (1147819283, 1147819287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1147819287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      1147819283n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (1147819287, 1147819287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1147819287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      1147819287n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (1147819287, 1147819283)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1147819283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      1147819287n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (1318813997, 4035250291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1318813997n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      4035250291n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1318813997n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (1318813993, 1318813997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1318813993n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      1318813997n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1318813993n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (1318813997, 1318813997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1318813997n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      1318813997n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1318813997n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (1318813997, 1318813993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1318813997n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      1318813993n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1318813993n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (2778879909, 4035250291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(4035250291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      2778879909n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2778879909n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (1318813993, 1318813997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1318813997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      1318813993n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1318813993n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (1318813997, 1318813997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1318813997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      1318813997n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1318813997n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (1318813997, 1318813993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1318813993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      1318813997n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1318813993n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (2603395582, 3829326395)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2603395582n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      3829326395n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3829326395n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (1657657819, 1657657823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1657657819n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1657657823n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1657657823n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (1657657823, 1657657823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1657657823n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1657657823n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1657657823n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (1657657823, 1657657819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1657657823n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1657657819n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1657657823n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (699652447, 3829326395)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3829326395n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      699652447n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3829326395n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (1657657819, 1657657823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1657657823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1657657819n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1657657823n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (1657657823, 1657657823)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1657657823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1657657823n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1657657823n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (1657657823, 1657657819)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1657657819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1657657823n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1657657823n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (9221329629035408200, 9221462931645789658)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9221329629035408200n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9221462931645789658n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18442792560681197858n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (9220732250900834195, 9220732250900834197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9220732250900834195n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9220732250900834197n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441464501801668392n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (9220732250900834197, 9220732250900834197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9220732250900834197n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9220732250900834197n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441464501801668394n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (9220732250900834197, 9220732250900834195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9220732250900834197n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9220732250900834195n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441464501801668392n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (9222538123408444196, 9221462931645789658)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9221462931645789658n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9222538123408444196n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18444001055054233854n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (9220732250900834195, 9220732250900834197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9220732250900834197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9220732250900834195n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441464501801668392n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (9220732250900834197, 9220732250900834197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9220732250900834197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9220732250900834197n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441464501801668394n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (9220732250900834197, 9220732250900834195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9220732250900834195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9220732250900834197n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441464501801668392n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (18440682051650300723, 18440682051650300723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440682051650300723n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18440682051650300723n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (18440682051650300723, 18440682051650300719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440682051650300723n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18440682051650300719n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (18440682051650300723, 18440682051650300723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440682051650300723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint64_euint64(
      18440682051650300723n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (18440682051650300723, 18440682051650300719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440682051650300719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint64_euint64(
      18440682051650300723n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (4294324555, 4293638467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294324555n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4293638467n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438277099130657185n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (4294324555, 4294324555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294324555n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294324555n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441223383675948025n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (4294324555, 4294324555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294324555n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294324555n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441223383675948025n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (4294324555, 4294324555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294324555n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294324555n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441223383675948025n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (4294389056, 4293638467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4293638467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4294389056n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438554043105417152n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (4294324555, 4294324555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4294324555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4294324555n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441223383675948025n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (4294324555, 4294324555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4294324555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4294324555n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441223383675948025n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (4294324555, 4294324555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4294324555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4294324555n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441223383675948025n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (18439362183723289927, 18442502311148478203)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439362183723289927n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18442502311148478203n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (18439362183723289923, 18439362183723289927)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439362183723289923n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439362183723289927n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (18439362183723289927, 18439362183723289927)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439362183723289927n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439362183723289927n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (18439362183723289927, 18439362183723289923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439362183723289927n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439362183723289923n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (18446689311553280681, 18444222891586080031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18446689311553280681n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18444222891586080031n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(2466419967200650n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (18441477558808698133, 18441477558808698137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441477558808698133n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18441477558808698137n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441477558808698133n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (18441477558808698137, 18441477558808698137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441477558808698137n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18441477558808698137n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (18441477558808698137, 18441477558808698133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441477558808698137n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18441477558808698133n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 1 (18443089347779807029, 18440141252866260399)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18443089347779807029n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18440141252866260399n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18437736877828375845n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 2 (18439708968362228715, 18439708968362228719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439708968362228715n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18439708968362228719n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18439708968362228715n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 3 (18439708968362228719, 18439708968362228719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439708968362228719n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18439708968362228719n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18439708968362228719n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 4 (18439708968362228719, 18439708968362228715)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439708968362228719n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18439708968362228715n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18439708968362228715n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 1 (18440310230104299939, 18440141252866260399)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440141252866260399n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18440310230104299939n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18439988966479239587n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 2 (18439708968362228715, 18439708968362228719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439708968362228719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18439708968362228715n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18439708968362228715n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 3 (18439708968362228719, 18439708968362228719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439708968362228719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18439708968362228719n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18439708968362228719n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 4 (18439708968362228719, 18439708968362228715)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439708968362228715n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18439708968362228719n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18439708968362228715n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 1 (18441344925300235447, 18445160024641436275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441344925300235447n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18445160024641436275n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18446427211926732535n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 2 (18441344925300235443, 18441344925300235447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441344925300235443n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18441344925300235447n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441344925300235447n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 3 (18441344925300235447, 18441344925300235447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441344925300235447n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18441344925300235447n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441344925300235447n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 4 (18441344925300235447, 18441344925300235443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441344925300235447n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18441344925300235443n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441344925300235447n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 1 (18443400465902333487, 18445160024641436275)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18445160024641436275n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18443400465902333487n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18446285929380183679n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 2 (18441344925300235443, 18441344925300235447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441344925300235447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18441344925300235443n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441344925300235447n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 3 (18441344925300235447, 18441344925300235447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441344925300235447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18441344925300235447n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441344925300235447n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 4 (18441344925300235447, 18441344925300235443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441344925300235443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18441344925300235447n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441344925300235447n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 1 (18446152010884387051, 18444580408028526127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18446152010884387051n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18444580408028526127n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(1607133581043396n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 2 (18441220256721527769, 18441220256721527773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441220256721527769n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441220256721527773n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 3 (18441220256721527773, 18441220256721527773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441220256721527773n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441220256721527773n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 4 (18441220256721527773, 18441220256721527769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441220256721527773n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441220256721527769n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 1 (18437975819219557253, 18444580408028526127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444580408028526127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18437975819219557253n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(6906556427382186n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 2 (18441220256721527769, 18441220256721527773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441220256721527773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18441220256721527769n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 3 (18441220256721527773, 18441220256721527773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441220256721527773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18441220256721527773n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 4 (18441220256721527773, 18441220256721527769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441220256721527769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18441220256721527773n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (18441937730309383741, 18439887869534896947)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441937730309383741n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18439887869534896947n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (18438190080728092155, 18438190080728092159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438190080728092155n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18438190080728092159n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (18438190080728092159, 18438190080728092159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438190080728092159n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18438190080728092159n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (18438190080728092159, 18438190080728092155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438190080728092159n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18438190080728092155n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (18438529685854017563, 18439887869534896947)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439887869534896947n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18438529685854017563n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (18438190080728092155, 18438190080728092159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438190080728092159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18438190080728092155n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (18438190080728092159, 18438190080728092159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438190080728092159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18438190080728092159n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (18438190080728092159, 18438190080728092155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438190080728092155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18438190080728092159n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (18442258317735800351, 18444515365680921153)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442258317735800351n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18444515365680921153n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (18442258317735800347, 18442258317735800351)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442258317735800347n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18442258317735800351n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (18442258317735800351, 18442258317735800351)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442258317735800351n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18442258317735800351n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (18442258317735800351, 18442258317735800347)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442258317735800351n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18442258317735800347n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (18440199149291543863, 18444515365680921153)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444515365680921153n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18440199149291543863n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (18442258317735800347, 18442258317735800351)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18442258317735800351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18442258317735800347n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (18442258317735800351, 18442258317735800351)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18442258317735800351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18442258317735800351n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (18442258317735800351, 18442258317735800347)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18442258317735800347n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18442258317735800351n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (18442691876989499033, 18444488477206491281)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442691876989499033n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18444488477206491281n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (18439901620554113167, 18439901620554113171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439901620554113167n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18439901620554113171n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (18439901620554113171, 18439901620554113171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439901620554113171n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18439901620554113171n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (18439901620554113171, 18439901620554113167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439901620554113171n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18439901620554113167n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (18438092024799211009, 18444488477206491281)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18444488477206491281n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18438092024799211009n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (18439901620554113167, 18439901620554113171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439901620554113171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18439901620554113167n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (18439901620554113171, 18439901620554113171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439901620554113171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18439901620554113171n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (18439901620554113171, 18439901620554113167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439901620554113167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18439901620554113171n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (18441303612401362499, 18440242668042314931)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441303612401362499n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18440242668042314931n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (18437763367601732737, 18437763367601732741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437763367601732737n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18437763367601732741n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (18437763367601732741, 18437763367601732741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437763367601732741n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18437763367601732741n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (18437763367601732741, 18437763367601732737)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18437763367601732741n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18437763367601732737n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (18446478505947616369, 18440242668042314931)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440242668042314931n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18446478505947616369n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (18437763367601732737, 18437763367601732741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437763367601732741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18437763367601732737n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (18437763367601732741, 18437763367601732741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437763367601732741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18437763367601732741n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (18437763367601732741, 18437763367601732737)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437763367601732737n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18437763367601732741n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (18438136005104024903, 18440443782987248255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438136005104024903n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18440443782987248255n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (18438136005104024899, 18438136005104024903)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438136005104024899n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18438136005104024903n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (18438136005104024903, 18438136005104024903)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438136005104024903n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18438136005104024903n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (18438136005104024903, 18438136005104024899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438136005104024903n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18438136005104024899n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18445373279953187667, 18440443782987248255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440443782987248255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18445373279953187667n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18438136005104024899, 18438136005104024903)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438136005104024903n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18438136005104024899n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18438136005104024903, 18438136005104024903)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438136005104024903n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18438136005104024903n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18438136005104024903, 18438136005104024899)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438136005104024899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18438136005104024903n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (18445604221905337531, 18443126090101604663)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18445604221905337531n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18443126090101604663n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (18441898882936062573, 18441898882936062577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441898882936062573n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18441898882936062577n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (18441898882936062577, 18441898882936062577)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441898882936062577n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18441898882936062577n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (18441898882936062577, 18441898882936062573)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441898882936062577n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18441898882936062573n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });
});
