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

describe('FHEVM operations 4', function () {
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

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (2532320727, 183)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2532320727n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (179, 183)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(179n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (183, 183)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(183n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (183, 179)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(183n);
    input.add8(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (2676084558, 237)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2676084558n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (233, 237)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(233n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (237, 237)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(237n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (237, 233)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(237n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (3548487647, 241)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3548487647n);
    input.add8(241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (237, 241)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(237n);
    input.add8(241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (241, 241)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(241n);
    input.add8(241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (241, 237)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(241n);
    input.add8(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (521887715, 124)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(521887715n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (120, 124)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(120n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (124, 124)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(124n);
    input.add8(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (124, 120)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(124n);
    input.add8(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (3671403941, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3671403941n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (30, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(30n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (34, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(34n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (34, 30)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(34n);
    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (3935223711, 65)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3935223711n);
    input.add8(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (61, 65)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(61n);
    input.add8(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (65, 65)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(65n);
    input.add8(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (65, 61)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(65n);
    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (4070815488, 204)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(4070815488n);
    input.add8(204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(204n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (200, 204)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(200n);
    input.add8(204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(200n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (204, 204)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(204n);
    input.add8(204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(204n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (204, 200)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(204n);
    input.add8(200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(200n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (2283898097, 228)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2283898097n);
    input.add8(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(2283898097n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (224, 228)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(224n);
    input.add8(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(228n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (228, 228)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(228n);
    input.add8(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(228n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (228, 224)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(228n);
    input.add8(224n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(228n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (45145, 3)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(45145n);
    input.add16(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(45148n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (31851, 31853)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(31851n);
    input.add16(31853n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(63704n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (31853, 31853)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(31853n);
    input.add16(31853n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(63706n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (31853, 31851)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(31853n);
    input.add16(31851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(63704n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (46156, 46156)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(46156n);
    input.add16(46156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (46156, 46152)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(46156n);
    input.add16(46152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (17102, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(17102n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(34204n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (156, 156)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(156n);
    input.add16(156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(24336n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (156, 156)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(156n);
    input.add16(156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(24336n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (156, 156)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(156n);
    input.add16(156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(24336n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (1484006749, 2919)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1484006749n);
    input.add16(2919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(325n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (2915, 2919)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2915n);
    input.add16(2919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(2915n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (2919, 2919)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2919n);
    input.add16(2919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(2919n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (2919, 2915)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2919n);
    input.add16(2915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(2915n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (3948604159, 128)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3948604159n);
    input.add16(128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(3948604159n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (124, 128)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(124n);
    input.add16(128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(252n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (128, 128)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(128n);
    input.add16(128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(128n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (128, 124)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(128n);
    input.add16(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(252n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (3244307804, 55182)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3244307804n);
    input.add16(55182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(3244352210n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (55178, 55182)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(55178n);
    input.add16(55182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (55182, 55182)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(55182n);
    input.add16(55182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (55182, 55178)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(55182n);
    input.add16(55178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (1084275393, 57843)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1084275393n);
    input.add16(57843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (57839, 57843)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(57839n);
    input.add16(57843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (57843, 57843)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(57843n);
    input.add16(57843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (57843, 57839)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(57843n);
    input.add16(57839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (316088001, 3787)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(316088001n);
    input.add16(3787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (3783, 3787)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3783n);
    input.add16(3787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (3787, 3787)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3787n);
    input.add16(3787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (3787, 3783)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3787n);
    input.add16(3783n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (140041814, 56977)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(140041814n);
    input.add16(56977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (56973, 56977)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(56973n);
    input.add16(56977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (56977, 56977)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(56977n);
    input.add16(56977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (56977, 56973)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(56977n);
    input.add16(56973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (3904113431, 40661)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3904113431n);
    input.add16(40661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (40657, 40661)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(40657n);
    input.add16(40661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (40661, 40661)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(40661n);
    input.add16(40661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (40661, 40657)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(40661n);
    input.add16(40657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (2187421426, 1257)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2187421426n);
    input.add16(1257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (1253, 1257)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1253n);
    input.add16(1257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (1257, 1257)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1257n);
    input.add16(1257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (1257, 1253)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1257n);
    input.add16(1253n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (2551437926, 55821)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(2551437926n);
    input.add16(55821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (55817, 55821)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(55817n);
    input.add16(55821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (55821, 55821)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(55821n);
    input.add16(55821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (55821, 55817)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(55821n);
    input.add16(55817n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (1068231898, 55730)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1068231898n);
    input.add16(55730n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(55730n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (55726, 55730)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(55726n);
    input.add16(55730n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(55726n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (55730, 55730)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(55730n);
    input.add16(55730n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(55730n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (55730, 55726)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(55730n);
    input.add16(55726n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(55726n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (475287265, 61314)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(475287265n);
    input.add16(61314n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(475287265n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (61310, 61314)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(61310n);
    input.add16(61314n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(61314n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (61314, 61314)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(61314n);
    input.add16(61314n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(61314n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (61314, 61310)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(61314n);
    input.add16(61310n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(61314n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (308575078, 3037034175)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(308575078n);
    input.add32(3037034175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(3345609253n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (308575074, 308575078)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(308575074n);
    input.add32(308575078n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(617150152n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (308575078, 308575078)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(308575078n);
    input.add32(308575078n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(617150156n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (308575078, 308575074)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(308575078n);
    input.add32(308575074n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(617150152n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (827505071, 827505071)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(827505071n);
    input.add32(827505071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (827505071, 827505067)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(827505071n);
    input.add32(827505067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (37540, 54954)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(37540n);
    input.add32(54954n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(2062973160n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (37540, 37540)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(37540n);
    input.add32(37540n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(1409251600n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (37540, 37540)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(37540n);
    input.add32(37540n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(1409251600n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (37540, 37540)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(37540n);
    input.add32(37540n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(1409251600n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (3106281428, 459255123)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3106281428n);
    input.add32(459255123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(419824976n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (459255119, 459255123)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(459255119n);
    input.add32(459255123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(459255107n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (459255123, 459255123)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(459255123n);
    input.add32(459255123n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(459255123n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (459255123, 459255119)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(459255123n);
    input.add32(459255119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(459255107n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (2935699084, 2988380217)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2935699084n);
    input.add32(2988380217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(3204396733n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (2935699080, 2935699084)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2935699080n);
    input.add32(2935699084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(2935699084n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (2935699084, 2935699084)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2935699084n);
    input.add32(2935699084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(2935699084n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (2935699084, 2935699080)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2935699084n);
    input.add32(2935699080n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(2935699084n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (1188690081, 3400202936)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1188690081n);
    input.add32(3400202936n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(2356347417n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (1188690077, 1188690081)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1188690077n);
    input.add32(1188690081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(60n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (1188690081, 1188690081)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1188690081n);
    input.add32(1188690081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (1188690081, 1188690077)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1188690081n);
    input.add32(1188690077n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(60n);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (3295570634, 96781377)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3295570634n);
    input.add32(96781377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (96781373, 96781377)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(96781373n);
    input.add32(96781377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (96781377, 96781377)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(96781377n);
    input.add32(96781377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (96781377, 96781373)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(96781377n);
    input.add32(96781373n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (3427057056, 345973165)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3427057056n);
    input.add32(345973165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (345973161, 345973165)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(345973161n);
    input.add32(345973165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (345973165, 345973165)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(345973165n);
    input.add32(345973165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (345973165, 345973161)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(345973165n);
    input.add32(345973161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (1065729491, 3661441654)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1065729491n);
    input.add32(3661441654n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (1065729487, 1065729491)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1065729487n);
    input.add32(1065729491n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (1065729491, 1065729491)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1065729491n);
    input.add32(1065729491n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (1065729491, 1065729487)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1065729491n);
    input.add32(1065729487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (1417495620, 3756148778)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1417495620n);
    input.add32(3756148778n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (1417495616, 1417495620)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1417495616n);
    input.add32(1417495620n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (1417495620, 1417495620)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1417495620n);
    input.add32(1417495620n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (1417495620, 1417495616)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1417495620n);
    input.add32(1417495616n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (2499693605, 3274381478)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2499693605n);
    input.add32(3274381478n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (2499693601, 2499693605)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2499693601n);
    input.add32(2499693605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (2499693605, 2499693605)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2499693605n);
    input.add32(2499693605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (2499693605, 2499693601)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2499693605n);
    input.add32(2499693601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (3759916356, 3277524466)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3759916356n);
    input.add32(3277524466n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (3277524462, 3277524466)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3277524462n);
    input.add32(3277524466n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (3277524466, 3277524466)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3277524466n);
    input.add32(3277524466n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (3277524466, 3277524462)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3277524466n);
    input.add32(3277524462n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (3856164871, 2385547109)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3856164871n);
    input.add32(2385547109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(2385547109n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (2385547105, 2385547109)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2385547105n);
    input.add32(2385547109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(2385547105n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (2385547109, 2385547109)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2385547109n);
    input.add32(2385547109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(2385547109n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (2385547109, 2385547105)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2385547109n);
    input.add32(2385547105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(2385547105n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (713485747, 621451200)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(713485747n);
    input.add32(621451200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(713485747n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (621451196, 621451200)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(621451196n);
    input.add32(621451200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(621451200n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (621451200, 621451200)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(621451200n);
    input.add32(621451200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(621451200n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (621451200, 621451196)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(621451200n);
    input.add32(621451196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract3.resEuint32());
    expect(res).to.equal(621451200n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (2, 4293537694)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(4293537694n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(4293537696n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (1235096539, 1235096541)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1235096539n);
    input.add64(1235096541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(2470193080n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (1235096541, 1235096541)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1235096541n);
    input.add64(1235096541n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(2470193082n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (1235096541, 1235096539)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1235096541n);
    input.add64(1235096539n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(2470193080n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (1159293805, 1159293805)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1159293805n);
    input.add64(1159293805n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (1159293805, 1159293801)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1159293805n);
    input.add64(1159293801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2146763810)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(2146763810n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(4293527620n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (56716, 56716)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(56716n);
    input.add64(56716n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(3216704656n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (56716, 56716)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(56716n);
    input.add64(56716n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(3216704656n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (56716, 56716)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(56716n);
    input.add64(56716n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(3216704656n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (946992826, 18441461428987257875)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(946992826n);
    input.add64(18441461428987257875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(676357138n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (946992822, 946992826)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(946992822n);
    input.add64(946992826n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(946992818n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (946992826, 946992826)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(946992826n);
    input.add64(946992826n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(946992826n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (946992826, 946992822)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(946992826n);
    input.add64(946992822n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(946992818n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (311778344, 18446699499439087951)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(311778344n);
    input.add64(18446699499439087951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18446699499447745903n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (311778340, 311778344)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(311778340n);
    input.add64(311778344n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(311778348n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (311778344, 311778344)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(311778344n);
    input.add64(311778344n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(311778344n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (311778344, 311778340)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(311778344n);
    input.add64(311778340n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(311778348n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (1100149359, 18440160241169480413)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1100149359n);
    input.add64(18440160241169480413n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18440160242267506866n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (1100149355, 1100149359)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1100149355n);
    input.add64(1100149359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (1100149359, 1100149359)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1100149359n);
    input.add64(1100149359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (1100149359, 1100149355)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1100149359n);
    input.add64(1100149355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (2324161012, 18439625567169454671)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2324161012n);
    input.add64(18439625567169454671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (2324161008, 2324161012)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2324161008n);
    input.add64(2324161012n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (2324161012, 2324161012)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2324161012n);
    input.add64(2324161012n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (2324161012, 2324161008)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2324161012n);
    input.add64(2324161008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (1743345271, 18445619894242149765)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1743345271n);
    input.add64(18445619894242149765n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (1743345267, 1743345271)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1743345267n);
    input.add64(1743345271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (1743345271, 1743345271)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1743345271n);
    input.add64(1743345271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (1743345271, 1743345267)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1743345271n);
    input.add64(1743345267n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (1358412880, 18443611153534772391)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1358412880n);
    input.add64(18443611153534772391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (1358412876, 1358412880)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1358412876n);
    input.add64(1358412880n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (1358412880, 1358412880)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1358412880n);
    input.add64(1358412880n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (1358412880, 1358412876)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1358412880n);
    input.add64(1358412876n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (3552831674, 18445107069229183219)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3552831674n);
    input.add64(18445107069229183219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (3552831670, 3552831674)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3552831670n);
    input.add64(3552831674n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (3552831674, 3552831674)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3552831674n);
    input.add64(3552831674n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (3552831674, 3552831670)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3552831674n);
    input.add64(3552831670n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (3076380922, 18443020918330004613)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3076380922n);
    input.add64(18443020918330004613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (3076380918, 3076380922)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3076380918n);
    input.add64(3076380922n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (3076380922, 3076380922)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3076380922n);
    input.add64(3076380922n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (3076380922, 3076380918)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3076380922n);
    input.add64(3076380918n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (3481458407, 18437842109699544467)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3481458407n);
    input.add64(18437842109699544467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (3481458403, 3481458407)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3481458403n);
    input.add64(3481458407n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (3481458407, 3481458407)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3481458407n);
    input.add64(3481458407n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (3481458407, 3481458403)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3481458407n);
    input.add64(3481458403n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (4260966548, 18440167583242822417)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4260966548n);
    input.add64(18440167583242822417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(4260966548n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (4260966544, 4260966548)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4260966544n);
    input.add64(4260966548n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(4260966544n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (4260966548, 4260966548)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4260966548n);
    input.add64(4260966548n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(4260966548n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (4260966548, 4260966544)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4260966548n);
    input.add64(4260966544n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(4260966544n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (1824035354, 18438154354738093649)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1824035354n);
    input.add64(18438154354738093649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18438154354738093649n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (1824035350, 1824035354)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1824035350n);
    input.add64(1824035354n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(1824035354n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (1824035354, 1824035354)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1824035354n);
    input.add64(1824035354n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(1824035354n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (1824035354, 1824035350)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1824035354n);
    input.add64(1824035350n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(1824035354n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 1 (2, 2147483649)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add128(2147483649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract3.resEuint128());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 2 (1362611807, 1362611809)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1362611807n);
    input.add128(1362611809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract3.resEuint128());
    expect(res).to.equal(2725223616n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 3 (1362611809, 1362611809)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1362611809n);
    input.add128(1362611809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract3.resEuint128());
    expect(res).to.equal(2725223618n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 4 (1362611809, 1362611807)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1362611809n);
    input.add128(1362611807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract3.resEuint128());
    expect(res).to.equal(2725223616n);
  });

  it('test operator "sub" overload (euint32, euint128) => euint128 test 1 (3564881227, 3564881227)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3564881227n);
    input.add128(3564881227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract3.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint128) => euint128 test 2 (3564881227, 3564881223)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3564881227n);
    input.add128(3564881223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract3.resEuint128());
    expect(res).to.equal(4n);
  });
});
