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

describe('FHEVM operations 7', function () {
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

  it('test operator "min" overload (euint128, euint8) => euint128 test 1 (340282366920938463463374373941804259365, 20)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374373941804259365n);
    input.add8(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(20n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 2 (16, 20)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(16n);
    input.add8(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(16n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 3 (20, 20)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(20n);
    input.add8(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(20n);
  });

  it('test operator "min" overload (euint128, euint8) => euint128 test 4 (20, 16)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(20n);
    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(16n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 1 (340282366920938463463370472029042344271, 59)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370472029042344271n);
    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370472029042344271n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 2 (55, 59)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(55n);
    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(59n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 3 (59, 59)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(59n);
    input.add8(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(59n);
  });

  it('test operator "max" overload (euint128, euint8) => euint128 test 4 (59, 55)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(59n);
    input.add8(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(59n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 1 (32769, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(32769n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(32771n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 2 (27954, 27958)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(27954n);
    input.add16(27958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(55912n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 3 (27958, 27958)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(27958n);
    input.add16(27958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(55916n);
  });

  it('test operator "add" overload (euint128, euint16) => euint128 test 4 (27958, 27954)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(27958n);
    input.add16(27954n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(55912n);
  });

  it('test operator "sub" overload (euint128, euint16) => euint128 test 1 (60117, 60117)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(60117n);
    input.add16(60117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint16) => euint128 test 2 (60117, 60113)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(60117n);
    input.add16(60113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 1 (16385, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(16385n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 2 (229, 229)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(229n);
    input.add16(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(52441n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 3 (229, 229)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(229n);
    input.add16(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(52441n);
  });

  it('test operator "mul" overload (euint128, euint16) => euint128 test 4 (229, 229)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(229n);
    input.add16(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(52441n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 1 (340282366920938463463371364911078032651, 59893)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371364911078032651n);
    input.add16(59893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(49409n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 2 (59889, 59893)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(59889n);
    input.add16(59893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(59889n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 3 (59893, 59893)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(59893n);
    input.add16(59893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(59893n);
  });

  it('test operator "and" overload (euint128, euint16) => euint128 test 4 (59893, 59889)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(59893n);
    input.add16(59889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(59889n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 1 (340282366920938463463372837720621820295, 64239)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372837720621820295n);
    input.add16(64239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463372837720621841391n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 2 (64235, 64239)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(64235n);
    input.add16(64239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(64239n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 3 (64239, 64239)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(64239n);
    input.add16(64239n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(64239n);
  });

  it('test operator "or" overload (euint128, euint16) => euint128 test 4 (64239, 64235)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(64239n);
    input.add16(64235n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(64239n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 1 (340282366920938463463372571988245566461, 28379)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372571988245566461n);
    input.add16(28379n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463372571988245560614n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 2 (28375, 28379)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(28375n);
    input.add16(28379n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 3 (28379, 28379)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(28379n);
    input.add16(28379n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 4 (28379, 28375)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(28379n);
    input.add16(28375n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 1 (340282366920938463463367978003322074553, 65222)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367978003322074553n);
    input.add16(65222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 2 (65218, 65222)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(65218n);
    input.add16(65222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 3 (65222, 65222)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(65222n);
    input.add16(65222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint16) => ebool test 4 (65222, 65218)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(65222n);
    input.add16(65218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 1 (340282366920938463463372536440380135105, 33682)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372536440380135105n);
    input.add16(33682n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 2 (33678, 33682)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(33678n);
    input.add16(33682n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 3 (33682, 33682)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(33682n);
    input.add16(33682n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint16) => ebool test 4 (33682, 33678)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(33682n);
    input.add16(33678n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 1 (340282366920938463463370573200526312211, 62578)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370573200526312211n);
    input.add16(62578n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 2 (62574, 62578)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(62574n);
    input.add16(62578n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 3 (62578, 62578)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(62578n);
    input.add16(62578n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 4 (62578, 62574)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(62578n);
    input.add16(62574n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 1 (340282366920938463463372705928695736385, 44945)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372705928695736385n);
    input.add16(44945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 2 (44941, 44945)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(44941n);
    input.add16(44945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 3 (44945, 44945)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(44945n);
    input.add16(44945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint16) => ebool test 4 (44945, 44941)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(44945n);
    input.add16(44941n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 1 (340282366920938463463368882639871317153, 16518)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368882639871317153n);
    input.add16(16518n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 2 (16514, 16518)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(16514n);
    input.add16(16518n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 3 (16518, 16518)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(16518n);
    input.add16(16518n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint16) => ebool test 4 (16518, 16514)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(16518n);
    input.add16(16514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 1 (340282366920938463463367312573489800843, 38303)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367312573489800843n);
    input.add16(38303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 2 (38299, 38303)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(38299n);
    input.add16(38303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 3 (38303, 38303)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(38303n);
    input.add16(38303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint16) => ebool test 4 (38303, 38299)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(38303n);
    input.add16(38299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 1 (340282366920938463463370244165272802687, 28746)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370244165272802687n);
    input.add16(28746n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(28746n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 2 (28742, 28746)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(28742n);
    input.add16(28746n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(28742n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 3 (28746, 28746)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(28746n);
    input.add16(28746n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(28746n);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 4 (28746, 28742)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(28746n);
    input.add16(28742n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(28742n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 1 (340282366920938463463368887445785260461, 3915)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368887445785260461n);
    input.add16(3915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463368887445785260461n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 2 (3911, 3915)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3911n);
    input.add16(3915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(3915n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 3 (3915, 3915)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3915n);
    input.add16(3915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(3915n);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 4 (3915, 3911)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3915n);
    input.add16(3911n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(3915n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 1 (2147483649, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2147483649n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 2 (1640687698, 1640687702)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1640687698n);
    input.add32(1640687702n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(3281375400n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 3 (1640687702, 1640687702)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1640687702n);
    input.add32(1640687702n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(3281375404n);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 4 (1640687702, 1640687698)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1640687702n);
    input.add32(1640687698n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(3281375400n);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 1 (1246942894, 1246942894)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1246942894n);
    input.add32(1246942894n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 2 (1246942894, 1246942890)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1246942894n);
    input.add32(1246942890n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 1 (1073741825, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1073741825n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 2 (64981, 64981)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(64981n);
    input.add32(64981n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4222530361n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 3 (64981, 64981)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(64981n);
    input.add32(64981n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4222530361n);
  });

  it('test operator "mul" overload (euint128, euint32) => euint128 test 4 (64981, 64981)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(64981n);
    input.add32(64981n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4222530361n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 1 (340282366920938463463366385249843627577, 4141348882)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366385249843627577n);
    input.add32(4141348882n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(370360336n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 2 (4141348878, 4141348882)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4141348878n);
    input.add32(4141348882n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4141348866n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 3 (4141348882, 4141348882)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4141348882n);
    input.add32(4141348882n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4141348882n);
  });

  it('test operator "and" overload (euint128, euint32) => euint128 test 4 (4141348882, 4141348878)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4141348882n);
    input.add32(4141348878n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4141348866n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 1 (340282366920938463463367331600429373593, 2842824901)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367331600429373593n);
    input.add32(2842824901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463367331601106753757n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 2 (2842824897, 2842824901)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2842824897n);
    input.add32(2842824901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(2842824901n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 3 (2842824901, 2842824901)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2842824901n);
    input.add32(2842824901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(2842824901n);
  });

  it('test operator "or" overload (euint128, euint32) => euint128 test 4 (2842824901, 2842824897)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2842824901n);
    input.add32(2842824897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(2842824901n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 1 (340282366920938463463366639372694525051, 1841057854)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366639372694525051n);
    input.add32(1841057854n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366639373021955141n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 2 (1841057850, 1841057854)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1841057850n);
    input.add32(1841057854n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 3 (1841057854, 1841057854)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1841057854n);
    input.add32(1841057854n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint32) => euint128 test 4 (1841057854, 1841057850)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1841057854n);
    input.add32(1841057850n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 1 (340282366920938463463367269130716522379, 372945808)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367269130716522379n);
    input.add32(372945808n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 2 (372945804, 372945808)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(372945804n);
    input.add32(372945808n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 3 (372945808, 372945808)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(372945808n);
    input.add32(372945808n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint32) => ebool test 4 (372945808, 372945804)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(372945808n);
    input.add32(372945804n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 1 (340282366920938463463370820091836516821, 2340468025)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370820091836516821n);
    input.add32(2340468025n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 2 (2340468021, 2340468025)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2340468021n);
    input.add32(2340468025n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 3 (2340468025, 2340468025)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2340468025n);
    input.add32(2340468025n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 4 (2340468025, 2340468021)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2340468025n);
    input.add32(2340468021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 1 (340282366920938463463369094660966137149, 2249851938)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369094660966137149n);
    input.add32(2249851938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 2 (2249851934, 2249851938)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2249851934n);
    input.add32(2249851938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 3 (2249851938, 2249851938)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2249851938n);
    input.add32(2249851938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint32) => ebool test 4 (2249851938, 2249851934)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(2249851938n);
    input.add32(2249851934n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 1 (340282366920938463463370036970340162527, 263842749)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370036970340162527n);
    input.add32(263842749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 2 (263842745, 263842749)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(263842745n);
    input.add32(263842749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 3 (263842749, 263842749)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(263842749n);
    input.add32(263842749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 4 (263842749, 263842745)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(263842749n);
    input.add32(263842745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 1 (340282366920938463463368645948758020821, 3868041185)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368645948758020821n);
    input.add32(3868041185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 2 (3868041181, 3868041185)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3868041181n);
    input.add32(3868041185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 3 (3868041185, 3868041185)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3868041185n);
    input.add32(3868041185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint32) => ebool test 4 (3868041185, 3868041181)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3868041185n);
    input.add32(3868041181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 1 (340282366920938463463371106202586075549, 3870932958)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371106202586075549n);
    input.add32(3870932958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 2 (3870932954, 3870932958)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3870932954n);
    input.add32(3870932958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 3 (3870932958, 3870932958)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3870932958n);
    input.add32(3870932958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint32) => ebool test 4 (3870932958, 3870932954)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(3870932958n);
    input.add32(3870932954n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 1 (340282366920938463463367426817413069917, 161812629)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367426817413069917n);
    input.add32(161812629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(161812629n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 2 (161812625, 161812629)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(161812625n);
    input.add32(161812629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(161812625n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 3 (161812629, 161812629)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(161812629n);
    input.add32(161812629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(161812629n);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 4 (161812629, 161812625)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(161812629n);
    input.add32(161812625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(161812625n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 1 (340282366920938463463368009494511989125, 1691838048)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368009494511989125n);
    input.add32(1691838048n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463368009494511989125n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 2 (1691838044, 1691838048)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1691838044n);
    input.add32(1691838048n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(1691838048n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 3 (1691838048, 1691838048)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1691838048n);
    input.add32(1691838048n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(1691838048n);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 4 (1691838048, 1691838044)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1691838048n);
    input.add32(1691838044n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(1691838048n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 1 (9223372036854775809, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 2 (9223133733725592012, 9223133733725592014)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223133733725592012n);
    input.add64(9223133733725592014n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18446267467451184026n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 3 (9223133733725592014, 9223133733725592014)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223133733725592014n);
    input.add64(9223133733725592014n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18446267467451184028n);
  });

  it('test operator "add" overload (euint128, euint64) => euint128 test 4 (9223133733725592014, 9223133733725592012)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223133733725592014n);
    input.add64(9223133733725592012n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18446267467451184026n);
  });

  it('test operator "sub" overload (euint128, euint64) => euint128 test 1 (18439741790219698213, 18439741790219698213)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439741790219698213n);
    input.add64(18439741790219698213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint64) => euint128 test 2 (18439741790219698213, 18439741790219698209)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439741790219698213n);
    input.add64(18439741790219698209n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 1 (4611686018427387905, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4611686018427387905n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 2 (4294514001, 4294514001)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294514001n);
    input.add64(4294514001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18442850504785028001n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 3 (4294514001, 4294514001)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294514001n);
    input.add64(4294514001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18442850504785028001n);
  });

  it('test operator "mul" overload (euint128, euint64) => euint128 test 4 (4294514001, 4294514001)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(4294514001n);
    input.add64(4294514001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18442850504785028001n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 1 (340282366920938463463371460613677378205, 18445585597076779761)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371460613677378205n);
    input.add64(18445585597076779761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18442451975476912785n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 2 (18445585597076779757, 18445585597076779761)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445585597076779757n);
    input.add64(18445585597076779761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18445585597076779745n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 3 (18445585597076779761, 18445585597076779761)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445585597076779761n);
    input.add64(18445585597076779761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18445585597076779761n);
  });

  it('test operator "and" overload (euint128, euint64) => euint128 test 4 (18445585597076779761, 18445585597076779757)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445585597076779761n);
    input.add64(18445585597076779757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18445585597076779745n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 1 (340282366920938463463368227616290696921, 18442643236153141279)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368227616290696921n);
    input.add64(18442643236153141279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463372777704811917023n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 2 (18442643236153141275, 18442643236153141279)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18442643236153141275n);
    input.add64(18442643236153141279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18442643236153141279n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 3 (18442643236153141279, 18442643236153141279)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18442643236153141279n);
    input.add64(18442643236153141279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18442643236153141279n);
  });

  it('test operator "or" overload (euint128, euint64) => euint128 test 4 (18442643236153141279, 18442643236153141275)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18442643236153141279n);
    input.add64(18442643236153141275n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18442643236153141279n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 1 (340282366920938463463369911388659828705, 18439443082590123897)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369911388659828705n);
    input.add64(18439443082590123897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463444930501296026136728n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 2 (18439443082590123893, 18439443082590123897)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439443082590123893n);
    input.add64(18439443082590123897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 3 (18439443082590123897, 18439443082590123897)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439443082590123897n);
    input.add64(18439443082590123897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint64) => euint128 test 4 (18439443082590123897, 18439443082590123893)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439443082590123897n);
    input.add64(18439443082590123893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 1 (340282366920938463463367729458279879823, 18446418476382821975)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367729458279879823n);
    input.add64(18446418476382821975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 2 (18446418476382821971, 18446418476382821975)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446418476382821971n);
    input.add64(18446418476382821975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 3 (18446418476382821975, 18446418476382821975)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446418476382821975n);
    input.add64(18446418476382821975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint64) => ebool test 4 (18446418476382821975, 18446418476382821971)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446418476382821975n);
    input.add64(18446418476382821971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 1 (340282366920938463463369925452909868907, 18441898098109877277)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369925452909868907n);
    input.add64(18441898098109877277n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 2 (18441898098109877273, 18441898098109877277)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441898098109877273n);
    input.add64(18441898098109877277n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 3 (18441898098109877277, 18441898098109877277)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441898098109877277n);
    input.add64(18441898098109877277n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint64) => ebool test 4 (18441898098109877277, 18441898098109877273)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18441898098109877277n);
    input.add64(18441898098109877273n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 1 (340282366920938463463370176900705427611, 18440255437118330271)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370176900705427611n);
    input.add64(18440255437118330271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 2 (18440255437118330267, 18440255437118330271)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440255437118330267n);
    input.add64(18440255437118330271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 3 (18440255437118330271, 18440255437118330271)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440255437118330271n);
    input.add64(18440255437118330271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 4 (18440255437118330271, 18440255437118330267)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440255437118330271n);
    input.add64(18440255437118330267n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 1 (340282366920938463463366238909600213739, 18446298522319998359)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366238909600213739n);
    input.add64(18446298522319998359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 2 (18446298522319998355, 18446298522319998359)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446298522319998355n);
    input.add64(18446298522319998359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 3 (18446298522319998359, 18446298522319998359)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446298522319998359n);
    input.add64(18446298522319998359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint64) => ebool test 4 (18446298522319998359, 18446298522319998355)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18446298522319998359n);
    input.add64(18446298522319998355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 1 (340282366920938463463365917627078018949, 18439032567082972609)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463365917627078018949n);
    input.add64(18439032567082972609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 2 (18439032567082972605, 18439032567082972609)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439032567082972605n);
    input.add64(18439032567082972609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 3 (18439032567082972609, 18439032567082972609)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439032567082972609n);
    input.add64(18439032567082972609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint64) => ebool test 4 (18439032567082972609, 18439032567082972605)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18439032567082972609n);
    input.add64(18439032567082972605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 1 (340282366920938463463374172085869425145, 18443206958285092697)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463374172085869425145n);
    input.add64(18443206958285092697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 2 (18443206958285092693, 18443206958285092697)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18443206958285092693n);
    input.add64(18443206958285092697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 3 (18443206958285092697, 18443206958285092697)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18443206958285092697n);
    input.add64(18443206958285092697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint64) => ebool test 4 (18443206958285092697, 18443206958285092693)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18443206958285092697n);
    input.add64(18443206958285092693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 1 (340282366920938463463372793588655024635, 18440669903957711279)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372793588655024635n);
    input.add64(18440669903957711279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18440669903957711279n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 2 (18440669903957711275, 18440669903957711279)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440669903957711275n);
    input.add64(18440669903957711279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18440669903957711275n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 3 (18440669903957711279, 18440669903957711279)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440669903957711279n);
    input.add64(18440669903957711279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18440669903957711279n);
  });

  it('test operator "min" overload (euint128, euint64) => euint128 test 4 (18440669903957711279, 18440669903957711275)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18440669903957711279n);
    input.add64(18440669903957711275n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18440669903957711275n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 1 (340282366920938463463370714318464992859, 18445224296450112107)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370714318464992859n);
    input.add64(18445224296450112107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370714318464992859n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 2 (18445224296450112103, 18445224296450112107)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445224296450112103n);
    input.add64(18445224296450112107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18445224296450112107n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 3 (18445224296450112107, 18445224296450112107)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445224296450112107n);
    input.add64(18445224296450112107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18445224296450112107n);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 4 (18445224296450112107, 18445224296450112103)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(18445224296450112107n);
    input.add64(18445224296450112103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18445224296450112107n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 1 (170141183460469231731686721535437342948, 170141183460469231731685124726320525391)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731686721535437342948n);
    input.add128(170141183460469231731685124726320525391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463371846261757868339n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 2 (170141183460469231731685124726320525389, 170141183460469231731685124726320525391)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731685124726320525389n);
    input.add128(170141183460469231731685124726320525391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370249452641050780n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 3 (170141183460469231731685124726320525391, 170141183460469231731685124726320525391)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731685124726320525391n);
    input.add128(170141183460469231731685124726320525391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370249452641050782n);
  });

  it('test operator "add" overload (euint128, euint128) => euint128 test 4 (170141183460469231731685124726320525391, 170141183460469231731685124726320525389)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(170141183460469231731685124726320525391n);
    input.add128(170141183460469231731685124726320525389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370249452641050780n);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 1 (340282366920938463463370119270961786529, 340282366920938463463370119270961786529)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370119270961786529n);
    input.add128(340282366920938463463370119270961786529n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint128) => euint128 test 2 (340282366920938463463370119270961786529, 340282366920938463463370119270961786525)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370119270961786529n);
    input.add128(340282366920938463463370119270961786525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 1 (9223372036854775809, 9223372036854775809)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint128, euint128) => euint128 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 1 (340282366920938463463367044172939119359, 340282366920938463463370687886919745557)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367044172939119359n);
    input.add128(340282366920938463463370687886919745557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463365601338738098197n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 2 (340282366920938463463367044172939119355, 340282366920938463463367044172939119359)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367044172939119355n);
    input.add128(340282366920938463463367044172939119359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463367044172939119355n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 3 (340282366920938463463367044172939119359, 340282366920938463463367044172939119359)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367044172939119359n);
    input.add128(340282366920938463463367044172939119359n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463367044172939119359n);
  });

  it('test operator "and" overload (euint128, euint128) => euint128 test 4 (340282366920938463463367044172939119359, 340282366920938463463367044172939119355)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367044172939119359n);
    input.add128(340282366920938463463367044172939119355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463367044172939119355n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 1 (340282366920938463463373650833185201525, 340282366920938463463367653969196802045)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463373650833185201525n);
    input.add128(340282366920938463463367653969196802045n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463374567844475625469n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 2 (340282366920938463463367653969196802041, 340282366920938463463367653969196802045)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367653969196802041n);
    input.add128(340282366920938463463367653969196802045n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463367653969196802045n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 3 (340282366920938463463367653969196802045, 340282366920938463463367653969196802045)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367653969196802045n);
    input.add128(340282366920938463463367653969196802045n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463367653969196802045n);
  });

  it('test operator "or" overload (euint128, euint128) => euint128 test 4 (340282366920938463463367653969196802045, 340282366920938463463367653969196802041)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367653969196802045n);
    input.add128(340282366920938463463367653969196802041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract5.resEuint128());
    expect(res).to.equal(340282366920938463463367653969196802045n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 1 (340282366920938463463372811472428286857, 340282366920938463463367127915705168171)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463372811472428286857n);
    input.add128(340282366920938463463367127915705168171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract5.resEuint128());
    expect(res).to.equal(8149575159984802n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 2 (340282366920938463463367127915705168167, 340282366920938463463367127915705168171)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367127915705168167n);
    input.add128(340282366920938463463367127915705168171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract5.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 3 (340282366920938463463367127915705168171, 340282366920938463463367127915705168171)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367127915705168171n);
    input.add128(340282366920938463463367127915705168171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract5.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint128) => euint128 test 4 (340282366920938463463367127915705168171, 340282366920938463463367127915705168167)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463367127915705168171n);
    input.add128(340282366920938463463367127915705168167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract5.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 1 (340282366920938463463372481304885606321, 340282366920938463463368792243012180477)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463372481304885606321n);
    input.add128(340282366920938463463368792243012180477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 2 (340282366920938463463368792243012180473, 340282366920938463463368792243012180477)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368792243012180473n);
    input.add128(340282366920938463463368792243012180477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 3 (340282366920938463463368792243012180477, 340282366920938463463368792243012180477)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368792243012180477n);
    input.add128(340282366920938463463368792243012180477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint128) => ebool test 4 (340282366920938463463368792243012180477, 340282366920938463463368792243012180473)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368792243012180477n);
    input.add128(340282366920938463463368792243012180473n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 1 (340282366920938463463369221426186835803, 340282366920938463463368143814524497097)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463369221426186835803n);
    input.add128(340282366920938463463368143814524497097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 2 (340282366920938463463368143814524497093, 340282366920938463463368143814524497097)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368143814524497093n);
    input.add128(340282366920938463463368143814524497097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 3 (340282366920938463463368143814524497097, 340282366920938463463368143814524497097)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368143814524497097n);
    input.add128(340282366920938463463368143814524497097n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint128) => ebool test 4 (340282366920938463463368143814524497097, 340282366920938463463368143814524497093)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add128(340282366920938463463368143814524497097n);
    input.add128(340282366920938463463368143814524497093n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint128_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract5.resEbool());
    expect(res).to.equal(true);
  });
});
