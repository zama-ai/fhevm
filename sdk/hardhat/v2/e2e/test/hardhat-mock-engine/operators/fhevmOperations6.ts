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

describe('FHEVM operations 6', function () {
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

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (18444576373012027697, 1140317332)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444576373012027697n);
    input.add32(1140317332n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (1140317328, 1140317332)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1140317328n);
    input.add32(1140317332n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (1140317332, 1140317332)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1140317332n);
    input.add32(1140317332n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (1140317332, 1140317328)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1140317332n);
    input.add32(1140317328n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (18439259678436532215, 1723718226)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439259678436532215n);
    input.add32(1723718226n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (1723718222, 1723718226)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1723718222n);
    input.add32(1723718226n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (1723718226, 1723718226)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1723718226n);
    input.add32(1723718226n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (1723718226, 1723718222)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1723718226n);
    input.add32(1723718222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (18445169967836092707, 1446091330)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445169967836092707n);
    input.add32(1446091330n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (1446091326, 1446091330)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1446091326n);
    input.add32(1446091330n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (1446091330, 1446091330)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1446091330n);
    input.add32(1446091330n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (1446091330, 1446091326)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1446091330n);
    input.add32(1446091326n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (18444653208367655615, 2974365445)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444653208367655615n);
    input.add32(2974365445n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (2974365441, 2974365445)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2974365441n);
    input.add32(2974365445n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (2974365445, 2974365445)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2974365445n);
    input.add32(2974365445n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (2974365445, 2974365441)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2974365445n);
    input.add32(2974365441n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (18446510856700067007, 2143779174)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18446510856700067007n);
    input.add32(2143779174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (2143779170, 2143779174)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2143779170n);
    input.add32(2143779174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (2143779174, 2143779174)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2143779174n);
    input.add32(2143779174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (2143779174, 2143779170)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2143779174n);
    input.add32(2143779170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18441403127050779527, 3217537777)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18441403127050779527n);
    input.add32(3217537777n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(3217537777n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (3217537773, 3217537777)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3217537773n);
    input.add32(3217537777n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(3217537773n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (3217537777, 3217537777)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3217537777n);
    input.add32(3217537777n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(3217537777n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (3217537777, 3217537773)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3217537777n);
    input.add32(3217537773n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(3217537773n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18439230533808120319, 1372552365)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439230533808120319n);
    input.add32(1372552365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18439230533808120319n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (1372552361, 1372552365)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1372552361n);
    input.add32(1372552365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(1372552365n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (1372552365, 1372552365)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1372552365n);
    input.add32(1372552365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(1372552365n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (1372552365, 1372552361)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1372552365n);
    input.add32(1372552361n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(1372552365n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9220531194564270417, 9222624851828819281)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9220531194564270417n);
    input.add64(9222624851828819281n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18443156046393089698n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9220531194564270415, 9220531194564270417)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9220531194564270415n);
    input.add64(9220531194564270417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18441062389128540832n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9220531194564270417, 9220531194564270417)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9220531194564270417n);
    input.add64(9220531194564270417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18441062389128540834n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9220531194564270417, 9220531194564270415)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9220531194564270417n);
    input.add64(9220531194564270415n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18441062389128540832n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18442343153520148641, 18442343153520148641)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442343153520148641n);
    input.add64(18442343153520148641n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18442343153520148641, 18442343153520148637)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442343153520148641n);
    input.add64(18442343153520148637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (4294070018, 4292952863)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4294070018n);
    input.add64(4292952863n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18434240177695561534n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (4292952863, 4292952863)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4292952863n);
    input.add64(4292952863n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18429444283939896769n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (4292952863, 4292952863)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4292952863n);
    input.add64(4292952863n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18429444283939896769n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (4292952863, 4292952863)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4292952863n);
    input.add64(4292952863n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract3.resEuint64());
    expect(res).to.equal(18429444283939896769n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (18445513906165703221, 18442154191212954237)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445513906165703221n);
    input.add64(18442154191212954237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18441010129228317237n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (18442154191212954233, 18442154191212954237)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442154191212954233n);
    input.add64(18442154191212954237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18442154191212954233n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (18442154191212954237, 18442154191212954237)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442154191212954237n);
    input.add64(18442154191212954237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18442154191212954237n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (18442154191212954237, 18442154191212954233)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442154191212954237n);
    input.add64(18442154191212954233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18442154191212954233n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (18440259459868756377, 18446431208995465061)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440259459868756377n);
    input.add64(18446431208995465061n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18446453217179709437n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (18440259459868756373, 18440259459868756377)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440259459868756373n);
    input.add64(18440259459868756377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18440259459868756381n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (18440259459868756377, 18440259459868756377)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440259459868756377n);
    input.add64(18440259459868756377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18440259459868756377n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (18440259459868756377, 18440259459868756373)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440259459868756377n);
    input.add64(18440259459868756373n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18440259459868756381n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (18443471195136889881, 18440807222368659245)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443471195136889881n);
    input.add64(18440807222368659245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(8645608239100724n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (18440807222368659241, 18440807222368659245)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440807222368659241n);
    input.add64(18440807222368659245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (18440807222368659245, 18440807222368659245)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440807222368659245n);
    input.add64(18440807222368659245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (18440807222368659245, 18440807222368659241)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440807222368659245n);
    input.add64(18440807222368659241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (18441678210113937609, 18440511762363712331)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441678210113937609n);
    input.add64(18440511762363712331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (18440511762363712327, 18440511762363712331)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440511762363712327n);
    input.add64(18440511762363712331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (18440511762363712331, 18440511762363712331)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440511762363712331n);
    input.add64(18440511762363712331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (18440511762363712331, 18440511762363712327)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440511762363712331n);
    input.add64(18440511762363712327n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (18442567653990481325, 18438601454400463899)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442567653990481325n);
    input.add64(18438601454400463899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (18438601454400463895, 18438601454400463899)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438601454400463895n);
    input.add64(18438601454400463899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (18438601454400463899, 18438601454400463899)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438601454400463899n);
    input.add64(18438601454400463899n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (18438601454400463899, 18438601454400463895)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438601454400463899n);
    input.add64(18438601454400463895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (18438909688745287627, 18441974479508078799)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438909688745287627n);
    input.add64(18441974479508078799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (18438909688745287623, 18438909688745287627)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438909688745287623n);
    input.add64(18438909688745287627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (18438909688745287627, 18438909688745287627)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438909688745287627n);
    input.add64(18438909688745287627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (18438909688745287627, 18438909688745287623)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438909688745287627n);
    input.add64(18438909688745287623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (18441447067300773743, 18439846175976014309)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441447067300773743n);
    input.add64(18439846175976014309n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (18439846175976014305, 18439846175976014309)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439846175976014305n);
    input.add64(18439846175976014309n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (18439846175976014309, 18439846175976014309)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439846175976014309n);
    input.add64(18439846175976014309n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (18439846175976014309, 18439846175976014305)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439846175976014309n);
    input.add64(18439846175976014305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (18437829734296327859, 18444036449020440203)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437829734296327859n);
    input.add64(18444036449020440203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (18437829734296327855, 18437829734296327859)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437829734296327855n);
    input.add64(18437829734296327859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (18437829734296327859, 18437829734296327859)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437829734296327859n);
    input.add64(18437829734296327859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (18437829734296327859, 18437829734296327855)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437829734296327859n);
    input.add64(18437829734296327855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (18445786402715338993, 18444790581199458979)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445786402715338993n);
    input.add64(18444790581199458979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (18444790581199458975, 18444790581199458979)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444790581199458975n);
    input.add64(18444790581199458979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (18444790581199458979, 18444790581199458979)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444790581199458979n);
    input.add64(18444790581199458979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (18444790581199458979, 18444790581199458975)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444790581199458979n);
    input.add64(18444790581199458975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (18437934124931735627, 18443680011906822939)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437934124931735627n);
    input.add64(18443680011906822939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18437934124931735627n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (18437934124931735623, 18437934124931735627)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437934124931735623n);
    input.add64(18437934124931735627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18437934124931735623n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (18437934124931735627, 18437934124931735627)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437934124931735627n);
    input.add64(18437934124931735627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18437934124931735627n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (18437934124931735627, 18437934124931735623)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18437934124931735627n);
    input.add64(18437934124931735623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18437934124931735623n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18441187267678247311, 18445777800386689089)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441187267678247311n);
    input.add64(18445777800386689089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18445777800386689089n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18441187267678247307, 18441187267678247311)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441187267678247307n);
    input.add64(18441187267678247311n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18441187267678247311n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18441187267678247311, 18441187267678247311)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441187267678247311n);
    input.add64(18441187267678247311n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18441187267678247311n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18441187267678247311, 18441187267678247307)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441187267678247311n);
    input.add64(18441187267678247307n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint64, await this.contract4.resEuint64());
    expect(res).to.equal(18441187267678247311n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 1 (2, 9223372036854775809)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 2 (9222022623962696004, 9222022623962696006)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9222022623962696004n);
    input.add128(9222022623962696006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18444045247925392010n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 3 (9222022623962696006, 9222022623962696006)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9222022623962696006n);
    input.add128(9222022623962696006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18444045247925392012n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 4 (9222022623962696006, 9222022623962696004)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9222022623962696006n);
    input.add128(9222022623962696004n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18444045247925392010n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 1 (18446674339865079557, 18446674339865079557)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446674339865079557n);
    input.add128(18446674339865079557n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 2 (18446674339865079557, 18446674339865079553)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446674339865079557n);
    input.add128(18446674339865079553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 1 (2, 4611686018427387905)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(4611686018427387905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 2 (4294353955, 4294353955)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4294353955n);
    input.add128(4294353955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18441475890824142025n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 3 (4294353955, 4294353955)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4294353955n);
    input.add128(4294353955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18441475890824142025n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 4 (4294353955, 4294353955)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4294353955n);
    input.add128(4294353955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18441475890824142025n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 1 (18444136896294723467, 340282366920938463463373030829049321823)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444136896294723467n);
    input.add128(340282366920938463463373030829049321823n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18442843006727887115n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 2 (18444136896294723463, 18444136896294723467)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444136896294723463n);
    input.add128(18444136896294723467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18444136896294723459n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 3 (18444136896294723467, 18444136896294723467)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444136896294723467n);
    input.add128(18444136896294723467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18444136896294723467n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 4 (18444136896294723467, 18444136896294723463)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444136896294723467n);
    input.add128(18444136896294723463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18444136896294723459n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 1 (18442854865253503051, 340282366920938463463373124198113828861)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442854865253503051n);
    input.add128(340282366920938463463373124198113828861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463373129455539937279n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 2 (18442854865253503047, 18442854865253503051)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442854865253503047n);
    input.add128(18442854865253503051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18442854865253503055n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 3 (18442854865253503051, 18442854865253503051)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442854865253503051n);
    input.add128(18442854865253503051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18442854865253503051n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 4 (18442854865253503051, 18442854865253503047)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442854865253503051n);
    input.add128(18442854865253503047n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18442854865253503055n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 1 (18440953796272056615, 340282366920938463463367127602902197787)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440953796272056615n);
    input.add128(340282366920938463463367127602902197787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463444931805001402683196n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 2 (18440953796272056611, 18440953796272056615)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440953796272056611n);
    input.add128(18440953796272056615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 3 (18440953796272056615, 18440953796272056615)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440953796272056615n);
    input.add128(18440953796272056615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 4 (18440953796272056615, 18440953796272056611)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440953796272056615n);
    input.add128(18440953796272056611n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 1 (18445639215986272813, 340282366920938463463373730005296398507)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445639215986272813n);
    input.add128(340282366920938463463373730005296398507n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 2 (18445639215986272809, 18445639215986272813)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445639215986272809n);
    input.add128(18445639215986272813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 3 (18445639215986272813, 18445639215986272813)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445639215986272813n);
    input.add128(18445639215986272813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 4 (18445639215986272813, 18445639215986272809)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445639215986272813n);
    input.add128(18445639215986272809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 1 (18445787169897838725, 340282366920938463463372426408043439299)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445787169897838725n);
    input.add128(340282366920938463463372426408043439299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 2 (18445787169897838721, 18445787169897838725)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445787169897838721n);
    input.add128(18445787169897838725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 3 (18445787169897838725, 18445787169897838725)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445787169897838725n);
    input.add128(18445787169897838725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 4 (18445787169897838725, 18445787169897838721)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445787169897838725n);
    input.add128(18445787169897838721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 1 (18445309627761582613, 340282366920938463463369857255590655677)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445309627761582613n);
    input.add128(340282366920938463463369857255590655677n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 2 (18445309627761582609, 18445309627761582613)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445309627761582609n);
    input.add128(18445309627761582613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 3 (18445309627761582613, 18445309627761582613)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445309627761582613n);
    input.add128(18445309627761582613n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 4 (18445309627761582613, 18445309627761582609)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445309627761582613n);
    input.add128(18445309627761582609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 1 (18443124971301294459, 340282366920938463463374489997768431841)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443124971301294459n);
    input.add128(340282366920938463463374489997768431841n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 2 (18443124971301294455, 18443124971301294459)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443124971301294455n);
    input.add128(18443124971301294459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 3 (18443124971301294459, 18443124971301294459)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443124971301294459n);
    input.add128(18443124971301294459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 4 (18443124971301294459, 18443124971301294455)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443124971301294459n);
    input.add128(18443124971301294455n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 1 (18441333521898461747, 340282366920938463463367203915348429323)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441333521898461747n);
    input.add128(340282366920938463463367203915348429323n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 2 (18441333521898461743, 18441333521898461747)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441333521898461743n);
    input.add128(18441333521898461747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 3 (18441333521898461747, 18441333521898461747)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441333521898461747n);
    input.add128(18441333521898461747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 4 (18441333521898461747, 18441333521898461743)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441333521898461747n);
    input.add128(18441333521898461743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 1 (18441473951603803523, 340282366920938463463373951684024878469)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441473951603803523n);
    input.add128(340282366920938463463373951684024878469n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 2 (18441473951603803519, 18441473951603803523)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441473951603803519n);
    input.add128(18441473951603803523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 3 (18441473951603803523, 18441473951603803523)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441473951603803523n);
    input.add128(18441473951603803523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 4 (18441473951603803523, 18441473951603803519)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441473951603803523n);
    input.add128(18441473951603803519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 1 (18438655808748848249, 340282366920938463463369181941454194387)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438655808748848249n);
    input.add128(340282366920938463463369181941454194387n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18438655808748848249n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 2 (18438655808748848245, 18438655808748848249)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438655808748848245n);
    input.add128(18438655808748848249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18438655808748848245n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 3 (18438655808748848249, 18438655808748848249)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438655808748848249n);
    input.add128(18438655808748848249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18438655808748848249n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 4 (18438655808748848249, 18438655808748848245)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438655808748848249n);
    input.add128(18438655808748848245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18438655808748848245n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 1 (18443270314619276329, 340282366920938463463365640590404440551)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443270314619276329n);
    input.add128(340282366920938463463365640590404440551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463365640590404440551n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 2 (18443270314619276325, 18443270314619276329)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443270314619276325n);
    input.add128(18443270314619276329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18443270314619276329n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 3 (18443270314619276329, 18443270314619276329)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443270314619276329n);
    input.add128(18443270314619276329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18443270314619276329n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 4 (18443270314619276329, 18443270314619276325)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443270314619276329n);
    input.add128(18443270314619276325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(18443270314619276329n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 1 (18438158446353933861, 115792089237316195423570985008687907853269984665640564039457583348083972711585)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438158446353933861n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583348083972711585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(18438061653595071521n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 2 (18438158446353933857, 18438158446353933861)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438158446353933857n);
    input.add256(18438158446353933861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(18438158446353933857n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 3 (18438158446353933861, 18438158446353933861)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438158446353933861n);
    input.add256(18438158446353933861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(18438158446353933861n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 4 (18438158446353933861, 18438158446353933857)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438158446353933861n);
    input.add256(18438158446353933857n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(18438158446353933857n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 1 (18441501105320133041, 115792089237316195423570985008687907853269984665640564039457577822437524218767)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441501105320133041n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577822437524218767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579328254132986815n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 2 (18441501105320133037, 18441501105320133041)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441501105320133037n);
    input.add256(18441501105320133041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(18441501105320133053n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 3 (18441501105320133041, 18441501105320133041)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441501105320133041n);
    input.add256(18441501105320133041n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(18441501105320133041n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 4 (18441501105320133041, 18441501105320133037)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441501105320133041n);
    input.add256(18441501105320133037n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(18441501105320133053n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 1 (18441420896046104535, 115792089237316195423570985008687907853269984665640564039457578364899804172819)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441420896046104535n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578364899804172819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039439139204668788239812n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 2 (18441420896046104531, 18441420896046104535)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441420896046104531n);
    input.add256(18441420896046104535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 3 (18441420896046104535, 18441420896046104535)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441420896046104535n);
    input.add256(18441420896046104535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 4 (18441420896046104535, 18441420896046104531)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441420896046104535n);
    input.add256(18441420896046104531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint256, await this.contract4.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 1 (18442343406754497989, 115792089237316195423570985008687907853269984665640564039457575418249582981437)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442343406754497989n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575418249582981437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 2 (18442343406754497985, 18442343406754497989)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442343406754497985n);
    input.add256(18442343406754497989n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 3 (18442343406754497989, 18442343406754497989)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442343406754497989n);
    input.add256(18442343406754497989n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 4 (18442343406754497989, 18442343406754497985)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442343406754497989n);
    input.add256(18442343406754497985n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 1 (18440425741325228723, 115792089237316195423570985008687907853269984665640564039457583067488613747179)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440425741325228723n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583067488613747179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 2 (18440425741325228719, 18440425741325228723)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440425741325228719n);
    input.add256(18440425741325228723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 3 (18440425741325228723, 18440425741325228723)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440425741325228723n);
    input.add256(18440425741325228723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 4 (18440425741325228723, 18440425741325228719)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440425741325228723n);
    input.add256(18440425741325228719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 1 (129, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 2 (93, 95)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(93n);
    input.add8(95n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(188n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 3 (95, 95)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(95n);
    input.add8(95n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(190n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 4 (95, 93)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(95n);
    input.add8(93n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(188n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 1 (146, 146)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(146n);
    input.add8(146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 2 (146, 142)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(146n);
    input.add8(142n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 1 (65, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 2 (10, 10)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 3 (10, 10)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 4 (10, 10)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(100n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 1 (340282366920938463463368445032783128369, 187)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368445032783128369n);
    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(49n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 2 (183, 187)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(183n);
    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(179n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 3 (187, 187)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(187n);
    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(187n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 4 (187, 183)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(187n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(179n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 1 (340282366920938463463365854072856775471, 197)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463365854072856775471n);
    input.add8(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463365854072856775663n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 2 (193, 197)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(193n);
    input.add8(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(197n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 3 (197, 197)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(197n);
    input.add8(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(197n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 4 (197, 193)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(197n);
    input.add8(193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(197n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 1 (340282366920938463463365781896954916335, 188)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463365781896954916335n);
    input.add8(188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463365781896954916179n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 2 (184, 188)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(184n);
    input.add8(188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 3 (188, 188)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(188n);
    input.add8(188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 4 (188, 184)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(188n);
    input.add8(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEuint(FhevmType.euint128, await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 1 (340282366920938463463367008038643604495, 202)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463367008038643604495n);
    input.add8(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 2 (198, 202)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(198n);
    input.add8(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 3 (202, 202)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(202n);
    input.add8(202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 4 (202, 198)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(202n);
    input.add8(198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 1 (340282366920938463463368600436805936733, 104)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368600436805936733n);
    input.add8(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 2 (100, 104)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(100n);
    input.add8(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 3 (104, 104)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(104n);
    input.add8(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 4 (104, 100)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(104n);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 1 (340282366920938463463373056948021363357, 5)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373056948021363357n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 2 (1, 5)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(1n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 3 (5, 5)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(5n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 4 (5, 1)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(5n);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 1 (340282366920938463463371107703407545563, 23)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371107703407545563n);
    input.add8(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 2 (19, 23)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(19n);
    input.add8(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 3 (23, 23)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(23n);
    input.add8(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 4 (23, 19)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(23n);
    input.add8(19n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 1 (340282366920938463463372713724322251459, 133)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463372713724322251459n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 2 (129, 133)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(129n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 3 (133, 133)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(133n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 4 (133, 129)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(133n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 1 (340282366920938463463373319305963929295, 38)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373319305963929295n);
    input.add8(38n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 2 (34, 38)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(34n);
    input.add8(38n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 3 (38, 38)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(38n);
    input.add8(38n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 4 (38, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(38n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await hre.fhevm.debugger.decryptEbool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });
});
