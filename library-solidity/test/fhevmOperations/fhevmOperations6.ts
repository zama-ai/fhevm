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

describe('FHEVM operations 6', function () {
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

  it('test operator "ne" overload (euint64, euint32) => ebool test 1 (18438109123246729977, 1042177167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438109123246729977n);
    input.add32(1042177167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 2 (1042177163, 1042177167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1042177163n);
    input.add32(1042177167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 3 (1042177167, 1042177167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1042177167n);
    input.add32(1042177167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint32) => ebool test 4 (1042177167, 1042177163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1042177167n);
    input.add32(1042177163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 1 (18442150157225615939, 3702007843)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442150157225615939n);
    input.add32(3702007843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 2 (3702007839, 3702007843)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3702007839n);
    input.add32(3702007843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 3 (3702007843, 3702007843)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3702007843n);
    input.add32(3702007843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint32) => ebool test 4 (3702007843, 3702007839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3702007843n);
    input.add32(3702007839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 1 (18445512764828899377, 4144563013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445512764828899377n);
    input.add32(4144563013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 2 (4144563009, 4144563013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4144563009n);
    input.add32(4144563013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 3 (4144563013, 4144563013)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4144563013n);
    input.add32(4144563013n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint32) => ebool test 4 (4144563013, 4144563009)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4144563013n);
    input.add32(4144563009n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 1 (18441110557587383101, 3327942350)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18441110557587383101n);
    input.add32(3327942350n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 2 (3327942346, 3327942350)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3327942346n);
    input.add32(3327942350n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 3 (3327942350, 3327942350)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3327942350n);
    input.add32(3327942350n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint32) => ebool test 4 (3327942350, 3327942346)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(3327942350n);
    input.add32(3327942346n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (18442415583644319837, 2112486447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442415583644319837n);
    input.add32(2112486447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (2112486443, 2112486447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2112486443n);
    input.add32(2112486447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (2112486447, 2112486447)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2112486447n);
    input.add32(2112486447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (2112486447, 2112486443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2112486447n);
    input.add32(2112486443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18446096812882064267, 1825362067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18446096812882064267n);
    input.add32(1825362067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1825362067n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (1825362063, 1825362067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1825362063n);
    input.add32(1825362067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1825362063n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (1825362067, 1825362067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1825362067n);
    input.add32(1825362067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1825362067n);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (1825362067, 1825362063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1825362067n);
    input.add32(1825362063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1825362063n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18441781480784489247, 1075181546)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18441781480784489247n);
    input.add32(1075181546n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18441781480784489247n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (1075181542, 1075181546)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1075181542n);
    input.add32(1075181546n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1075181546n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (1075181546, 1075181546)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1075181546n);
    input.add32(1075181546n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1075181546n);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (1075181546, 1075181542)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1075181546n);
    input.add32(1075181542n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1075181546n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9221990911440551113, 9221765408825701752)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9221990911440551113n);
    input.add64(9221765408825701752n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18443756320266252865n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9221765408825701750, 9221765408825701752)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9221765408825701750n);
    input.add64(9221765408825701752n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18443530817651403502n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9221765408825701752, 9221765408825701752)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9221765408825701752n);
    input.add64(9221765408825701752n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18443530817651403504n);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9221765408825701752, 9221765408825701750)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(9221765408825701752n);
    input.add64(9221765408825701750n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18443530817651403502n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18439849903880373441, 18439849903880373441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439849903880373441n);
    input.add64(18439849903880373441n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18439849903880373441, 18439849903880373437)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439849903880373441n);
    input.add64(18439849903880373437n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (4294267291, 4293313196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4294267291n);
    input.add64(4293313196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18436634427601472036n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (4293313196, 4293313196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4293313196n);
    input.add64(4293313196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18432538198947734416n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (4293313196, 4293313196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4293313196n);
    input.add64(4293313196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18432538198947734416n);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (4293313196, 4293313196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4293313196n);
    input.add64(4293313196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18432538198947734416n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 1 (18445660910980967303, 18441854787773399751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445660910980967303n);
    input.add64(18441854787773399751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18441149861627691655n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 2 (18441854787773399747, 18441854787773399751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441854787773399747n);
    input.add64(18441854787773399751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18441854787773399747n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 3 (18441854787773399751, 18441854787773399751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441854787773399751n);
    input.add64(18441854787773399751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18441854787773399751n);
  });

  it('test operator "and" overload (euint64, euint64) => euint64 test 4 (18441854787773399751, 18441854787773399747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441854787773399751n);
    input.add64(18441854787773399747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18441854787773399747n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 1 (18438845513135114693, 18442681803102459281)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438845513135114693n);
    input.add64(18442681803102459281n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443350362082237909n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 2 (18438845513135114689, 18438845513135114693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438845513135114689n);
    input.add64(18438845513135114693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18438845513135114693n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 3 (18438845513135114693, 18438845513135114693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438845513135114693n);
    input.add64(18438845513135114693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18438845513135114693n);
  });

  it('test operator "or" overload (euint64, euint64) => euint64 test 4 (18438845513135114693, 18438845513135114689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438845513135114693n);
    input.add64(18438845513135114689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18438845513135114693n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 1 (18443883720811313743, 18444422543880600627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443883720811313743n);
    input.add64(18444422543880600627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(587218777589372n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 2 (18443883720811313739, 18443883720811313743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443883720811313739n);
    input.add64(18443883720811313743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 3 (18443883720811313743, 18443883720811313743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443883720811313743n);
    input.add64(18443883720811313743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint64) => euint64 test 4 (18443883720811313743, 18443883720811313739)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443883720811313743n);
    input.add64(18443883720811313739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 1 (18445767439788352565, 18443333274960293011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445767439788352565n);
    input.add64(18443333274960293011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 2 (18443333274960293007, 18443333274960293011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443333274960293007n);
    input.add64(18443333274960293011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 3 (18443333274960293011, 18443333274960293011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443333274960293011n);
    input.add64(18443333274960293011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint64) => ebool test 4 (18443333274960293011, 18443333274960293007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443333274960293011n);
    input.add64(18443333274960293007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 1 (18440810195630480087, 18446163383956699125)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440810195630480087n);
    input.add64(18446163383956699125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 2 (18440810195630480083, 18440810195630480087)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440810195630480083n);
    input.add64(18440810195630480087n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 3 (18440810195630480087, 18440810195630480087)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440810195630480087n);
    input.add64(18440810195630480087n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint64) => ebool test 4 (18440810195630480087, 18440810195630480083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440810195630480087n);
    input.add64(18440810195630480083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 1 (18438708058610153987, 18439672688089610567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438708058610153987n);
    input.add64(18439672688089610567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 2 (18438708058610153983, 18438708058610153987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438708058610153983n);
    input.add64(18438708058610153987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 3 (18438708058610153987, 18438708058610153987)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438708058610153987n);
    input.add64(18438708058610153987n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint64) => ebool test 4 (18438708058610153987, 18438708058610153983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438708058610153987n);
    input.add64(18438708058610153983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 1 (18440598506953306067, 18440603873116904369)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440598506953306067n);
    input.add64(18440603873116904369n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 2 (18440598506953306063, 18440598506953306067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440598506953306063n);
    input.add64(18440598506953306067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 3 (18440598506953306067, 18440598506953306067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440598506953306067n);
    input.add64(18440598506953306067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint64) => ebool test 4 (18440598506953306067, 18440598506953306063)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440598506953306067n);
    input.add64(18440598506953306063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 1 (18445465295792791433, 18446567431263506143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445465295792791433n);
    input.add64(18446567431263506143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 2 (18445465295792791429, 18445465295792791433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445465295792791429n);
    input.add64(18445465295792791433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 3 (18445465295792791433, 18445465295792791433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445465295792791433n);
    input.add64(18445465295792791433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint64) => ebool test 4 (18445465295792791433, 18445465295792791429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445465295792791433n);
    input.add64(18445465295792791429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 1 (18438663246166747889, 18446642037746374993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438663246166747889n);
    input.add64(18446642037746374993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 2 (18438663246166747885, 18438663246166747889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438663246166747885n);
    input.add64(18438663246166747889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 3 (18438663246166747889, 18438663246166747889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438663246166747889n);
    input.add64(18438663246166747889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint64) => ebool test 4 (18438663246166747889, 18438663246166747885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438663246166747889n);
    input.add64(18438663246166747885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 1 (18443752933039972901, 18443712363065069767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443752933039972901n);
    input.add64(18443712363065069767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443712363065069767n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 2 (18443712363065069763, 18443712363065069767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443712363065069763n);
    input.add64(18443712363065069767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443712363065069763n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 3 (18443712363065069767, 18443712363065069767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443712363065069767n);
    input.add64(18443712363065069767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443712363065069767n);
  });

  it('test operator "min" overload (euint64, euint64) => euint64 test 4 (18443712363065069767, 18443712363065069763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443712363065069767n);
    input.add64(18443712363065069763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18443712363065069763n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18438451294108378795, 18442577687927296389)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438451294108378795n);
    input.add64(18442577687927296389n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18442577687927296389n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18438451294108378791, 18438451294108378795)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438451294108378791n);
    input.add64(18438451294108378795n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18438451294108378795n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18438451294108378795, 18438451294108378795)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438451294108378795n);
    input.add64(18438451294108378795n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18438451294108378795n);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18438451294108378795, 18438451294108378791)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438451294108378795n);
    input.add64(18438451294108378791n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.resEuint64());
    expect(res).to.equal(18438451294108378795n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 1 (2, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 2 (9221755304374661500, 9221755304374661502)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9221755304374661500n);
    input.add128(9221755304374661502n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443510608749323002n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 3 (9221755304374661502, 9221755304374661502)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9221755304374661502n);
    input.add128(9221755304374661502n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443510608749323004n);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 4 (9221755304374661502, 9221755304374661500)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9221755304374661502n);
    input.add128(9221755304374661500n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443510608749323002n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 1 (18440732139362520939, 18440732139362520939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440732139362520939n);
    input.add128(18440732139362520939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 2 (18440732139362520939, 18440732139362520935)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440732139362520939n);
    input.add128(18440732139362520935n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 1 (2, 4611686018427387905)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(2n);
    input.add128(4611686018427387905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 2 (4293253776, 4293253776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4293253776n);
    input.add128(4293253776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18432027985138258176n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 3 (4293253776, 4293253776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4293253776n);
    input.add128(4293253776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18432027985138258176n);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 4 (4293253776, 4293253776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4293253776n);
    input.add128(4293253776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18432027985138258176n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 1 (18446087737904928923, 340282366920938463463372853857336537531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446087737904928923n);
    input.add128(340282366920938463463372853857336537531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18444915586417889435n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 2 (18446087737904928919, 18446087737904928923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446087737904928919n);
    input.add128(18446087737904928923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446087737904928915n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 3 (18446087737904928923, 18446087737904928923)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446087737904928923n);
    input.add128(18446087737904928923n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446087737904928923n);
  });

  it('test operator "and" overload (euint64, euint128) => euint128 test 4 (18446087737904928923, 18446087737904928919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446087737904928923n);
    input.add128(18446087737904928919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18446087737904928915n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 1 (18439648407873410973, 340282366920938463463367744294061168847)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439648407873410973n);
    input.add128(340282366920938463463367744294061168847n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463367815231955623903n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 2 (18439648407873410969, 18439648407873410973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439648407873410969n);
    input.add128(18439648407873410973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439648407873410973n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 3 (18439648407873410973, 18439648407873410973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439648407873410973n);
    input.add128(18439648407873410973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439648407873410973n);
  });

  it('test operator "or" overload (euint64, euint128) => euint128 test 4 (18439648407873410973, 18439648407873410969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439648407873410973n);
    input.add128(18439648407873410969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439648407873410973n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 1 (18439072047193176593, 340282366920938463463367280447885697999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439072047193176593n);
    input.add128(340282366920938463463367280447885697999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463444928222282043677150n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 2 (18439072047193176589, 18439072047193176593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439072047193176589n);
    input.add128(18439072047193176593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 3 (18439072047193176593, 18439072047193176593)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439072047193176593n);
    input.add128(18439072047193176593n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint128) => euint128 test 4 (18439072047193176593, 18439072047193176589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439072047193176593n);
    input.add128(18439072047193176589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 1 (18438774181738162367, 340282366920938463463374453258166270581)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438774181738162367n);
    input.add128(340282366920938463463374453258166270581n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 2 (18438774181738162363, 18438774181738162367)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438774181738162363n);
    input.add128(18438774181738162367n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 3 (18438774181738162367, 18438774181738162367)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438774181738162367n);
    input.add128(18438774181738162367n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint128) => ebool test 4 (18438774181738162367, 18438774181738162363)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438774181738162367n);
    input.add128(18438774181738162363n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 1 (18444606898825576857, 340282366920938463463373123137609345139)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444606898825576857n);
    input.add128(340282366920938463463373123137609345139n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 2 (18444606898825576853, 18444606898825576857)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444606898825576853n);
    input.add128(18444606898825576857n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 3 (18444606898825576857, 18444606898825576857)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444606898825576857n);
    input.add128(18444606898825576857n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint128) => ebool test 4 (18444606898825576857, 18444606898825576853)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444606898825576857n);
    input.add128(18444606898825576853n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 1 (18440312622281386065, 340282366920938463463367748232228352681)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440312622281386065n);
    input.add128(340282366920938463463367748232228352681n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 2 (18440312622281386061, 18440312622281386065)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440312622281386061n);
    input.add128(18440312622281386065n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 3 (18440312622281386065, 18440312622281386065)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440312622281386065n);
    input.add128(18440312622281386065n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 4 (18440312622281386065, 18440312622281386061)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18440312622281386065n);
    input.add128(18440312622281386061n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 1 (18443123772467143345, 340282366920938463463371844095590377027)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443123772467143345n);
    input.add128(340282366920938463463371844095590377027n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 2 (18443123772467143341, 18443123772467143345)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443123772467143341n);
    input.add128(18443123772467143345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 3 (18443123772467143345, 18443123772467143345)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443123772467143345n);
    input.add128(18443123772467143345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint128) => ebool test 4 (18443123772467143345, 18443123772467143341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443123772467143345n);
    input.add128(18443123772467143341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 1 (18446658830723668423, 340282366920938463463374305777419920015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446658830723668423n);
    input.add128(340282366920938463463374305777419920015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 2 (18446658830723668419, 18446658830723668423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446658830723668419n);
    input.add128(18446658830723668423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 3 (18446658830723668423, 18446658830723668423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446658830723668423n);
    input.add128(18446658830723668423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint128) => ebool test 4 (18446658830723668423, 18446658830723668419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446658830723668423n);
    input.add128(18446658830723668419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 1 (18445245135119463633, 340282366920938463463371748142939541543)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445245135119463633n);
    input.add128(340282366920938463463371748142939541543n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 2 (18445245135119463629, 18445245135119463633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445245135119463629n);
    input.add128(18445245135119463633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 3 (18445245135119463633, 18445245135119463633)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445245135119463633n);
    input.add128(18445245135119463633n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint128) => ebool test 4 (18445245135119463633, 18445245135119463629)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18445245135119463633n);
    input.add128(18445245135119463629n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 1 (18439035553571510001, 340282366920938463463372795789838376963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439035553571510001n);
    input.add128(340282366920938463463372795789838376963n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439035553571510001n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 2 (18439035553571509997, 18439035553571510001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439035553571509997n);
    input.add128(18439035553571510001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439035553571509997n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 3 (18439035553571510001, 18439035553571510001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439035553571510001n);
    input.add128(18439035553571510001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439035553571510001n);
  });

  it('test operator "min" overload (euint64, euint128) => euint128 test 4 (18439035553571510001, 18439035553571509997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439035553571510001n);
    input.add128(18439035553571509997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18439035553571509997n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 1 (18443760701340075461, 340282366920938463463370806858300131911)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443760701340075461n);
    input.add128(340282366920938463463370806858300131911n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370806858300131911n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 2 (18443760701340075457, 18443760701340075461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443760701340075457n);
    input.add128(18443760701340075461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443760701340075461n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 3 (18443760701340075461, 18443760701340075461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443760701340075461n);
    input.add128(18443760701340075461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443760701340075461n);
  });

  it('test operator "max" overload (euint64, euint128) => euint128 test 4 (18443760701340075461, 18443760701340075457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443760701340075461n);
    input.add128(18443760701340075457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(18443760701340075461n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 1 (18442097981397184345, 115792089237316195423570985008687907853269984665640564039457576996357063348639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442097981397184345n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576996357063348639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18439731415424729369n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 2 (18442097981397184341, 18442097981397184345)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442097981397184341n);
    input.add256(18442097981397184345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18442097981397184337n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 3 (18442097981397184345, 18442097981397184345)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442097981397184345n);
    input.add256(18442097981397184345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18442097981397184345n);
  });

  it('test operator "and" overload (euint64, euint256) => euint256 test 4 (18442097981397184345, 18442097981397184341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442097981397184345n);
    input.add256(18442097981397184341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18442097981397184337n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 1 (18442641511828976627, 115792089237316195423570985008687907853269984665640564039457582830231390299255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442641511828976627n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582830231390299255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582878714594916343n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 2 (18442641511828976623, 18442641511828976627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442641511828976623n);
    input.add256(18442641511828976627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18442641511828976639n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 3 (18442641511828976627, 18442641511828976627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442641511828976627n);
    input.add256(18442641511828976627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18442641511828976627n);
  });

  it('test operator "or" overload (euint64, euint256) => euint256 test 4 (18442641511828976627, 18442641511828976623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442641511828976627n);
    input.add256(18442641511828976623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(18442641511828976639n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 1 (18438327630715902513, 115792089237316195423570985008687907853269984665640564039457581226045386430157)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438327630715902513n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581226045386430157n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039439142898483659132156n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 2 (18438327630715902509, 18438327630715902513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438327630715902509n);
    input.add256(18438327630715902513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 3 (18438327630715902513, 18438327630715902513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438327630715902513n);
    input.add256(18438327630715902513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint256) => euint256 test 4 (18438327630715902513, 18438327630715902509)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438327630715902513n);
    input.add256(18438327630715902509n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.resEuint256());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 1 (18441565538804893333, 115792089237316195423570985008687907853269984665640564039457577939798401874071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441565538804893333n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577939798401874071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 2 (18441565538804893329, 18441565538804893333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441565538804893329n);
    input.add256(18441565538804893333n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 3 (18441565538804893333, 18441565538804893333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441565538804893333n);
    input.add256(18441565538804893333n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint256) => ebool test 4 (18441565538804893333, 18441565538804893329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441565538804893333n);
    input.add256(18441565538804893329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 1 (18439128674919201929, 115792089237316195423570985008687907853269984665640564039457580257471884057801)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439128674919201929n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580257471884057801n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 2 (18439128674919201925, 18439128674919201929)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439128674919201925n);
    input.add256(18439128674919201929n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 3 (18439128674919201929, 18439128674919201929)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439128674919201929n);
    input.add256(18439128674919201929n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint256) => ebool test 4 (18439128674919201929, 18439128674919201925)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439128674919201929n);
    input.add256(18439128674919201925n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 2 (25, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(25n);
    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(54n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 3 (29, 29)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(29n);
    input.add8(29n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(58n);
  });

  it('test operator "add" overload (euint128, euint8) => euint128 test 4 (29, 25)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(29n);
    input.add8(25n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(54n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 1 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(233n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint128, euint8) => euint128 test 2 (233, 229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(233n);
    input.add8(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 2 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(100n);
  });

  it('test operator "mul" overload (euint128, euint8) => euint128 test 4 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(100n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 1 (340282366920938463463368232445136682125, 96)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463368232445136682125n);
    input.add8(96n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 2 (92, 96)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(92n);
    input.add8(96n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(64n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 3 (96, 96)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(96n);
    input.add8(96n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(96n);
  });

  it('test operator "and" overload (euint128, euint8) => euint128 test 4 (96, 92)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(96n);
    input.add8(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(64n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 1 (340282366920938463463366376974412700909, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463366376974412700909n);
    input.add8(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463366376974412700927n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 2 (243, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(243n);
    input.add8(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(247n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 3 (247, 247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(247n);
    input.add8(247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(247n);
  });

  it('test operator "or" overload (euint128, euint8) => euint128 test 4 (247, 243)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(247n);
    input.add8(243n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(247n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 1 (340282366920938463463370879517420402989, 215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370879517420402989n);
    input.add8(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(340282366920938463463370879517420403194n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 2 (211, 215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(211n);
    input.add8(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 3 (215, 215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(215n);
    input.add8(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint128, euint8) => euint128 test 4 (215, 211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(215n);
    input.add8(211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.resEuint128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 1 (340282366920938463463370140168362424987, 80)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463370140168362424987n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 2 (76, 80)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(76n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 3 (80, 80)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(80n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint128, euint8) => ebool test 4 (80, 76)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(80n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 1 (340282366920938463463371805744081765363, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463371805744081765363n);
    input.add8(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 2 (86, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(86n);
    input.add8(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 3 (90, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(90n);
    input.add8(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint128, euint8) => ebool test 4 (90, 86)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(90n);
    input.add8(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 1 (340282366920938463463369439662798642887, 152)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369439662798642887n);
    input.add8(152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 2 (148, 152)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(148n);
    input.add8(152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 3 (152, 152)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(152n);
    input.add8(152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint128, euint8) => ebool test 4 (152, 148)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(152n);
    input.add8(148n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 1 (340282366920938463463373376221366404481, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373376221366404481n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 2 (229, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(229n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 3 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(233n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint128, euint8) => ebool test 4 (233, 229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(233n);
    input.add8(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 1 (340282366920938463463373189218389193517, 173)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463373189218389193517n);
    input.add8(173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 2 (169, 173)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(169n);
    input.add8(173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 3 (173, 173)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(173n);
    input.add8(173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint128, euint8) => ebool test 4 (173, 169)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(173n);
    input.add8(169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 1 (340282366920938463463369063931545643077, 119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(340282366920938463463369063931545643077n);
    input.add8(119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 2 (115, 119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(115n);
    input.add8(119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 3 (119, 119)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(119n);
    input.add8(119n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 4 (119, 115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add128(119n);
    input.add8(115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resEbool());
    expect(res).to.equal(false);
  });
});
