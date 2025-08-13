import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMTestSuite1 } from '../../types/contracts/tests/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../types/contracts/tests/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../types/contracts/tests/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../types/contracts/tests/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../types/contracts/tests/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../types/contracts/tests/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../types/contracts/tests/FHEVMTestSuite7';
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

async function deployFHEVMTestFixture1(): Promise<FHEVMTestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture2(): Promise<FHEVMTestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture3(): Promise<FHEVMTestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture4(): Promise<FHEVMTestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture5(): Promise<FHEVMTestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture6(): Promise<FHEVMTestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture7(): Promise<FHEVMTestSuite7> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('FHEVM operations 7', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract1 = await deployFHEVMTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployFHEVMTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployFHEVMTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployFHEVMTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployFHEVMTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployFHEVMTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployFHEVMTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 1 (340282366920938463463373712383194766181, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373712383194766181n);
    input.add8(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(213n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 2 (209, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(209n);
    input.add8(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(209n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 3 (213, 213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(213n);
    input.add8(213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(213n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 4 (213, 209)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(213n);
    input.add8(209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(209n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 1 (340282366920938463463370515751395838767, 19)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370515751395838767n);
    input.add8(19n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370515751395838767n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 2 (15, 19)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(15n);
    input.add8(19n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(19n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 3 (19, 19)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(19n);
    input.add8(19n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(19n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 4 (19, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(19n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(19n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 1 (32769, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(32769n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 2 (28198, 28200)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(28198n);
    input.add16(28200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(56398n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 3 (28200, 28200)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(28200n);
    input.add16(28200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(56400n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 4 (28200, 28198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(28200n);
    input.add16(28198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(56398n);
  });

  it('test operator "sub" overload (euint128, euint16) => euint128 test 1 (58739, 58739)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(58739n);
    input.add16(58739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint16) => euint128 test 2 (58739, 58735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(58739n);
    input.add16(58735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 1 (16385, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(16385n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 2 (246, 246)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(246n);
    input.add16(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(60516n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 3 (246, 246)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(246n);
    input.add16(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(60516n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 4 (246, 246)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(246n);
    input.add16(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(60516n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 1 (340282366920938463463365840963366195251, 46330)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463365840963366195251n);
    input.add16(46330n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(36914n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 2 (46326, 46330)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(46326n);
    input.add16(46330n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(46322n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 3 (46330, 46330)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(46330n);
    input.add16(46330n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(46330n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 4 (46330, 46326)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(46330n);
    input.add16(46326n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(46322n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 1 (340282366920938463463370109409565596097, 19092)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370109409565596097n);
    input.add16(19092n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370109409565596629n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 2 (19088, 19092)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(19088n);
    input.add16(19092n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(19092n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 3 (19092, 19092)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(19092n);
    input.add16(19092n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(19092n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 4 (19092, 19088)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(19092n);
    input.add16(19088n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(19092n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 1 (340282366920938463463366600946328404861, 42311)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366600946328404861n);
    input.add16(42311n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366600946328446522n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 2 (42307, 42311)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(42307n);
    input.add16(42311n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 3 (42311, 42311)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(42311n);
    input.add16(42311n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 4 (42311, 42307)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(42311n);
    input.add16(42307n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 1 (340282366920938463463369603529254774375, 435)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369603529254774375n);
    input.add16(435n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 2 (431, 435)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(431n);
    input.add16(435n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 3 (435, 435)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(435n);
    input.add16(435n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 4 (435, 431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(435n);
    input.add16(431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 1 (340282366920938463463367577179995422187, 57380)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367577179995422187n);
    input.add16(57380n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 2 (57376, 57380)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(57376n);
    input.add16(57380n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 3 (57380, 57380)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(57380n);
    input.add16(57380n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 4 (57380, 57376)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(57380n);
    input.add16(57376n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 1 (340282366920938463463371405293525753631, 1511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371405293525753631n);
    input.add16(1511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 2 (1507, 1511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1507n);
    input.add16(1511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 3 (1511, 1511)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1511n);
    input.add16(1511n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 4 (1511, 1507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1511n);
    input.add16(1507n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 1 (340282366920938463463374017144193171283, 18971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374017144193171283n);
    input.add16(18971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 2 (18967, 18971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18967n);
    input.add16(18971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 3 (18971, 18971)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18971n);
    input.add16(18971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 4 (18971, 18967)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18971n);
    input.add16(18967n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 1 (340282366920938463463367268746062670043, 46576)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367268746062670043n);
    input.add16(46576n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 2 (46572, 46576)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(46572n);
    input.add16(46576n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 3 (46576, 46576)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(46576n);
    input.add16(46576n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 4 (46576, 46572)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(46576n);
    input.add16(46572n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 1 (340282366920938463463367640782400264165, 49782)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367640782400264165n);
    input.add16(49782n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 2 (49778, 49782)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(49778n);
    input.add16(49782n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 3 (49782, 49782)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(49782n);
    input.add16(49782n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 4 (49782, 49778)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(49782n);
    input.add16(49778n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 1 (340282366920938463463370188253689929817, 50174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370188253689929817n);
    input.add16(50174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(50174n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 2 (50170, 50174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(50170n);
    input.add16(50174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(50170n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 3 (50174, 50174)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(50174n);
    input.add16(50174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(50174n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 4 (50174, 50170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(50174n);
    input.add16(50170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(50170n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 1 (340282366920938463463370333225401477095, 49965)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370333225401477095n);
    input.add16(49965n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370333225401477095n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 2 (49961, 49965)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(49961n);
    input.add16(49965n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(49965n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 3 (49965, 49965)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(49965n);
    input.add16(49965n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(49965n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 4 (49965, 49961)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(49965n);
    input.add16(49961n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(49965n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 1 (2147483649, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2147483649n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 2 (1926780905, 1926780907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1926780905n);
    input.add32(1926780907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3853561812n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 3 (1926780907, 1926780907)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1926780907n);
    input.add32(1926780907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3853561814n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 4 (1926780907, 1926780905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1926780907n);
    input.add32(1926780905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3853561812n);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 1 (4051927688, 4051927688)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4051927688n);
    input.add32(4051927688n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 2 (4051927688, 4051927684)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4051927688n);
    input.add32(4051927684n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 1 (1073741825, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1073741825n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 2 (50628, 50628)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(50628n);
    input.add32(50628n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2563194384n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 3 (50628, 50628)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(50628n);
    input.add32(50628n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2563194384n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 4 (50628, 50628)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(50628n);
    input.add32(50628n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2563194384n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 1 (340282366920938463463367210869542499059, 3798359286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367210869542499059n);
    input.add32(3798359286n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2724335858n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 2 (3798359282, 3798359286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3798359282n);
    input.add32(3798359286n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3798359282n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 3 (3798359286, 3798359286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3798359286n);
    input.add32(3798359286n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3798359286n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 4 (3798359286, 3798359282)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3798359286n);
    input.add32(3798359282n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3798359282n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 1 (340282366920938463463370318636661625815, 65246623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370318636661625815n);
    input.add32(65246623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370318636674439135n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 2 (65246619, 65246623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(65246619n);
    input.add32(65246623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(65246623n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 3 (65246623, 65246623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(65246623n);
    input.add32(65246623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(65246623n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 4 (65246623, 65246619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(65246623n);
    input.add32(65246619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(65246623n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 1 (340282366920938463463371111118093528875, 1586645711)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371111118093528875n);
    input.add32(1586645711n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371111119411733988n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 2 (1586645707, 1586645711)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1586645707n);
    input.add32(1586645711n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 3 (1586645711, 1586645711)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1586645711n);
    input.add32(1586645711n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 4 (1586645711, 1586645707)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1586645711n);
    input.add32(1586645707n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 1 (340282366920938463463366554881786977911, 2025082564)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366554881786977911n);
    input.add32(2025082564n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 2 (2025082560, 2025082564)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2025082560n);
    input.add32(2025082564n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 3 (2025082564, 2025082564)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2025082564n);
    input.add32(2025082564n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 4 (2025082564, 2025082560)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2025082564n);
    input.add32(2025082560n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 1 (340282366920938463463373777598119117621, 310808118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373777598119117621n);
    input.add32(310808118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 2 (310808114, 310808118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(310808114n);
    input.add32(310808118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 3 (310808118, 310808118)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(310808118n);
    input.add32(310808118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 4 (310808118, 310808114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(310808118n);
    input.add32(310808114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 1 (340282366920938463463374338464110609847, 4226996236)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374338464110609847n);
    input.add32(4226996236n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 2 (4226996232, 4226996236)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4226996232n);
    input.add32(4226996236n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 3 (4226996236, 4226996236)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4226996236n);
    input.add32(4226996236n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 4 (4226996236, 4226996232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4226996236n);
    input.add32(4226996232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 1 (340282366920938463463369949945469637645, 1429700809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369949945469637645n);
    input.add32(1429700809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 2 (1429700805, 1429700809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1429700805n);
    input.add32(1429700809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 3 (1429700809, 1429700809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1429700809n);
    input.add32(1429700809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 4 (1429700809, 1429700805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1429700809n);
    input.add32(1429700805n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 1 (340282366920938463463369132451194074141, 1463506190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369132451194074141n);
    input.add32(1463506190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 2 (1463506186, 1463506190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1463506186n);
    input.add32(1463506190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 3 (1463506190, 1463506190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1463506190n);
    input.add32(1463506190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 4 (1463506190, 1463506186)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1463506190n);
    input.add32(1463506186n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 1 (340282366920938463463366241687933087833, 1690386793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366241687933087833n);
    input.add32(1690386793n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 2 (1690386789, 1690386793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1690386789n);
    input.add32(1690386793n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 3 (1690386793, 1690386793)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1690386793n);
    input.add32(1690386793n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 4 (1690386793, 1690386789)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1690386793n);
    input.add32(1690386789n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 1 (340282366920938463463374606209574976639, 1193695991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374606209574976639n);
    input.add32(1193695991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1193695991n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 2 (1193695987, 1193695991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1193695987n);
    input.add32(1193695991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1193695987n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 3 (1193695991, 1193695991)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1193695991n);
    input.add32(1193695991n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1193695991n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 4 (1193695991, 1193695987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1193695991n);
    input.add32(1193695987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1193695987n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 1 (340282366920938463463369781199782230541, 2269916773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369781199782230541n);
    input.add32(2269916773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463369781199782230541n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 2 (2269916769, 2269916773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2269916769n);
    input.add32(2269916773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2269916773n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 3 (2269916773, 2269916773)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2269916773n);
    input.add32(2269916773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2269916773n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 4 (2269916773, 2269916769)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2269916773n);
    input.add32(2269916769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(2269916773n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 1 (9223372036854775809, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 2 (9220387769815558308, 9220387769815558310)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9220387769815558308n);
    input.add64(9220387769815558310n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440775539631116618n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 3 (9220387769815558310, 9220387769815558310)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9220387769815558310n);
    input.add64(9220387769815558310n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440775539631116620n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 4 (9220387769815558310, 9220387769815558308)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9220387769815558310n);
    input.add64(9220387769815558308n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440775539631116618n);
  });

  it('test operator "sub" overload (euint128, euint64) => euint128 test 1 (18446557582845248131, 18446557582845248131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446557582845248131n);
    input.add64(18446557582845248131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint64) => euint128 test 2 (18446557582845248131, 18446557582845248127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446557582845248131n);
    input.add64(18446557582845248127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 1 (4611686018427387905, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4611686018427387905n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 2 (4294908146, 4294908146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294908146n);
    input.add64(4294908146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446235982577157316n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 3 (4294908146, 4294908146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294908146n);
    input.add64(4294908146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446235982577157316n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 4 (4294908146, 4294908146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294908146n);
    input.add64(4294908146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446235982577157316n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 1 (340282366920938463463370028051412685409, 18438268215129351125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370028051412685409n);
    input.add64(18438268215129351125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438197571505095233n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 2 (18438268215129351121, 18438268215129351125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438268215129351121n);
    input.add64(18438268215129351125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438268215129351121n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 3 (18438268215129351125, 18438268215129351125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438268215129351125n);
    input.add64(18438268215129351125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438268215129351125n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 4 (18438268215129351125, 18438268215129351121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438268215129351125n);
    input.add64(18438268215129351121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438268215129351121n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 1 (340282366920938463463370245599355965555, 18438829389148483539)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370245599355965555n);
    input.add64(18438829389148483539n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371196677551554547n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 2 (18438829389148483535, 18438829389148483539)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438829389148483535n);
    input.add64(18438829389148483539n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438829389148483551n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 3 (18438829389148483539, 18438829389148483539)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438829389148483539n);
    input.add64(18438829389148483539n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438829389148483539n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 4 (18438829389148483539, 18438829389148483535)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438829389148483539n);
    input.add64(18438829389148483535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438829389148483551n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 1 (340282366920938463463366859875428594207, 18445481895167772599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366859875428594207n);
    input.add64(18445481895167772599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463444936868003152147880n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 2 (18445481895167772595, 18445481895167772599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445481895167772595n);
    input.add64(18445481895167772599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 3 (18445481895167772599, 18445481895167772599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445481895167772599n);
    input.add64(18445481895167772599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 4 (18445481895167772599, 18445481895167772595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445481895167772599n);
    input.add64(18445481895167772595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 1 (340282366920938463463367486014501377879, 18441426019593071011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367486014501377879n);
    input.add64(18441426019593071011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 2 (18441426019593071007, 18441426019593071011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441426019593071007n);
    input.add64(18441426019593071011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 3 (18441426019593071011, 18441426019593071011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441426019593071011n);
    input.add64(18441426019593071011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 4 (18441426019593071011, 18441426019593071007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441426019593071011n);
    input.add64(18441426019593071007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 1 (340282366920938463463367946937919051677, 18438742407150831101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367946937919051677n);
    input.add64(18438742407150831101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 2 (18438742407150831097, 18438742407150831101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438742407150831097n);
    input.add64(18438742407150831101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 3 (18438742407150831101, 18438742407150831101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438742407150831101n);
    input.add64(18438742407150831101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 4 (18438742407150831101, 18438742407150831097)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438742407150831101n);
    input.add64(18438742407150831097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 1 (340282366920938463463366965352591873483, 18439719846785093451)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366965352591873483n);
    input.add64(18439719846785093451n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 2 (18439719846785093447, 18439719846785093451)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439719846785093447n);
    input.add64(18439719846785093451n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 3 (18439719846785093451, 18439719846785093451)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439719846785093451n);
    input.add64(18439719846785093451n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 4 (18439719846785093451, 18439719846785093447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439719846785093451n);
    input.add64(18439719846785093447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 1 (340282366920938463463366689563124206111, 18442820679988408627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366689563124206111n);
    input.add64(18442820679988408627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 2 (18442820679988408623, 18442820679988408627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18442820679988408623n);
    input.add64(18442820679988408627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 3 (18442820679988408627, 18442820679988408627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18442820679988408627n);
    input.add64(18442820679988408627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 4 (18442820679988408627, 18442820679988408623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18442820679988408627n);
    input.add64(18442820679988408623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 1 (340282366920938463463369591428861408255, 18445207643589976027)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369591428861408255n);
    input.add64(18445207643589976027n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 2 (18445207643589976023, 18445207643589976027)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445207643589976023n);
    input.add64(18445207643589976027n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 3 (18445207643589976027, 18445207643589976027)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445207643589976027n);
    input.add64(18445207643589976027n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 4 (18445207643589976027, 18445207643589976023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445207643589976027n);
    input.add64(18445207643589976023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 1 (340282366920938463463372964104969673773, 18444702305784464133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372964104969673773n);
    input.add64(18444702305784464133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 2 (18444702305784464129, 18444702305784464133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18444702305784464129n);
    input.add64(18444702305784464133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 3 (18444702305784464133, 18444702305784464133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18444702305784464133n);
    input.add64(18444702305784464133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 4 (18444702305784464133, 18444702305784464129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18444702305784464133n);
    input.add64(18444702305784464129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 1 (340282366920938463463373464420071282883, 18445537200000331709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373464420071282883n);
    input.add64(18445537200000331709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18445537200000331709n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 2 (18445537200000331705, 18445537200000331709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445537200000331705n);
    input.add64(18445537200000331709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18445537200000331705n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 3 (18445537200000331709, 18445537200000331709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445537200000331709n);
    input.add64(18445537200000331709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18445537200000331709n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 4 (18445537200000331709, 18445537200000331705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445537200000331709n);
    input.add64(18445537200000331705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18445537200000331705n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 1 (340282366920938463463369333068699781755, 18444572605014144099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369333068699781755n);
    input.add64(18444572605014144099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463369333068699781755n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 2 (18444572605014144095, 18444572605014144099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18444572605014144095n);
    input.add64(18444572605014144099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18444572605014144099n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 3 (18444572605014144099, 18444572605014144099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18444572605014144099n);
    input.add64(18444572605014144099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18444572605014144099n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 4 (18444572605014144099, 18444572605014144095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18444572605014144099n);
    input.add64(18444572605014144095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18444572605014144099n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 1 (170141183460469231731686180630984747189, 170141183460469231731686680124907782207)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731686180630984747189n);
    input.add128(170141183460469231731686680124907782207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463372860755892529396n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 2 (170141183460469231731686180630984747187, 170141183460469231731686180630984747189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731686180630984747187n);
    input.add128(170141183460469231731686180630984747189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463372361261969494376n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 3 (170141183460469231731686180630984747189, 170141183460469231731686180630984747189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731686180630984747189n);
    input.add128(170141183460469231731686180630984747189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463372361261969494378n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 4 (170141183460469231731686180630984747189, 170141183460469231731686180630984747187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731686180630984747189n);
    input.add128(170141183460469231731686180630984747187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463372361261969494376n);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 1 (340282366920938463463371871844265663885, 340282366920938463463371871844265663885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371871844265663885n);
    input.add128(340282366920938463463371871844265663885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 2 (340282366920938463463371871844265663885, 340282366920938463463371871844265663881)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371871844265663885n);
    input.add128(340282366920938463463371871844265663881n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 1 (340282366920938463463372843526008590363, 340282366920938463463371220725893332965)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372843526008590363n);
    input.add128(340282366920938463463371220725893332965n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370582721116393473n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 2 (340282366920938463463371220725893332961, 340282366920938463463371220725893332965)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371220725893332961n);
    input.add128(340282366920938463463371220725893332965n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371220725893332961n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 3 (340282366920938463463371220725893332965, 340282366920938463463371220725893332965)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371220725893332965n);
    input.add128(340282366920938463463371220725893332965n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371220725893332965n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 4 (340282366920938463463371220725893332965, 340282366920938463463371220725893332961)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371220725893332965n);
    input.add128(340282366920938463463371220725893332961n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371220725893332961n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 1 (340282366920938463463366036478345547755, 340282366920938463463373182896094972297)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366036478345547755n);
    input.add128(340282366920938463463373182896094972297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463373478127931690987n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 2 (340282366920938463463366036478345547751, 340282366920938463463366036478345547755)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366036478345547751n);
    input.add128(340282366920938463463366036478345547755n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463366036478345547759n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 3 (340282366920938463463366036478345547755, 340282366920938463463366036478345547755)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366036478345547755n);
    input.add128(340282366920938463463366036478345547755n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463366036478345547755n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 4 (340282366920938463463366036478345547755, 340282366920938463463366036478345547751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463366036478345547755n);
    input.add128(340282366920938463463366036478345547751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463366036478345547759n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 1 (340282366920938463463373897122645062103, 340282366920938463463370810404340945273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373897122645062103n);
    input.add128(340282366920938463463370810404340945273n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(4498539853275310n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 2 (340282366920938463463370810404340945269, 340282366920938463463370810404340945273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370810404340945269n);
    input.add128(340282366920938463463370810404340945273n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 3 (340282366920938463463370810404340945273, 340282366920938463463370810404340945273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370810404340945273n);
    input.add128(340282366920938463463370810404340945273n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 4 (340282366920938463463370810404340945273, 340282366920938463463370810404340945269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370810404340945273n);
    input.add128(340282366920938463463370810404340945269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 1 (340282366920938463463373790323591415877, 340282366920938463463371545466002568545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373790323591415877n);
    input.add128(340282366920938463463371545466002568545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 2 (340282366920938463463371545466002568541, 340282366920938463463371545466002568545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371545466002568541n);
    input.add128(340282366920938463463371545466002568545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 3 (340282366920938463463371545466002568545, 340282366920938463463371545466002568545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371545466002568545n);
    input.add128(340282366920938463463371545466002568545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 4 (340282366920938463463371545466002568545, 340282366920938463463371545466002568541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463371545466002568545n);
    input.add128(340282366920938463463371545466002568541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 1 (340282366920938463463373314492552810485, 340282366920938463463373653994273277967)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373314492552810485n);
    input.add128(340282366920938463463373653994273277967n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 2 (340282366920938463463373314492552810481, 340282366920938463463373314492552810485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373314492552810481n);
    input.add128(340282366920938463463373314492552810485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 3 (340282366920938463463373314492552810485, 340282366920938463463373314492552810485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373314492552810485n);
    input.add128(340282366920938463463373314492552810485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 4 (340282366920938463463373314492552810485, 340282366920938463463373314492552810481)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373314492552810485n);
    input.add128(340282366920938463463373314492552810481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });
});
