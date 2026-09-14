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

describe('FHEVM operations 9', function () {
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

  it('test operator "or" overload (uint8, euint8) => euint8 test 1 (99, 140)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(99n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(239n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 2 (33, 37)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(33n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(37n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 3 (37, 37)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(37n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(37n);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 4 (37, 33)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint8_euint8(37n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(37n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 1 (239, 5)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(239n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(234n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 2 (10, 14)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 3 (14, 14)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 14n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint8, uint8) => euint8 test 4 (14, 10)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 1 (80, 5)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(80n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(85n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 2 (10, 14)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(10n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 3 (14, 14)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(14n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 4 (14, 10)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint8_euint8(14n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 1 (221, 162)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 162n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 2 (217, 221)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(217n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 221n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 3 (221, 221)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 221n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint8, uint8) => ebool test 4 (221, 217)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(221n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint8_uint8(encryptedAmount.handles[0], 217n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 1 (123, 162)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(123n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 2 (217, 221)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(217n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 3 (221, 221)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(221n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint8, euint8) => ebool test 4 (221, 217)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_uint8_euint8(221n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 1 (151, 81)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(151n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 81n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 2 (147, 151)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(147n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 151n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 3 (151, 151)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(151n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 151n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint8, uint8) => ebool test 4 (151, 147)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(151n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint8_uint8(encryptedAmount.handles[0], 147n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 1 (218, 81)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(218n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 2 (147, 151)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(147n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 3 (151, 151)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(151n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(151n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint8, euint8) => ebool test 4 (151, 147)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_uint8_euint8(151n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (36, 47)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(36n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 47n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (32, 36)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(32n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 36n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (36, 36)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(36n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 36n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 4 (36, 32)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(36n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint8_uint8(encryptedAmount.handles[0], 32n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 1 (87, 47)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(47n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(87n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 2 (32, 36)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(36n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(32n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 3 (36, 36)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(36n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(36n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint8, euint8) => ebool test 4 (36, 32)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(32n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_uint8_euint8(36n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 1 (244, 24)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(244n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 24n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 2 (200, 204)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(200n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 204n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 3 (204, 204)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(204n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 204n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint8, uint8) => ebool test 4 (204, 200)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(204n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint8_uint8(encryptedAmount.handles[0], 200n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 1 (175, 24)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(24n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(175n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 2 (200, 204)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(200n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 3 (204, 204)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(204n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(204n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint8, euint8) => ebool test 4 (204, 200)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_uint8_euint8(204n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (212, 111)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(212n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 111n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (30, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(30n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 34n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (34, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(34n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 34n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (34, 30)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(34n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint8_uint8(encryptedAmount.handles[0], 30n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 1 (5, 111)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(111n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(5n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 2 (30, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(30n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 3 (34, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(34n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint8, euint8) => ebool test 4 (34, 30)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_uint8_euint8(34n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 1 (218, 86)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(218n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 86n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 2 (194, 198)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(194n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 198n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 3 (198, 198)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(198n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 198n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint8, uint8) => ebool test 4 (198, 194)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(198n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint8_uint8(encryptedAmount.handles[0], 194n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (112, 86)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(112n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (194, 198)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(194n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (198, 198)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(198n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (198, 194)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(194n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_uint8_euint8(198n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 1 (137, 8)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(137n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 2 (133, 137)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(133n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 137n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(133n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 3 (137, 137)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(137n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 137n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(137n);
  });

  it('test operator "min" overload (euint8, uint8) => euint8 test 4 (137, 133)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(137n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint8_uint8(encryptedAmount.handles[0], 133n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(133n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (27, 8)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(27n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(8n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (133, 137)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(133n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(133n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (137, 137)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(137n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(137n);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (137, 133)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_uint8_euint8(137n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(133n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 1 (125, 250)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(125n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 250n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(250n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 2 (11, 15)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 15n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(15n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 3 (15, 15)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(15n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 15n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(15n);
  });

  it('test operator "max" overload (euint8, uint8) => euint8 test 4 (15, 11)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add8(15n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint8_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(15n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 1 (91, 250)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(91n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(250n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 2 (11, 15)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(11n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(15n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 3 (15, 15)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(15n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(15n);
  });

  it('test operator "max" overload (uint8, euint8) => euint8 test 4 (15, 11)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_uint8_euint8(15n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint8, await this.contract5.resEuint8());
    expect(res).to.equal(15n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 1 (13756, 11971)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(13756n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 11971n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(25727n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 2 (13752, 13756)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(13752n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 13756n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27508n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 3 (13756, 13756)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(13756n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 13756n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27512n);
  });

  it('test operator "add" overload (euint16, uint16) => euint16 test 4 (13756, 13752)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(13756n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint16_uint16(encryptedAmount.handles[0], 13752n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27508n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 1 (10760, 11971)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(11971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(10760n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(22731n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 2 (13752, 13756)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(13756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(13752n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27508n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 3 (13756, 13756)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(13756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(13756n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27512n);
  });

  it('test operator "add" overload (uint16, euint16) => euint16 test 4 (13756, 13752)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(13752n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_uint16_euint16(13756n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27508n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 1 (48348, 48348)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(48348n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint16_uint16(encryptedAmount.handles[0], 48348n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, uint16) => euint16 test 2 (48348, 48344)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(48348n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint16_uint16(encryptedAmount.handles[0], 48344n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 1 (48348, 48348)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(48348n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint16_euint16(48348n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint16, euint16) => euint16 test 2 (48348, 48344)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(48344n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_uint16_euint16(48348n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (163, 138)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(163n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 138n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(22494n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (163, 163)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(163n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 163n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(26569n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (163, 163)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(163n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 163n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(26569n);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (163, 163)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(163n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint16_uint16(encryptedAmount.handles[0], 163n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(26569n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 1 (169, 138)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(138n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(169n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(23322n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 2 (163, 163)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(163n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(26569n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 3 (163, 163)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(163n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(26569n);
  });

  it('test operator "mul" overload (uint16, euint16) => euint16 test 4 (163, 163)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_uint16_euint16(163n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(26569n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (27898, 26874)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(27898n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 26874n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (27894, 27898)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(27894n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 27898n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (27898, 27898)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(27898n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 27898n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (27898, 27894)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(27898n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.div_euint16_uint16(encryptedAmount.handles[0], 27894n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 1 (41027, 40539)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(41027n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 40539n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(488n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 2 (41023, 41027)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(41023n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 41027n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(41023n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 3 (41027, 41027)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(41027n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 41027n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint16, uint16) => euint16 test 4 (41027, 41023)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(41027n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rem_euint16_uint16(encryptedAmount.handles[0], 41023n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 1 (129, 55907)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(129n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 55907n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 2 (125, 129)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(125n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 129n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 3 (129, 129)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(129n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 129n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(129n);
  });

  it('test operator "and" overload (euint16, uint16) => euint16 test 4 (129, 125)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(129n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint16_uint16(encryptedAmount.handles[0], 125n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 1 (63546, 55907)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(55907n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(63546n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(55330n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 2 (125, 129)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(125n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 3 (129, 129)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(129n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(129n);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 4 (129, 125)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(129n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(1n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 1 (34917, 28609)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(34917n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 28609n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(61413n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 2 (27584, 27588)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(27584n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 27588n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27588n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 3 (27588, 27588)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(27588n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 27588n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27588n);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 4 (27588, 27584)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(27588n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 27584n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27588n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 1 (9466, 28609)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(28609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(9466n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(28667n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 2 (27584, 27588)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(27588n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(27584n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27588n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 3 (27588, 27588)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(27588n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(27588n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27588n);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 4 (27588, 27584)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(27584n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(27588n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(27588n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 1 (16637, 60190)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 60190n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(44003n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 2 (16633, 16637)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16633n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 16637n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 3 (16637, 16637)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 16637n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 4 (16637, 16633)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(16637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 16633n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 1 (9591, 60190)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(60190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(9591n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(52841n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 2 (16633, 16637)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(16633n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 3 (16637, 16637)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(16637n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 4 (16637, 16633)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);

    input.add16(16633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(16637n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract5.resEuint16());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (56747, 879)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(56747n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 879n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (20691, 20695)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20691n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 20695n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (20695, 20695)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20695n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 20695n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (20695, 20691)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add16(20695n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 20691n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 1 (20159, 879)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(20159n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 2 (20691, 20695)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(20695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(20691n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 3 (20695, 20695)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(20695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(20695n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint16, euint16) => ebool test 4 (20695, 20691)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(20691n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint16_euint16(20695n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 1 (24928, 54474)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(24928n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 54474n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 2 (4053, 4057)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(4053n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 4057n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 3 (4057, 4057)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(4057n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 4057n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, uint16) => ebool test 4 (4057, 4053)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(4057n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint16_uint16(encryptedAmount.handles[0], 4053n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 1 (46070, 54474)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(54474n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(46070n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 2 (4053, 4057)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(4057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(4053n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 3 (4057, 4057)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(4057n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(4057n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint16, euint16) => ebool test 4 (4057, 4053)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(4053n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint16_euint16(4057n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 1 (22657, 36808)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(22657n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 36808n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 2 (22653, 22657)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(22653n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 22657n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 3 (22657, 22657)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(22657n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 22657n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, uint16) => ebool test 4 (22657, 22653)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(22657n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint16_uint16(encryptedAmount.handles[0], 22653n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 1 (60677, 36808)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(36808n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(60677n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 2 (22653, 22657)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(22657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(22653n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 3 (22657, 22657)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(22657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(22657n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint16, euint16) => ebool test 4 (22657, 22653)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(22653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint16_euint16(22657n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 1 (59175, 42694)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(59175n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 42694n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 2 (12668, 12672)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(12668n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 12672n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 3 (12672, 12672)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(12672n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 12672n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, uint16) => ebool test 4 (12672, 12668)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(12672n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint16_uint16(encryptedAmount.handles[0], 12668n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 1 (47338, 42694)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(42694n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(47338n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 2 (12668, 12672)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(12672n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(12668n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 3 (12672, 12672)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(12672n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(12672n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint16, euint16) => ebool test 4 (12672, 12668)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(12668n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint16_euint16(12672n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 1 (25325, 42114)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(25325n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 42114n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 2 (18192, 18196)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18192n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 18196n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 3 (18196, 18196)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18196n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 18196n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, uint16) => ebool test 4 (18196, 18192)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18196n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint16_uint16(encryptedAmount.handles[0], 18192n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (850, 42114)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(42114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(850n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (18192, 18196)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(18192n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (18196, 18196)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(18196n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (18196, 18192)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint16_euint16(18196n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 1 (64205, 35823)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(64205n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 35823n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 2 (56301, 56305)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(56301n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 56305n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 3 (56305, 56305)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(56305n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 56305n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, uint16) => ebool test 4 (56305, 56301)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(56305n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint16_uint16(encryptedAmount.handles[0], 56301n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 1 (34271, 35823)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(35823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(34271n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 2 (56301, 56305)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(56305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(56301n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 3 (56305, 56305)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(56305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(56305n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint16, euint16) => ebool test 4 (56305, 56301)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(56301n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint16_euint16(56305n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 1 (58529, 26425)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(58529n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 26425n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(26425n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 2 (18199, 18203)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18199n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 18203n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(18199n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 3 (18203, 18203)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18203n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 18203n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(18203n);
  });

  it('test operator "min" overload (euint16, uint16) => euint16 test 4 (18203, 18199)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(18203n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint16_uint16(encryptedAmount.handles[0], 18199n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(18199n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (4160, 26425)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(26425n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(4160n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(4160n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (18199, 18203)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(18199n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(18199n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (18203, 18203)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(18203n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(18203n);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (18203, 18199)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(18199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(18203n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(18199n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (64651, 6915)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(64651n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 6915n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(64651n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (60625, 60629)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(60625n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 60629n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(60629n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (60629, 60629)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(60629n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 60629n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(60629n);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (60629, 60625)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add16(60629n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 60625n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(60629n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (3722, 6915)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(6915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(3722n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(6915n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (60625, 60629)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(60629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(60625n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(60629n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (60629, 60629)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(60629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(60629n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(60629n);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (60629, 60625)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add16(60625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(60629n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint16, await this.contract6.resEuint16());
    expect(res).to.equal(60629n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (308575078, 2056988092)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(308575078n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      2056988092n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract6.resEuint32());
    expect(res).to.equal(2365563170n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (308575074, 308575078)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(308575074n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      308575078n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract6.resEuint32());
    expect(res).to.equal(617150152n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (308575078, 308575078)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(308575078n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      308575078n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract6.resEuint32());
    expect(res).to.equal(617150156n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (308575078, 308575074)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(308575078n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      308575074n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract6.resEuint32());
    expect(res).to.equal(617150152n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (905498101, 2056988092)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2056988092n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      905498101n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract6.resEuint32());
    expect(res).to.equal(2962486193n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (308575074, 308575078)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(308575078n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      308575074n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract6.resEuint32());
    expect(res).to.equal(617150152n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (308575078, 308575078)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(308575078n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      308575078n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract6.resEuint32());
    expect(res).to.equal(617150156n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (308575078, 308575074)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(308575074n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      308575078n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract6.resEuint32());
    expect(res).to.equal(617150152n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (827505071, 827505071)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(827505071n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_uint32(
      encryptedAmount.handles[0],
      827505071n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (827505071, 827505067)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(827505071n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_uint32(
      encryptedAmount.handles[0],
      827505067n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract6.resEuint32());
    expect(res).to.equal(4n);
  });
});
