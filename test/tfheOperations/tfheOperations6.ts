import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import type { TFHETestSuite3 } from '../../types/contracts/tests/TFHETestSuite3';
import type { TFHETestSuite4 } from '../../types/contracts/tests/TFHETestSuite4';
import type { TFHETestSuite5 } from '../../types/contracts/tests/TFHETestSuite5';
import type { TFHETestSuite6 } from '../../types/contracts/tests/TFHETestSuite6';
import { createInstances, decrypt4, decrypt8, decrypt16, decrypt32, decrypt64, decryptBool } from '../instance';
import { getSigners, initSigners } from '../signers';

async function deployTfheTestFixture1(): Promise<TFHETestSuite1> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite1');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture2(): Promise<TFHETestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite2');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture3(): Promise<TFHETestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite3');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture4(): Promise<TFHETestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite4');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture5(): Promise<TFHETestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite5');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture6(): Promise<TFHETestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite6');
  const contract = await contractFactory.connect(admin).deploy({
    value: ethers.parseEther('0.001'),
  });
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations 6', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();

    const contract1 = await deployTfheTestFixture1();
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployTfheTestFixture2();
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployTfheTestFixture3();
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployTfheTestFixture4();
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployTfheTestFixture5();
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployTfheTestFixture6();
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 1 (46663, 31)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46663n);
    input.add8(31n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(46663n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 2 (27, 31)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(27n);
    input.add8(31n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(31n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 3 (31, 31)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(31n);
    input.add8(31n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(31n);
  });

  it('test operator "max" overload (euint16, euint8) => euint16 test 4 (31, 27)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(31n);
    input.add8(27n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(31n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 1 (12135, 15379)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12135n);
    input.add16(15379n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(27514n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 2 (12131, 12135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12131n);
    input.add16(12135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24266n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 3 (12135, 12135)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12135n);
    input.add16(12135n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24270n);
  });

  it('test operator "add" overload (euint16, euint16) => euint16 test 4 (12135, 12131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12135n);
    input.add16(12131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24266n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 1 (3171, 3171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3171n);
    input.add16(3171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint16) => euint16 test 2 (3171, 3167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3171n);
    input.add16(3167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 1 (78, 381)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(78n);
    input.add16(381n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(29718n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 2 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(155n);
    input.add16(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24025n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 3 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(155n);
    input.add16(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24025n);
  });

  it('test operator "mul" overload (euint16, euint16) => euint16 test 4 (155, 155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(155n);
    input.add16(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(24025n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 1 (3535, 5032)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3535n);
    input.add16(5032n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(392n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 2 (3531, 3535)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3531n);
    input.add16(3535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(3531n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 3 (3535, 3535)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3535n);
    input.add16(3535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(3535n);
  });

  it('test operator "and" overload (euint16, euint16) => euint16 test 4 (3535, 3531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3535n);
    input.add16(3531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(3531n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 1 (33634, 27003)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(33634n);
    input.add16(27003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(60283n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 2 (26999, 27003)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(26999n);
    input.add16(27003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(27007n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 3 (27003, 27003)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(27003n);
    input.add16(27003n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(27003n);
  });

  it('test operator "or" overload (euint16, euint16) => euint16 test 4 (27003, 26999)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(27003n);
    input.add16(26999n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(27007n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (64619, 2520)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(64619n);
    input.add16(2520n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(62899n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (2516, 2520)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(2516n);
    input.add16(2520n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (2520, 2520)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(2520n);
    input.add16(2520n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (2520, 2516)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(2520n);
    input.add16(2516n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 1 (63705, 3249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(63705n);
    input.add16(3249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 2 (3245, 3249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3245n);
    input.add16(3249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 3 (3249, 3249)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3249n);
    input.add16(3249n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint16) => ebool test 4 (3249, 3245)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(3249n);
    input.add16(3245n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 1 (27750, 23851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(27750n);
    input.add16(23851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 2 (23847, 23851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(23847n);
    input.add16(23851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 3 (23851, 23851)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(23851n);
    input.add16(23851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint16) => ebool test 4 (23851, 23847)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(23851n);
    input.add16(23847n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 1 (8825, 39131)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8825n);
    input.add16(39131n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 2 (8821, 8825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8821n);
    input.add16(8825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 3 (8825, 8825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8825n);
    input.add16(8825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint16) => ebool test 4 (8825, 8821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(8825n);
    input.add16(8821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 1 (33735, 54333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(33735n);
    input.add16(54333n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 2 (33731, 33735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(33731n);
    input.add16(33735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 3 (33735, 33735)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(33735n);
    input.add16(33735n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint16) => ebool test 4 (33735, 33731)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(33735n);
    input.add16(33731n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 1 (38605, 53936)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(38605n);
    input.add16(53936n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 2 (38601, 38605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(38601n);
    input.add16(38605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 3 (38605, 38605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(38605n);
    input.add16(38605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint16) => ebool test 4 (38605, 38601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(38605n);
    input.add16(38601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 1 (59332, 12427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(59332n);
    input.add16(12427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 2 (12423, 12427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12423n);
    input.add16(12427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 3 (12427, 12427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12427n);
    input.add16(12427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint16) => ebool test 4 (12427, 12423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12427n);
    input.add16(12423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 1 (18209, 14696)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(18209n);
    input.add16(14696n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14696n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 2 (14692, 14696)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(14692n);
    input.add16(14696n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14692n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 3 (14696, 14696)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(14696n);
    input.add16(14696n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14696n);
  });

  it('test operator "min" overload (euint16, euint16) => euint16 test 4 (14696, 14692)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(14696n);
    input.add16(14692n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(14692n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (35722, 7688)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(35722n);
    input.add16(7688n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(35722n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (7684, 7688)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7684n);
    input.add16(7688n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7688n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (7688, 7688)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7688n);
    input.add16(7688n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7688n);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (7688, 7684)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(7688n);
    input.add16(7684n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract3.res16());
    expect(res).to.equal(7688n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (2, 40496)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(2n);
    input.add32(40496n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(40498n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (18317, 18319)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(18317n);
    input.add32(18319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(36636n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (18319, 18319)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(18319n);
    input.add32(18319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(36638n);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (18319, 18317)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(18319n);
    input.add32(18317n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(36636n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (17812, 17812)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(17812n);
    input.add32(17812n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (17812, 17808)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(17812n);
    input.add32(17808n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (2, 18411)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(2n);
    input.add32(18411n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(36822n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (139, 139)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(139n);
    input.add32(139n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(19321n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (139, 139)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(139n);
    input.add32(139n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(19321n);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (139, 139)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(139n);
    input.add32(139n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(19321n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (27862, 1736690738)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(27862n);
    input.add32(1736690738n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(19474n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (27858, 27862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(27858n);
    input.add32(27862n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(27858n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (27862, 27862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(27862n);
    input.add32(27862n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(27862n);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (27862, 27858)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(27862n);
    input.add32(27858n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(27858n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (440, 1863142416)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(440n);
    input.add32(1863142416n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(1863142840n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (436, 440)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(436n);
    input.add32(440n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(444n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (440, 440)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(440n);
    input.add32(440n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(440n);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (440, 436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(440n);
    input.add32(436n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(444n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 1 (51717, 501889469)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(51717n);
    input.add32(501889469n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(501937080n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 2 (51713, 51717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(51713n);
    input.add32(51717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 3 (51717, 51717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(51717n);
    input.add32(51717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint32) => euint32 test 4 (51717, 51713)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(51717n);
    input.add32(51713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 1 (19428, 3260350416)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(19428n);
    input.add32(3260350416n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 2 (19424, 19428)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(19424n);
    input.add32(19428n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 3 (19428, 19428)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(19428n);
    input.add32(19428n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint32) => ebool test 4 (19428, 19424)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(19428n);
    input.add32(19424n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 1 (4461, 3322499889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(4461n);
    input.add32(3322499889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 2 (4457, 4461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(4457n);
    input.add32(4461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 3 (4461, 4461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(4461n);
    input.add32(4461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint32) => ebool test 4 (4461, 4457)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(4461n);
    input.add32(4457n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 1 (23826, 3191695510)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(23826n);
    input.add32(3191695510n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 2 (23822, 23826)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(23822n);
    input.add32(23826n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 3 (23826, 23826)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(23826n);
    input.add32(23826n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint32) => ebool test 4 (23826, 23822)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(23826n);
    input.add32(23822n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 1 (20452, 2742403767)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(20452n);
    input.add32(2742403767n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 2 (20448, 20452)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(20448n);
    input.add32(20452n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 3 (20452, 20452)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(20452n);
    input.add32(20452n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint32) => ebool test 4 (20452, 20448)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(20452n);
    input.add32(20448n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (32489, 2097840840)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(32489n);
    input.add32(2097840840n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (32485, 32489)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(32485n);
    input.add32(32489n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (32489, 32489)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(32489n);
    input.add32(32489n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (32489, 32485)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(32489n);
    input.add32(32485n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 1 (46117, 3066026054)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46117n);
    input.add32(3066026054n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 2 (46113, 46117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46113n);
    input.add32(46117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 3 (46117, 46117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46117n);
    input.add32(46117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint32) => ebool test 4 (46117, 46113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46117n);
    input.add32(46113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 1 (5231, 2030449754)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(5231n);
    input.add32(2030449754n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(5231n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 2 (5227, 5231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(5227n);
    input.add32(5231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(5227n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 3 (5231, 5231)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(5231n);
    input.add32(5231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(5231n);
  });

  it('test operator "min" overload (euint16, euint32) => euint32 test 4 (5231, 5227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(5231n);
    input.add32(5227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(5227n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 1 (34046, 3126513605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34046n);
    input.add32(3126513605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(3126513605n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 2 (34042, 34046)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34042n);
    input.add32(34046n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(34046n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 3 (34046, 34046)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34046n);
    input.add32(34046n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(34046n);
  });

  it('test operator "max" overload (euint16, euint32) => euint32 test 4 (34046, 34042)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(34046n);
    input.add32(34042n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(34046n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 1 (2, 65506)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(2n);
    input.add64(65506n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(65508n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 2 (12725, 12729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12725n);
    input.add64(12729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(25454n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 3 (12729, 12729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12729n);
    input.add64(12729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(25458n);
  });

  it('test operator "add" overload (euint16, euint64) => euint64 test 4 (12729, 12725)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12729n);
    input.add64(12725n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(25454n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 1 (45021, 45021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(45021n);
    input.add64(45021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint16, euint64) => euint64 test 2 (45021, 45017)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(45021n);
    input.add64(45017n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (2, 32761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(2n);
    input.add64(32761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(65522n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(221n);
    input.add64(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(48841n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(221n);
    input.add64(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(48841n);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (221, 221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(221n);
    input.add64(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(48841n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (17953, 18443736631892089939)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(17953n);
    input.add64(18443736631892089939n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(1025n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (17949, 17953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(17949n);
    input.add64(17953n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(17921n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (17953, 17953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(17953n);
    input.add64(17953n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(17953n);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (17953, 17949)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(17953n);
    input.add64(17949n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(17921n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 1 (12837, 18444850378165005521)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12837n);
    input.add64(18444850378165005521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(18444850378165010165n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 2 (12833, 12837)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12833n);
    input.add64(12837n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(12837n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 3 (12837, 12837)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12837n);
    input.add64(12837n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(12837n);
  });

  it('test operator "or" overload (euint16, euint64) => euint64 test 4 (12837, 12833)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12837n);
    input.add64(12833n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(12837n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 1 (12673, 18438027026491597713)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12673n);
    input.add64(18438027026491597713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(18438027026491593232n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 2 (12669, 12673)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12669n);
    input.add64(12673n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(252n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 3 (12673, 12673)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12673n);
    input.add64(12673n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint16, euint64) => euint64 test 4 (12673, 12669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(12673n);
    input.add64(12669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(252n);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 1 (15185, 18442214214141554887)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(15185n);
    input.add64(18442214214141554887n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 2 (15181, 15185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(15181n);
    input.add64(15185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 3 (15185, 15185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(15185n);
    input.add64(15185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint16, euint64) => ebool test 4 (15185, 15181)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(15185n);
    input.add64(15181n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 1 (6072, 18442627728041568827)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(6072n);
    input.add64(18442627728041568827n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 2 (6068, 6072)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(6068n);
    input.add64(6072n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 3 (6072, 6072)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(6072n);
    input.add64(6072n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint16, euint64) => ebool test 4 (6072, 6068)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(6072n);
    input.add64(6068n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 1 (63199, 18444428591303537867)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(63199n);
    input.add64(18444428591303537867n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 2 (63195, 63199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(63195n);
    input.add64(63199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 3 (63199, 63199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(63199n);
    input.add64(63199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint16, euint64) => ebool test 4 (63199, 63195)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(63199n);
    input.add64(63195n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 1 (63105, 18440430414982980885)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(63105n);
    input.add64(18440430414982980885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 2 (63101, 63105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(63101n);
    input.add64(63105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 3 (63105, 63105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(63105n);
    input.add64(63105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint16, euint64) => ebool test 4 (63105, 63101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(63105n);
    input.add64(63101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 1 (46492, 18439416152876310159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46492n);
    input.add64(18439416152876310159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 2 (46488, 46492)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46488n);
    input.add64(46492n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 3 (46492, 46492)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46492n);
    input.add64(46492n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint16, euint64) => ebool test 4 (46492, 46488)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(46492n);
    input.add64(46488n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 1 (20266, 18439286572290779257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(20266n);
    input.add64(18439286572290779257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 2 (20262, 20266)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(20262n);
    input.add64(20266n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 3 (20266, 20266)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(20266n);
    input.add64(20266n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint16, euint64) => ebool test 4 (20266, 20262)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(20266n);
    input.add64(20262n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 1 (44251, 18443711845572494969)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(44251n);
    input.add64(18443711845572494969n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(44251n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 2 (44247, 44251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(44247n);
    input.add64(44251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(44247n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 3 (44251, 44251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(44251n);
    input.add64(44251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(44251n);
  });

  it('test operator "min" overload (euint16, euint64) => euint64 test 4 (44251, 44247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(44251n);
    input.add64(44247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(44247n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 1 (28367, 18440532375906129937)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(28367n);
    input.add64(18440532375906129937n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(18440532375906129937n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 2 (28363, 28367)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(28363n);
    input.add64(28367n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(28367n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 3 (28367, 28367)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(28367n);
    input.add64(28367n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(28367n);
  });

  it('test operator "max" overload (euint16, euint64) => euint64 test 4 (28367, 28363)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add16(28367n);
    input.add64(28363n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract3.res64());
    expect(res).to.equal(28367n);
  });
});
