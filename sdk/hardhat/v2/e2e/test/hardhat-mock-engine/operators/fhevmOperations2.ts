import { FhevmType } from '@fhevm/hardhat-plugin';
import { expect } from 'chai';
import { ethers } from 'hardhat';
import * as hre from 'hardhat';

import type { FHEVMTestSuite1 } from '../../../typechain-types/contracts/operators/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../../typechain-types/contracts/operators/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../../typechain-types/contracts/operators/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../../typechain-types/contracts/operators/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../../typechain-types/contracts/operators/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../../typechain-types/contracts/operators/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../../typechain-types/contracts/operators/FHEVMTestSuite7';
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

describe('FHEVM operations 2', function () {
  before(async function () {
    await initSigners();
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
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 1 (82, 18442538173712758245)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(82n);
    input.add64(18442538173712758245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 2 (78, 82)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(78n);
    input.add64(82n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 3 (82, 82)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(82n);
    input.add64(82n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint64) => ebool test 4 (82, 78)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(82n);
    input.add64(78n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 1 (158, 18446654203617240269)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(158n);
    input.add64(18446654203617240269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 2 (154, 158)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(154n);
    input.add64(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 3 (158, 158)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(158n);
    input.add64(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint64) => ebool test 4 (158, 154)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(158n);
    input.add64(154n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 1 (73, 18440064211635517333)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(73n);
    input.add64(18440064211635517333n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(73n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 2 (69, 73)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(69n);
    input.add64(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(69n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 3 (73, 73)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(73n);
    input.add64(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(73n);
  });

  it('test operator "min" overload (euint8, euint64) => euint64 test 4 (73, 69)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(73n);
    input.add64(69n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(69n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 1 (152, 18440285318812855091)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(152n);
    input.add64(18440285318812855091n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(18440285318812855091n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 2 (148, 152)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(148n);
    input.add64(152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(152n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 3 (152, 152)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(152n);
    input.add64(152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(152n);
  });

  it('test operator "max" overload (euint8, euint64) => euint64 test 4 (152, 148)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(152n);
    input.add64(148n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(152n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 1 (2, 129)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add128(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 2 (114, 116)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add128(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(230n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 3 (116, 116)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(116n);
    input.add128(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(232n);
  });

  it('test operator "add" overload (euint8, euint128) => euint128 test 4 (116, 114)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(116n);
    input.add128(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(230n);
  });

  it('test operator "sub" overload (euint8, euint128) => euint128 test 1 (218, 218)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(218n);
    input.add128(218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint128) => euint128 test 2 (218, 214)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(218n);
    input.add128(214n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 1 (2, 65)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add128(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 2 (9, 9)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add128(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 3 (9, 9)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add128(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint8, euint128) => euint128 test 4 (9, 9)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add128(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(81n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 1 (160, 340282366920938463463369765673248528819)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(160n);
    input.add128(340282366920938463463369765673248528819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(160n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 2 (156, 160)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(156n);
    input.add128(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(128n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 3 (160, 160)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(160n);
    input.add128(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(160n);
  });

  it('test operator "and" overload (euint8, euint128) => euint128 test 4 (160, 156)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(160n);
    input.add128(156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(128n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 1 (152, 340282366920938463463367166500429742599)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(152n);
    input.add128(340282366920938463463367166500429742599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(340282366920938463463367166500429742751n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 2 (148, 152)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(148n);
    input.add128(152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(156n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 3 (152, 152)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(152n);
    input.add128(152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(152n);
  });

  it('test operator "or" overload (euint8, euint128) => euint128 test 4 (152, 148)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(152n);
    input.add128(148n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(156n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 1 (47, 340282366920938463463372634439594458629)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(47n);
    input.add128(340282366920938463463372634439594458629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(340282366920938463463372634439594458666n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 2 (43, 47)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(43n);
    input.add128(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 3 (47, 47)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(47n);
    input.add128(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint128) => euint128 test 4 (47, 43)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(47n);
    input.add128(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 1 (136, 340282366920938463463373309899574889569)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(136n);
    input.add128(340282366920938463463373309899574889569n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 2 (132, 136)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(132n);
    input.add128(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 3 (136, 136)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(136n);
    input.add128(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint128) => ebool test 4 (136, 132)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(136n);
    input.add128(132n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 1 (81, 340282366920938463463365606732718999303)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(81n);
    input.add128(340282366920938463463365606732718999303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 2 (77, 81)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(77n);
    input.add128(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 3 (81, 81)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(81n);
    input.add128(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 4 (81, 77)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(81n);
    input.add128(77n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 1 (87, 340282366920938463463373094541029016309)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(87n);
    input.add128(340282366920938463463373094541029016309n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 2 (83, 87)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(83n);
    input.add128(87n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 3 (87, 87)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(87n);
    input.add128(87n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint128) => ebool test 4 (87, 83)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(87n);
    input.add128(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 1 (122, 340282366920938463463367910411609424995)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(122n);
    input.add128(340282366920938463463367910411609424995n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 2 (118, 122)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(118n);
    input.add128(122n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 3 (122, 122)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(122n);
    input.add128(122n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint128) => ebool test 4 (122, 118)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(122n);
    input.add128(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 1 (115, 340282366920938463463373601023326867235)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(115n);
    input.add128(340282366920938463463373601023326867235n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 2 (111, 115)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(111n);
    input.add128(115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 3 (115, 115)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(115n);
    input.add128(115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint128) => ebool test 4 (115, 111)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(115n);
    input.add128(111n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 1 (173, 340282366920938463463369530904172568335)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(173n);
    input.add128(340282366920938463463369530904172568335n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 2 (169, 173)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(169n);
    input.add128(173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 3 (173, 173)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(173n);
    input.add128(173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint128) => ebool test 4 (173, 169)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(173n);
    input.add128(169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 1 (118, 340282366920938463463369096355574974773)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(118n);
    input.add128(340282366920938463463369096355574974773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(118n);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 2 (114, 118)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add128(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(114n);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 3 (118, 118)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(118n);
    input.add128(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(118n);
  });

  it('test operator "min" overload (euint8, euint128) => euint128 test 4 (118, 114)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(118n);
    input.add128(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(114n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 1 (5, 340282366920938463463366673524785465781)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add128(340282366920938463463366673524785465781n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(340282366920938463463366673524785465781n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 2 (1, 5)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(1n);
    input.add128(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 3 (5, 5)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add128(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(5n);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 4 (5, 1)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add128(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract1.resEuint128());
    expect(res).to.equal(5n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 1 (9, 115792089237316195423570985008687907853269984665640564039457575680225403883879)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575680225403883879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 2 (5, 9)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add256(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 3 (9, 9)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add256(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(9n);
  });

  it('test operator "and" overload (euint8, euint256) => euint256 test 4 (9, 5)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(9n);
    input.add256(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 1 (5, 115792089237316195423570985008687907853269984665640564039457583017965900168219)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583017965900168219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583017965900168223n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 2 (1, 5)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(1n);
    input.add256(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(5n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 3 (5, 5)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add256(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(5n);
  });

  it('test operator "or" overload (euint8, euint256) => euint256 test 4 (5, 1)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(5n);
    input.add256(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(5n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 1 (250, 115792089237316195423570985008687907853269984665640564039457575385389601432361)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575385389601432361n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575385389601432531n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 2 (246, 250)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(246n);
    input.add256(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 3 (250, 250)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add256(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint256) => euint256 test 4 (250, 246)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add256(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract1.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 1 (162, 115792089237316195423570985008687907853269984665640564039457580375233959522263)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(162n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580375233959522263n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 2 (158, 162)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(158n);
    input.add256(162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 3 (162, 162)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(162n);
    input.add256(162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 4 (162, 158)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(162n);
    input.add256(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 1 (62, 115792089237316195423570985008687907853269984665640564039457581331145154314037)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(62n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581331145154314037n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 2 (58, 62)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(58n);
    input.add256(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 3 (62, 62)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(62n);
    input.add256(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint256) => ebool test 4 (62, 58)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(62n);
    input.add256(58n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (154, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(154n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(156n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (109, 113)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(109n);
    input.add8(113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(222n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (113, 113)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(113n);
    input.add8(113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(226n);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (113, 109)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(113n);
    input.add8(109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(222n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (103, 103)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(103n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (103, 99)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(103n);
    input.add8(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (101, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(101n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(202n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (11, 13)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(11n);
    input.add8(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(143n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (13, 13)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(13n);
    input.add8(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(169n);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (13, 11)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(13n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(143n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (16857, 136)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(16857n);
    input.add8(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(136n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (132, 136)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(132n);
    input.add8(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(128n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (136, 136)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(136n);
    input.add8(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(136n);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (136, 132)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(136n);
    input.add8(132n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(128n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (2595, 41)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(2595n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(2603n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (37, 41)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(37n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(45n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (41, 41)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(41n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(41n);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (41, 37)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(41n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(45n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (40601, 238)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(40601n);
    input.add8(238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(40567n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (234, 238)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(234n);
    input.add8(238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (238, 238)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(238n);
    input.add8(238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (238, 234)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(238n);
    input.add8(234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (40796, 50)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(40796n);
    input.add8(50n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (46, 50)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(46n);
    input.add8(50n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (50, 50)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(50n);
    input.add8(50n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (50, 46)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(50n);
    input.add8(46n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (45943, 178)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(45943n);
    input.add8(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (174, 178)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(174n);
    input.add8(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (178, 178)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(178n);
    input.add8(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (178, 174)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(178n);
    input.add8(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (38746, 51)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(38746n);
    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (47, 51)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(47n);
    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (51, 51)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(51n);
    input.add8(51n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (51, 47)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(51n);
    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (32390, 103)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(32390n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (99, 103)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(99n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (103, 103)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(103n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 4 (103, 99)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(103n);
    input.add8(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 1 (11122, 142)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(11122n);
    input.add8(142n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 2 (138, 142)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(138n);
    input.add8(142n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 3 (142, 142)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(142n);
    input.add8(142n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint8) => ebool test 4 (142, 138)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(142n);
    input.add8(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 1 (16422, 110)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(16422n);
    input.add8(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 2 (106, 110)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(106n);
    input.add8(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 3 (110, 110)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(110n);
    input.add8(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint8) => ebool test 4 (110, 106)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(110n);
    input.add8(106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 1 (7218, 86)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(7218n);
    input.add8(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(86n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 2 (82, 86)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(82n);
    input.add8(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(82n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 3 (86, 86)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(86n);
    input.add8(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(86n);
  });

  it('test operator "min" overload (euint16, euint8) => euint16 test 4 (86, 82)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(86n);
    input.add8(82n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(82n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (40114, 33)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(40114n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(40114n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (29, 33)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(29n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(33n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (33, 33)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(33n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(33n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (33, 29)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(33n);
    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(33n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (13756, 43006)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(13756n);
    input.add16(43006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(56762n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 2 (13752, 13756)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(13752n);
    input.add16(13756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(27508n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 3 (13756, 13756)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(13756n);
    input.add16(13756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(27512n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 4 (13756, 13752)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add16(13756n);
    input.add16(13752n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(27508n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (48348, 48348)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(48348n);
    input.add16(48348n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 2 (48348, 48344)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(48348n);
    input.add16(48344n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (163, 317)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(163n);
    input.add16(317n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(51671n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 2 (163, 163)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(163n);
    input.add16(163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(26569n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 3 (163, 163)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(163n);
    input.add16(163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(26569n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 4 (163, 163)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(163n);
    input.add16(163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(26569n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (129, 21640)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(129n);
    input.add16(21640n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(128n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (125, 129)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(125n);
    input.add16(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (129, 129)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(129n);
    input.add16(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(129n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (129, 125)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(129n);
    input.add16(125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (34917, 27588)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(34917n);
    input.add16(27588n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(60389n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (27584, 27588)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27584n);
    input.add16(27588n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(27588n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 3 (27588, 27588)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27588n);
    input.add16(27588n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(27588n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 4 (27588, 27584)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(27588n);
    input.add16(27584n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(27588n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (16637, 56674)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16637n);
    input.add16(56674n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(40351n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (16633, 16637)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16633n);
    input.add16(16637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (16637, 16637)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16637n);
    input.add16(16637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (16637, 16633)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16637n);
    input.add16(16633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (56747, 20695)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(56747n);
    input.add16(20695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (20691, 20695)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20691n);
    input.add16(20695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 3 (20695, 20695)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20695n);
    input.add16(20695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 4 (20695, 20691)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(20695n);
    input.add16(20691n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (24928, 4057)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24928n);
    input.add16(4057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (4053, 4057)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4053n);
    input.add16(4057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (4057, 4057)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4057n);
    input.add16(4057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (4057, 4053)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(4057n);
    input.add16(4053n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (22657, 38840)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22657n);
    input.add16(38840n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (22653, 22657)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22653n);
    input.add16(22657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (22657, 22657)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22657n);
    input.add16(22657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 4 (22657, 22653)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22657n);
    input.add16(22653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (59175, 12672)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(59175n);
    input.add16(12672n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (12668, 12672)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12668n);
    input.add16(12672n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (12672, 12672)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12672n);
    input.add16(12672n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 4 (12672, 12668)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12672n);
    input.add16(12668n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (25325, 18196)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(25325n);
    input.add16(18196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (18192, 18196)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18192n);
    input.add16(18196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (18196, 18196)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18196n);
    input.add16(18196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 4 (18196, 18192)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18196n);
    input.add16(18192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (64205, 56305)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64205n);
    input.add16(56305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (56301, 56305)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(56301n);
    input.add16(56305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (56305, 56305)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(56305n);
    input.add16(56305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (56305, 56301)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(56305n);
    input.add16(56301n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (58529, 18203)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(58529n);
    input.add16(18203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(18203n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (18199, 18203)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18199n);
    input.add16(18203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(18199n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (18203, 18203)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18203n);
    input.add16(18203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(18203n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 4 (18203, 18199)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18203n);
    input.add16(18199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(18199n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (64651, 60629)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(64651n);
    input.add16(60629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(64651n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (60625, 60629)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60625n);
    input.add16(60629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(60629n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (60629, 60629)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60629n);
    input.add16(60629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(60629n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (60629, 60625)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60629n);
    input.add16(60625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract2.resEuint16());
    expect(res).to.equal(60629n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (2, 54917)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add32(54917n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(54919n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (2808, 2812)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2808n);
    input.add32(2812n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(5620n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (2812, 2812)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2812n);
    input.add32(2812n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(5624n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (2812, 2808)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2812n);
    input.add32(2808n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(5620n);
  });
});
