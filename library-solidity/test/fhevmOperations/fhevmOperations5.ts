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

describe('FHEVM operations 5', function () {
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

  it('test operator "mul" overload (euint32, euint128) => euint128 test 1 (2, 1073741825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2n);
    input.add128(1073741825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 2 (60830, 60830)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(60830n);
    input.add128(60830n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(3700288900n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 3 (60830, 60830)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(60830n);
    input.add128(60830n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(3700288900n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 4 (60830, 60830)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(60830n);
    input.add128(60830n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(3700288900n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 1 (1997451033, 340282366920938463463369942827932883171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1997451033n);
    input.add128(340282366920938463463369942827932883171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1090553857n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 2 (1997451029, 1997451033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1997451029n);
    input.add128(1997451033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1997451025n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 3 (1997451033, 1997451033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1997451033n);
    input.add128(1997451033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1997451033n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 4 (1997451033, 1997451029)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1997451033n);
    input.add128(1997451029n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1997451025n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 1 (591798078, 340282366920938463463373100740066557395)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(591798078n);
    input.add128(340282366920938463463373100740066557395n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(340282366920938463463373100740653760511n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 2 (591798074, 591798078)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(591798074n);
    input.add128(591798078n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(591798078n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 3 (591798078, 591798078)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(591798078n);
    input.add128(591798078n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(591798078n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 4 (591798078, 591798074)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(591798078n);
    input.add128(591798074n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(591798078n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 1 (3169446936, 340282366920938463463368419797078474003)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3169446936n);
    input.add128(340282366920938463463368419797078474003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(340282366920938463463368419799970910475n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 2 (3169446932, 3169446936)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3169446932n);
    input.add128(3169446936n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 3 (3169446936, 3169446936)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3169446936n);
    input.add128(3169446936n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 4 (3169446936, 3169446932)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3169446936n);
    input.add128(3169446932n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 1 (2415242219, 340282366920938463463369945263328343293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2415242219n);
    input.add128(340282366920938463463369945263328343293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 2 (2415242215, 2415242219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2415242215n);
    input.add128(2415242219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 3 (2415242219, 2415242219)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2415242219n);
    input.add128(2415242219n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 4 (2415242219, 2415242215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2415242219n);
    input.add128(2415242215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 1 (2427864873, 340282366920938463463374382805944303091)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2427864873n);
    input.add128(340282366920938463463374382805944303091n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 2 (2427864869, 2427864873)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2427864869n);
    input.add128(2427864873n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 3 (2427864873, 2427864873)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2427864873n);
    input.add128(2427864873n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 4 (2427864873, 2427864869)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2427864873n);
    input.add128(2427864869n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 1 (782444147, 340282366920938463463374051705821950555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(782444147n);
    input.add128(340282366920938463463374051705821950555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 2 (782444143, 782444147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(782444143n);
    input.add128(782444147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 3 (782444147, 782444147)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(782444147n);
    input.add128(782444147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 4 (782444147, 782444143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(782444147n);
    input.add128(782444143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 1 (2491536855, 340282366920938463463372115819442705117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2491536855n);
    input.add128(340282366920938463463372115819442705117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 2 (2491536851, 2491536855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2491536851n);
    input.add128(2491536855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 3 (2491536855, 2491536855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2491536855n);
    input.add128(2491536855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 4 (2491536855, 2491536851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2491536855n);
    input.add128(2491536851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 1 (212538201, 340282366920938463463365701725008226387)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(212538201n);
    input.add128(340282366920938463463365701725008226387n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 2 (212538197, 212538201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(212538197n);
    input.add128(212538201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 3 (212538201, 212538201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(212538201n);
    input.add128(212538201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 4 (212538201, 212538197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(212538201n);
    input.add128(212538197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 1 (3269635554, 340282366920938463463374052337005621173)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3269635554n);
    input.add128(340282366920938463463374052337005621173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 2 (3269635550, 3269635554)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3269635550n);
    input.add128(3269635554n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 3 (3269635554, 3269635554)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3269635554n);
    input.add128(3269635554n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 4 (3269635554, 3269635550)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3269635554n);
    input.add128(3269635550n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 1 (2619152467, 340282366920938463463369483310847580777)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2619152467n);
    input.add128(340282366920938463463369483310847580777n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2619152467n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 2 (2619152463, 2619152467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2619152463n);
    input.add128(2619152467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2619152463n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 3 (2619152467, 2619152467)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2619152467n);
    input.add128(2619152467n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2619152467n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 4 (2619152467, 2619152463)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2619152467n);
    input.add128(2619152463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(2619152463n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 1 (1481745465, 340282366920938463463368802736595593169)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1481745465n);
    input.add128(340282366920938463463368802736595593169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(340282366920938463463368802736595593169n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 2 (1481745461, 1481745465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1481745461n);
    input.add128(1481745465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1481745465n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 3 (1481745465, 1481745465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1481745465n);
    input.add128(1481745465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1481745465n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 4 (1481745465, 1481745461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1481745465n);
    input.add128(1481745461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract3.resEuint128());
    expect(res).to.equal(1481745465n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 1 (535310887, 115792089237316195423570985008687907853269984665640564039457583588000152226895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(535310887n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583588000152226895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(69206023n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 2 (535310883, 535310887)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(535310883n);
    input.add256(535310887n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(535310883n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 3 (535310887, 535310887)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(535310887n);
    input.add256(535310887n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(535310887n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 4 (535310887, 535310883)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(535310887n);
    input.add256(535310883n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(535310883n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 1 (4191159839, 115792089237316195423570985008687907853269984665640564039457579951280683637579)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4191159839n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579951280683637579n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579951282055176031n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 2 (4191159835, 4191159839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4191159835n);
    input.add256(4191159839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(4191159839n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 3 (4191159839, 4191159839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4191159839n);
    input.add256(4191159839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(4191159839n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 4 (4191159839, 4191159835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(4191159839n);
    input.add256(4191159835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(4191159839n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 1 (3860826133, 115792089237316195423570985008687907853269984665640564039457577835989184439075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3860826133n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577835989184439075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577835985527761718n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 2 (3860826129, 3860826133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3860826129n);
    input.add256(3860826133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 3 (3860826133, 3860826133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3860826133n);
    input.add256(3860826133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 4 (3860826133, 3860826129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3860826133n);
    input.add256(3860826129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract3.resEuint256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 1 (1721291556, 115792089237316195423570985008687907853269984665640564039457575577106058957921)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1721291556n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575577106058957921n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 2 (1721291552, 1721291556)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1721291552n);
    input.add256(1721291556n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 3 (1721291556, 1721291556)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1721291556n);
    input.add256(1721291556n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 4 (1721291556, 1721291552)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1721291556n);
    input.add256(1721291552n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 1 (3680858468, 115792089237316195423570985008687907853269984665640564039457575201278275019623)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3680858468n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575201278275019623n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 2 (3680858464, 3680858468)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3680858464n);
    input.add256(3680858468n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 3 (3680858468, 3680858468)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3680858468n);
    input.add256(3680858468n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 4 (3680858468, 3680858464)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3680858468n);
    input.add256(3680858464n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 1 (129, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(129n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(131n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 2 (70, 74)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(70n);
    input.add8(74n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(144n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 3 (74, 74)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(74n);
    input.add8(74n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(148n);
  });

  it('test operator "add" overload (euint64, euint8) => euint64 test 4 (74, 70)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(74n);
    input.add8(70n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(144n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (88, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(88n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (88, 84)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(88n);
    input.add8(84n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 1 (65, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(65n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(130n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 2 (13, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(13n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(182n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 3 (14, 14)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(14n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(196n);
  });

  it('test operator "mul" overload (euint64, euint8) => euint64 test 4 (14, 13)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(14n);
    input.add8(13n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(182n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 1 (18438709455774753353, 108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438709455774753353n);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(72n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 2 (104, 108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(104n);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(104n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 3 (108, 108)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(108n);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(108n);
  });

  it('test operator "and" overload (euint64, euint8) => euint64 test 4 (108, 104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(108n);
    input.add8(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(104n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (18445703274918298681, 205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445703274918298681n);
    input.add8(205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18445703274918298877n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (201, 205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(201n);
    input.add8(205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(205n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (205, 205)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(205n);
    input.add8(205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(205n);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (205, 201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(205n);
    input.add8(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(205n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 1 (18438703116124632189, 197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438703116124632189n);
    input.add8(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18438703116124632248n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 2 (193, 197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(193n);
    input.add8(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 3 (197, 197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(197n);
    input.add8(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint8) => euint64 test 4 (197, 193)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(197n);
    input.add8(193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 1 (18437740182078246311, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18437740182078246311n);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 2 (38, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(38n);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 3 (42, 42)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(42n);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint8) => ebool test 4 (42, 38)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(42n);
    input.add8(38n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (18442817551586829883, 203)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442817551586829883n);
    input.add8(203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (199, 203)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(199n);
    input.add8(203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (203, 203)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(203n);
    input.add8(203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (203, 199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(203n);
    input.add8(199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 1 (18442960751149440157, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442960751149440157n);
    input.add8(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 2 (86, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(86n);
    input.add8(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 3 (90, 90)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(90n);
    input.add8(90n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint8) => ebool test 4 (90, 86)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(90n);
    input.add8(86n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 1 (18443543847979543195, 66)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18443543847979543195n);
    input.add8(66n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 2 (62, 66)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(62n);
    input.add8(66n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 3 (66, 66)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(66n);
    input.add8(66n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint8) => ebool test 4 (66, 62)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(66n);
    input.add8(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 1 (18439913580202901419, 159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439913580202901419n);
    input.add8(159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 2 (155, 159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(155n);
    input.add8(159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 3 (159, 159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(159n);
    input.add8(159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint8) => ebool test 4 (159, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(159n);
    input.add8(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (18442981272892827053, 91)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442981272892827053n);
    input.add8(91n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (87, 91)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(87n);
    input.add8(91n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (91, 91)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(91n);
    input.add8(91n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (91, 87)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(91n);
    input.add8(87n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 1 (18444658198376730513, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444658198376730513n);
    input.add8(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(45n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 2 (41, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(41n);
    input.add8(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(41n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 3 (45, 45)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(45n);
    input.add8(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(45n);
  });

  it('test operator "min" overload (euint64, euint8) => euint64 test 4 (45, 41)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(45n);
    input.add8(41n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(41n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 1 (18442216450702800511, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442216450702800511n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18442216450702800511n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 2 (84, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(84n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(88n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 3 (88, 88)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(88n);
    input.add8(88n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(88n);
  });

  it('test operator "max" overload (euint64, euint8) => euint64 test 4 (88, 84)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(88n);
    input.add8(84n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(88n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 1 (65525, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(65525n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(65527n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 2 (22316, 22318)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(22316n);
    input.add16(22318n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(44634n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 3 (22318, 22318)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(22318n);
    input.add16(22318n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(44636n);
  });

  it('test operator "add" overload (euint64, euint16) => euint64 test 4 (22318, 22316)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(22318n);
    input.add16(22316n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(44634n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (53406, 53406)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(53406n);
    input.add16(53406n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (53406, 53402)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(53406n);
    input.add16(53402n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 1 (16385, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(16385n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(32770n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 2 (191, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(191n);
    input.add16(191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(36481n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 3 (191, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(191n);
    input.add16(191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(36481n);
  });

  it('test operator "mul" overload (euint64, euint16) => euint64 test 4 (191, 191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(191n);
    input.add16(191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(36481n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (18445357578470770837, 45183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445357578470770837n);
    input.add16(45183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(21n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (45179, 45183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(45179n);
    input.add16(45183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(45179n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (45183, 45183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(45183n);
    input.add16(45183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(45183n);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (45183, 45179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(45183n);
    input.add16(45179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(45179n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 1 (18439033205155064567, 49930)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18439033205155064567n);
    input.add16(49930n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18439033205155097599n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 2 (49926, 49930)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(49926n);
    input.add16(49930n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(49934n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 3 (49930, 49930)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(49930n);
    input.add16(49930n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(49930n);
  });

  it('test operator "or" overload (euint64, euint16) => euint64 test 4 (49930, 49926)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(49930n);
    input.add16(49926n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(49934n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 1 (18443449853644770941, 24290)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18443449853644770941n);
    input.add16(24290n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18443449853644787871n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 2 (24286, 24290)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(24286n);
    input.add16(24290n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(60n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 3 (24290, 24290)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(24290n);
    input.add16(24290n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint16) => euint64 test 4 (24290, 24286)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(24290n);
    input.add16(24286n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(60n);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 1 (18444484589757065035, 57668)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444484589757065035n);
    input.add16(57668n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 2 (57664, 57668)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(57664n);
    input.add16(57668n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 3 (57668, 57668)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(57668n);
    input.add16(57668n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint16) => ebool test 4 (57668, 57664)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(57668n);
    input.add16(57664n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 1 (18444745361940887203, 53150)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18444745361940887203n);
    input.add16(53150n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 2 (53146, 53150)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(53146n);
    input.add16(53150n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 3 (53150, 53150)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(53150n);
    input.add16(53150n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint16) => ebool test 4 (53150, 53146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(53150n);
    input.add16(53146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 1 (18445407486885492837, 63977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445407486885492837n);
    input.add16(63977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 2 (63973, 63977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(63973n);
    input.add16(63977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 3 (63977, 63977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(63977n);
    input.add16(63977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint16) => ebool test 4 (63977, 63973)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(63977n);
    input.add16(63973n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 1 (18443208422804943567, 50160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18443208422804943567n);
    input.add16(50160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 2 (50156, 50160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(50156n);
    input.add16(50160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 3 (50160, 50160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(50160n);
    input.add16(50160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint16) => ebool test 4 (50160, 50156)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(50160n);
    input.add16(50156n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 1 (18445361999917839537, 57803)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445361999917839537n);
    input.add16(57803n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 2 (57799, 57803)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(57799n);
    input.add16(57803n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 3 (57803, 57803)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(57803n);
    input.add16(57803n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint16) => ebool test 4 (57803, 57799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(57803n);
    input.add16(57799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (18441213039173179351, 25840)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18441213039173179351n);
    input.add16(25840n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (25836, 25840)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(25836n);
    input.add16(25840n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (25840, 25840)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(25840n);
    input.add16(25840n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (25840, 25836)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(25840n);
    input.add16(25836n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 1 (18446408891367972699, 20433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18446408891367972699n);
    input.add16(20433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(20433n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 2 (20429, 20433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(20429n);
    input.add16(20433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(20429n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 3 (20433, 20433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(20433n);
    input.add16(20433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(20433n);
  });

  it('test operator "min" overload (euint64, euint16) => euint64 test 4 (20433, 20429)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(20433n);
    input.add16(20429n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(20429n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18438207570834497397, 11753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18438207570834497397n);
    input.add16(11753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18438207570834497397n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (11749, 11753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(11749n);
    input.add16(11753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(11753n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (11753, 11753)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(11753n);
    input.add16(11753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(11753n);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (11753, 11749)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(11753n);
    input.add16(11749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(11753n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 1 (4293484833, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4293484833n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4293484835n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 2 (1800841496, 1800841500)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1800841496n);
    input.add32(1800841500n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3601682996n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 3 (1800841500, 1800841500)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1800841500n);
    input.add32(1800841500n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3601683000n);
  });

  it('test operator "add" overload (euint64, euint32) => euint64 test 4 (1800841500, 1800841496)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1800841500n);
    input.add32(1800841496n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(3601682996n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (1432316955, 1432316955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1432316955n);
    input.add32(1432316955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (1432316955, 1432316951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1432316955n);
    input.add32(1432316951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (2147439794, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2147439794n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(4294879588n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (33368, 33368)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(33368n);
    input.add32(33368n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1113423424n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (33368, 33368)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(33368n);
    input.add32(33368n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1113423424n);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (33368, 33368)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(33368n);
    input.add32(33368n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1113423424n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 1 (18445356475742800535, 2278520798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18445356475742800535n);
    input.add32(2278520798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2215199382n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 2 (2278520794, 2278520798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2278520794n);
    input.add32(2278520798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2278520794n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 3 (2278520798, 2278520798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2278520798n);
    input.add32(2278520798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2278520798n);
  });

  it('test operator "and" overload (euint64, euint32) => euint64 test 4 (2278520798, 2278520794)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2278520798n);
    input.add32(2278520794n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(2278520794n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 1 (18440484494595149887, 1126692253)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18440484494595149887n);
    input.add32(1126692253n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18440484495686104511n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 2 (1126692249, 1126692253)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1126692249n);
    input.add32(1126692253n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1126692253n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 3 (1126692253, 1126692253)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1126692253n);
    input.add32(1126692253n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1126692253n);
  });

  it('test operator "or" overload (euint64, euint32) => euint64 test 4 (1126692253, 1126692249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(1126692253n);
    input.add32(1126692249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(1126692253n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 1 (18441878331604531325, 2672430440)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18441878331604531325n);
    input.add32(2672430440n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(18441878334201197845n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 2 (2672430436, 2672430440)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2672430436n);
    input.add32(2672430440n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 3 (2672430440, 2672430440)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2672430440n);
    input.add32(2672430440n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint32) => euint64 test 4 (2672430440, 2672430436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(2672430440n);
    input.add32(2672430436n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.resEuint64());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 1 (18442751040149615567, 4287528180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(18442751040149615567n);
    input.add32(4287528180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 2 (4287528176, 4287528180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4287528176n);
    input.add32(4287528180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 3 (4287528180, 4287528180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4287528180n);
    input.add32(4287528180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint32) => ebool test 4 (4287528180, 4287528176)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add64(4287528180n);
    input.add32(4287528176n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resEbool());
    expect(res).to.equal(false);
  });
});
