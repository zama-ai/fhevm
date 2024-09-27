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
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture2(): Promise<TFHETestSuite2> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture3(): Promise<TFHETestSuite3> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture4(): Promise<TFHETestSuite4> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture5(): Promise<TFHETestSuite5> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture6(): Promise<TFHETestSuite6> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations 8', function () {
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

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (2442028658, 23)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2442028658n);
    input.add8(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2442028645n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (19, 23)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(19n);
    input.add8(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (23, 23)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(23n);
    input.add8(23n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (23, 19)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(23n);
    input.add8(19n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (1831274090, 104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1831274090n);
    input.add8(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (100, 104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(100n);
    input.add8(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (104, 104)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(104n);
    input.add8(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (104, 100)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(104n);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (3821365412, 152)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3821365412n);
    input.add8(152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (148, 152)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(148n);
    input.add8(152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (152, 152)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(152n);
    input.add8(152n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (152, 148)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(152n);
    input.add8(148n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (1799963609, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1799963609n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (229, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(229n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (233, 233)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(233n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (233, 229)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(233n);
    input.add8(229n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (3893344872, 37)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3893344872n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (33, 37)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(33n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (37, 37)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(37n);
    input.add8(37n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (37, 33)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(37n);
    input.add8(33n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (804391035, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(804391035n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (129, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(129n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (133, 133)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(133n);
    input.add8(133n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (133, 129)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(133n);
    input.add8(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (3782116241, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3782116241n);
    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (154, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(154n);
    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (158, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(158n);
    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (158, 154)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(158n);
    input.add8(154n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (2689679661, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2689679661n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(114n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (110, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(110n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(110n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (114, 114)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(114n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(114n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (114, 110)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(114n);
    input.add8(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(110n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (2001471362, 68)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2001471362n);
    input.add8(68n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2001471362n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (64, 68)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(64n);
    input.add8(68n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(68n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (68, 68)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(68n);
    input.add8(68n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(68n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (68, 64)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(68n);
    input.add8(64n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(68n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (62662, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(62662n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(62664n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (19188, 19190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(19188n);
    input.add16(19190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(38378n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (19190, 19190)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(19190n);
    input.add16(19190n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(38380n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (19190, 19188)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(19190n);
    input.add16(19188n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(38378n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (35686, 35686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(35686n);
    input.add16(35686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (35686, 35682)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(35686n);
    input.add16(35682n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (22131, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(22131n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(44262n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (158, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(158n);
    input.add16(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(24964n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (158, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(158n);
    input.add16(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(24964n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (158, 158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(158n);
    input.add16(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(24964n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (46593294, 4785)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(46593294n);
    input.add16(4785n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4096n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (4781, 4785)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4781n);
    input.add16(4785n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4769n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (4785, 4785)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4785n);
    input.add16(4785n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4785n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (4785, 4781)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4785n);
    input.add16(4781n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4769n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (1914437939, 49825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1914437939n);
    input.add16(49825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1914487731n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (49821, 49825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(49821n);
    input.add16(49825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(49853n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (49825, 49825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(49825n);
    input.add16(49825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(49825n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (49825, 49821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(49825n);
    input.add16(49821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(49853n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (4035221230, 58371)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4035221230n);
    input.add16(58371n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4035212013n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (58367, 58371)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(58367n);
    input.add16(58371n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2044n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (58371, 58371)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(58371n);
    input.add16(58371n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (58371, 58367)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(58371n);
    input.add16(58367n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2044n);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (3944731587, 33929)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3944731587n);
    input.add16(33929n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (33925, 33929)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(33925n);
    input.add16(33929n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (33929, 33929)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(33929n);
    input.add16(33929n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (33929, 33925)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(33929n);
    input.add16(33925n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (3302913406, 10832)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3302913406n);
    input.add16(10832n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (10828, 10832)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(10828n);
    input.add16(10832n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (10832, 10832)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(10832n);
    input.add16(10832n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (10832, 10828)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(10832n);
    input.add16(10828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (1742405068, 56059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1742405068n);
    input.add16(56059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (56055, 56059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(56055n);
    input.add16(56059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (56059, 56059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(56059n);
    input.add16(56059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (56059, 56055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(56059n);
    input.add16(56055n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (4017313863, 63284)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4017313863n);
    input.add16(63284n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (63280, 63284)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(63280n);
    input.add16(63284n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (63284, 63284)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(63284n);
    input.add16(63284n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (63284, 63280)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(63284n);
    input.add16(63280n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (1660443539, 63916)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1660443539n);
    input.add16(63916n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (63912, 63916)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(63912n);
    input.add16(63916n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (63916, 63916)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(63916n);
    input.add16(63916n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (63916, 63912)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(63916n);
    input.add16(63912n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (1150183024, 6871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1150183024n);
    input.add16(6871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (6867, 6871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(6867n);
    input.add16(6871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (6871, 6871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(6871n);
    input.add16(6871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (6871, 6867)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(6871n);
    input.add16(6867n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (2355937467, 15361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2355937467n);
    input.add16(15361n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(15361n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (15357, 15361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(15357n);
    input.add16(15361n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(15357n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (15361, 15361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(15361n);
    input.add16(15361n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(15361n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (15361, 15357)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(15361n);
    input.add16(15357n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(15357n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (4156714889, 5440)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4156714889n);
    input.add16(5440n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4156714889n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (5436, 5440)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(5436n);
    input.add16(5440n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(5440n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (5440, 5440)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(5440n);
    input.add16(5440n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(5440n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (5440, 5436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(5440n);
    input.add16(5436n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(5440n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (451501910, 1802073207)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(451501910n);
    input.add32(1802073207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2253575117n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (903003814, 903003818)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(903003814n);
    input.add32(903003818n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1806007632n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (903003818, 903003818)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(903003818n);
    input.add32(903003818n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1806007636n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (903003818, 903003814)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(903003818n);
    input.add32(903003814n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1806007632n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (296156690, 296156690)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(296156690n);
    input.add32(296156690n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (296156690, 296156686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(296156690n);
    input.add32(296156686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (54583, 21224)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(54583n);
    input.add32(21224n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1158469592n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (42446, 42446)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(42446n);
    input.add32(42446n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1801662916n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (42446, 42446)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(42446n);
    input.add32(42446n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1801662916n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (42446, 42446)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(42446n);
    input.add32(42446n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(1801662916n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (2522719500, 2344855070)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2522719500n);
    input.add32(2344855070n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2185339916n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (2344855066, 2344855070)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2344855066n);
    input.add32(2344855070n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2344855066n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (2344855070, 2344855070)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2344855070n);
    input.add32(2344855070n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2344855070n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (2344855070, 2344855066)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2344855070n);
    input.add32(2344855066n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2344855066n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (1757798833, 311935858)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1757798833n);
    input.add32(311935858n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2060968947n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (311935854, 311935858)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(311935854n);
    input.add32(311935858n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(311935870n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (311935858, 311935858)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(311935858n);
    input.add32(311935858n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(311935858n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (311935858, 311935854)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(311935858n);
    input.add32(311935854n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(311935870n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (3321450403, 449148535)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3321450403n);
    input.add32(449148535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(3745266132n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (449148531, 449148535)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(449148531n);
    input.add32(449148535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (449148535, 449148535)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(449148535n);
    input.add32(449148535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (449148535, 449148531)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(449148535n);
    input.add32(449148531n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (287391998, 2464993294)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(287391998n);
    input.add32(2464993294n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (287391994, 287391998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(287391994n);
    input.add32(287391998n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (287391998, 287391998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(287391998n);
    input.add32(287391998n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (287391998, 287391994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(287391998n);
    input.add32(287391994n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (221510600, 3939036059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(221510600n);
    input.add32(3939036059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (221510596, 221510600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(221510596n);
    input.add32(221510600n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (221510600, 221510600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(221510600n);
    input.add32(221510600n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (221510600, 221510596)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(221510600n);
    input.add32(221510596n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (1736787381, 970577162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1736787381n);
    input.add32(970577162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (970577158, 970577162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(970577158n);
    input.add32(970577162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (970577162, 970577162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(970577162n);
    input.add32(970577162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (970577162, 970577158)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(970577162n);
    input.add32(970577158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (212629196, 2875731526)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(212629196n);
    input.add32(2875731526n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (212629192, 212629196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(212629192n);
    input.add32(212629196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (212629196, 212629196)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(212629196n);
    input.add32(212629196n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (212629196, 212629192)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(212629196n);
    input.add32(212629192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (2708913268, 1446383134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2708913268n);
    input.add32(1446383134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (1446383130, 1446383134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1446383130n);
    input.add32(1446383134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (1446383134, 1446383134)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1446383134n);
    input.add32(1446383134n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (1446383134, 1446383130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1446383134n);
    input.add32(1446383130n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (1837089382, 4089682184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1837089382n);
    input.add32(4089682184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (1837089378, 1837089382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1837089378n);
    input.add32(1837089382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (1837089382, 1837089382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1837089382n);
    input.add32(1837089382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (1837089382, 1837089378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1837089382n);
    input.add32(1837089378n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (4226618007, 2940808913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4226618007n);
    input.add32(2940808913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2940808913n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (2940808909, 2940808913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2940808909n);
    input.add32(2940808913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2940808909n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (2940808913, 2940808913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2940808913n);
    input.add32(2940808913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2940808913n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (2940808913, 2940808909)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2940808913n);
    input.add32(2940808909n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2940808909n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (3535438432, 2851290845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3535438432n);
    input.add32(2851290845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(3535438432n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (2851290841, 2851290845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2851290841n);
    input.add32(2851290845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2851290845n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (2851290845, 2851290845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2851290845n);
    input.add32(2851290845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2851290845n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (2851290845, 2851290841)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2851290845n);
    input.add32(2851290841n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract4.res32());
    expect(res).to.equal(2851290845n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (2, 4294200415)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(4294200415n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4294200417n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (184177949, 184177953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(184177949n);
    input.add64(184177953n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(368355902n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (184177953, 184177953)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(184177953n);
    input.add64(184177953n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(368355906n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (184177953, 184177949)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(184177953n);
    input.add64(184177949n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(368355902n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (2163998221, 2163998221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2163998221n);
    input.add64(2163998221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (2163998221, 2163998217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2163998221n);
    input.add64(2163998217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2146443814)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(2146443814n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4292887628n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (61456, 61456)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(61456n);
    input.add64(61456n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3776839936n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (61456, 61456)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(61456n);
    input.add64(61456n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3776839936n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (61456, 61456)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(61456n);
    input.add64(61456n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3776839936n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (595906685, 18438632387690440007)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(595906685n);
    input.add64(18438632387690440007n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(537133125n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (595906681, 595906685)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(595906681n);
    input.add64(595906685n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(595906681n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (595906685, 595906685)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(595906685n);
    input.add64(595906685n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(595906685n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (595906685, 595906681)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(595906685n);
    input.add64(595906681n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(595906681n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (1312726121, 18440203558036734807)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1312726121n);
    input.add64(18440203558036734807n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18440203559245881215n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (1312726117, 1312726121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1312726117n);
    input.add64(1312726121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(1312726125n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (1312726121, 1312726121)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1312726121n);
    input.add64(1312726121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(1312726121n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (1312726121, 1312726117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1312726121n);
    input.add64(1312726117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(1312726125n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (3933920874, 18441384818446198365)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3933920874n);
    input.add64(18441384818446198365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18441384821297952823n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (3933920870, 3933920874)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3933920870n);
    input.add64(3933920874n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (3933920874, 3933920874)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3933920874n);
    input.add64(3933920874n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (3933920874, 3933920870)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3933920874n);
    input.add64(3933920870n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(12n);
  });
});
