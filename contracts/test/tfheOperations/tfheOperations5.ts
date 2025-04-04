import { expect } from 'chai';
import { ethers } from 'hardhat';

import type { TFHETestSuite1 } from '../../types/contracts/tests/TFHETestSuite1';
import type { TFHETestSuite2 } from '../../types/contracts/tests/TFHETestSuite2';
import type { TFHETestSuite3 } from '../../types/contracts/tests/TFHETestSuite3';
import type { TFHETestSuite4 } from '../../types/contracts/tests/TFHETestSuite4';
import type { TFHETestSuite5 } from '../../types/contracts/tests/TFHETestSuite5';
import type { TFHETestSuite6 } from '../../types/contracts/tests/TFHETestSuite6';
import type { TFHETestSuite7 } from '../../types/contracts/tests/TFHETestSuite7';
import type { TFHETestSuite8 } from '../../types/contracts/tests/TFHETestSuite8';
import type { TFHETestSuite9 } from '../../types/contracts/tests/TFHETestSuite9';
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

async function deployTfheTestFixture7(): Promise<TFHETestSuite7> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture8(): Promise<TFHETestSuite8> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite8');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture9(): Promise<TFHETestSuite9> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite9');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('TFHE operations 5', function () {
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

    const contract7 = await deployTfheTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const contract8 = await deployTfheTestFixture8();
    this.contract8Address = await contract8.getAddress();
    this.contract8 = contract8;

    const contract9 = await deployTfheTestFixture9();
    this.contract9Address = await contract9.getAddress();
    this.contract9 = contract9;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (2536478837, 43238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2536478837n);
    input.add16(43238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (43234, 43238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(43234n);
    input.add16(43238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (43238, 43238)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(43238n);
    input.add16(43238n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (43238, 43234)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(43238n);
    input.add16(43234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (1659189406, 29236)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1659189406n);
    input.add16(29236n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (29232, 29236)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(29232n);
    input.add16(29236n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (29236, 29236)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(29236n);
    input.add16(29236n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (29236, 29232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(29236n);
    input.add16(29232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (2417778902, 47468)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2417778902n);
    input.add16(47468n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (47464, 47468)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(47464n);
    input.add16(47468n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (47468, 47468)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(47468n);
    input.add16(47468n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (47468, 47464)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(47468n);
    input.add16(47464n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (3090588715, 50333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3090588715n);
    input.add16(50333n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (50329, 50333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(50329n);
    input.add16(50333n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (50333, 50333)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(50333n);
    input.add16(50333n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (50333, 50329)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(50333n);
    input.add16(50329n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (554401729, 54005)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(554401729n);
    input.add16(54005n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (54001, 54005)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(54001n);
    input.add16(54005n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (54005, 54005)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(54005n);
    input.add16(54005n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (54005, 54001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(54005n);
    input.add16(54001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (2180318134, 18492)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2180318134n);
    input.add16(18492n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (18488, 18492)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(18488n);
    input.add16(18492n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (18492, 18492)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(18492n);
    input.add16(18492n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (18492, 18488)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(18492n);
    input.add16(18488n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (3992416953, 58639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3992416953n);
    input.add16(58639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(58639n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (58635, 58639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(58635n);
    input.add16(58639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(58635n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (58639, 58639)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(58639n);
    input.add16(58639n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(58639n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (58639, 58635)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(58639n);
    input.add16(58635n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(58635n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (3167270574, 189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3167270574n);
    input.add16(189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(3167270574n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (185, 189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(185n);
    input.add16(189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(189n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (189, 189)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(189n);
    input.add16(189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(189n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (189, 185)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(189n);
    input.add16(185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(189n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (736743635, 1597655761)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(736743635n);
    input.add32(1597655761n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2334399396n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (1473487264, 1473487268)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1473487264n);
    input.add32(1473487268n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2946974532n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (1473487268, 1473487268)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1473487268n);
    input.add32(1473487268n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2946974536n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (1473487268, 1473487264)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1473487268n);
    input.add32(1473487264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2946974532n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (2630999733, 2630999733)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2630999733n);
    input.add32(2630999733n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (2630999733, 2630999729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2630999733n);
    input.add32(2630999729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (115789, 29423)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(115789n);
    input.add32(29423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(3406859747n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (58844, 58844)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(58844n);
    input.add32(58844n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(3462616336n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (58844, 58844)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(58844n);
    input.add32(58844n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(3462616336n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (58844, 58844)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(58844n);
    input.add32(58844n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(3462616336n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (1634775410, 117791202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1634775410n);
    input.add32(117791202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(16781666n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (117791198, 117791202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(117791198n);
    input.add32(117791202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(117791170n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (117791202, 117791202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(117791202n);
    input.add32(117791202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(117791202n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (117791202, 117791198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(117791202n);
    input.add32(117791198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(117791170n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (566808747, 3773596170)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(566808747n);
    input.add32(3773596170n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(3790394027n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (566808743, 566808747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(566808743n);
    input.add32(566808747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(566808751n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (566808747, 566808747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(566808747n);
    input.add32(566808747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(566808747n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (566808747, 566808743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(566808747n);
    input.add32(566808743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(566808751n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (3546807643, 1914181669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3546807643n);
    input.add32(1914181669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2709513598n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (1914181665, 1914181669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1914181665n);
    input.add32(1914181669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (1914181669, 1914181669)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1914181669n);
    input.add32(1914181669n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (1914181669, 1914181665)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1914181669n);
    input.add32(1914181665n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (3405236202, 3866172680)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3405236202n);
    input.add32(3866172680n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (3405236198, 3405236202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3405236198n);
    input.add32(3405236202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (3405236202, 3405236202)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3405236202n);
    input.add32(3405236202n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (3405236202, 3405236198)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3405236202n);
    input.add32(3405236198n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (2863544693, 3985121898)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2863544693n);
    input.add32(3985121898n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (2863544689, 2863544693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2863544689n);
    input.add32(2863544693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (2863544693, 2863544693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2863544693n);
    input.add32(2863544693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (2863544693, 2863544689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2863544693n);
    input.add32(2863544689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (551100436, 4195481799)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(551100436n);
    input.add32(4195481799n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (551100432, 551100436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(551100432n);
    input.add32(551100436n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (551100436, 551100436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(551100436n);
    input.add32(551100436n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (551100436, 551100432)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(551100436n);
    input.add32(551100432n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (3680469840, 2255802163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(3680469840n);
    input.add32(2255802163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (2255802159, 2255802163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2255802159n);
    input.add32(2255802163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (2255802163, 2255802163)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2255802163n);
    input.add32(2255802163n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (2255802163, 2255802159)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(2255802163n);
    input.add32(2255802159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (1828796222, 1877605280)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1828796222n);
    input.add32(1877605280n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (1828796218, 1828796222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1828796218n);
    input.add32(1828796222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (1828796222, 1828796222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1828796222n);
    input.add32(1828796222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (1828796222, 1828796218)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1828796222n);
    input.add32(1828796218n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (1506398496, 222862394)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(1506398496n);
    input.add32(222862394n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (222862390, 222862394)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(222862390n);
    input.add32(222862394n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (222862394, 222862394)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(222862394n);
    input.add32(222862394n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (222862394, 222862390)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(222862394n);
    input.add32(222862390n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract3.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (719746117, 1908350845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(719746117n);
    input.add32(1908350845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(719746117n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (719746113, 719746117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(719746113n);
    input.add32(719746117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(719746113n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (719746117, 719746117)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(719746117n);
    input.add32(719746117n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(719746117n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (719746117, 719746113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(719746117n);
    input.add32(719746113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(719746113n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (335410015, 2954890567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(335410015n);
    input.add32(2954890567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(2954890567n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (335410011, 335410015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(335410011n);
    input.add32(335410015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(335410015n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (335410015, 335410015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(335410015n);
    input.add32(335410015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(335410015n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (335410015, 335410011)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract3Address, this.signers.alice.address);
    input.add32(335410015n);
    input.add32(335410011n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract3.res32());
    expect(res).to.equal(335410015n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (2, 4294068376)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(4294068376n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4294068378n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (1589114167, 1589114169)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1589114167n);
    input.add64(1589114169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3178228336n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (1589114169, 1589114169)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1589114169n);
    input.add64(1589114169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3178228338n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (1589114169, 1589114167)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1589114169n);
    input.add64(1589114167n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3178228336n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (2317838102, 2317838102)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2317838102n);
    input.add64(2317838102n);
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

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (2317838102, 2317838098)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2317838102n);
    input.add64(2317838098n);
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

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2146970397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(2146970397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4293940794n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (50512, 50512)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(50512n);
    input.add64(50512n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(2551462144n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (50512, 50512)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(50512n);
    input.add64(50512n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(2551462144n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (50512, 50512)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(50512n);
    input.add64(50512n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(2551462144n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (3271837023, 18444745911191297127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3271837023n);
    input.add64(18444745911191297127n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(2147763271n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (3271837019, 3271837023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3271837019n);
    input.add64(3271837023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3271837019n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (3271837023, 3271837023)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3271837023n);
    input.add64(3271837023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3271837023n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (3271837023, 3271837019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3271837023n);
    input.add64(3271837019n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(3271837019n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (2768628384, 18440693184934323215)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2768628384n);
    input.add64(18440693184934323215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18440693185001811631n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (2768628380, 2768628384)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2768628380n);
    input.add64(2768628384n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(2768628412n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (2768628384, 2768628384)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2768628384n);
    input.add64(2768628384n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(2768628384n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (2768628384, 2768628380)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2768628384n);
    input.add64(2768628380n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(2768628412n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (911962221, 18437830365897295553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(911962221n);
    input.add64(18437830365897295553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18437830365598152364n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (911962217, 911962221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(911962217n);
    input.add64(911962221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (911962221, 911962221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(911962221n);
    input.add64(911962221n);
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

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (911962221, 911962217)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(911962221n);
    input.add64(911962217n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (2316315032, 18441358452311135271)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2316315032n);
    input.add64(18441358452311135271n);
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

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (2316315028, 2316315032)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2316315028n);
    input.add64(2316315032n);
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

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (2316315032, 2316315032)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2316315032n);
    input.add64(2316315032n);
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

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (2316315032, 2316315028)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2316315032n);
    input.add64(2316315028n);
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

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (3617808473, 18446238628676626265)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3617808473n);
    input.add64(18446238628676626265n);
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

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (3617808469, 3617808473)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3617808469n);
    input.add64(3617808473n);
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

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (3617808473, 3617808473)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3617808473n);
    input.add64(3617808473n);
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

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (3617808473, 3617808469)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3617808473n);
    input.add64(3617808469n);
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

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (1065691998, 18438169614023487959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1065691998n);
    input.add64(18438169614023487959n);
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

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (1065691994, 1065691998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1065691994n);
    input.add64(1065691998n);
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

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (1065691998, 1065691998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1065691998n);
    input.add64(1065691998n);
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

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (1065691998, 1065691994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1065691998n);
    input.add64(1065691994n);
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

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (1414631604, 18444508028856313471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1414631604n);
    input.add64(18444508028856313471n);
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

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (1414631600, 1414631604)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1414631600n);
    input.add64(1414631604n);
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

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (1414631604, 1414631604)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1414631604n);
    input.add64(1414631604n);
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

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (1414631604, 1414631600)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1414631604n);
    input.add64(1414631600n);
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

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (2987721361, 18446084963107924653)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2987721361n);
    input.add64(18446084963107924653n);
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

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (2987721357, 2987721361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2987721357n);
    input.add64(2987721361n);
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

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (2987721361, 2987721361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2987721361n);
    input.add64(2987721361n);
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

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (2987721361, 2987721357)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2987721361n);
    input.add64(2987721357n);
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

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (3189677127, 18439097348591638651)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3189677127n);
    input.add64(18439097348591638651n);
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

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (3189677123, 3189677127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3189677123n);
    input.add64(3189677127n);
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

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (3189677127, 3189677127)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3189677127n);
    input.add64(3189677127n);
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

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (3189677127, 3189677123)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3189677127n);
    input.add64(3189677123n);
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

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (1263551143, 18444167452872635657)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1263551143n);
    input.add64(18444167452872635657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(1263551143n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (1263551139, 1263551143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1263551139n);
    input.add64(1263551143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(1263551139n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (1263551143, 1263551143)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1263551143n);
    input.add64(1263551143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(1263551143n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (1263551143, 1263551139)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1263551143n);
    input.add64(1263551139n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(1263551139n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (1477599291, 18445021157189492237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1477599291n);
    input.add64(18445021157189492237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(18445021157189492237n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (1477599287, 1477599291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1477599287n);
    input.add64(1477599291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(1477599291n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (1477599291, 1477599291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1477599291n);
    input.add64(1477599291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(1477599291n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (1477599291, 1477599287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1477599291n);
    input.add64(1477599287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract4.res64());
    expect(res).to.equal(1477599291n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 1 (2, 2147483649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2n);
    input.add128(2147483649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 2 (2001668781, 2001668785)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2001668781n);
    input.add128(2001668785n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(4003337566n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 3 (2001668785, 2001668785)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2001668785n);
    input.add128(2001668785n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(4003337570n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 4 (2001668785, 2001668781)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2001668785n);
    input.add128(2001668781n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(4003337566n);
  });

  it('test operator "sub" overload (euint32, euint128) => euint128 test 1 (3737861002, 3737861002)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3737861002n);
    input.add128(3737861002n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint128) => euint128 test 2 (3737861002, 3737860998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3737861002n);
    input.add128(3737860998n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 1 (2, 1073741825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2n);
    input.add128(1073741825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 2 (53889, 53889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(53889n);
    input.add128(53889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(2904024321n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 3 (53889, 53889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(53889n);
    input.add128(53889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(2904024321n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 4 (53889, 53889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(53889n);
    input.add128(53889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(2904024321n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 1 (209673745, 340282366920938463463368018236354117943)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(209673745n);
    input.add128(340282366920938463463368018236354117943n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(8142865n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 2 (209673741, 209673745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(209673741n);
    input.add128(209673745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(209673729n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 3 (209673745, 209673745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(209673745n);
    input.add128(209673745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(209673745n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 4 (209673745, 209673741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(209673745n);
    input.add128(209673741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(209673729n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 1 (1292743544, 340282366920938463463370643672579696235)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1292743544n);
    input.add128(340282366920938463463370643672579696235n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(340282366920938463463370643672714442619n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 2 (1292743540, 1292743544)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1292743540n);
    input.add128(1292743544n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(1292743548n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 3 (1292743544, 1292743544)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1292743544n);
    input.add128(1292743544n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(1292743544n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 4 (1292743544, 1292743540)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1292743544n);
    input.add128(1292743540n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(1292743548n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 1 (670517798, 340282366920938463463369487828919035477)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(670517798n);
    input.add128(340282366920938463463369487828919035477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(340282366920938463463369487829497277555n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 2 (670517794, 670517798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(670517794n);
    input.add128(670517798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 3 (670517798, 670517798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(670517798n);
    input.add128(670517798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 4 (670517798, 670517794)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(670517798n);
    input.add128(670517794n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 1 (2243828946, 340282366920938463463369876370248579089)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2243828946n);
    input.add128(340282366920938463463369876370248579089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 2 (2243828942, 2243828946)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2243828942n);
    input.add128(2243828946n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 3 (2243828946, 2243828946)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2243828946n);
    input.add128(2243828946n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 4 (2243828946, 2243828942)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2243828946n);
    input.add128(2243828942n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 1 (767347736, 340282366920938463463372710637937521107)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(767347736n);
    input.add128(340282366920938463463372710637937521107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 2 (767347732, 767347736)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(767347732n);
    input.add128(767347736n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 3 (767347736, 767347736)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(767347736n);
    input.add128(767347736n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 4 (767347736, 767347732)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(767347736n);
    input.add128(767347732n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 1 (1553393887, 340282366920938463463367696515232664453)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1553393887n);
    input.add128(340282366920938463463367696515232664453n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 2 (1553393883, 1553393887)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1553393883n);
    input.add128(1553393887n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 3 (1553393887, 1553393887)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1553393887n);
    input.add128(1553393887n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 4 (1553393887, 1553393883)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1553393887n);
    input.add128(1553393883n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 1 (771825525, 340282366920938463463373279522588491391)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(771825525n);
    input.add128(340282366920938463463373279522588491391n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 2 (771825521, 771825525)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(771825521n);
    input.add128(771825525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 3 (771825525, 771825525)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(771825525n);
    input.add128(771825525n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 4 (771825525, 771825521)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(771825525n);
    input.add128(771825521n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 1 (663322866, 340282366920938463463367387565940297507)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(663322866n);
    input.add128(340282366920938463463367387565940297507n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 2 (663322862, 663322866)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(663322862n);
    input.add128(663322866n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 3 (663322866, 663322866)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(663322866n);
    input.add128(663322866n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 4 (663322866, 663322862)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(663322866n);
    input.add128(663322862n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 1 (944488770, 340282366920938463463374396448902249171)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(944488770n);
    input.add128(340282366920938463463374396448902249171n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 2 (944488766, 944488770)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(944488766n);
    input.add128(944488770n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 3 (944488770, 944488770)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(944488770n);
    input.add128(944488770n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 4 (944488770, 944488766)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(944488770n);
    input.add128(944488766n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 1 (352526802, 340282366920938463463366913454762182575)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(352526802n);
    input.add128(340282366920938463463366913454762182575n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(352526802n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 2 (352526798, 352526802)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(352526798n);
    input.add128(352526802n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(352526798n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 3 (352526802, 352526802)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(352526802n);
    input.add128(352526802n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(352526802n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 4 (352526802, 352526798)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(352526802n);
    input.add128(352526798n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(352526798n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 1 (811204431, 340282366920938463463369047699274502021)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(811204431n);
    input.add128(340282366920938463463369047699274502021n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(340282366920938463463369047699274502021n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 2 (811204427, 811204431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(811204427n);
    input.add128(811204431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(811204431n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 3 (811204431, 811204431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(811204431n);
    input.add128(811204431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(811204431n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 4 (811204431, 811204427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(811204431n);
    input.add128(811204427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract4.res128());
    expect(res).to.equal(811204431n);
  });

  it('test operator "add" overload (euint32, euint256) => euint256 test 1 (2, 2147483649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2n);
    input.add256(2147483649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint32, euint256) => euint256 test 2 (1356877770, 1356877772)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1356877770n);
    input.add256(1356877772n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2713755542n);
  });

  it('test operator "add" overload (euint32, euint256) => euint256 test 3 (1356877772, 1356877772)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1356877772n);
    input.add256(1356877772n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2713755544n);
  });

  it('test operator "add" overload (euint32, euint256) => euint256 test 4 (1356877772, 1356877770)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(1356877772n);
    input.add256(1356877770n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2713755542n);
  });

  it('test operator "sub" overload (euint32, euint256) => euint256 test 1 (3004337012, 3004337012)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3004337012n);
    input.add256(3004337012n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint256) => euint256 test 2 (3004337012, 3004337008)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3004337012n);
    input.add256(3004337008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint256) => euint256 test 1 (2, 1073741825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2n);
    input.add256(1073741825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint32, euint256) => euint256 test 2 (49958, 49958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(49958n);
    input.add256(49958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2495801764n);
  });

  it('test operator "mul" overload (euint32, euint256) => euint256 test 3 (49958, 49958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(49958n);
    input.add256(49958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2495801764n);
  });

  it('test operator "mul" overload (euint32, euint256) => euint256 test 4 (49958, 49958)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(49958n);
    input.add256(49958n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(2495801764n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 1 (3911655747, 115792089237316195423570985008687907853269984665640564039457577643618672371033)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3911655747n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577643618672371033n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(689971521n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 2 (3911655743, 3911655747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3911655743n);
    input.add256(3911655747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(3911655683n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 3 (3911655747, 3911655747)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3911655747n);
    input.add256(3911655747n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(3911655747n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 4 (3911655747, 3911655743)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3911655747n);
    input.add256(3911655743n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(3911655683n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 1 (3028250348, 115792089237316195423570985008687907853269984665640564039457576545515971268763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3028250348n);
    input.add256(115792089237316195423570985008687907853269984665640564039457576545515971268763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576545518929051391n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 2 (3028250344, 3028250348)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3028250344n);
    input.add256(3028250348n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(3028250348n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 3 (3028250348, 3028250348)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3028250348n);
    input.add256(3028250348n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(3028250348n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 4 (3028250348, 3028250344)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3028250348n);
    input.add256(3028250344n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(3028250348n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 1 (2519727551, 115792089237316195423570985008687907853269984665640564039457581373716596323299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2519727551n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581373716596323299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581373718510374492n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 2 (2519727547, 2519727551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2519727547n);
    input.add256(2519727551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 3 (2519727551, 2519727551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2519727551n);
    input.add256(2519727551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 4 (2519727551, 2519727547)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2519727551n);
    input.add256(2519727547n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract4.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 1 (4241138689, 115792089237316195423570985008687907853269984665640564039457577775117573167619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4241138689n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577775117573167619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 2 (4241138685, 4241138689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4241138685n);
    input.add256(4241138689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 3 (4241138689, 4241138689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4241138689n);
    input.add256(4241138689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 4 (4241138689, 4241138685)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(4241138689n);
    input.add256(4241138685n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 1 (3951737368, 115792089237316195423570985008687907853269984665640564039457577820652439476745)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3951737368n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577820652439476745n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 2 (3951737364, 3951737368)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3951737364n);
    input.add256(3951737368n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 3 (3951737368, 3951737368)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3951737368n);
    input.add256(3951737368n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 4 (3951737368, 3951737364)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3951737368n);
    input.add256(3951737364n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint256) => ebool test 1 (3503726959, 115792089237316195423570985008687907853269984665640564039457583359209887804553)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3503726959n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583359209887804553n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint256) => ebool test 2 (3503726955, 3503726959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3503726955n);
    input.add256(3503726959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint256) => ebool test 3 (3503726959, 3503726959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3503726959n);
    input.add256(3503726959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint256) => ebool test 4 (3503726959, 3503726955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(3503726959n);
    input.add256(3503726955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint256) => ebool test 1 (284017610, 115792089237316195423570985008687907853269984665640564039457581326953176676625)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(284017610n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581326953176676625n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint256) => ebool test 2 (284017606, 284017610)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(284017606n);
    input.add256(284017610n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint256) => ebool test 3 (284017610, 284017610)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(284017610n);
    input.add256(284017610n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint256) => ebool test 4 (284017610, 284017606)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(284017610n);
    input.add256(284017606n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint256) => ebool test 1 (2586025321, 115792089237316195423570985008687907853269984665640564039457577346035601426303)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2586025321n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577346035601426303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint256) => ebool test 2 (2586025317, 2586025321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2586025317n);
    input.add256(2586025321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint256) => ebool test 3 (2586025321, 2586025321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2586025321n);
    input.add256(2586025321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint256) => ebool test 4 (2586025321, 2586025317)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2586025321n);
    input.add256(2586025317n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint256) => ebool test 1 (2238236814, 115792089237316195423570985008687907853269984665640564039457575826150117568785)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2238236814n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575826150117568785n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint256) => ebool test 2 (2238236810, 2238236814)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2238236810n);
    input.add256(2238236814n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint256) => ebool test 3 (2238236814, 2238236814)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2238236814n);
    input.add256(2238236814n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint256) => ebool test 4 (2238236814, 2238236810)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract4Address, this.signers.alice.address);
    input.add32(2238236814n);
    input.add256(2238236810n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract4.resb());
    expect(res).to.equal(false);
  });
});
