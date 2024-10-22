import { expect } from 'chai';
import { ethers } from 'hardhat';

import type {
  TFHETestSuite1,
  TFHETestSuite10,
  TFHETestSuite11,
  TFHETestSuite2,
  TFHETestSuite3,
  TFHETestSuite4,
  TFHETestSuite5,
  TFHETestSuite6,
  TFHETestSuite7,
  TFHETestSuite8,
  TFHETestSuite9,
} from '../../types';
import { createInstances, decrypt128, decrypt256, decrypt32, decrypt64, decryptBool } from '../instance';
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

async function deployTfheTestFixture10(): Promise<TFHETestSuite10> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite10');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployTfheTestFixture11(): Promise<TFHETestSuite11> {
  const signers = await getSigners();
  const admin = signers.alice;

  const contractFactory = await ethers.getContractFactory('TFHETestSuite11');
  const contract = await contractFactory.connect(admin).deploy();
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

    const contract7 = await deployTfheTestFixture7();
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const contract8 = await deployTfheTestFixture8();
    this.contract8Address = await contract8.getAddress();
    this.contract8 = contract8;

    const contract9 = await deployTfheTestFixture9();
    this.contract9Address = await contract9.getAddress();
    this.contract9 = contract9;

    const contract10 = await deployTfheTestFixture10();
    this.contract10Address = await contract10.getAddress();
    this.contract10 = contract10;

    const contract11 = await deployTfheTestFixture11();
    this.contract11Address = await contract11.getAddress();
    this.contract11 = contract11;

    const instances = await createInstances(this.signers);
    this.instances = instances;
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 1 (114, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(114n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(228n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 2 (15, 16)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(15n);
    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(240n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 3 (9, 9)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(81n);
  });

  it('test operator "mul" overload (euint32, euint8) => euint32 test 4 (16, 15)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(16n);
    input.add8(15n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(240n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 1 (2846739700, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2846739700n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(32n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 2 (30, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(30n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 3 (34, 34)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(34n);
    input.add8(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(34n);
  });

  it('test operator "and" overload (euint32, euint8) => euint32 test 4 (34, 30)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(34n);
    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (143668880, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(143668880n);
    input.add8(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(143668920n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (180, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(180n);
    input.add8(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(188n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (184, 184)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(184n);
    input.add8(184n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(184n);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (184, 180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(184n);
    input.add8(180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(188n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 1 (1756250914, 211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1756250914n);
    input.add8(211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1756251121n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 2 (207, 211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(207n);
    input.add8(211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 3 (211, 211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(211n);
    input.add8(211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint8) => euint32 test 4 (211, 207)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(211n);
    input.add8(207n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 1 (2371399582, 166)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2371399582n);
    input.add8(166n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 2 (162, 166)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(162n);
    input.add8(166n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 3 (166, 166)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(166n);
    input.add8(166n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint8) => ebool test 4 (166, 162)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(166n);
    input.add8(162n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 1 (1535914080, 80)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1535914080n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 2 (76, 80)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(76n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 3 (80, 80)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(80n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint8) => ebool test 4 (80, 76)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(80n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 1 (1009223821, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1009223821n);
    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 2 (183, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(183n);
    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 3 (187, 187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(187n);
    input.add8(187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint8) => ebool test 4 (187, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(187n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 1 (1381529874, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1381529874n);
    input.add8(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 2 (53, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(53n);
    input.add8(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 3 (57, 57)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(57n);
    input.add8(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint8) => ebool test 4 (57, 53)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(57n);
    input.add8(53n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 1 (884339915, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(884339915n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 2 (179, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(179n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 3 (183, 183)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(183n);
    input.add8(183n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint8) => ebool test 4 (183, 179)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(183n);
    input.add8(179n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 1 (1459989337, 220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1459989337n);
    input.add8(220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 2 (216, 220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(216n);
    input.add8(220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 3 (220, 220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(220n);
    input.add8(220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint8) => ebool test 4 (220, 216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(220n);
    input.add8(216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (2070023085, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2070023085n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(28n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (24, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(24n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(24n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (28, 28)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(28n);
    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(28n);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (28, 24)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(28n);
    input.add8(24n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(24n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 1 (2231185000, 65)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2231185000n);
    input.add8(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2231185000n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 2 (61, 65)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(61n);
    input.add8(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(65n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 3 (65, 65)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(65n);
    input.add8(65n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(65n);
  });

  it('test operator "max" overload (euint32, euint8) => euint32 test 4 (65, 61)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(65n);
    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(65n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 1 (35595, 11)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(35595n);
    input.add16(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(35606n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 2 (18477, 18479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(18477n);
    input.add16(18479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(36956n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 3 (18479, 18479)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(18479n);
    input.add16(18479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(36958n);
  });

  it('test operator "add" overload (euint32, euint16) => euint32 test 4 (18479, 18477)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(18479n);
    input.add16(18477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(36956n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 1 (18922, 18922)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(18922n);
    input.add16(18922n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint16) => euint32 test 2 (18922, 18918)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(18922n);
    input.add16(18918n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (30224, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(30224n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(60448n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (160, 160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(160n);
    input.add16(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(25600n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (160, 160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(160n);
    input.add16(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(25600n);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (160, 160)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(160n);
    input.add16(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(25600n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 1 (1681139161, 6880)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1681139161n);
    input.add16(6880n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(192n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 2 (6876, 6880)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(6876n);
    input.add16(6880n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(6848n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 3 (6880, 6880)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(6880n);
    input.add16(6880n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(6880n);
  });

  it('test operator "and" overload (euint32, euint16) => euint32 test 4 (6880, 6876)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(6880n);
    input.add16(6876n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(6848n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 1 (68701034, 41103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(68701034n);
    input.add16(41103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(68742127n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 2 (41099, 41103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(41099n);
    input.add16(41103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(41103n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 3 (41103, 41103)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(41103n);
    input.add16(41103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(41103n);
  });

  it('test operator "or" overload (euint32, euint16) => euint32 test 4 (41103, 41099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(41103n);
    input.add16(41099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(41103n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 1 (3818112177, 23871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3818112177n);
    input.add16(23871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3818101134n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 2 (23867, 23871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(23867n);
    input.add16(23871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 3 (23871, 23871)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(23871n);
    input.add16(23871n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint16) => euint32 test 4 (23871, 23867)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(23871n);
    input.add16(23867n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 1 (843782707, 34473)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(843782707n);
    input.add16(34473n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 2 (34469, 34473)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(34469n);
    input.add16(34473n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 3 (34473, 34473)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(34473n);
    input.add16(34473n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint16) => ebool test 4 (34473, 34469)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(34473n);
    input.add16(34469n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 1 (3735850389, 7960)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3735850389n);
    input.add16(7960n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 2 (7956, 7960)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(7956n);
    input.add16(7960n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 3 (7960, 7960)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(7960n);
    input.add16(7960n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint16) => ebool test 4 (7960, 7956)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(7960n);
    input.add16(7956n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 1 (3233856655, 54626)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3233856655n);
    input.add16(54626n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 2 (54622, 54626)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(54622n);
    input.add16(54626n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 3 (54626, 54626)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(54626n);
    input.add16(54626n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint16) => ebool test 4 (54626, 54622)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(54626n);
    input.add16(54622n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 1 (1059995818, 54018)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1059995818n);
    input.add16(54018n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 2 (54014, 54018)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(54014n);
    input.add16(54018n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 3 (54018, 54018)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(54018n);
    input.add16(54018n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint16) => ebool test 4 (54018, 54014)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(54018n);
    input.add16(54014n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (787386715, 25220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(787386715n);
    input.add16(25220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (25216, 25220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(25216n);
    input.add16(25220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (25220, 25220)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(25220n);
    input.add16(25220n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (25220, 25216)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(25220n);
    input.add16(25216n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (2091084769, 25571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2091084769n);
    input.add16(25571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (25567, 25571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(25567n);
    input.add16(25571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (25571, 25571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(25571n);
    input.add16(25571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (25571, 25567)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(25571n);
    input.add16(25567n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (3265803083, 25661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3265803083n);
    input.add16(25661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(25661n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (25657, 25661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(25657n);
    input.add16(25661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(25657n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (25661, 25661)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(25661n);
    input.add16(25661n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(25661n);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (25661, 25657)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(25661n);
    input.add16(25657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(25657n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (1606859063, 42684)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1606859063n);
    input.add16(42684n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1606859063n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (42680, 42684)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(42680n);
    input.add16(42684n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(42684n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (42684, 42684)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(42684n);
    input.add16(42684n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(42684n);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (42684, 42680)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(42684n);
    input.add16(42680n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(42684n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (1855544902, 1743021443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1855544902n);
    input.add32(1743021443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3598566345n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (1743021441, 1743021443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1743021441n);
    input.add32(1743021443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3486042884n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (1743021443, 1743021443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1743021443n);
    input.add32(1743021443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3486042886n);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (1743021443, 1743021441)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1743021443n);
    input.add32(1743021441n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3486042884n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (1818111464, 1818111464)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1818111464n);
    input.add32(1818111464n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (1818111464, 1818111460)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1818111464n);
    input.add32(1818111460n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 1 (96111, 23856)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(96111n);
    input.add32(23856n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2292824016n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 2 (47710, 47710)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(47710n);
    input.add32(47710n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2276244100n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 3 (47710, 47710)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(47710n);
    input.add32(47710n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2276244100n);
  });

  it('test operator "mul" overload (euint32, euint32) => euint32 test 4 (47710, 47710)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(47710n);
    input.add32(47710n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(2276244100n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 1 (3317288822, 1342588191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3317288822n);
    input.add32(1342588191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1073759510n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 2 (1342588187, 1342588191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1342588187n);
    input.add32(1342588191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1342588187n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 3 (1342588191, 1342588191)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1342588191n);
    input.add32(1342588191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1342588191n);
  });

  it('test operator "and" overload (euint32, euint32) => euint32 test 4 (1342588191, 1342588187)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1342588191n);
    input.add32(1342588187n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1342588187n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 1 (1707541891, 1244554527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1707541891n);
    input.add32(1244554527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1877960095n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 2 (1244554523, 1244554527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1244554523n);
    input.add32(1244554527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1244554527n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 3 (1244554527, 1244554527)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1244554527n);
    input.add32(1244554527n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1244554527n);
  });

  it('test operator "or" overload (euint32, euint32) => euint32 test 4 (1244554527, 1244554523)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1244554527n);
    input.add32(1244554523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1244554527n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 1 (3766539452, 3557659859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3766539452n);
    input.add32(3557659859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(881672303n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 2 (3557659855, 3557659859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3557659855n);
    input.add32(3557659859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(28n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 3 (3557659859, 3557659859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3557659859n);
    input.add32(3557659859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint32) => euint32 test 4 (3557659859, 3557659855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3557659859n);
    input.add32(3557659855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(28n);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (3899668665, 3823635226)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3899668665n);
    input.add32(3823635226n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (3823635222, 3823635226)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3823635222n);
    input.add32(3823635226n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (3823635226, 3823635226)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3823635226n);
    input.add32(3823635226n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (3823635226, 3823635222)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3823635226n);
    input.add32(3823635222n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 1 (440396727, 1222940316)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(440396727n);
    input.add32(1222940316n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 2 (440396723, 440396727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(440396723n);
    input.add32(440396727n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 3 (440396727, 440396727)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(440396727n);
    input.add32(440396727n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint32) => ebool test 4 (440396727, 440396723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(440396727n);
    input.add32(440396723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 1 (929694790, 907768232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(929694790n);
    input.add32(907768232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 2 (907768228, 907768232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(907768228n);
    input.add32(907768232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 3 (907768232, 907768232)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(907768232n);
    input.add32(907768232n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint32) => ebool test 4 (907768232, 907768228)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(907768232n);
    input.add32(907768228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 1 (3794643996, 619151916)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3794643996n);
    input.add32(619151916n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 2 (619151912, 619151916)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(619151912n);
    input.add32(619151916n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 3 (619151916, 619151916)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(619151916n);
    input.add32(619151916n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint32) => ebool test 4 (619151916, 619151912)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(619151916n);
    input.add32(619151912n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 1 (1814515265, 1215301262)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1814515265n);
    input.add32(1215301262n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 2 (1215301258, 1215301262)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1215301258n);
    input.add32(1215301262n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 3 (1215301262, 1215301262)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1215301262n);
    input.add32(1215301262n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint32) => ebool test 4 (1215301262, 1215301258)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1215301262n);
    input.add32(1215301258n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.le_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 1 (597865082, 2797648845)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(597865082n);
    input.add32(2797648845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 2 (597865078, 597865082)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(597865078n);
    input.add32(597865082n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 3 (597865082, 597865082)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(597865082n);
    input.add32(597865082n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint32) => ebool test 4 (597865082, 597865078)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(597865082n);
    input.add32(597865078n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.lt_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 1 (3167993396, 3198148813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3167993396n);
    input.add32(3198148813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3167993396n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 2 (3167993392, 3167993396)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3167993392n);
    input.add32(3167993396n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3167993392n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 3 (3167993396, 3167993396)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3167993396n);
    input.add32(3167993396n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3167993396n);
  });

  it('test operator "min" overload (euint32, euint32) => euint32 test 4 (3167993396, 3167993392)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3167993396n);
    input.add32(3167993392n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.min_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(3167993392n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 1 (1310408559, 1943659319)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1310408559n);
    input.add32(1943659319n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1943659319n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 2 (1310408555, 1310408559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1310408555n);
    input.add32(1310408559n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1310408559n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 3 (1310408559, 1310408559)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1310408559n);
    input.add32(1310408559n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1310408559n);
  });

  it('test operator "max" overload (euint32, euint32) => euint32 test 4 (1310408559, 1310408555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1310408559n);
    input.add32(1310408555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.max_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract5.res32());
    expect(res).to.equal(1310408559n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 1 (2, 4294187631)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(4294187631n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4294187633n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 2 (1977492101, 1977492105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1977492101n);
    input.add64(1977492105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3954984206n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 3 (1977492105, 1977492105)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1977492105n);
    input.add64(1977492105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3954984210n);
  });

  it('test operator "add" overload (euint32, euint64) => euint64 test 4 (1977492105, 1977492101)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1977492105n);
    input.add64(1977492101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.add_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(3954984206n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (2880371585, 2880371585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2880371585n);
    input.add64(2880371585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (2880371585, 2880371581)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2880371585n);
    input.add64(2880371581n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 1 (2, 2146877501)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2n);
    input.add64(2146877501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4293755002n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 2 (43587, 43587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(43587n);
    input.add64(43587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1899826569n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 3 (43587, 43587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(43587n);
    input.add64(43587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1899826569n);
  });

  it('test operator "mul" overload (euint32, euint64) => euint64 test 4 (43587, 43587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(43587n);
    input.add64(43587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.mul_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1899826569n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 1 (1745478480, 18442616126135451977)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1745478480n);
    input.add64(18442616126135451977n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1208074560n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 2 (1745478476, 1745478480)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1745478476n);
    input.add64(1745478480n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1745478464n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 3 (1745478480, 1745478480)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1745478480n);
    input.add64(1745478480n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1745478480n);
  });

  it('test operator "and" overload (euint32, euint64) => euint64 test 4 (1745478480, 1745478476)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1745478480n);
    input.add64(1745478476n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1745478464n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 1 (1546853992, 18445417582546475671)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1546853992n);
    input.add64(18445417582546475671n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18445417583755615999n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 2 (1546853988, 1546853992)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1546853988n);
    input.add64(1546853992n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1546853996n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 3 (1546853992, 1546853992)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1546853992n);
    input.add64(1546853992n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1546853992n);
  });

  it('test operator "or" overload (euint32, euint64) => euint64 test 4 (1546853992, 1546853988)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1546853992n);
    input.add64(1546853988n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(1546853996n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (390302983, 18441970713749353087)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(390302983n);
    input.add64(18441970713749353087n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(18441970713904316280n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (390302979, 390302983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(390302979n);
    input.add64(390302983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (390302983, 390302983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(390302983n);
    input.add64(390302983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (390302983, 390302979)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(390302983n);
    input.add64(390302979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract5.res64());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 1 (3575591686, 18441139457541916589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3575591686n);
    input.add64(18441139457541916589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 2 (3575591682, 3575591686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3575591682n);
    input.add64(3575591686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 3 (3575591686, 3575591686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3575591686n);
    input.add64(3575591686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint64) => ebool test 4 (3575591686, 3575591682)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(3575591686n);
    input.add64(3575591682n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (1132249413, 18438312419950369711)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1132249413n);
    input.add64(18438312419950369711n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (1132249409, 1132249413)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1132249409n);
    input.add64(1132249413n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (1132249413, 1132249413)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1132249413n);
    input.add64(1132249413n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (1132249413, 1132249409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1132249413n);
    input.add64(1132249409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 1 (2716663813, 18446666024340671893)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2716663813n);
    input.add64(18446666024340671893n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 2 (2716663809, 2716663813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2716663809n);
    input.add64(2716663813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 3 (2716663813, 2716663813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2716663813n);
    input.add64(2716663813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint64) => ebool test 4 (2716663813, 2716663809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(2716663813n);
    input.add64(2716663809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ge_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 1 (1898220780, 18441112726231908571)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1898220780n);
    input.add64(18441112726231908571n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 2 (1898220776, 1898220780)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1898220776n);
    input.add64(1898220780n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 3 (1898220780, 1898220780)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1898220780n);
    input.add64(1898220780n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint64) => ebool test 4 (1898220780, 1898220776)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract5Address, this.signers.alice.address);
    input.add32(1898220780n);
    input.add64(1898220776n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.gt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract5.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 1 (1718048813, 18439987431102861375)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1718048813n);
    input.add64(18439987431102861375n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 2 (1718048809, 1718048813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1718048809n);
    input.add64(1718048813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 3 (1718048813, 1718048813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1718048813n);
    input.add64(1718048813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint64) => ebool test 4 (1718048813, 1718048809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1718048813n);
    input.add64(1718048809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 1 (3259927115, 18445091130782070813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3259927115n);
    input.add64(18445091130782070813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 2 (3259927111, 3259927115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3259927111n);
    input.add64(3259927115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 3 (3259927115, 3259927115)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3259927115n);
    input.add64(3259927115n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint64) => ebool test 4 (3259927115, 3259927111)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3259927115n);
    input.add64(3259927111n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 1 (1841925700, 18442933982305840083)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1841925700n);
    input.add64(18442933982305840083n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(1841925700n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 2 (1841925696, 1841925700)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1841925696n);
    input.add64(1841925700n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(1841925696n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 3 (1841925700, 1841925700)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1841925700n);
    input.add64(1841925700n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(1841925700n);
  });

  it('test operator "min" overload (euint32, euint64) => euint64 test 4 (1841925700, 1841925696)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1841925700n);
    input.add64(1841925696n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(1841925696n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 1 (1980815436, 18446246159979986155)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1980815436n);
    input.add64(18446246159979986155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(18446246159979986155n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 2 (1980815432, 1980815436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1980815432n);
    input.add64(1980815436n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(1980815436n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 3 (1980815436, 1980815436)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1980815436n);
    input.add64(1980815436n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(1980815436n);
  });

  it('test operator "max" overload (euint32, euint64) => euint64 test 4 (1980815436, 1980815432)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1980815436n);
    input.add64(1980815432n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract6.res64());
    expect(res).to.equal(1980815436n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 1 (2, 2147483649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2n);
    input.add128(2147483649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 2 (788156404, 788156408)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(788156404n);
    input.add128(788156408n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(1576312812n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 3 (788156408, 788156408)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(788156408n);
    input.add128(788156408n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(1576312816n);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 4 (788156408, 788156404)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(788156408n);
    input.add128(788156404n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(1576312812n);
  });

  it('test operator "sub" overload (euint32, euint128) => euint128 test 1 (3561141951, 3561141951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3561141951n);
    input.add128(3561141951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint128) => euint128 test 2 (3561141951, 3561141947)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3561141951n);
    input.add128(3561141947n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 1 (2, 1073741825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2n);
    input.add128(1073741825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 2 (63676, 63676)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(63676n);
    input.add128(63676n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4054632976n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 3 (63676, 63676)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(63676n);
    input.add128(63676n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4054632976n);
  });

  it('test operator "mul" overload (euint32, euint128) => euint128 test 4 (63676, 63676)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(63676n);
    input.add128(63676n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4054632976n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 1 (2659354872, 340282366920938463463373182215335994821)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2659354872n);
    input.add128(340282366920938463463373182215335994821n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2424307904n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 2 (2659354868, 2659354872)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2659354868n);
    input.add128(2659354872n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2659354864n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 3 (2659354872, 2659354872)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2659354872n);
    input.add128(2659354872n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2659354872n);
  });

  it('test operator "and" overload (euint32, euint128) => euint128 test 4 (2659354872, 2659354868)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2659354872n);
    input.add128(2659354868n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(2659354864n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 1 (1775515615, 340282366920938463463365726409727391809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1775515615n);
    input.add128(340282366920938463463365726409727391809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463365726409758064607n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 2 (1775515611, 1775515615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1775515611n);
    input.add128(1775515615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(1775515615n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 3 (1775515615, 1775515615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1775515615n);
    input.add128(1775515615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(1775515615n);
  });

  it('test operator "or" overload (euint32, euint128) => euint128 test 4 (1775515615, 1775515611)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1775515615n);
    input.add128(1775515611n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(1775515615n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 1 (3049009589, 340282366920938463463368216078619273371)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3049009589n);
    input.add128(340282366920938463463368216078619273371n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463368216080538425646n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 2 (3049009585, 3049009589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3049009585n);
    input.add128(3049009589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 3 (3049009589, 3049009589)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3049009589n);
    input.add128(3049009589n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint128) => euint128 test 4 (3049009589, 3049009585)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3049009589n);
    input.add128(3049009585n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 1 (764411697, 340282366920938463463368167171281881439)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(764411697n);
    input.add128(340282366920938463463368167171281881439n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 2 (764411693, 764411697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(764411693n);
    input.add128(764411697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 3 (764411697, 764411697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(764411697n);
    input.add128(764411697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 4 (764411697, 764411693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(764411697n);
    input.add128(764411693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 1 (3303815983, 340282366920938463463373693800095408137)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3303815983n);
    input.add128(340282366920938463463373693800095408137n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 2 (3303815979, 3303815983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3303815979n);
    input.add128(3303815983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 3 (3303815983, 3303815983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3303815983n);
    input.add128(3303815983n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint128) => ebool test 4 (3303815983, 3303815979)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3303815983n);
    input.add128(3303815979n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 1 (3786584955, 340282366920938463463367364294961686443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3786584955n);
    input.add128(340282366920938463463367364294961686443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 2 (3786584951, 3786584955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3786584951n);
    input.add128(3786584955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 3 (3786584955, 3786584955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3786584955n);
    input.add128(3786584955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 4 (3786584955, 3786584951)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3786584955n);
    input.add128(3786584951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 1 (4277077408, 340282366920938463463374604764991737889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4277077408n);
    input.add128(340282366920938463463374604764991737889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 2 (4277077404, 4277077408)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4277077404n);
    input.add128(4277077408n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 3 (4277077408, 4277077408)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4277077408n);
    input.add128(4277077408n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 4 (4277077408, 4277077404)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4277077408n);
    input.add128(4277077404n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 1 (867198677, 340282366920938463463369757600308236313)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(867198677n);
    input.add128(340282366920938463463369757600308236313n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 2 (867198673, 867198677)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(867198673n);
    input.add128(867198677n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 3 (867198677, 867198677)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(867198677n);
    input.add128(867198677n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint128) => ebool test 4 (867198677, 867198673)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(867198677n);
    input.add128(867198673n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 1 (2093199071, 340282366920938463463370622002914075235)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2093199071n);
    input.add128(340282366920938463463370622002914075235n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 2 (2093199067, 2093199071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2093199067n);
    input.add128(2093199071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 3 (2093199071, 2093199071)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2093199071n);
    input.add128(2093199071n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint128) => ebool test 4 (2093199071, 2093199067)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2093199071n);
    input.add128(2093199067n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 1 (4003399201, 340282366920938463463365902462339741051)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4003399201n);
    input.add128(340282366920938463463365902462339741051n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4003399201n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 2 (4003399197, 4003399201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4003399197n);
    input.add128(4003399201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4003399197n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 3 (4003399201, 4003399201)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4003399201n);
    input.add128(4003399201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4003399201n);
  });

  it('test operator "min" overload (euint32, euint128) => euint128 test 4 (4003399201, 4003399197)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(4003399201n);
    input.add128(4003399197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(4003399197n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 1 (731106649, 340282366920938463463370948491424745963)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(731106649n);
    input.add128(340282366920938463463370948491424745963n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(340282366920938463463370948491424745963n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 2 (731106645, 731106649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(731106645n);
    input.add128(731106649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(731106649n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 3 (731106649, 731106649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(731106649n);
    input.add128(731106649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(731106649n);
  });

  it('test operator "max" overload (euint32, euint128) => euint128 test 4 (731106649, 731106645)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(731106649n);
    input.add128(731106645n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract6.res128());
    expect(res).to.equal(731106649n);
  });

  it('test operator "add" overload (euint32, euint256) => euint256 test 1 (2, 2147483649)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2n);
    input.add256(2147483649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint32, euint256) => euint256 test 2 (1370548128, 1370548130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1370548128n);
    input.add256(1370548130n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(2741096258n);
  });

  it('test operator "add" overload (euint32, euint256) => euint256 test 3 (1370548130, 1370548130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1370548130n);
    input.add256(1370548130n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(2741096260n);
  });

  it('test operator "add" overload (euint32, euint256) => euint256 test 4 (1370548130, 1370548128)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1370548130n);
    input.add256(1370548128n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(2741096258n);
  });

  it('test operator "sub" overload (euint32, euint256) => euint256 test 1 (3022897647, 3022897647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3022897647n);
    input.add256(3022897647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint32, euint256) => euint256 test 2 (3022897647, 3022897643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3022897647n);
    input.add256(3022897643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint32, euint256) => euint256 test 1 (2, 1073741825)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2n);
    input.add256(1073741825n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint32, euint256) => euint256 test 2 (59596, 59596)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(59596n);
    input.add256(59596n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(3551683216n);
  });

  it('test operator "mul" overload (euint32, euint256) => euint256 test 3 (59596, 59596)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(59596n);
    input.add256(59596n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(3551683216n);
  });

  it('test operator "mul" overload (euint32, euint256) => euint256 test 4 (59596, 59596)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(59596n);
    input.add256(59596n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(3551683216n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 1 (3994882895, 115792089237316195423570985008687907853269984665640564039457577397354470817443)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3994882895n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577397354470817443n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(1310528003n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 2 (3994882891, 3994882895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3994882891n);
    input.add256(3994882895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(3994882891n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 3 (3994882895, 3994882895)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3994882895n);
    input.add256(3994882895n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(3994882895n);
  });

  it('test operator "and" overload (euint32, euint256) => euint256 test 4 (3994882895, 3994882891)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3994882895n);
    input.add256(3994882891n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(3994882891n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 1 (2321952704, 115792089237316195423570985008687907853269984665640564039457581516412469896587)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2321952704n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581516412469896587n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581516414655131595n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 2 (2321952700, 2321952704)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2321952700n);
    input.add256(2321952704n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(2321952764n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 3 (2321952704, 2321952704)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2321952704n);
    input.add256(2321952704n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(2321952704n);
  });

  it('test operator "or" overload (euint32, euint256) => euint256 test 4 (2321952704, 2321952700)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2321952704n);
    input.add256(2321952700n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(2321952764n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 1 (3706671686, 115792089237316195423570985008687907853269984665640564039457578090382088595177)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3706671686n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578090382088595177n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578090383101310127n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 2 (3706671682, 3706671686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3706671682n);
    input.add256(3706671686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 3 (3706671686, 3706671686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3706671686n);
    input.add256(3706671686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint32, euint256) => euint256 test 4 (3706671686, 3706671682)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3706671686n);
    input.add256(3706671682n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract6.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 1 (1808856605, 115792089237316195423570985008687907853269984665640564039457582106419349698079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1808856605n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582106419349698079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 2 (1808856601, 1808856605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1808856601n);
    input.add256(1808856605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 3 (1808856605, 1808856605)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1808856605n);
    input.add256(1808856605n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint32, euint256) => ebool test 4 (1808856605, 1808856601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(1808856605n);
    input.add256(1808856601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 1 (3268569536, 115792089237316195423570985008687907853269984665640564039457577368549107012293)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3268569536n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577368549107012293n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 2 (3268569532, 3268569536)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3268569532n);
    input.add256(3268569536n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 3 (3268569536, 3268569536)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3268569536n);
    input.add256(3268569536n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint32, euint256) => ebool test 4 (3268569536, 3268569532)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3268569536n);
    input.add256(3268569532n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ne_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint256) => ebool test 1 (3646475026, 115792089237316195423570985008687907853269984665640564039457583473653753763859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3646475026n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583473653753763859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint256) => ebool test 2 (3646475022, 3646475026)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3646475022n);
    input.add256(3646475026n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint32, euint256) => ebool test 3 (3646475026, 3646475026)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3646475026n);
    input.add256(3646475026n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint32, euint256) => ebool test 4 (3646475026, 3646475022)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3646475026n);
    input.add256(3646475022n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint32, euint256) => ebool test 1 (27376130, 115792089237316195423570985008687907853269984665640564039457582301524979306099)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(27376130n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582301524979306099n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint256) => ebool test 2 (27376126, 27376130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(27376126n);
    input.add256(27376130n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint256) => ebool test 3 (27376130, 27376130)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(27376130n);
    input.add256(27376130n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint32, euint256) => ebool test 4 (27376130, 27376126)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(27376130n);
    input.add256(27376126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint256) => ebool test 1 (2709381066, 115792089237316195423570985008687907853269984665640564039457582000267248354603)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2709381066n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582000267248354603n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint256) => ebool test 2 (2709381062, 2709381066)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2709381062n);
    input.add256(2709381066n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint256) => ebool test 3 (2709381066, 2709381066)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2709381066n);
    input.add256(2709381066n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint32, euint256) => ebool test 4 (2709381066, 2709381062)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(2709381066n);
    input.add256(2709381062n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint256) => ebool test 1 (3016190595, 115792089237316195423570985008687907853269984665640564039457583853908247743739)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3016190595n);
    input.add256(115792089237316195423570985008687907853269984665640564039457583853908247743739n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint256) => ebool test 2 (3016190591, 3016190595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3016190591n);
    input.add256(3016190595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint32, euint256) => ebool test 3 (3016190595, 3016190595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3016190595n);
    input.add256(3016190595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint32, euint256) => ebool test 4 (3016190595, 3016190591)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract6Address, this.signers.alice.address);
    input.add32(3016190595n);
    input.add256(3016190591n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract6.resb());
    expect(res).to.equal(false);
  });
});
