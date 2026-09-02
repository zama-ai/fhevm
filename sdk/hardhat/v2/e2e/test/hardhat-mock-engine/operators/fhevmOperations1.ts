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

describe('FHEVM operations 1', function () {
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

  it('test operator "add" overload (euint8, euint8) => euint8 test 1 (80, 133)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(80n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(213n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 2 (76, 80)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(76n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(156n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 3 (80, 80)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(80n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(160n);
  });

  it('test operator "add" overload (euint8, euint8) => euint8 test 4 (80, 76)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(80n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(156n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (34, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(34n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 2 (34, 30)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(34n);
    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (13, 7)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(13n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(91n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 2 (11, 12)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(132n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 3 (12, 12)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(12n);
    input.add8(12n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(144n);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 4 (12, 11)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(12n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(132n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (200, 121)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(200n);
    input.add8(121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(72n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 2 (117, 121)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(117n);
    input.add8(121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(113n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 3 (121, 121)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(121n);
    input.add8(121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(121n);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 4 (121, 117)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(121n);
    input.add8(117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(113n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (119, 37)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(119n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(119n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (33, 37)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(33n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(37n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (37, 37)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(37n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(37n);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (37, 33)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(37n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(37n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 1 (239, 14)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(239n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(225n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 2 (10, 14)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(10n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 3 (14, 14)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint8) => euint8 test 4 (14, 10)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(14n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 1 (221, 223)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(221n);
    input.add8(223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 2 (217, 221)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(217n);
    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 3 (221, 221)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(221n);
    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint8) => ebool test 4 (221, 217)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(221n);
    input.add8(217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 1 (151, 233)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(151n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 2 (147, 151)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(147n);
    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 3 (151, 151)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(151n);
    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint8) => ebool test 4 (151, 147)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(151n);
    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 1 (36, 38)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(36n);
    input.add8(38n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 2 (32, 36)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(32n);
    input.add8(36n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 3 (36, 36)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(36n);
    input.add8(36n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint8) => ebool test 4 (36, 32)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(36n);
    input.add8(32n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (244, 204)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(244n);
    input.add8(204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 2 (200, 204)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(200n);
    input.add8(204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 3 (204, 204)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(204n);
    input.add8(204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 4 (204, 200)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(204n);
    input.add8(200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 1 (212, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(212n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 2 (30, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(30n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 3 (34, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(34n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint8) => ebool test 4 (34, 30)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(34n);
    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 1 (218, 198)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(218n);
    input.add8(198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 2 (194, 198)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(194n);
    input.add8(198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 3 (198, 198)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(198n);
    input.add8(198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint8) => ebool test 4 (198, 194)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(198n);
    input.add8(194n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (137, 232)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(137n);
    input.add8(232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(137n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (133, 137)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(133n);
    input.add8(137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(133n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (137, 137)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(137n);
    input.add8(137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(137n);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 4 (137, 133)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(137n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(133n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (125, 15)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(125n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(125n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (11, 15)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(15n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (15, 15)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(15n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(15n);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 4 (15, 11)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(15n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract1.resEuint8());
    expect(res).to.equal(15n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (2, 159)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add16(159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(161n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (43, 47)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(43n);
    input.add16(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(90n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (47, 47)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(47n);
    input.add16(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(94n);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (47, 43)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(47n);
    input.add16(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(90n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (49, 49)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(49n);
    input.add16(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (49, 45)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(49n);
    input.add16(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (2, 79)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add16(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(158n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 2 (10, 11)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(10n);
    input.add16(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(110n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 3 (11, 11)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add16(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(121n);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 4 (11, 10)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(11n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(110n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (243, 47811)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(243n);
    input.add16(47811n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(195n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (239, 243)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(239n);
    input.add16(243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(227n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (243, 243)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(243n);
    input.add16(243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(243n);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (243, 239)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(243n);
    input.add16(239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(227n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (88, 40375)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(88n);
    input.add16(40375n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(40447n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (84, 88)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(84n);
    input.add16(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(92n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (88, 88)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(88n);
    input.add16(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(88n);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (88, 84)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(88n);
    input.add16(84n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(92n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (20, 36098)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(20n);
    input.add16(36098n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(36118n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (16, 20)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(16n);
    input.add16(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 3 (20, 20)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(20n);
    input.add16(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 4 (20, 16)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(20n);
    input.add16(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 1 (225, 14088)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(225n);
    input.add16(14088n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 2 (221, 225)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(221n);
    input.add16(225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 3 (225, 225)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(225n);
    input.add16(225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint16) => ebool test 4 (225, 221)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(225n);
    input.add16(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (138, 38804)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(138n);
    input.add16(38804n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (134, 138)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(134n);
    input.add16(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 3 (138, 138)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(138n);
    input.add16(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 4 (138, 134)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(138n);
    input.add16(134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 1 (250, 25860)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add16(25860n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 2 (246, 250)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(246n);
    input.add16(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 3 (250, 250)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add16(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint16) => ebool test 4 (250, 246)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(250n);
    input.add16(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (197, 6725)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(197n);
    input.add16(6725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (193, 197)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(193n);
    input.add16(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (197, 197)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(197n);
    input.add16(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (197, 193)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(197n);
    input.add16(193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 1 (91, 17968)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(91n);
    input.add16(17968n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 2 (87, 91)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(87n);
    input.add16(91n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 3 (91, 91)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(91n);
    input.add16(91n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint16) => ebool test 4 (91, 87)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(91n);
    input.add16(87n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 1 (157, 48773)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(157n);
    input.add16(48773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 2 (153, 157)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(153n);
    input.add16(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 3 (157, 157)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(157n);
    input.add16(157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint16) => ebool test 4 (157, 153)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(157n);
    input.add16(153n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 1 (61, 47120)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(61n);
    input.add16(47120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(61n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 2 (57, 61)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(57n);
    input.add16(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(57n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 3 (61, 61)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(61n);
    input.add16(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(61n);
  });

  it('test operator "min" overload (euint8, euint16) => euint16 test 4 (61, 57)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(61n);
    input.add16(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(57n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 1 (45, 40681)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(45n);
    input.add16(40681n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(40681n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 2 (41, 45)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(41n);
    input.add16(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(45n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 3 (45, 45)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(45n);
    input.add16(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(45n);
  });

  it('test operator "max" overload (euint8, euint16) => euint16 test 4 (45, 41)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(45n);
    input.add16(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract1.resEuint16());
    expect(res).to.equal(45n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 1 (2, 183)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add32(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(185n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 2 (81, 83)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(81n);
    input.add32(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(164n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 3 (83, 83)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(83n);
    input.add32(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(166n);
  });

  it('test operator "add" overload (euint8, euint32) => euint32 test 4 (83, 81)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(83n);
    input.add32(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(164n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 1 (92, 92)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(92n);
    input.add32(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint32) => euint32 test 2 (92, 88)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(92n);
    input.add32(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 1 (2, 65)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add32(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 2 (13, 14)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(13n);
    input.add32(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(182n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 3 (14, 14)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(14n);
    input.add32(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint8, euint32) => euint32 test 4 (14, 13)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(14n);
    input.add32(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(182n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 1 (135, 173269101)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(135n);
    input.add32(173269101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(5n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 2 (131, 135)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(131n);
    input.add32(135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(131n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 3 (135, 135)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(135n);
    input.add32(135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(135n);
  });

  it('test operator "and" overload (euint8, euint32) => euint32 test 4 (135, 131)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(135n);
    input.add32(131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(131n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 1 (199, 3525720338)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(199n);
    input.add32(3525720338n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(3525720535n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 2 (195, 199)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(195n);
    input.add32(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(199n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 3 (199, 199)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(199n);
    input.add32(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(199n);
  });

  it('test operator "or" overload (euint8, euint32) => euint32 test 4 (199, 195)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(199n);
    input.add32(195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(199n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 1 (135, 1170050720)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(135n);
    input.add32(1170050720n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(1170050599n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 2 (131, 135)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(131n);
    input.add32(135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 3 (135, 135)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(135n);
    input.add32(135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint32) => euint32 test 4 (135, 131)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(135n);
    input.add32(131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 1 (188, 2380814349)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(188n);
    input.add32(2380814349n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 2 (184, 188)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(184n);
    input.add32(188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 3 (188, 188)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(188n);
    input.add32(188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint32) => ebool test 4 (188, 184)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(188n);
    input.add32(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 1 (137, 4250199492)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(137n);
    input.add32(4250199492n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 2 (133, 137)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(133n);
    input.add32(137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 3 (137, 137)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(137n);
    input.add32(137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint32) => ebool test 4 (137, 133)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(137n);
    input.add32(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 1 (120, 3166550340)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(120n);
    input.add32(3166550340n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 2 (116, 120)', async function () {
    await hre.fhevm.assertCoprocessorInitialized(this.contract1Address);
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(116n);
    input.add32(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 3 (120, 120)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(120n);
    input.add32(120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint32) => ebool test 4 (120, 116)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(120n);
    input.add32(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (162, 1397295057)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(162n);
    input.add32(1397295057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (158, 162)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(158n);
    input.add32(162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (162, 162)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(162n);
    input.add32(162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 4 (162, 158)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(162n);
    input.add32(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (113, 3145214746)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(113n);
    input.add32(3145214746n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (109, 113)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(109n);
    input.add32(113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (113, 113)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(113n);
    input.add32(113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 4 (113, 109)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(113n);
    input.add32(109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 1 (155, 2538428262)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(155n);
    input.add32(2538428262n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 2 (151, 155)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(151n);
    input.add32(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 3 (155, 155)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(155n);
    input.add32(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, euint32) => ebool test 4 (155, 151)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(155n);
    input.add32(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.lt_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 1 (114, 2894219978)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add32(2894219978n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(114n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 2 (110, 114)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(110n);
    input.add32(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(110n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 3 (114, 114)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add32(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(114n);
  });

  it('test operator "min" overload (euint8, euint32) => euint32 test 4 (114, 110)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(114n);
    input.add32(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(110n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 1 (186, 312047724)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(186n);
    input.add32(312047724n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(312047724n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 2 (182, 186)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(182n);
    input.add32(186n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(186n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 3 (186, 186)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(186n);
    input.add32(186n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(186n);
  });

  it('test operator "max" overload (euint8, euint32) => euint32 test 4 (186, 182)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(186n);
    input.add32(182n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract1.resEuint32());
    expect(res).to.equal(186n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 1 (2, 129)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add64(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 2 (125, 129)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(125n);
    input.add64(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(254n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 3 (65, 65)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(65n);
    input.add64(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(130n);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 4 (129, 125)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(129n);
    input.add64(125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(254n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 1 (220, 220)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(220n);
    input.add64(220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint8, euint64) => euint64 test 2 (220, 216)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(220n);
    input.add64(216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 1 (2, 65)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(2n);
    input.add64(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 2 (15, 15)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(15n);
    input.add64(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(225n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 3 (15, 15)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(15n);
    input.add64(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(225n);
  });

  it('test operator "mul" overload (euint8, euint64) => euint64 test 4 (15, 15)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(15n);
    input.add64(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(225n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 1 (178, 18445664465334909103)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(178n);
    input.add64(18445664465334909103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(162n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 2 (174, 178)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(174n);
    input.add64(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(162n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 3 (178, 178)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(178n);
    input.add64(178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(178n);
  });

  it('test operator "and" overload (euint8, euint64) => euint64 test 4 (178, 174)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(178n);
    input.add64(174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(162n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 1 (29, 18438856880396735223)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(29n);
    input.add64(18438856880396735223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(18438856880396735231n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 2 (25, 29)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(25n);
    input.add64(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(29n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 3 (29, 29)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(29n);
    input.add64(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(29n);
  });

  it('test operator "or" overload (euint8, euint64) => euint64 test 4 (29, 25)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(29n);
    input.add64(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(29n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (20, 18439440834440328623)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(20n);
    input.add64(18439440834440328623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(18439440834440328635n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 2 (16, 20)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(16n);
    input.add64(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 3 (20, 20)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(20n);
    input.add64(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 4 (20, 16)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(20n);
    input.add64(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract1.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 1 (26, 18443686415883984161)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(26n);
    input.add64(18443686415883984161n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 2 (22, 26)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(22n);
    input.add64(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 3 (26, 26)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(26n);
    input.add64(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, euint64) => ebool test 4 (26, 22)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(26n);
    input.add64(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 1 (248, 18443146105099135433)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(248n);
    input.add64(18443146105099135433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 2 (244, 248)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(244n);
    input.add64(248n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 3 (248, 248)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(248n);
    input.add64(248n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, euint64) => ebool test 4 (248, 244)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(248n);
    input.add64(244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 1 (233, 18439455908696976195)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(233n);
    input.add64(18439455908696976195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 2 (229, 233)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(229n);
    input.add64(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 3 (233, 233)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(233n);
    input.add64(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, euint64) => ebool test 4 (233, 229)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(233n);
    input.add64(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 1 (241, 18444839135906371855)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(241n);
    input.add64(18444839135906371855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 2 (237, 241)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(237n);
    input.add64(241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 3 (241, 241)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(241n);
    input.add64(241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, euint64) => ebool test 4 (241, 237)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract1Address, this.signers.alice.address);
    input.add8(241n);
    input.add64(237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.gt_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract1.resEbool());
    expect(res).to.equal(true);
  });
});
