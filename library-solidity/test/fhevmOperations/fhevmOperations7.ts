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

  it('test operator "min" overload (euint128, euint8) => euint128 test 1 (340282366920938463463368749792204418053, 196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368749792204418053n);
    input.add8(196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(196n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 2 (192, 196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(192n);
    input.add8(196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(192n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 3 (196, 196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(196n);
    input.add8(196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(196n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 4 (196, 192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(196n);
    input.add8(192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(192n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 1 (340282366920938463463369433024950062731, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369433024950062731n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463369433024950062731n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 2 (11, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(11n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(15n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 3 (15, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(15n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(15n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 4 (15, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(15n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(15n);
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

  it('test operator "add" overload (euint128, euint16) => euint128 test 2 (21906, 21910)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(21906n);
    input.add16(21910n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(43816n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 3 (21910, 21910)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(21910n);
    input.add16(21910n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(43820n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 4 (21910, 21906)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(21910n);
    input.add16(21906n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(43816n);
  });

  it('test operator "sub" overload (euint128, euint16) => euint128 test 1 (2449, 2449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2449n);
    input.add16(2449n);
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

  it('test operator "sub" overload (euint128, euint16) => euint128 test 2 (2449, 2445)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2449n);
    input.add16(2445n);
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

  it('test operator "mul" overload (euint128, euint16) => euint128 test 2 (189, 189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(189n);
    input.add16(189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(35721n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 3 (189, 189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(189n);
    input.add16(189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(35721n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 4 (189, 189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(189n);
    input.add16(189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(35721n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 1 (340282366920938463463365821344904800295, 18717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463365821344904800295n);
    input.add16(18717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18437n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 2 (18713, 18717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18713n);
    input.add16(18717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18713n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 3 (18717, 18717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18717n);
    input.add16(18717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18717n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 4 (18717, 18713)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18717n);
    input.add16(18713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18713n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 1 (340282366920938463463373499970731382127, 14084)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373499970731382127n);
    input.add16(14084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463373499970731390831n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 2 (14080, 14084)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(14080n);
    input.add16(14084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(14084n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 3 (14084, 14084)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(14084n);
    input.add16(14084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(14084n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 4 (14084, 14080)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(14084n);
    input.add16(14080n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(14084n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 1 (340282366920938463463367565775351644241, 37404)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367565775351644241n);
    input.add16(37404n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463367565775351681613n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 2 (37400, 37404)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(37400n);
    input.add16(37404n);
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

  it('test operator "xor" overload (euint128, euint16) => euint128 test 3 (37404, 37404)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(37404n);
    input.add16(37404n);
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

  it('test operator "xor" overload (euint128, euint16) => euint128 test 4 (37404, 37400)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(37404n);
    input.add16(37400n);
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

  it('test operator "eq" overload (euint128, euint16) => ebool test 1 (340282366920938463463371270378667624327, 5595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371270378667624327n);
    input.add16(5595n);
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

  it('test operator "eq" overload (euint128, euint16) => ebool test 2 (5591, 5595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(5591n);
    input.add16(5595n);
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

  it('test operator "eq" overload (euint128, euint16) => ebool test 3 (5595, 5595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(5595n);
    input.add16(5595n);
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

  it('test operator "eq" overload (euint128, euint16) => ebool test 4 (5595, 5591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(5595n);
    input.add16(5591n);
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

  it('test operator "ne" overload (euint128, euint16) => ebool test 1 (340282366920938463463370878371788870531, 24449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370878371788870531n);
    input.add16(24449n);
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

  it('test operator "ne" overload (euint128, euint16) => ebool test 2 (24445, 24449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(24445n);
    input.add16(24449n);
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

  it('test operator "ne" overload (euint128, euint16) => ebool test 3 (24449, 24449)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(24449n);
    input.add16(24449n);
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

  it('test operator "ne" overload (euint128, euint16) => ebool test 4 (24449, 24445)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(24449n);
    input.add16(24445n);
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

  it('test operator "ge" overload (euint128, euint16) => ebool test 1 (340282366920938463463372273490454653341, 34502)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372273490454653341n);
    input.add16(34502n);
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

  it('test operator "ge" overload (euint128, euint16) => ebool test 2 (34498, 34502)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(34498n);
    input.add16(34502n);
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

  it('test operator "ge" overload (euint128, euint16) => ebool test 3 (34502, 34502)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(34502n);
    input.add16(34502n);
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

  it('test operator "ge" overload (euint128, euint16) => ebool test 4 (34502, 34498)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(34502n);
    input.add16(34498n);
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

  it('test operator "gt" overload (euint128, euint16) => ebool test 1 (340282366920938463463368528880392332407, 29460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368528880392332407n);
    input.add16(29460n);
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

  it('test operator "gt" overload (euint128, euint16) => ebool test 2 (29456, 29460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(29456n);
    input.add16(29460n);
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

  it('test operator "gt" overload (euint128, euint16) => ebool test 3 (29460, 29460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(29460n);
    input.add16(29460n);
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

  it('test operator "gt" overload (euint128, euint16) => ebool test 4 (29460, 29456)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(29460n);
    input.add16(29456n);
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

  it('test operator "le" overload (euint128, euint16) => ebool test 1 (340282366920938463463368731341291730543, 26155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368731341291730543n);
    input.add16(26155n);
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

  it('test operator "le" overload (euint128, euint16) => ebool test 2 (26151, 26155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(26151n);
    input.add16(26155n);
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

  it('test operator "le" overload (euint128, euint16) => ebool test 3 (26155, 26155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(26155n);
    input.add16(26155n);
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

  it('test operator "le" overload (euint128, euint16) => ebool test 4 (26155, 26151)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(26155n);
    input.add16(26151n);
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

  it('test operator "lt" overload (euint128, euint16) => ebool test 1 (340282366920938463463374056393427354435, 23833)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374056393427354435n);
    input.add16(23833n);
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

  it('test operator "lt" overload (euint128, euint16) => ebool test 2 (23829, 23833)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(23829n);
    input.add16(23833n);
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

  it('test operator "lt" overload (euint128, euint16) => ebool test 3 (23833, 23833)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(23833n);
    input.add16(23833n);
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

  it('test operator "lt" overload (euint128, euint16) => ebool test 4 (23833, 23829)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(23833n);
    input.add16(23829n);
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

  it('test operator "min" overload (euint128, euint16) => euint128 test 1 (340282366920938463463370414442120328805, 24548)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370414442120328805n);
    input.add16(24548n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(24548n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 2 (24544, 24548)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(24544n);
    input.add16(24548n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(24544n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 3 (24548, 24548)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(24548n);
    input.add16(24548n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(24548n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 4 (24548, 24544)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(24548n);
    input.add16(24544n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(24544n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 1 (340282366920938463463369927824485556667, 3479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369927824485556667n);
    input.add16(3479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463369927824485556667n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 2 (3475, 3479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3475n);
    input.add16(3479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3479n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 3 (3479, 3479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3479n);
    input.add16(3479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3479n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 4 (3479, 3475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3479n);
    input.add16(3475n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3479n);
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

  it('test operator "add" overload (euint128, euint32) => euint128 test 2 (44467341, 44467345)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(44467341n);
    input.add32(44467345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(88934686n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 3 (44467345, 44467345)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(44467345n);
    input.add32(44467345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(88934690n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 4 (44467345, 44467341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(44467345n);
    input.add32(44467341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(88934686n);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 1 (908256240, 908256240)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(908256240n);
    input.add32(908256240n);
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

  it('test operator "sub" overload (euint128, euint32) => euint128 test 2 (908256240, 908256236)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(908256240n);
    input.add32(908256236n);
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

  it('test operator "mul" overload (euint128, euint32) => euint128 test 2 (33083, 33083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(33083n);
    input.add32(33083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1094484889n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 3 (33083, 33083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(33083n);
    input.add32(33083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1094484889n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 4 (33083, 33083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(33083n);
    input.add32(33083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(1094484889n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 1 (340282366920938463463367809771399155669, 80445671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367809771399155669n);
    input.add32(80445671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(71434437n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 2 (80445667, 80445671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(80445667n);
    input.add32(80445671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(80445667n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 3 (80445671, 80445671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(80445671n);
    input.add32(80445671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(80445671n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 4 (80445671, 80445667)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(80445671n);
    input.add32(80445667n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(80445667n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 1 (340282366920938463463370933462251100067, 3429250778)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370933462251100067n);
    input.add32(3429250778n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370933464606341115n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 2 (3429250774, 3429250778)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3429250774n);
    input.add32(3429250778n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3429250782n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 3 (3429250778, 3429250778)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3429250778n);
    input.add32(3429250778n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3429250778n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 4 (3429250778, 3429250774)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3429250778n);
    input.add32(3429250774n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(3429250782n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 1 (340282366920938463463372457247018415433, 1530849304)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372457247018415433n);
    input.add32(1530849304n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463372457247709306193n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 2 (1530849300, 1530849304)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1530849300n);
    input.add32(1530849304n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 3 (1530849304, 1530849304)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1530849304n);
    input.add32(1530849304n);
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

  it('test operator "xor" overload (euint128, euint32) => euint128 test 4 (1530849304, 1530849300)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1530849304n);
    input.add32(1530849300n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 1 (340282366920938463463370149725165328815, 3320821411)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370149725165328815n);
    input.add32(3320821411n);
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

  it('test operator "eq" overload (euint128, euint32) => ebool test 2 (3320821407, 3320821411)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3320821407n);
    input.add32(3320821411n);
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

  it('test operator "eq" overload (euint128, euint32) => ebool test 3 (3320821411, 3320821411)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3320821411n);
    input.add32(3320821411n);
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

  it('test operator "eq" overload (euint128, euint32) => ebool test 4 (3320821411, 3320821407)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3320821411n);
    input.add32(3320821407n);
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

  it('test operator "ne" overload (euint128, euint32) => ebool test 1 (340282366920938463463372089959714153057, 3362521183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372089959714153057n);
    input.add32(3362521183n);
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

  it('test operator "ne" overload (euint128, euint32) => ebool test 2 (3362521179, 3362521183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3362521179n);
    input.add32(3362521183n);
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

  it('test operator "ne" overload (euint128, euint32) => ebool test 3 (3362521183, 3362521183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3362521183n);
    input.add32(3362521183n);
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

  it('test operator "ne" overload (euint128, euint32) => ebool test 4 (3362521183, 3362521179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3362521183n);
    input.add32(3362521179n);
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

  it('test operator "ge" overload (euint128, euint32) => ebool test 1 (340282366920938463463373459680024309985, 1434066458)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373459680024309985n);
    input.add32(1434066458n);
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

  it('test operator "ge" overload (euint128, euint32) => ebool test 2 (1434066454, 1434066458)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1434066454n);
    input.add32(1434066458n);
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

  it('test operator "ge" overload (euint128, euint32) => ebool test 3 (1434066458, 1434066458)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1434066458n);
    input.add32(1434066458n);
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

  it('test operator "ge" overload (euint128, euint32) => ebool test 4 (1434066458, 1434066454)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1434066458n);
    input.add32(1434066454n);
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

  it('test operator "gt" overload (euint128, euint32) => ebool test 1 (340282366920938463463368552183349007511, 1319197405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368552183349007511n);
    input.add32(1319197405n);
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

  it('test operator "gt" overload (euint128, euint32) => ebool test 2 (1319197401, 1319197405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1319197401n);
    input.add32(1319197405n);
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

  it('test operator "gt" overload (euint128, euint32) => ebool test 3 (1319197405, 1319197405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1319197405n);
    input.add32(1319197405n);
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

  it('test operator "gt" overload (euint128, euint32) => ebool test 4 (1319197405, 1319197401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1319197405n);
    input.add32(1319197401n);
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

  it('test operator "le" overload (euint128, euint32) => ebool test 1 (340282366920938463463374320003736177683, 651017583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374320003736177683n);
    input.add32(651017583n);
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

  it('test operator "le" overload (euint128, euint32) => ebool test 2 (651017579, 651017583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(651017579n);
    input.add32(651017583n);
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

  it('test operator "le" overload (euint128, euint32) => ebool test 3 (651017583, 651017583)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(651017583n);
    input.add32(651017583n);
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

  it('test operator "le" overload (euint128, euint32) => ebool test 4 (651017583, 651017579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(651017583n);
    input.add32(651017579n);
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

  it('test operator "lt" overload (euint128, euint32) => ebool test 1 (340282366920938463463365982476715735411, 500186261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463365982476715735411n);
    input.add32(500186261n);
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

  it('test operator "lt" overload (euint128, euint32) => ebool test 2 (500186257, 500186261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(500186257n);
    input.add32(500186261n);
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

  it('test operator "lt" overload (euint128, euint32) => ebool test 3 (500186261, 500186261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(500186261n);
    input.add32(500186261n);
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

  it('test operator "lt" overload (euint128, euint32) => ebool test 4 (500186261, 500186257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(500186261n);
    input.add32(500186257n);
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

  it('test operator "min" overload (euint128, euint32) => euint128 test 1 (340282366920938463463371680362683849941, 422044629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371680362683849941n);
    input.add32(422044629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(422044629n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 2 (422044625, 422044629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(422044625n);
    input.add32(422044629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(422044625n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 3 (422044629, 422044629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(422044629n);
    input.add32(422044629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(422044629n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 4 (422044629, 422044625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(422044629n);
    input.add32(422044625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(422044625n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 1 (340282366920938463463371510056848163261, 4058704567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371510056848163261n);
    input.add32(4058704567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371510056848163261n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 2 (4058704563, 4058704567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4058704563n);
    input.add32(4058704567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4058704567n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 3 (4058704567, 4058704567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4058704567n);
    input.add32(4058704567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4058704567n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 4 (4058704567, 4058704563)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4058704567n);
    input.add32(4058704563n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4058704567n);
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

  it('test operator "add" overload (euint128, euint64) => euint128 test 2 (9221889519951262285, 9221889519951262287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9221889519951262285n);
    input.add64(9221889519951262287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443779039902524572n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 3 (9221889519951262287, 9221889519951262287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9221889519951262287n);
    input.add64(9221889519951262287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443779039902524574n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 4 (9221889519951262287, 9221889519951262285)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9221889519951262287n);
    input.add64(9221889519951262285n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443779039902524572n);
  });

  it('test operator "sub" overload (euint128, euint64) => euint128 test 1 (18438765302878833987, 18438765302878833987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438765302878833987n);
    input.add64(18438765302878833987n);
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

  it('test operator "sub" overload (euint128, euint64) => euint128 test 2 (18438765302878833987, 18438765302878833983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438765302878833987n);
    input.add64(18438765302878833983n);
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

  it('test operator "mul" overload (euint128, euint64) => euint128 test 2 (4294227882, 4294227882)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294227882n);
    input.add64(4294227882n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440393102546205924n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 3 (4294227882, 4294227882)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294227882n);
    input.add64(4294227882n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440393102546205924n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 4 (4294227882, 4294227882)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294227882n);
    input.add64(4294227882n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18440393102546205924n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 1 (340282366920938463463365956259734248409, 18441699040297115341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463365956259734248409n);
    input.add64(18441699040297115341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18437740722946988745n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 2 (18441699040297115337, 18441699040297115341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441699040297115337n);
    input.add64(18441699040297115341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18441699040297115337n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 3 (18441699040297115341, 18441699040297115341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441699040297115341n);
    input.add64(18441699040297115341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18441699040297115341n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 4 (18441699040297115341, 18441699040297115337)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441699040297115341n);
    input.add64(18441699040297115337n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18441699040297115337n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 1 (340282366920938463463368624340520157807, 18438666460550112459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368624340520157807n);
    input.add64(18438666460550112459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463368977370197255919n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 2 (18438666460550112455, 18438666460550112459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438666460550112455n);
    input.add64(18438666460550112459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438666460550112463n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 3 (18438666460550112459, 18438666460550112459)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438666460550112459n);
    input.add64(18438666460550112459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438666460550112459n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 4 (18438666460550112459, 18438666460550112455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18438666460550112459n);
    input.add64(18438666460550112455n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18438666460550112463n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 1 (340282366920938463463374131549843300149, 18441071673858865637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374131549843300149n);
    input.add64(18441071673858865637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463444933941232432391888n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 2 (18441071673858865633, 18441071673858865637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441071673858865633n);
    input.add64(18441071673858865637n);
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

  it('test operator "xor" overload (euint128, euint64) => euint128 test 3 (18441071673858865637, 18441071673858865637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441071673858865637n);
    input.add64(18441071673858865637n);
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

  it('test operator "xor" overload (euint128, euint64) => euint128 test 4 (18441071673858865637, 18441071673858865633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441071673858865637n);
    input.add64(18441071673858865633n);
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

  it('test operator "eq" overload (euint128, euint64) => ebool test 1 (340282366920938463463373651057040999159, 18440354892621386093)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373651057040999159n);
    input.add64(18440354892621386093n);
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

  it('test operator "eq" overload (euint128, euint64) => ebool test 2 (18440354892621386089, 18440354892621386093)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440354892621386089n);
    input.add64(18440354892621386093n);
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

  it('test operator "eq" overload (euint128, euint64) => ebool test 3 (18440354892621386093, 18440354892621386093)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440354892621386093n);
    input.add64(18440354892621386093n);
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

  it('test operator "eq" overload (euint128, euint64) => ebool test 4 (18440354892621386093, 18440354892621386089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440354892621386093n);
    input.add64(18440354892621386089n);
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

  it('test operator "ne" overload (euint128, euint64) => ebool test 1 (340282366920938463463366057353922793235, 18441659477397293923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366057353922793235n);
    input.add64(18441659477397293923n);
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

  it('test operator "ne" overload (euint128, euint64) => ebool test 2 (18441659477397293919, 18441659477397293923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441659477397293919n);
    input.add64(18441659477397293923n);
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

  it('test operator "ne" overload (euint128, euint64) => ebool test 3 (18441659477397293923, 18441659477397293923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441659477397293923n);
    input.add64(18441659477397293923n);
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

  it('test operator "ne" overload (euint128, euint64) => ebool test 4 (18441659477397293923, 18441659477397293919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441659477397293923n);
    input.add64(18441659477397293919n);
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

  it('test operator "ge" overload (euint128, euint64) => ebool test 1 (340282366920938463463371939447736902053, 18439986339814288475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371939447736902053n);
    input.add64(18439986339814288475n);
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

  it('test operator "ge" overload (euint128, euint64) => ebool test 2 (18439986339814288471, 18439986339814288475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439986339814288471n);
    input.add64(18439986339814288475n);
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

  it('test operator "ge" overload (euint128, euint64) => ebool test 3 (18439986339814288475, 18439986339814288475)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439986339814288475n);
    input.add64(18439986339814288475n);
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

  it('test operator "ge" overload (euint128, euint64) => ebool test 4 (18439986339814288475, 18439986339814288471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439986339814288475n);
    input.add64(18439986339814288471n);
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

  it('test operator "gt" overload (euint128, euint64) => ebool test 1 (340282366920938463463372759700155087561, 18446698742994908123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372759700155087561n);
    input.add64(18446698742994908123n);
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

  it('test operator "gt" overload (euint128, euint64) => ebool test 2 (18446698742994908119, 18446698742994908123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446698742994908119n);
    input.add64(18446698742994908123n);
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

  it('test operator "gt" overload (euint128, euint64) => ebool test 3 (18446698742994908123, 18446698742994908123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446698742994908123n);
    input.add64(18446698742994908123n);
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

  it('test operator "gt" overload (euint128, euint64) => ebool test 4 (18446698742994908123, 18446698742994908119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446698742994908123n);
    input.add64(18446698742994908119n);
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

  it('test operator "le" overload (euint128, euint64) => ebool test 1 (340282366920938463463372923550116566803, 18445346417459058069)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372923550116566803n);
    input.add64(18445346417459058069n);
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

  it('test operator "le" overload (euint128, euint64) => ebool test 2 (18445346417459058065, 18445346417459058069)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445346417459058065n);
    input.add64(18445346417459058069n);
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

  it('test operator "le" overload (euint128, euint64) => ebool test 3 (18445346417459058069, 18445346417459058069)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445346417459058069n);
    input.add64(18445346417459058069n);
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

  it('test operator "le" overload (euint128, euint64) => ebool test 4 (18445346417459058069, 18445346417459058065)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445346417459058069n);
    input.add64(18445346417459058065n);
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

  it('test operator "lt" overload (euint128, euint64) => ebool test 1 (340282366920938463463366858719355890949, 18441588436119377755)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366858719355890949n);
    input.add64(18441588436119377755n);
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

  it('test operator "lt" overload (euint128, euint64) => ebool test 2 (18441588436119377751, 18441588436119377755)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441588436119377751n);
    input.add64(18441588436119377755n);
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

  it('test operator "lt" overload (euint128, euint64) => ebool test 3 (18441588436119377755, 18441588436119377755)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441588436119377755n);
    input.add64(18441588436119377755n);
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

  it('test operator "lt" overload (euint128, euint64) => ebool test 4 (18441588436119377755, 18441588436119377751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441588436119377755n);
    input.add64(18441588436119377751n);
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

  it('test operator "min" overload (euint128, euint64) => euint128 test 1 (340282366920938463463368030389371024005, 18441485620924608669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368030389371024005n);
    input.add64(18441485620924608669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18441485620924608669n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 2 (18441485620924608665, 18441485620924608669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441485620924608665n);
    input.add64(18441485620924608669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18441485620924608665n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 3 (18441485620924608669, 18441485620924608669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441485620924608669n);
    input.add64(18441485620924608669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18441485620924608669n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 4 (18441485620924608669, 18441485620924608665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441485620924608669n);
    input.add64(18441485620924608665n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18441485620924608665n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 1 (340282366920938463463370357527072616433, 18443278680333991919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370357527072616433n);
    input.add64(18443278680333991919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370357527072616433n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 2 (18443278680333991915, 18443278680333991919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18443278680333991915n);
    input.add64(18443278680333991919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443278680333991919n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 3 (18443278680333991919, 18443278680333991919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18443278680333991919n);
    input.add64(18443278680333991919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443278680333991919n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 4 (18443278680333991919, 18443278680333991915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18443278680333991919n);
    input.add64(18443278680333991915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443278680333991919n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 1 (170141183460469231731683660477207909513, 170141183460469231731683131555950184095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731683660477207909513n);
    input.add128(170141183460469231731683131555950184095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366792033158093608n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 2 (170141183460469231731683131555950184093, 170141183460469231731683131555950184095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731683131555950184093n);
    input.add128(170141183460469231731683131555950184095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366263111900368188n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 3 (170141183460469231731683131555950184095, 170141183460469231731683131555950184095)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731683131555950184095n);
    input.add128(170141183460469231731683131555950184095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366263111900368190n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 4 (170141183460469231731683131555950184095, 170141183460469231731683131555950184093)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731683131555950184095n);
    input.add128(170141183460469231731683131555950184093n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366263111900368188n);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 1 (340282366920938463463371992799637379637, 340282366920938463463371992799637379637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371992799637379637n);
    input.add128(340282366920938463463371992799637379637n);
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

  it('test operator "sub" overload (euint128, euint128) => euint128 test 2 (340282366920938463463371992799637379637, 340282366920938463463371992799637379633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371992799637379637n);
    input.add128(340282366920938463463371992799637379633n);
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

  it('test operator "and" overload (euint128, euint128) => euint128 test 1 (340282366920938463463371932248882114205, 340282366920938463463371545091546498825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371932248882114205n);
    input.add128(340282366920938463463371545091546498825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371263547705476617n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 2 (340282366920938463463371545091546498821, 340282366920938463463371545091546498825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371545091546498821n);
    input.add128(340282366920938463463371545091546498825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371545091546498817n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 3 (340282366920938463463371545091546498825, 340282366920938463463371545091546498825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371545091546498825n);
    input.add128(340282366920938463463371545091546498825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371545091546498825n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 4 (340282366920938463463371545091546498825, 340282366920938463463371545091546498821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371545091546498825n);
    input.add128(340282366920938463463371545091546498821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371545091546498817n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 1 (340282366920938463463372292218849760581, 340282366920938463463369412692211173589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463372292218849760581n);
    input.add128(340282366920938463463369412692211173589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463374554086100344277n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 2 (340282366920938463463369412692211173585, 340282366920938463463369412692211173589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369412692211173585n);
    input.add128(340282366920938463463369412692211173589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369412692211173589n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 3 (340282366920938463463369412692211173589, 340282366920938463463369412692211173589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369412692211173589n);
    input.add128(340282366920938463463369412692211173589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369412692211173589n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 4 (340282366920938463463369412692211173589, 340282366920938463463369412692211173585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369412692211173589n);
    input.add128(340282366920938463463369412692211173585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463369412692211173589n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 1 (340282366920938463463374282009991941377, 340282366920938463463370992300019806545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463374282009991941377n);
    input.add128(340282366920938463463370992300019806545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(3923097666092112n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 2 (340282366920938463463370992300019806541, 340282366920938463463370992300019806545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370992300019806541n);
    input.add128(340282366920938463463370992300019806545n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 3 (340282366920938463463370992300019806545, 340282366920938463463370992300019806545)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370992300019806545n);
    input.add128(340282366920938463463370992300019806545n);
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

  it('test operator "xor" overload (euint128, euint128) => euint128 test 4 (340282366920938463463370992300019806545, 340282366920938463463370992300019806541)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463370992300019806545n);
    input.add128(340282366920938463463370992300019806541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract5.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 1 (340282366920938463463369660429237688665, 340282366920938463463371712204601477691)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369660429237688665n);
    input.add128(340282366920938463463371712204601477691n);
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

  it('test operator "eq" overload (euint128, euint128) => ebool test 2 (340282366920938463463369660429237688661, 340282366920938463463369660429237688665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369660429237688661n);
    input.add128(340282366920938463463369660429237688665n);
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

  it('test operator "eq" overload (euint128, euint128) => ebool test 3 (340282366920938463463369660429237688665, 340282366920938463463369660429237688665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369660429237688665n);
    input.add128(340282366920938463463369660429237688665n);
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

  it('test operator "eq" overload (euint128, euint128) => ebool test 4 (340282366920938463463369660429237688665, 340282366920938463463369660429237688661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369660429237688665n);
    input.add128(340282366920938463463369660429237688661n);
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

  it('test operator "ne" overload (euint128, euint128) => ebool test 1 (340282366920938463463373573707769836885, 340282366920938463463369848690637762901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373573707769836885n);
    input.add128(340282366920938463463369848690637762901n);
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

  it('test operator "ne" overload (euint128, euint128) => ebool test 2 (340282366920938463463369848690637762897, 340282366920938463463369848690637762901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369848690637762897n);
    input.add128(340282366920938463463369848690637762901n);
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

  it('test operator "ne" overload (euint128, euint128) => ebool test 3 (340282366920938463463369848690637762901, 340282366920938463463369848690637762901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369848690637762901n);
    input.add128(340282366920938463463369848690637762901n);
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

  it('test operator "ne" overload (euint128, euint128) => ebool test 4 (340282366920938463463369848690637762901, 340282366920938463463369848690637762897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369848690637762901n);
    input.add128(340282366920938463463369848690637762897n);
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
