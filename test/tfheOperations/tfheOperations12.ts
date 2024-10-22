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
import {
  createInstances,
  decrypt128,
  decrypt16,
  decrypt256,
  decrypt32,
  decrypt4,
  decrypt64,
  decrypt8,
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

describe('TFHE operations 12', function () {
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

  it('test operator "eq" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579028937132718443, 115792089237316195423570985008687907853269984665640564039457576393900678416361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579028937132718443n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576393900678416361n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575281612927984427, 115792089237316195423570985008687907853269984665640564039457575281612927984431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984427n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575281612927984431n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575281612927984431, 115792089237316195423570985008687907853269984665640564039457575281612927984431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984431n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575281612927984431n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575281612927984431, 115792089237316195423570985008687907853269984665640564039457575281612927984427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984431n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575281612927984427n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577173439170014457, 115792089237316195423570985008687907853269984665640564039457576393900678416361)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576393900678416361n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577173439170014457n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575281612927984427, 115792089237316195423570985008687907853269984665640564039457575281612927984431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575281612927984427n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575281612927984431, 115792089237316195423570985008687907853269984665640564039457575281612927984431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575281612927984431n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575281612927984431, 115792089237316195423570985008687907853269984665640564039457575281612927984427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575281612927984431n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581421114730215071, 115792089237316195423570985008687907853269984665640564039457583491257588609271)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581421114730215071n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457583491257588609271n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457577745710288733719, 115792089237316195423570985008687907853269984665640564039457577745710288733723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733719n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577745710288733723n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457577745710288733723, 115792089237316195423570985008687907853269984665640564039457577745710288733723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733723n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577745710288733723n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457577745710288733723, 115792089237316195423570985008687907853269984665640564039457577745710288733719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733723n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577745710288733719n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583026693216867801, 115792089237316195423570985008687907853269984665640564039457583491257588609271)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457583491257588609271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457583026693216867801n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457577745710288733719, 115792089237316195423570985008687907853269984665640564039457577745710288733723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577745710288733719n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457577745710288733723, 115792089237316195423570985008687907853269984665640564039457577745710288733723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577745710288733723n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457577745710288733723, 115792089237316195423570985008687907853269984665640564039457577745710288733719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577745710288733723n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579759156646767465, 115792089237316195423570985008687907853269984665640564039457580411660636009627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580411660636009627n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457579759156646767461, 115792089237316195423570985008687907853269984665640564039457579759156646767465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767461n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579759156646767465n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457579759156646767465, 115792089237316195423570985008687907853269984665640564039457579759156646767465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579759156646767465n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457579759156646767465, 115792089237316195423570985008687907853269984665640564039457579759156646767461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767465n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579759156646767461n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581430272333247909, 115792089237316195423570985008687907853269984665640564039457580411660636009627)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580411660636009627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457581430272333247909n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457579759156646767461, 115792089237316195423570985008687907853269984665640564039457579759156646767465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579759156646767461n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457579759156646767465, 115792089237316195423570985008687907853269984665640564039457579759156646767465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579759156646767465n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457579759156646767465, 115792089237316195423570985008687907853269984665640564039457579759156646767461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579759156646767465n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580885855005541993, 115792089237316195423570985008687907853269984665640564039457576330419987992913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541993n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576330419987992913n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457580885855005541989, 115792089237316195423570985008687907853269984665640564039457580885855005541993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541989n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580885855005541993n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457580885855005541993, 115792089237316195423570985008687907853269984665640564039457580885855005541993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541993n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580885855005541993n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457580885855005541993, 115792089237316195423570985008687907853269984665640564039457580885855005541989)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541993n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580885855005541989n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583273117366659453, 115792089237316195423570985008687907853269984665640564039457576330419987992913)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576330419987992913n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457583273117366659453n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457580885855005541989, 115792089237316195423570985008687907853269984665640564039457580885855005541993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580885855005541989n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457580885855005541993, 115792089237316195423570985008687907853269984665640564039457580885855005541993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580885855005541993n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457580885855005541993, 115792089237316195423570985008687907853269984665640564039457580885855005541989)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541989n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580885855005541993n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583300217912499775, 115792089237316195423570985008687907853269984665640564039457580310546116642945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583300217912499775n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457580310546116642945n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457582857952818399805, 115792089237316195423570985008687907853269984665640564039457582857952818399809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399805n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582857952818399809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457582857952818399809, 115792089237316195423570985008687907853269984665640564039457582857952818399809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582857952818399809n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457582857952818399809, 115792089237316195423570985008687907853269984665640564039457582857952818399805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399809n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582857952818399805n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579919776168086147, 115792089237316195423570985008687907853269984665640564039457580310546116642945)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457580310546116642945n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579919776168086147n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457582857952818399805, 115792089237316195423570985008687907853269984665640564039457582857952818399809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582857952818399805n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457582857952818399809, 115792089237316195423570985008687907853269984665640564039457582857952818399809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582857952818399809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457582857952818399809, 115792089237316195423570985008687907853269984665640564039457582857952818399805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399805n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582857952818399809n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, uint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579345439988102233, 115792089237316195423570985008687907853269984665640564039457581362267810923471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579345439988102233n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581362267810923471n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, uint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575518813902238257, 115792089237316195423570985008687907853269984665640564039457575518813902238261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238257n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575518813902238261n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, uint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575518813902238261, 115792089237316195423570985008687907853269984665640564039457575518813902238261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238261n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575518813902238261n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, uint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575518813902238261, 115792089237316195423570985008687907853269984665640564039457575518813902238257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238261n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575518813902238257n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578628645763210627, 115792089237316195423570985008687907853269984665640564039457581362267810923471)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581362267810923471n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457578628645763210627n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575518813902238257, 115792089237316195423570985008687907853269984665640564039457575518813902238261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575518813902238257n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575518813902238261, 115792089237316195423570985008687907853269984665640564039457575518813902238261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575518813902238261n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575518813902238261, 115792089237316195423570985008687907853269984665640564039457575518813902238257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457575518813902238261n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579091097262441647, 115792089237316195423570985008687907853269984665640564039457576915935387349417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579091097262441647n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576915935387349417n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576915935387349417n);
  });

  it('test operator "min" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577447088911778287, 115792089237316195423570985008687907853269984665640564039457577447088911778291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778287n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577447088911778291n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577447088911778287n);
  });

  it('test operator "min" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577447088911778291, 115792089237316195423570985008687907853269984665640564039457577447088911778291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778291n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577447088911778291n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577447088911778291n);
  });

  it('test operator "min" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577447088911778291, 115792089237316195423570985008687907853269984665640564039457577447088911778287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778291n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577447088911778287n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577447088911778287n);
  });

  it('test operator "min" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579127166009597401, 115792089237316195423570985008687907853269984665640564039457576915935387349417)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457576915935387349417n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579127166009597401n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576915935387349417n);
  });

  it('test operator "min" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577447088911778287, 115792089237316195423570985008687907853269984665640564039457577447088911778291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577447088911778287n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577447088911778287n);
  });

  it('test operator "min" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577447088911778291, 115792089237316195423570985008687907853269984665640564039457577447088911778291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577447088911778291n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577447088911778291n);
  });

  it('test operator "min" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577447088911778291, 115792089237316195423570985008687907853269984665640564039457577447088911778287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577447088911778291n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577447088911778287n);
  });

  it('test operator "max" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577915867722108039, 115792089237316195423570985008687907853269984665640564039457582764130067775401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577915867722108039n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457582764130067775401n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582764130067775401n);
  });

  it('test operator "max" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577809030115355405, 115792089237316195423570985008687907853269984665640564039457577809030115355409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355405n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577809030115355409n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
  });

  it('test operator "max" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577809030115355409, 115792089237316195423570985008687907853269984665640564039457577809030115355409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355409n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577809030115355409n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
  });

  it('test operator "max" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577809030115355409, 115792089237316195423570985008687907853269984665640564039457577809030115355405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355409n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577809030115355405n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
  });

  it('test operator "max" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582906409122638869, 115792089237316195423570985008687907853269984665640564039457582764130067775401)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457582764130067775401n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457582906409122638869n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582906409122638869n);
  });

  it('test operator "max" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577809030115355405, 115792089237316195423570985008687907853269984665640564039457577809030115355409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577809030115355405n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
  });

  it('test operator "max" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577809030115355409, 115792089237316195423570985008687907853269984665640564039457577809030115355409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577809030115355409n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
  });

  it('test operator "max" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577809030115355409, 115792089237316195423570985008687907853269984665640564039457577809030115355405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577809030115355409n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 1 (1, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.shl_euint4_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.shl_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.shl_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.shl_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 1 (4, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.shr_euint4_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.shr_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.shr_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.shr_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "rotl" overload (euint4, uint8) => euint4 test 1 (1, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(1n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rotl_euint4_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "rotl" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rotl_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "rotl" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rotl_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "rotl" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rotl_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "rotr" overload (euint4, uint8) => euint4 test 1 (14, 1)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(14n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rotr_euint4_uint8(encryptedAmount.handles[0], 1n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(7n);
  });

  it('test operator "rotr" overload (euint4, uint8) => euint4 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rotr_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(4n);
  });

  it('test operator "rotr" overload (euint4, uint8) => euint4 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rotr_euint4_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "rotr" overload (euint4, uint8) => euint4 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add4(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rotr_euint4_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract10.res4());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (44, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(44n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(192n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 1 (44, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(44n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(192n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shl" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 1 (160, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(160n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(20n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 1 (160, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(160n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint8_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(20n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "shr" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 1 (150, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(150n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(105n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotl" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 1 (150, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(150n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(105n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotl" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 1 (52, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(52n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(208n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 1 (52, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(52n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint8_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(208n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(4n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint8_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(8n);
  });

  it('test operator "rotr" overload (euint8, uint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint8_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 1 (10029, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(10029n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(58784n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 1 (10029, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(10029n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint16_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(58784n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 1 (63034, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(63034n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(984n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (63034, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(63034n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint16_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(984n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 1 (34007, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(34007n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(19832n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 1 (34007, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(34007n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(19832n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 1 (32201, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(32201n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(12217n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotr" overload (euint16, euint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(32768n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 1 (32201, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(32201n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint16_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(12217n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotr" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(32768n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 1 (3189891720, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(3189891720n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(3793627264n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 1 (3189891720, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(3189891720n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(3793627264n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 1 (1720949149, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(1720949149n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(107559321n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 1 (1720949149, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(1720949149n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(107559321n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 1 (10430463, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(10430463n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(83443704n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 1 (10430463, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(10430463n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint32_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(83443704n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 1 (3489309750, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(3489309750n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(1839199560n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(67108864n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(134217728n);
  });

  it('test operator "rotr" overload (euint32, euint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(2147483648n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 1 (3489309750, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(3489309750n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint32_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(1839199560n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(67108864n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint32_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(134217728n);
  });

  it('test operator "rotr" overload (euint32, uint8) => euint32 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint32_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(2147483648n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 1 (18438852048643145403, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(18438852048643145403n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(17941654469459553984n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 1 (18438852048643145403, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(18438852048643145403n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(17941654469459553984n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 1 (18444486134915793097, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(18444486134915793097n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(288195095858059267n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (18444486134915793097, 6)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(18444486134915793097n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(288195095858059267n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 1 (18440494415218168649, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(18440494415218168649n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(17646787786812531967n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 1 (18440494415218168649, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(18440494415218168649n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint64_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(17646787786812531967n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 1 (18444475066789617897, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(18444475066789617897n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(4611402392562396189n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(288230376151711744n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(576460752303423488n);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(9223372036854775808n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 1 (18444475066789617897, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(18444475066789617897n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint64_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(4611402392562396189n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(288230376151711744n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(576460752303423488n);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(9223372036854775808n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463366293596486217477, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(340282366920938463463366293596486217477n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(340282366920938463463108564702744404128n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463366293596486217477, 5)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(340282366920938463463366293596486217477n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint128_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(340282366920938463463108564702744404128n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint128_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463368208225205051841, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(340282366920938463463368208225205051841n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(85070591730234615865842052056301262960n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463368208225205051841, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(340282366920938463463368208225205051841n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint128_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(85070591730234615865842052056301262960n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint128, uint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint128_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463369231473752264653, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(340282366920938463463369231473752264653n);
    input.add8(3n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(340282366920938463463331599767640637039n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463369231473752264653, 3)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(340282366920938463463369231473752264653n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint128_uint8(encryptedAmount.handles[0], 3n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(340282366920938463463331599767640637039n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint128, uint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint128_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463366872490223703215, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(340282366920938463463366872490223703215n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(340282366920938463463372673696382084395n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(5316911983139663491615228241121378304n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(10633823966279326983230456482242756608n);
  });

  it('test operator "rotr" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(170141183460469231731687303715884105728n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 1 (340282366920938463463366872490223703215, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(340282366920938463463366872490223703215n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint128_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(340282366920938463463372673696382084395n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(5316911983139663491615228241121378304n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint128_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(10633823966279326983230456482242756608n);
  });

  it('test operator "rotr" overload (euint128, uint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint128_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(170141183460469231731687303715884105728n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578572299000463425, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578572299000463425n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039456888249304595046528n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint256, euint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(128n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578572299000463425, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578572299000463425n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039456888249304595046528n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(1024n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(2048n);
  });

  it('test operator "shl" overload (euint256, uint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shl_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(128n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578470671649972655, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578470671649972655n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(28948022309329048855892746252171976963317496166410141009864394617667912493163n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, euint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578470671649972655, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578470671649972655n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint256_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(28948022309329048855892746252171976963317496166410141009864394617667912493163n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "shr" overload (euint256, uint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.shr_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582882208711886349, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582882208711886349n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457439917747657180927n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint256, euint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(128n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582882208711886349, 7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582882208711886349n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint256_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457439917747657180927n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(1024n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(2048n);
  });

  it('test operator "rotl" overload (euint256, uint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotl_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(128n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581545102921797223, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581545102921797223n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(57896044618658097711785492504343953926634992332820282019728791850030926829798n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(1809251394333065553493296640760748560207343510400633813116524750123642650624n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(3618502788666131106986593281521497120414687020801267626233049500247285301248n);
  });

  it('test operator "rotr" overload (euint256, euint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint256_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(57896044618658097711785492504343953926634992332820282019728792003956564819968n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581545102921797223, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581545102921797223n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(57896044618658097711785492504343953926634992332820282019728791850030926829798n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 2 (4, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(1809251394333065553493296640760748560207343510400633813116524750123642650624n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 3 (8, 8)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint256_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(3618502788666131106986593281521497120414687020801267626233049500247285301248n);
  });

  it('test operator "rotr" overload (euint256, uint8) => euint256 test 4 (8, 4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add256(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.rotr_euint256_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt256(await this.contract11.res256());
    expect(res).to.equal(57896044618658097711785492504343953926634992332820282019728792003956564819968n);
  });

  it('test operator "neg" overload (euint4) => euint4 test 1 (7)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add4(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.neg_euint4(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract11.res4());
    expect(res).to.equal(9n);
  });

  it('test operator "not" overload (euint4) => euint4 test 1 (4)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add4(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.not_euint4(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt4(await this.contract11.res4());
    expect(res).to.equal(11n);
  });

  it('test operator "neg" overload (euint8) => euint8 test 1 (211)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.neg_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(45n);
  });

  it('test operator "not" overload (euint8) => euint8 test 1 (221)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.not_euint8(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt8(await this.contract11.res8());
    expect(res).to.equal(34n);
  });

  it('test operator "neg" overload (euint16) => euint16 test 1 (26686)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(26686n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.neg_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(38850n);
  });

  it('test operator "not" overload (euint16) => euint16 test 1 (1813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add16(1813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.not_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt16(await this.contract11.res16());
    expect(res).to.equal(63722n);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (3401457377)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(3401457377n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.neg_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(893509919n);
  });

  it('test operator "not" overload (euint32) => euint32 test 1 (2456808918)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add32(2456808918n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.not_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt32(await this.contract11.res32());
    expect(res).to.equal(1838158377n);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (18441984683925217775)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(18441984683925217775n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.neg_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(4759389784333841n);
  });

  it('test operator "not" overload (euint64) => euint64 test 1 (18438471929660169839)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add64(18438471929660169839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.not_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt64(await this.contract11.res64());
    expect(res).to.equal(8272144049381776n);
  });

  it('test operator "neg" overload (euint128) => euint128 test 1 (340282366920938463463373162594455712433)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(340282366920938463463373162594455712433n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.neg_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(1444837312499023n);
  });

  it('test operator "not" overload (euint128) => euint128 test 1 (340282366920938463463374264817331350741)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract11Address, this.signers.alice.address);
    input.add128(340282366920938463463374264817331350741n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract11.not_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const res = await decrypt128(await this.contract11.res128());
    expect(res).to.equal(342614436860714n);
  });
});
