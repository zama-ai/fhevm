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

describe('TFHE operations 9', function () {
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

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (709957546, 18441724614470442911)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(709957546n);
    input.add64(18441724614470442911n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (709957542, 709957546)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(709957542n);
    input.add64(709957546n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (709957546, 709957546)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(709957546n);
    input.add64(709957546n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (709957546, 709957542)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(709957546n);
    input.add64(709957542n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (897008808, 18446223426742336677)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(897008808n);
    input.add64(18446223426742336677n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (897008804, 897008808)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(897008804n);
    input.add64(897008808n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (897008808, 897008808)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(897008808n);
    input.add64(897008808n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (897008808, 897008804)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(897008808n);
    input.add64(897008804n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (2551620349, 18444102078323452175)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2551620349n);
    input.add64(18444102078323452175n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (2551620345, 2551620349)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2551620345n);
    input.add64(2551620349n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (2551620349, 2551620349)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2551620349n);
    input.add64(2551620349n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (2551620349, 2551620345)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2551620349n);
    input.add64(2551620345n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (2256977835, 18442208006659778105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2256977835n);
    input.add64(18442208006659778105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (2256977831, 2256977835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2256977831n);
    input.add64(2256977835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (2256977835, 2256977835)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2256977835n);
    input.add64(2256977835n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (2256977835, 2256977831)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2256977835n);
    input.add64(2256977831n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (557237517, 18442682499983449751)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(557237517n);
    input.add64(18442682499983449751n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (557237513, 557237517)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(557237513n);
    input.add64(557237517n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (557237517, 557237517)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(557237517n);
    input.add64(557237517n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (557237517, 557237513)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(557237517n);
    input.add64(557237513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (1941297565, 18438856464497751915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1941297565n);
    input.add64(18438856464497751915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (1941297561, 1941297565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1941297561n);
    input.add64(1941297565n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (1941297565, 1941297565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1941297565n);
    input.add64(1941297565n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (1941297565, 1941297561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1941297565n);
    input.add64(1941297561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (3364933042, 18443534177311383841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3364933042n);
    input.add64(18443534177311383841n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3364933042n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (3364933038, 3364933042)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3364933038n);
    input.add64(3364933042n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3364933038n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (3364933042, 3364933042)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3364933042n);
    input.add64(3364933042n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3364933042n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (3364933042, 3364933038)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3364933042n);
    input.add64(3364933038n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3364933038n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (821915788, 18441217303884363213)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(821915788n);
    input.add64(18441217303884363213n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18441217303884363213n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (821915784, 821915788)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(821915784n);
    input.add64(821915788n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(821915788n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (821915788, 821915788)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(821915788n);
    input.add64(821915788n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(821915788n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (821915788, 821915784)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(821915788n);
    input.add64(821915784n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(821915788n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (451501910, 2087738652)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(451501910n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_uint32(
      encryptedAmount.handles[0],
      2087738652n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2539240562n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (903003814, 903003818)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(903003814n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_uint32(
      encryptedAmount.handles[0],
      903003818n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1806007632n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (903003818, 903003818)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(903003818n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_uint32(
      encryptedAmount.handles[0],
      903003818n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1806007636n);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (903003818, 903003814)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(903003818n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_uint32(
      encryptedAmount.handles[0],
      903003814n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1806007632n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (823987277, 2087738652)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2087738652n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint32_euint32(
      823987277n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2911725929n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (903003814, 903003818)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(903003818n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint32_euint32(
      903003814n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1806007632n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (903003818, 903003818)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(903003818n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint32_euint32(
      903003818n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1806007636n);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (903003818, 903003814)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(903003814n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_uint32_euint32(
      903003818n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1806007632n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (296156690, 296156690)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(296156690n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_uint32(
      encryptedAmount.handles[0],
      296156690n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (296156690, 296156686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(296156690n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_uint32(
      encryptedAmount.handles[0],
      296156686n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 1 (296156690, 296156690)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(296156690n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_uint32_euint32(
      296156690n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint32, euint32) => euint32 test 2 (296156690, 296156686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(296156686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_uint32_euint32(
      296156690n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 1 (54583, 52500)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(54583n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_uint32(encryptedAmount.handles[0], 52500n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2865607500n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 2 (42446, 42446)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(42446n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_uint32(encryptedAmount.handles[0], 42446n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1801662916n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 3 (42446, 42446)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(42446n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_uint32(encryptedAmount.handles[0], 42446n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1801662916n);
  });

  it('test operator "mul" overload (euint32, uint32) => euint32 test 4 (42446, 42446)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(42446n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_uint32(encryptedAmount.handles[0], 42446n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1801662916n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (39745, 104998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(104998n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint32_euint32(39745n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4173145510n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (42446, 42446)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(42446n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint32_euint32(42446n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1801662916n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (42446, 42446)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(42446n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint32_euint32(42446n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1801662916n);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (42446, 42446)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(42446n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_uint32_euint32(42446n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1801662916n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 1 (1758653773, 1926631032)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1758653773n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.div_euint32_uint32(
      encryptedAmount.handles[0],
      1926631032n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 2 (569518498, 569518502)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(569518498n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.div_euint32_uint32(
      encryptedAmount.handles[0],
      569518502n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 3 (569518502, 569518502)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(569518502n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.div_euint32_uint32(
      encryptedAmount.handles[0],
      569518502n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint32, uint32) => euint32 test 4 (569518502, 569518498)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(569518502n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.div_euint32_uint32(
      encryptedAmount.handles[0],
      569518498n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 1 (514826887, 2242906184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(514826887n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rem_euint32_uint32(
      encryptedAmount.handles[0],
      2242906184n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(514826887n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 2 (514826883, 514826887)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(514826883n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rem_euint32_uint32(
      encryptedAmount.handles[0],
      514826887n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(514826883n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 3 (514826887, 514826887)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(514826887n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rem_euint32_uint32(
      encryptedAmount.handles[0],
      514826887n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint32, uint32) => euint32 test 4 (514826887, 514826883)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(514826887n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rem_euint32_uint32(
      encryptedAmount.handles[0],
      514826883n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (287391998, 3065919083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(287391998n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_uint32(
      encryptedAmount.handles[0],
      3065919083n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (287391994, 287391998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(287391994n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_uint32(
      encryptedAmount.handles[0],
      287391998n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (287391998, 287391998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(287391998n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_uint32(
      encryptedAmount.handles[0],
      287391998n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (287391998, 287391994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(287391998n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_uint32(
      encryptedAmount.handles[0],
      287391994n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 1 (833280044, 3065919083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(3065919083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint32_euint32(
      833280044n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 2 (287391994, 287391998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(287391998n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint32_euint32(
      287391994n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 3 (287391998, 287391998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(287391998n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint32_euint32(
      287391998n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint32, euint32) => ebool test 4 (287391998, 287391994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(287391994n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint32_euint32(
      287391998n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 1 (221510600, 454323706)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(221510600n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_uint32(
      encryptedAmount.handles[0],
      454323706n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 2 (221510596, 221510600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(221510596n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_uint32(
      encryptedAmount.handles[0],
      221510600n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 3 (221510600, 221510600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(221510600n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_uint32(
      encryptedAmount.handles[0],
      221510600n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, uint32) => ebool test 4 (221510600, 221510596)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(221510600n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_uint32(
      encryptedAmount.handles[0],
      221510596n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 1 (2107136281, 454323706)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(454323706n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint32_euint32(
      2107136281n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 2 (221510596, 221510600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(221510600n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint32_euint32(
      221510596n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 3 (221510600, 221510600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(221510600n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint32_euint32(
      221510600n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint32, euint32) => ebool test 4 (221510600, 221510596)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(221510596n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint32_euint32(
      221510600n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 1 (1736787381, 3290514741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1736787381n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_uint32(
      encryptedAmount.handles[0],
      3290514741n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 2 (970577158, 970577162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(970577158n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_uint32(
      encryptedAmount.handles[0],
      970577162n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 3 (970577162, 970577162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(970577162n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_uint32(
      encryptedAmount.handles[0],
      970577162n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, uint32) => ebool test 4 (970577162, 970577158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(970577162n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_uint32(
      encryptedAmount.handles[0],
      970577158n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 1 (586395582, 3290514741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(3290514741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_uint32_euint32(
      586395582n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 2 (970577158, 970577162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(970577162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_uint32_euint32(
      970577158n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 3 (970577162, 970577162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(970577162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_uint32_euint32(
      970577162n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint32, euint32) => ebool test 4 (970577162, 970577158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(970577158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_uint32_euint32(
      970577162n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 1 (212629196, 4059879084)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(212629196n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      4059879084n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 2 (212629192, 212629196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(212629192n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      212629196n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 3 (212629196, 212629196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(212629196n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      212629196n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, uint32) => ebool test 4 (212629196, 212629192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(212629196n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_uint32(
      encryptedAmount.handles[0],
      212629192n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (2732454384, 4059879084)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(4059879084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_uint32_euint32(
      2732454384n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (212629192, 212629196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(212629196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_uint32_euint32(
      212629192n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (212629196, 212629196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(212629196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_uint32_euint32(
      212629196n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (212629196, 212629192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(212629192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_uint32_euint32(
      212629196n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (2708913268, 3120465053)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2708913268n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_uint32(
      encryptedAmount.handles[0],
      3120465053n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (1446383130, 1446383134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1446383130n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_uint32(
      encryptedAmount.handles[0],
      1446383134n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (1446383134, 1446383134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1446383134n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_uint32(
      encryptedAmount.handles[0],
      1446383134n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (1446383134, 1446383130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1446383134n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_uint32(
      encryptedAmount.handles[0],
      1446383130n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (518556131, 3120465053)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(3120465053n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint32_euint32(
      518556131n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (1446383130, 1446383134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1446383134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint32_euint32(
      1446383130n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (1446383134, 1446383134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1446383134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint32_euint32(
      1446383134n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (1446383134, 1446383130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1446383130n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_uint32_euint32(
      1446383134n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (1837089382, 2866298985)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1837089382n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_uint32(
      encryptedAmount.handles[0],
      2866298985n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (1837089378, 1837089382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1837089378n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_uint32(
      encryptedAmount.handles[0],
      1837089382n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (1837089382, 1837089382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1837089382n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_uint32(
      encryptedAmount.handles[0],
      1837089382n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (1837089382, 1837089378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1837089382n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_uint32(
      encryptedAmount.handles[0],
      1837089378n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (391192733, 2866298985)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2866298985n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint32_euint32(
      391192733n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (1837089378, 1837089382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1837089382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint32_euint32(
      1837089378n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (1837089382, 1837089382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1837089382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint32_euint32(
      1837089382n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (1837089382, 1837089378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1837089378n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_uint32_euint32(
      1837089382n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (4226618007, 2039277146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4226618007n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_uint32(
      encryptedAmount.handles[0],
      2039277146n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2039277146n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (2940808909, 2940808913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2940808909n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_uint32(
      encryptedAmount.handles[0],
      2940808913n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2940808909n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (2940808913, 2940808913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2940808913n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_uint32(
      encryptedAmount.handles[0],
      2940808913n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2940808913n);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (2940808913, 2940808909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2940808913n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_uint32(
      encryptedAmount.handles[0],
      2940808909n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2940808909n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 1 (1778120575, 2039277146)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2039277146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint32_euint32(
      1778120575n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1778120575n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 2 (2940808909, 2940808913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2940808913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint32_euint32(
      2940808909n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2940808909n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 3 (2940808913, 2940808913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2940808913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint32_euint32(
      2940808913n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2940808913n);
  });

  it('test operator "min" overload (uint32, euint32) => euint32 test 4 (2940808913, 2940808909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2940808909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint32_euint32(
      2940808913n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2940808909n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 1 (3535438432, 1480992326)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3535438432n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_uint32(
      encryptedAmount.handles[0],
      1480992326n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(3535438432n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 2 (2851290841, 2851290845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2851290841n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_uint32(
      encryptedAmount.handles[0],
      2851290845n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2851290845n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 3 (2851290845, 2851290845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2851290845n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_uint32(
      encryptedAmount.handles[0],
      2851290845n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2851290845n);
  });

  it('test operator "max" overload (euint32, uint32) => euint32 test 4 (2851290845, 2851290841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2851290845n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_uint32(
      encryptedAmount.handles[0],
      2851290841n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2851290845n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 1 (760733114, 1480992326)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(1480992326n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint32_euint32(
      760733114n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1480992326n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 2 (2851290841, 2851290845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2851290845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint32_euint32(
      2851290841n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2851290845n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 3 (2851290845, 2851290845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2851290845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint32_euint32(
      2851290845n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2851290845n);
  });

  it('test operator "max" overload (uint32, euint32) => euint32 test 4 (2851290845, 2851290841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);

    input.add32(2851290841n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint32_euint32(
      2851290845n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2851290845n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 1 (9, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(11n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 3 (5, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(5n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(10n);
  });

  it('test operator "add" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 1 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(10n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint64, euint4) => euint64 test 2 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(10n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 1 (5, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(5n);
    input.add4(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(10n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 2 (3, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(3n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(15n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 3 (3, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(3n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(9n);
  });

  it('test operator "mul" overload (euint64, euint4) => euint64 test 4 (5, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(5n);
    input.add4(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(15n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 1 (18444456172028398167, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18444456172028398167n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "and" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 1 (18443742079082438219, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18443742079082438219n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18443742079082438223n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(8n);
  });

  it('test operator "or" overload (euint64, euint4) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 1 (18442015029052104347, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18442015029052104347n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18442015029052104337n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(6n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(10n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint64, euint4) => euint64 test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(10n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 1 (18446728320459652857, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446728320459652857n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 2 (6, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(6n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 3 (10, 10)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(10n);
    input.add4(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint64, euint4) => ebool test 4 (10, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(10n);
    input.add4(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 1 (18446041636272307563, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18446041636272307563n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 1 (18439251641807718563, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18439251641807718563n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 2 (5, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(5n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9n);
    input.add4(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint64, euint4) => ebool test 4 (9, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(9n);
    input.add4(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 1 (18441781451110763323, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18441781451110763323n);
    input.add4(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 1 (18438089675582739325, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(18438089675582739325n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(4n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint64, euint4) => ebool test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add64(8n);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint64_euint4(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });
});
