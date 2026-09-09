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

describe('FHEVM operations 3', function () {
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

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (17823, 17823)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17823n);
    input.add32(17823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (17823, 17819)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17823n);
    input.add32(17819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (2, 27583)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add32(27583n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(55166n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (248, 249)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(248n);
    input.add32(249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(61752n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (249, 249)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(249n);
    input.add32(249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(62001n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (249, 248)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(249n);
    input.add32(248n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(61752n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (38355, 3514412921)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(38355n);
    input.add32(3514412921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(34129n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (38351, 38355)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(38351n);
    input.add32(38355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(38339n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (38355, 38355)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(38355n);
    input.add32(38355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(38355n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (38355, 38351)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(38355n);
    input.add32(38351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(38339n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (50707, 3046530735)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50707n);
    input.add32(3046530735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(3046563519n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (50703, 50707)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50703n);
    input.add32(50707n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(50719n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (50707, 50707)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50707n);
    input.add32(50707n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(50707n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (50707, 50703)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(50707n);
    input.add32(50703n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(50719n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (17678, 4281320766)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17678n);
    input.add32(4281320766n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(4281303088n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (17674, 17678)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17674n);
    input.add32(17678n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (17678, 17678)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17678n);
    input.add32(17678n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (17678, 17674)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17678n);
    input.add32(17674n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (37283, 146078640)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37283n);
    input.add32(146078640n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (37279, 37283)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37279n);
    input.add32(37283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (37283, 37283)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37283n);
    input.add32(37283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (37283, 37279)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37283n);
    input.add32(37279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (51533, 2161321973)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51533n);
    input.add32(2161321973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (51529, 51533)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51529n);
    input.add32(51533n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (51533, 51533)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51533n);
    input.add32(51533n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (51533, 51529)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51533n);
    input.add32(51529n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (43120, 3918934362)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(43120n);
    input.add32(3918934362n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (43116, 43120)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(43116n);
    input.add32(43120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (43120, 43120)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(43120n);
    input.add32(43120n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (43120, 43116)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(43120n);
    input.add32(43116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (53350, 2040212002)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53350n);
    input.add32(2040212002n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (53346, 53350)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53346n);
    input.add32(53350n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (53350, 53350)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53350n);
    input.add32(53350n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (53350, 53346)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53350n);
    input.add32(53346n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (39114, 2957921879)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39114n);
    input.add32(2957921879n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (39110, 39114)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39110n);
    input.add32(39114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (39114, 39114)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39114n);
    input.add32(39114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (39114, 39110)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39114n);
    input.add32(39110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (14807, 2824712233)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(14807n);
    input.add32(2824712233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (14803, 14807)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(14803n);
    input.add32(14807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (14807, 14807)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(14807n);
    input.add32(14807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (14807, 14803)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(14807n);
    input.add32(14803n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (21351, 328331938)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21351n);
    input.add32(328331938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(21351n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (21347, 21351)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21347n);
    input.add32(21351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(21347n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (21351, 21351)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21351n);
    input.add32(21351n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(21351n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (21351, 21347)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(21351n);
    input.add32(21347n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(21347n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (16090, 2930327358)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16090n);
    input.add32(2930327358n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(2930327358n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (16086, 16090)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16086n);
    input.add32(16090n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(16090n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (16090, 16090)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16090n);
    input.add32(16090n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(16090n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (16090, 16086)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(16090n);
    input.add32(16086n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(16090n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (2, 65516)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add64(65516n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(65518n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (12418, 12422)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12418n);
    input.add64(12422n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(24840n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (12422, 12422)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12422n);
    input.add64(12422n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(24844n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (12422, 12418)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(12422n);
    input.add64(12418n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(24840n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (39196, 39196)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39196n);
    input.add64(39196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (39196, 39192)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(39196n);
    input.add64(39192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (2, 32757)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add64(32757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(65514n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (254, 254)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(254n);
    input.add64(254n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(64516n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (254, 254)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(254n);
    input.add64(254n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(64516n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (254, 254)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(254n);
    input.add64(254n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(64516n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (24454, 18442097008169977087)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24454n);
    input.add64(18442097008169977087n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(1158n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (24450, 24454)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24450n);
    input.add64(24454n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(24450n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (24454, 24454)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24454n);
    input.add64(24454n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(24454n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (24454, 24450)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(24454n);
    input.add64(24450n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(24450n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (36052, 18439235578679962485)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36052n);
    input.add64(18439235578679962485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(18439235578679963637n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (36048, 36052)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36048n);
    input.add64(36052n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(36052n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (36052, 36052)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36052n);
    input.add64(36052n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(36052n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (36052, 36048)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(36052n);
    input.add64(36048n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(36052n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (15806, 18440069628500100245)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15806n);
    input.add64(18440069628500100245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(18440069628500113707n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (15802, 15806)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15802n);
    input.add64(15806n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (15806, 15806)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15806n);
    input.add64(15806n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (15806, 15802)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(15806n);
    input.add64(15802n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (43112, 18438145067791250747)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(43112n);
    input.add64(18438145067791250747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (43108, 43112)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(43108n);
    input.add64(43112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (43112, 43112)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(43112n);
    input.add64(43112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (43112, 43108)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(43112n);
    input.add64(43108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (9302, 18444273334839440731)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9302n);
    input.add64(18444273334839440731n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (9298, 9302)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9298n);
    input.add64(9302n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (9302, 9302)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9302n);
    input.add64(9302n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (9302, 9298)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(9302n);
    input.add64(9298n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (63664, 18444928696114503107)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63664n);
    input.add64(18444928696114503107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (63660, 63664)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63660n);
    input.add64(63664n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (63664, 63664)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63664n);
    input.add64(63664n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (63664, 63660)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(63664n);
    input.add64(63660n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (17460, 18444655219557170751)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17460n);
    input.add64(18444655219557170751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (17456, 17460)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17456n);
    input.add64(17460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (17460, 17460)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17460n);
    input.add64(17460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (17460, 17456)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(17460n);
    input.add64(17456n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (28401, 18443005890184320847)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(28401n);
    input.add64(18443005890184320847n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (28397, 28401)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(28397n);
    input.add64(28401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (28401, 28401)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(28401n);
    input.add64(28401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (28401, 28397)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(28401n);
    input.add64(28397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (1001, 18440471599208664139)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(1001n);
    input.add64(18440471599208664139n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (997, 1001)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(997n);
    input.add64(1001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (1001, 1001)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(1001n);
    input.add64(1001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (1001, 997)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(1001n);
    input.add64(997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (37593, 18442484881376073199)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37593n);
    input.add64(18442484881376073199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(37593n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (37589, 37593)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37589n);
    input.add64(37593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(37589n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (37593, 37593)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37593n);
    input.add64(37593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(37593n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (37593, 37589)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37593n);
    input.add64(37589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(37589n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (51474, 18442485804632668521)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51474n);
    input.add64(18442485804632668521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(18442485804632668521n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (51470, 51474)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51470n);
    input.add64(51474n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(51474n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (51474, 51474)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51474n);
    input.add64(51474n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(51474n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (51474, 51470)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51474n);
    input.add64(51470n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract2.resEuint64());
    expect(res).to.equal(51474n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 1 (2, 32769)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add128(32769n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 2 (22240, 22244)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22240n);
    input.add128(22244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(44484n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 3 (22244, 22244)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22244n);
    input.add128(22244n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(44488n);
  });

  it('test operator "add" overload (euint16, euint128) => euint128 test 4 (22244, 22240)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22244n);
    input.add128(22240n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(44484n);
  });

  it('test operator "sub" overload (euint16, euint128) => euint128 test 1 (61789, 61789)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(61789n);
    input.add128(61789n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint128) => euint128 test 2 (61789, 61785)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(61789n);
    input.add128(61785n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 1 (2, 16385)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(2n);
    input.add128(16385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 2 (181, 181)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(181n);
    input.add128(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(32761n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 3 (181, 181)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(181n);
    input.add128(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(32761n);
  });

  it('test operator "mul" overload (euint16, euint128) => euint128 test 4 (181, 181)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(181n);
    input.add128(181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(32761n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 1 (26934, 340282366920938463463368136312298369445)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26934n);
    input.add128(340282366920938463463368136312298369445n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(24868n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 2 (26930, 26934)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26930n);
    input.add128(26934n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(26930n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 3 (26934, 26934)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26934n);
    input.add128(26934n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(26934n);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 4 (26934, 26930)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(26934n);
    input.add128(26930n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(26930n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 1 (35626, 340282366920938463463367367548823798633)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35626n);
    input.add128(340282366920938463463367367548823798633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(340282366920938463463367367548823833451n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 2 (35622, 35626)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35622n);
    input.add128(35626n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(35630n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 3 (35626, 35626)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35626n);
    input.add128(35626n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(35626n);
  });

  it('test operator "or" overload (euint16, euint128) => euint128 test 4 (35626, 35622)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(35626n);
    input.add128(35622n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(35630n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 1 (51696, 340282366920938463463365858706383788715)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51696n);
    input.add128(340282366920938463463365858706383788715n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(340282366920938463463365858706383803227n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 2 (51692, 51696)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51692n);
    input.add128(51696n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 3 (51696, 51696)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51696n);
    input.add128(51696n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 4 (51696, 51692)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(51696n);
    input.add128(51692n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 1 (201, 340282366920938463463371348786025383095)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(201n);
    input.add128(340282366920938463463371348786025383095n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 2 (197, 201)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(197n);
    input.add128(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 3 (201, 201)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(201n);
    input.add128(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint128) => ebool test 4 (201, 197)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(201n);
    input.add128(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 1 (32481, 340282366920938463463368312324551912537)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32481n);
    input.add128(340282366920938463463368312324551912537n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 2 (32477, 32481)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32477n);
    input.add128(32481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 3 (32481, 32481)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32481n);
    input.add128(32481n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint128) => ebool test 4 (32481, 32477)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(32481n);
    input.add128(32477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 1 (37196, 340282366920938463463371691844692065385)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37196n);
    input.add128(340282366920938463463371691844692065385n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 2 (37192, 37196)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37192n);
    input.add128(37196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 3 (37196, 37196)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37196n);
    input.add128(37196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint128) => ebool test 4 (37196, 37192)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(37196n);
    input.add128(37192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ge_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 1 (3789, 340282366920938463463373501551957981423)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3789n);
    input.add128(340282366920938463463373501551957981423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 2 (3785, 3789)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3785n);
    input.add128(3789n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 3 (3789, 3789)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3789n);
    input.add128(3789n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint128) => ebool test 4 (3789, 3785)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(3789n);
    input.add128(3785n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 1 (60496, 340282366920938463463373629795867414229)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60496n);
    input.add128(340282366920938463463373629795867414229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 2 (60492, 60496)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60492n);
    input.add128(60496n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 3 (60496, 60496)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60496n);
    input.add128(60496n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint128) => ebool test 4 (60496, 60492)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(60496n);
    input.add128(60492n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 1 (47407, 340282366920938463463371089031505239981)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(47407n);
    input.add128(340282366920938463463371089031505239981n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 2 (47403, 47407)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(47403n);
    input.add128(47407n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 3 (47407, 47407)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(47407n);
    input.add128(47407n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint128) => ebool test 4 (47407, 47403)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(47407n);
    input.add128(47403n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 1 (23312, 340282366920938463463367062133032109037)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23312n);
    input.add128(340282366920938463463367062133032109037n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(23312n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 2 (23308, 23312)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23308n);
    input.add128(23312n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(23308n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 3 (23312, 23312)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23312n);
    input.add128(23312n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(23312n);
  });

  it('test operator "min" overload (euint16, euint128) => euint128 test 4 (23312, 23308)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(23312n);
    input.add128(23308n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(23308n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 1 (22856, 340282366920938463463371547066021869219)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22856n);
    input.add128(340282366920938463463371547066021869219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(340282366920938463463371547066021869219n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 2 (22852, 22856)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22852n);
    input.add128(22856n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(22856n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 3 (22856, 22856)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22856n);
    input.add128(22856n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(22856n);
  });

  it('test operator "max" overload (euint16, euint128) => euint128 test 4 (22856, 22852)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(22856n);
    input.add128(22852n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract2.resEuint128());
    expect(res).to.equal(22856n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 1 (18525, 115792089237316195423570985008687907853269984665640564039457578381450541511597)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18525n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578381450541511597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(13n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 2 (18521, 18525)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18521n);
    input.add256(18525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(18521n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 3 (18525, 18525)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18525n);
    input.add256(18525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(18525n);
  });

  it('test operator "and" overload (euint16, euint256) => euint256 test 4 (18525, 18521)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(18525n);
    input.add256(18521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(18521n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 1 (45787, 115792089237316195423570985008687907853269984665640564039457582122953227843443)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45787n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582122953227843443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582122953227876347n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 2 (45783, 45787)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45783n);
    input.add256(45787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(45791n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 3 (45787, 45787)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45787n);
    input.add256(45787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(45787n);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 4 (45787, 45783)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(45787n);
    input.add256(45783n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(45791n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 1 (57659, 115792089237316195423570985008687907853269984665640564039457575065063120537073)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57659n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575065063120537073n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575065063120528586n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 2 (57655, 57659)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57655n);
    input.add256(57659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 3 (57659, 57659)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57659n);
    input.add256(57659n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint256) => euint256 test 4 (57659, 57655)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(57659n);
    input.add256(57655n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract2.resEuint256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 1 (49525, 115792089237316195423570985008687907853269984665640564039457581621960944525773)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49525n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581621960944525773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 2 (49521, 49525)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49521n);
    input.add256(49525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 3 (49525, 49525)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49525n);
    input.add256(49525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint256) => ebool test 4 (49525, 49521)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(49525n);
    input.add256(49521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 1 (53899, 115792089237316195423570985008687907853269984665640564039457583238318790175429)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53899n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583238318790175429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 2 (53895, 53899)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53895n);
    input.add256(53899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 3 (53899, 53899)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53899n);
    input.add256(53899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint256) => ebool test 4 (53899, 53895)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add16(53899n);
    input.add256(53895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.ne_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract2.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 1 (154, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(154n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(156n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 2 (72, 74)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(72n);
    input.add8(74n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(146n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 3 (74, 74)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(74n);
    input.add8(74n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(148n);
  });

  it('test operator "add" overload (euint32, euint8) => euint32 test 4 (74, 72)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(74n);
    input.add8(72n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(146n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 1 (175, 175)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(175n);
    input.add8(175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint8) => euint32 test 2 (175, 171)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(175n);
    input.add8(171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (93, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(93n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(186n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (9, 9)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (9, 9)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (9, 9)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(81n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (3355410399, 50)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(3355410399n);
    input.add8(50n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(18n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (46, 50)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(46n);
    input.add8(50n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(34n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (50, 50)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(50n);
    input.add8(50n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(50n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (50, 46)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(50n);
    input.add8(46n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(34n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (1238476899, 187)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1238476899n);
    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(1238477051n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (183, 187)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(183n);
    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(191n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (187, 187)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(187n);
    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(187n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (187, 183)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(187n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(191n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (1557932409, 52)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(1557932409n);
    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(1557932365n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (48, 52)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(48n);
    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (52, 52)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(52n);
    input.add8(52n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (52, 48)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract2Address, this.signers.alice.address);
    input.add32(52n);
    input.add8(48n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint32, await this.contract2.resEuint32());
    expect(res).to.equal(4n);
  });
});
