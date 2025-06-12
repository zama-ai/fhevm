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

describe('FHEVM operations 10', function () {
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

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (15443261, 15443261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(15443261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint32_euint32(
      15443261n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (15443261, 15443257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(15443257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint32_euint32(
      15443261n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (11236, 124778)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(11236n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 124778n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1402005608n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (44938, 44938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(44938n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 44938n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2019423844n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (44938, 44938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(44938n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 44938n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2019423844n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (44938, 44938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(44938n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_uint32(encryptedAmount.handles[0], 44938n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2019423844n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (5029, 249554)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(249554n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(5029n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1255007066n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (44938, 44938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(44938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(44938n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2019423844n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (44938, 44938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(44938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(44938n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2019423844n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (44938, 44938)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(44938n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint32_euint32(44938n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2019423844n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (833316743, 2024391455)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(833316743n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      2024391455n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (833316739, 833316743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(833316739n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      833316743n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (833316743, 833316743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(833316743n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      833316743n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (833316743, 833316739)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(833316743n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint32_uint32(
      encryptedAmount.handles[0],
      833316739n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (4217302894, 2041484320)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4217302894n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      2041484320n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(134334254n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (2368691544, 2368691548)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2368691544n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      2368691548n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2368691544n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (2368691548, 2368691548)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2368691548n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      2368691548n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (2368691548, 2368691544)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2368691548n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint32_uint32(
      encryptedAmount.handles[0],
      2368691544n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 1 (4243271706, 3443529448)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4243271706n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      3443529448n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3426750472n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 2 (1169099336, 1169099340)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1169099336n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      1169099340n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1169099336n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 3 (1169099340, 1169099340)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1169099340n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      1169099340n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1169099340n);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 4 (1169099340, 1169099336)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1169099340n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_uint32(
      encryptedAmount.handles[0],
      1169099336n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1169099336n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 1 (963978083, 3443529448)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3443529448n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      963978083n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(155189856n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 2 (1169099336, 1169099340)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1169099340n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      1169099336n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1169099336n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 3 (1169099340, 1169099340)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1169099340n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      1169099340n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1169099340n);
  });

  it('test operator "and" overload (uint32, euint32) => euint32 test 4 (1169099340, 1169099336)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1169099336n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint32_euint32(
      1169099340n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1169099336n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 1 (1160502017, 2787774839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1160502017n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      2787774839n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3878411127n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 2 (1160502013, 1160502017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1160502013n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      1160502017n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1160502269n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 3 (1160502017, 1160502017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1160502017n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      1160502017n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1160502017n);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 4 (1160502017, 1160502013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1160502017n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_uint32(
      encryptedAmount.handles[0],
      1160502013n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1160502269n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 1 (2228306677, 2787774839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2787774839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      2228306677n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2801488887n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 2 (1160502013, 1160502017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1160502017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      1160502013n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1160502269n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 3 (1160502017, 1160502017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1160502017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      1160502017n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1160502017n);
  });

  it('test operator "or" overload (uint32, euint32) => euint32 test 4 (1160502017, 1160502013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1160502013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint32_euint32(
      1160502017n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1160502269n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 1 (37108106, 3725785721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(37108106n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      3725785721n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(3693396979n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 2 (37108102, 37108106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(37108102n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      37108106n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 3 (37108106, 37108106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(37108106n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      37108106n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, uint32) => euint32 test 4 (37108106, 37108102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(37108106n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_uint32(
      encryptedAmount.handles[0],
      37108102n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 1 (3984930214, 3725785721)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3725785721n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      3984930214n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(865587167n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 2 (37108102, 37108106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(37108106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      37108102n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 3 (37108106, 37108106)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(37108106n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      37108106n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint32, euint32) => euint32 test 4 (37108106, 37108102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(37108102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint32_euint32(
      37108106n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (1987927751, 2707746343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1987927751n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      2707746343n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (1987927747, 1987927751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1987927747n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      1987927751n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (1987927751, 1987927751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1987927751n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      1987927751n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (1987927751, 1987927747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1987927751n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_uint32(
      encryptedAmount.handles[0],
      1987927747n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (2749128076, 2707746343)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2707746343n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      2749128076n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (1987927747, 1987927751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1987927751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      1987927747n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (1987927751, 1987927751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1987927751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      1987927751n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (1987927751, 1987927747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1987927747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint32_euint32(
      1987927751n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (984863625, 1317271922)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(984863625n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      1317271922n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (984863621, 984863625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(984863621n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      984863625n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (984863625, 984863625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(984863625n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      984863625n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (984863625, 984863621)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(984863625n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_uint32(
      encryptedAmount.handles[0],
      984863621n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (1923876321, 1317271922)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1317271922n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      1923876321n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (984863621, 984863625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(984863625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      984863621n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (984863625, 984863625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(984863625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      984863625n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (984863625, 984863621)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(984863621n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint32_euint32(
      984863625n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (4026144610, 2922022897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4026144610n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      2922022897n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (3109238835, 3109238839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3109238835n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      3109238839n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (3109238839, 3109238839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3109238839n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      3109238839n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (3109238839, 3109238835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3109238839n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_uint32(
      encryptedAmount.handles[0],
      3109238835n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (145029184, 2922022897)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2922022897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      145029184n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (3109238835, 3109238839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3109238839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      3109238835n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (3109238839, 3109238839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3109238839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      3109238839n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (3109238839, 3109238835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3109238835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint32_euint32(
      3109238839n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (319754915, 3663194782)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(319754915n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      3663194782n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (319754911, 319754915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(319754911n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      319754915n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (319754915, 319754915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(319754915n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      319754915n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (319754915, 319754911)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(319754915n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_uint32(
      encryptedAmount.handles[0],
      319754911n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (1025029826, 3663194782)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(3663194782n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      1025029826n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (319754911, 319754915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(319754915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      319754911n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (319754915, 319754915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(319754915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      319754915n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (319754915, 319754911)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(319754911n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      319754915n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (3813081277, 19013530)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3813081277n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      19013530n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (1147958827, 1147958831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1147958827n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      1147958831n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (1147958831, 1147958831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1147958831n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      1147958831n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (1147958831, 1147958827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1147958831n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      1147958827n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (1361653268, 19013530)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(19013530n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      1361653268n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (1147958827, 1147958831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1147958831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      1147958827n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (1147958831, 1147958831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1147958831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      1147958831n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (1147958831, 1147958827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1147958827n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      1147958831n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (2748635537, 2215656510)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2748635537n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      2215656510n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (958311247, 958311251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(958311247n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      958311251n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (958311251, 958311251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(958311251n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      958311251n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (958311251, 958311247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(958311251n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      958311247n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (3698223298, 2215656510)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2215656510n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      3698223298n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (958311247, 958311251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(958311251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      958311247n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (958311251, 958311251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(958311251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      958311251n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (958311251, 958311247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(958311247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      958311251n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (287613709, 2068510788)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(287613709n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      2068510788n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(287613709n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (287613705, 287613709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(287613705n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      287613709n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(287613705n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (287613709, 287613709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(287613709n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      287613709n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(287613709n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (287613709, 287613705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(287613709n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      287613705n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(287613705n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (4096161673, 2068510788)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(2068510788n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      4096161673n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(2068510788n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (287613705, 287613709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(287613709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      287613705n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(287613705n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (287613709, 287613709)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(287613709n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      287613709n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(287613709n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (287613709, 287613705)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(287613705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint32_euint32(
      287613709n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(287613705n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (1500560192, 4126365844)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1500560192n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      4126365844n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(4126365844n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (1500560188, 1500560192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1500560188n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1500560192n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1500560192n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (1500560192, 1500560192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1500560192n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1500560192n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1500560192n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (1500560192, 1500560188)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1500560192n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_uint32(
      encryptedAmount.handles[0],
      1500560188n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1500560192n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (2094710496, 4126365844)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(4126365844n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      2094710496n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(4126365844n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (1500560188, 1500560192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1500560192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1500560188n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1500560192n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (1500560192, 1500560192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1500560192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1500560192n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1500560192n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (1500560192, 1500560188)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add32(1500560188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint32_euint32(
      1500560192n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract6.resEuint32());
    expect(res).to.equal(1500560192n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 1 (9220542010693254756, 9219357535638057665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9220542010693254756n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9219357535638057665n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18439899546331312421n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 2 (9220542010693254754, 9220542010693254756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9220542010693254754n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9220542010693254756n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441084021386509510n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 3 (9220542010693254756, 9220542010693254756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9220542010693254756n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9220542010693254756n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441084021386509512n);
  });

  it('test operator "add" overload (euint64, uint64) => euint64 test 4 (9220542010693254756, 9220542010693254754)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(9220542010693254756n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint64_uint64(
      encryptedAmount.handles[0],
      9220542010693254754n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441084021386509510n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 1 (9223364576715779762, 9219357535638057665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9219357535638057665n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9223364576715779762n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18442722112353837427n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 2 (9220542010693254754, 9220542010693254756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9220542010693254756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9220542010693254754n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441084021386509510n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 3 (9220542010693254756, 9220542010693254756)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9220542010693254756n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9220542010693254756n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441084021386509512n);
  });

  it('test operator "add" overload (uint64, euint64) => euint64 test 4 (9220542010693254756, 9220542010693254754)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(9220542010693254754n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint64_euint64(
      9220542010693254756n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441084021386509510n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 1 (18441530897933378281, 18441530897933378281)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441530897933378281n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18441530897933378281n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, uint64) => euint64 test 2 (18441530897933378281, 18441530897933378277)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441530897933378281n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_uint64(
      encryptedAmount.handles[0],
      18441530897933378277n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (18441530897933378281, 18441530897933378281)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441530897933378281n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint64_euint64(
      18441530897933378281n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (18441530897933378281, 18441530897933378277)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441530897933378277n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_uint64_euint64(
      18441530897933378281n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 1 (4294260953, 4294737074)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294260953n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294737074n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18442721720279671522n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 2 (4294260953, 4294260953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294260953n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294260953n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440677132460468209n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 3 (4294260953, 4294260953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294260953n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294260953n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440677132460468209n);
  });

  it('test operator "mul" overload (euint64, uint64) => euint64 test 4 (4294260953, 4294260953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(4294260953n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint64_uint64(
      encryptedAmount.handles[0],
      4294260953n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440677132460468209n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 1 (4293150241, 4294737074)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4294737074n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4293150241n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18437951504274734834n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 2 (4294260953, 4294260953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4294260953n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4294260953n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440677132460468209n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 3 (4294260953, 4294260953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4294260953n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4294260953n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440677132460468209n);
  });

  it('test operator "mul" overload (uint64, euint64) => euint64 test 4 (4294260953, 4294260953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(4294260953n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_uint64_euint64(
      4294260953n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440677132460468209n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 1 (18439995193509322827, 18439311313223488159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439995193509322827n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439311313223488159n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 2 (18439515412172094097, 18439515412172094101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439515412172094097n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439515412172094101n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 3 (18439515412172094101, 18439515412172094101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439515412172094101n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439515412172094101n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint64, uint64) => euint64 test 4 (18439515412172094101, 18439515412172094097)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439515412172094101n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.div_euint64_uint64(
      encryptedAmount.handles[0],
      18439515412172094097n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 1 (18444954315240529219, 18439436981071901031)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18444954315240529219n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18439436981071901031n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(5517334168628188n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 2 (18440781372851678517, 18440781372851678521)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440781372851678517n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18440781372851678521n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440781372851678517n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 3 (18440781372851678521, 18440781372851678521)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440781372851678521n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18440781372851678521n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint64, uint64) => euint64 test 4 (18440781372851678521, 18440781372851678517)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440781372851678521n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.rem_euint64_uint64(
      encryptedAmount.handles[0],
      18440781372851678517n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 1 (18446382720574463127, 18440941984952836875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18446382720574463127n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18440941984952836875n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440590115193618435n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 2 (18441552068260310113, 18441552068260310117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441552068260310113n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18441552068260310117n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441552068260310113n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 3 (18441552068260310117, 18441552068260310117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441552068260310117n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18441552068260310117n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441552068260310117n);
  });

  it('test operator "and" overload (euint64, uint64) => euint64 test 4 (18441552068260310117, 18441552068260310113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441552068260310117n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint64_uint64(
      encryptedAmount.handles[0],
      18441552068260310113n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441552068260310113n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 1 (18441671977550228521, 18440941984952836875)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440941984952836875n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18441671977550228521n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18440377920982622217n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 2 (18441552068260310113, 18441552068260310117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441552068260310117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18441552068260310113n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441552068260310113n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 3 (18441552068260310117, 18441552068260310117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441552068260310117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18441552068260310117n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441552068260310117n);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 4 (18441552068260310117, 18441552068260310113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441552068260310113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18441552068260310117n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441552068260310113n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 1 (18438315912404753171, 18439432024198635671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438315912404753171n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18439432024198635671n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18439443023626730391n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 2 (18438315912404753167, 18438315912404753171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438315912404753167n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18438315912404753171n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438315912404753183n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 3 (18438315912404753171, 18438315912404753171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438315912404753171n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18438315912404753171n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438315912404753171n);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 4 (18438315912404753171, 18438315912404753167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438315912404753171n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18438315912404753167n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438315912404753183n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 1 (18441322756931160503, 18439432024198635671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439432024198635671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18441322756931160503n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18441886430792247735n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 2 (18438315912404753167, 18438315912404753171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438315912404753171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18438315912404753167n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438315912404753183n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 3 (18438315912404753171, 18438315912404753171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438315912404753171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18438315912404753171n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438315912404753171n);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 4 (18438315912404753171, 18438315912404753167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438315912404753167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18438315912404753171n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(18438315912404753183n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 1 (18439557724603662637, 18440482834152360037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439557724603662637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18440482834152360037n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4425955847752008n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 2 (18439557724603662633, 18439557724603662637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439557724603662633n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18439557724603662637n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 3 (18439557724603662637, 18439557724603662637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439557724603662637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18439557724603662637n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 4 (18439557724603662637, 18439557724603662633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439557724603662637n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18439557724603662633n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 1 (18444559076422492139, 18440482834152360037)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440482834152360037n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18444559076422492139n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(5063999385868174n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 2 (18439557724603662633, 18439557724603662637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439557724603662637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18439557724603662633n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 3 (18439557724603662637, 18439557724603662637)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439557724603662637n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18439557724603662637n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 4 (18439557724603662637, 18439557724603662633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439557724603662633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18439557724603662637n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (18440015172278097121, 18441506361306475997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440015172278097121n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18441506361306475997n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (18438616479091672595, 18438616479091672599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438616479091672595n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18438616479091672599n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (18438616479091672599, 18438616479091672599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438616479091672599n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18438616479091672599n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (18438616479091672599, 18438616479091672595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18438616479091672599n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18438616479091672595n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (18444550641102450915, 18441506361306475997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441506361306475997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18444550641102450915n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (18438616479091672595, 18438616479091672599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438616479091672599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18438616479091672595n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (18438616479091672599, 18438616479091672599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438616479091672599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18438616479091672599n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (18438616479091672599, 18438616479091672595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438616479091672595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_uint64_euint64(
      18438616479091672599n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (18440336707363205611, 18437917082674589473)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440336707363205611n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18437917082674589473n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (18440336707363205607, 18440336707363205611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440336707363205607n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18440336707363205611n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (18440336707363205611, 18440336707363205611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440336707363205611n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18440336707363205611n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (18440336707363205611, 18440336707363205607)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440336707363205611n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18440336707363205607n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 1 (18437943554419590619, 18437917082674589473)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18437917082674589473n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18437943554419590619n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 2 (18440336707363205607, 18440336707363205611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440336707363205611n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18440336707363205607n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 3 (18440336707363205611, 18440336707363205611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440336707363205611n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18440336707363205611n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint64, euint64) => ebool test 4 (18440336707363205611, 18440336707363205607)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440336707363205607n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_uint64_euint64(
      18440336707363205611n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (18441764074357289241, 18441595117874496727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441764074357289241n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18441595117874496727n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (18439361804174229901, 18439361804174229905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439361804174229901n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18439361804174229905n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (18439361804174229905, 18439361804174229905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439361804174229905n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18439361804174229905n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (18439361804174229905, 18439361804174229901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439361804174229905n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18439361804174229901n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 1 (18438687885812058387, 18441595117874496727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441595117874496727n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18438687885812058387n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 2 (18439361804174229901, 18439361804174229905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439361804174229905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18439361804174229901n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 3 (18439361804174229905, 18439361804174229905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439361804174229905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18439361804174229905n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint64, euint64) => ebool test 4 (18439361804174229905, 18439361804174229901)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18439361804174229901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_uint64_euint64(
      18439361804174229905n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (18445698250927967321, 18438797955515936399)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18445698250927967321n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18438797955515936399n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (18440077893768052501, 18440077893768052505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440077893768052501n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18440077893768052505n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (18440077893768052505, 18440077893768052505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440077893768052505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18440077893768052505n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (18440077893768052505, 18440077893768052501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18440077893768052505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18440077893768052501n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 1 (18445818556278122379, 18438797955515936399)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18438797955515936399n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18445818556278122379n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 2 (18440077893768052501, 18440077893768052505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440077893768052505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18440077893768052501n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 3 (18440077893768052505, 18440077893768052505)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440077893768052505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18440077893768052505n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint64, euint64) => ebool test 4 (18440077893768052505, 18440077893768052501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18440077893768052501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint64_euint64(
      18440077893768052505n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 1 (18441451101101739063, 18443251378499312157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441451101101739063n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18443251378499312157n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 2 (18441451101101739059, 18441451101101739063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441451101101739059n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18441451101101739063n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 3 (18441451101101739063, 18441451101101739063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441451101101739063n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18441451101101739063n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, uint64) => ebool test 4 (18441451101101739063, 18441451101101739059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18441451101101739063n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint64_uint64(
      encryptedAmount.handles[0],
      18441451101101739059n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18440350753137228077, 18443251378499312157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18443251378499312157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18440350753137228077n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18441451101101739059, 18441451101101739063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441451101101739063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18441451101101739059n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18441451101101739063, 18441451101101739063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441451101101739063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18441451101101739063n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18441451101101739063, 18441451101101739059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);

    input.add64(18441451101101739059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint64_euint64(
      18441451101101739063n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 1 (18442930857400477495, 18445098809648433805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18442930857400477495n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18445098809648433805n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 2 (18439528997555571215, 18439528997555571219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439528997555571215n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18439528997555571219n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 3 (18439528997555571219, 18439528997555571219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439528997555571219n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18439528997555571219n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, uint64) => ebool test 4 (18439528997555571219, 18439528997555571215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add64(18439528997555571219n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint64_uint64(
      encryptedAmount.handles[0],
      18439528997555571215n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resEbool());
    expect(res).to.equal(false);
  });
});
