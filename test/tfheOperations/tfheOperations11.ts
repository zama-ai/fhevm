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
import type { TFHETestSuite10 } from '../../types/contracts/tests/TFHETestSuite10';
import type { TFHETestSuite11 } from '../../types/contracts/tests/TFHETestSuite11';
import { createInstances, decrypt256, decryptBool } from '../instance';
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

describe('TFHE operations 11', function () {
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

  it('test operator "or" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582624616109811381, 9754)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582624616109811381n);
    input.add16(9754n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582624616109811391n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 2 (9750, 9754)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(9750n);
    input.add16(9754n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(9758n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 3 (9754, 9754)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(9754n);
    input.add16(9754n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(9754n);
  });

  it('test operator "or" overload (euint256, euint16) => euint256 test 4 (9754, 9750)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(9754n);
    input.add16(9750n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.or_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(9758n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583698273346637887, 24298)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583698273346637887n);
    input.add16(24298n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583698273346621141n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 2 (24294, 24298)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(24294n);
    input.add16(24298n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 3 (24298, 24298)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(24298n);
    input.add16(24298n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint16) => euint256 test 4 (24298, 24294)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(24298n);
    input.add16(24294n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.xor_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581937945784624293, 42555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581937945784624293n);
    input.add16(42555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 2 (42551, 42555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(42551n);
    input.add16(42555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 3 (42555, 42555)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(42555n);
    input.add16(42555n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint16) => ebool test 4 (42555, 42551)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(42555n);
    input.add16(42551n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.eq_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575904968867077761, 30382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575904968867077761n);
    input.add16(30382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 2 (30378, 30382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(30378n);
    input.add16(30382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 3 (30382, 30382)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(30382n);
    input.add16(30382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint16) => ebool test 4 (30382, 30378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(30382n);
    input.add16(30378n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ne_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457575882393460599321, 29301)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575882393460599321n);
    input.add16(29301n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint16) => ebool test 2 (29297, 29301)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(29297n);
    input.add16(29301n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint16) => ebool test 3 (29301, 29301)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(29301n);
    input.add16(29301n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint16) => ebool test 4 (29301, 29297)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(29301n);
    input.add16(29297n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.ge_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580207253841161561, 59068)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580207253841161561n);
    input.add16(59068n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint16) => ebool test 2 (59064, 59068)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(59064n);
    input.add16(59068n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint16) => ebool test 3 (59068, 59068)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(59068n);
    input.add16(59068n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint16) => ebool test 4 (59068, 59064)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(59068n);
    input.add16(59064n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.gt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582351250639102167, 47994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582351250639102167n);
    input.add16(47994n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint16) => ebool test 2 (47990, 47994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(47990n);
    input.add16(47994n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint16) => ebool test 3 (47994, 47994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(47994n);
    input.add16(47994n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint16) => ebool test 4 (47994, 47990)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(47994n);
    input.add16(47990n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.le_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint16) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581793657869781609, 60020)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581793657869781609n);
    input.add16(60020n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint16) => ebool test 2 (60016, 60020)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(60016n);
    input.add16(60020n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint16) => ebool test 3 (60020, 60020)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(60020n);
    input.add16(60020n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint16) => ebool test 4 (60020, 60016)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(60020n);
    input.add16(60016n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.lt_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract9.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579076951559173157, 24679)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579076951559173157n);
    input.add16(24679n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(24679n);
  });

  it('test operator "min" overload (euint256, euint16) => euint256 test 2 (24675, 24679)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(24675n);
    input.add16(24679n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(24675n);
  });

  it('test operator "min" overload (euint256, euint16) => euint256 test 3 (24679, 24679)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(24679n);
    input.add16(24679n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(24679n);
  });

  it('test operator "min" overload (euint256, euint16) => euint256 test 4 (24679, 24675)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(24679n);
    input.add16(24675n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.min_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(24675n);
  });

  it('test operator "max" overload (euint256, euint16) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580891899920920375, 31998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580891899920920375n);
    input.add16(31998n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580891899920920375n);
  });

  it('test operator "max" overload (euint256, euint16) => euint256 test 2 (31994, 31998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(31994n);
    input.add16(31998n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(31998n);
  });

  it('test operator "max" overload (euint256, euint16) => euint256 test 3 (31998, 31998)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(31998n);
    input.add16(31998n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(31998n);
  });

  it('test operator "max" overload (euint256, euint16) => euint256 test 4 (31998, 31994)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract9Address, this.signers.alice.address);
    input.add256(31998n);
    input.add16(31994n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract9.max_euint256_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract9.res256());
    expect(res).to.equal(31998n);
  });

  it('test operator "add" overload (euint256, euint32) => euint256 test 1 (2147483649, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2147483649n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(2147483651n);
  });

  it('test operator "add" overload (euint256, euint32) => euint256 test 2 (1298573356, 1298573358)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1298573356n);
    input.add32(1298573358n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(2597146714n);
  });

  it('test operator "add" overload (euint256, euint32) => euint256 test 3 (1298573358, 1298573358)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1298573358n);
    input.add32(1298573358n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(2597146716n);
  });

  it('test operator "add" overload (euint256, euint32) => euint256 test 4 (1298573358, 1298573356)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1298573358n);
    input.add32(1298573356n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(2597146714n);
  });

  it('test operator "sub" overload (euint256, euint32) => euint256 test 1 (2775500374, 2775500374)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2775500374n);
    input.add32(2775500374n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint32) => euint256 test 2 (2775500374, 2775500370)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2775500374n);
    input.add32(2775500370n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint32) => euint256 test 1 (1073741825, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1073741825n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(2147483650n);
  });

  it('test operator "mul" overload (euint256, euint32) => euint256 test 2 (60165, 60165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(60165n);
    input.add32(60165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(3619827225n);
  });

  it('test operator "mul" overload (euint256, euint32) => euint256 test 3 (60165, 60165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(60165n);
    input.add32(60165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(3619827225n);
  });

  it('test operator "mul" overload (euint256, euint32) => euint256 test 4 (60165, 60165)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(60165n);
    input.add32(60165n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(3619827225n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579114687562092581, 2312861009)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579114687562092581n);
    input.add32(2312861009n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(1642497n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 2 (2312861005, 2312861009)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2312861005n);
    input.add32(2312861009n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(2312860993n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 3 (2312861009, 2312861009)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2312861009n);
    input.add32(2312861009n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(2312861009n);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 4 (2312861009, 2312861005)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2312861009n);
    input.add32(2312861005n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(2312860993n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577374322978195673, 2490018786)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577374322978195673n);
    input.add32(2490018786n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577374323247328251n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 2 (2490018782, 2490018786)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2490018782n);
    input.add32(2490018786n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(2490018814n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 3 (2490018786, 2490018786)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2490018786n);
    input.add32(2490018786n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(2490018786n);
  });

  it('test operator "or" overload (euint256, euint32) => euint256 test 4 (2490018786, 2490018782)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2490018786n);
    input.add32(2490018782n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(2490018814n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575870021368542809, 1137098251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575870021368542809n);
    input.add32(1137098251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575870020349413458n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 2 (1137098247, 1137098251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1137098247n);
    input.add32(1137098251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 3 (1137098251, 1137098251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1137098251n);
    input.add32(1137098251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint32) => euint256 test 4 (1137098251, 1137098247)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1137098251n);
    input.add32(1137098247n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580556825298662935, 2172668277)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580556825298662935n);
    input.add32(2172668277n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 2 (2172668273, 2172668277)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2172668273n);
    input.add32(2172668277n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 3 (2172668277, 2172668277)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2172668277n);
    input.add32(2172668277n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint32) => ebool test 4 (2172668277, 2172668273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2172668277n);
    input.add32(2172668273n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583230004924210357, 3013060698)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583230004924210357n);
    input.add32(3013060698n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 2 (3013060694, 3013060698)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(3013060694n);
    input.add32(3013060698n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 3 (3013060698, 3013060698)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(3013060698n);
    input.add32(3013060698n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint32) => ebool test 4 (3013060698, 3013060694)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(3013060698n);
    input.add32(3013060694n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576727756037416977, 2466446241)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576727756037416977n);
    input.add32(2466446241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint32) => ebool test 2 (2466446237, 2466446241)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2466446237n);
    input.add32(2466446241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint32) => ebool test 3 (2466446241, 2466446241)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2466446241n);
    input.add32(2466446241n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint32) => ebool test 4 (2466446241, 2466446237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(2466446241n);
    input.add32(2466446237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580729313167530831, 902297828)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580729313167530831n);
    input.add32(902297828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint32) => ebool test 2 (902297824, 902297828)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(902297824n);
    input.add32(902297828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint32) => ebool test 3 (902297828, 902297828)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(902297828n);
    input.add32(902297828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint32) => ebool test 4 (902297828, 902297824)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(902297828n);
    input.add32(902297824n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582845443410851747, 1340548693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582845443410851747n);
    input.add32(1340548693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint32) => ebool test 2 (1340548689, 1340548693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1340548689n);
    input.add32(1340548693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint32) => ebool test 3 (1340548693, 1340548693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1340548693n);
    input.add32(1340548693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint32) => ebool test 4 (1340548693, 1340548689)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1340548693n);
    input.add32(1340548689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint32) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577148837567008665, 1716202338)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577148837567008665n);
    input.add32(1716202338n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint32) => ebool test 2 (1716202334, 1716202338)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1716202334n);
    input.add32(1716202338n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint32) => ebool test 3 (1716202338, 1716202338)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1716202338n);
    input.add32(1716202338n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint32) => ebool test 4 (1716202338, 1716202334)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1716202338n);
    input.add32(1716202334n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582240718630307135, 1248940496)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582240718630307135n);
    input.add32(1248940496n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(1248940496n);
  });

  it('test operator "min" overload (euint256, euint32) => euint256 test 2 (1248940492, 1248940496)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1248940492n);
    input.add32(1248940496n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(1248940492n);
  });

  it('test operator "min" overload (euint256, euint32) => euint256 test 3 (1248940496, 1248940496)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1248940496n);
    input.add32(1248940496n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(1248940496n);
  });

  it('test operator "min" overload (euint256, euint32) => euint256 test 4 (1248940496, 1248940492)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(1248940496n);
    input.add32(1248940492n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(1248940492n);
  });

  it('test operator "max" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581701635304814489, 4274177144)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581701635304814489n);
    input.add32(4274177144n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457581701635304814489n);
  });

  it('test operator "max" overload (euint256, euint32) => euint256 test 2 (4274177140, 4274177144)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(4274177140n);
    input.add32(4274177144n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(4274177144n);
  });

  it('test operator "max" overload (euint256, euint32) => euint256 test 3 (4274177144, 4274177144)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(4274177144n);
    input.add32(4274177144n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(4274177144n);
  });

  it('test operator "max" overload (euint256, euint32) => euint256 test 4 (4274177144, 4274177140)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(4274177144n);
    input.add32(4274177140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(4274177144n);
  });

  it('test operator "add" overload (euint256, euint64) => euint256 test 1 (9223372036854775809, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(9223372036854775809n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(9223372036854775811n);
  });

  it('test operator "add" overload (euint256, euint64) => euint256 test 2 (9220748041203345234, 9220748041203345236)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(9220748041203345234n);
    input.add64(9220748041203345236n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18441496082406690470n);
  });

  it('test operator "add" overload (euint256, euint64) => euint256 test 3 (9220748041203345236, 9220748041203345236)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(9220748041203345236n);
    input.add64(9220748041203345236n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18441496082406690472n);
  });

  it('test operator "add" overload (euint256, euint64) => euint256 test 4 (9220748041203345236, 9220748041203345234)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(9220748041203345236n);
    input.add64(9220748041203345234n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18441496082406690470n);
  });

  it('test operator "sub" overload (euint256, euint64) => euint256 test 1 (18445296154698553701, 18445296154698553701)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18445296154698553701n);
    input.add64(18445296154698553701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint64) => euint256 test 2 (18445296154698553701, 18445296154698553697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18445296154698553701n);
    input.add64(18445296154698553697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint64) => euint256 test 1 (4611686018427387905, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(4611686018427387905n);
    input.add64(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(9223372036854775810n);
  });

  it('test operator "mul" overload (euint256, euint64) => euint256 test 2 (4292912378, 4292912378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(4292912378n);
    input.add64(4292912378n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18429096685185614884n);
  });

  it('test operator "mul" overload (euint256, euint64) => euint256 test 3 (4292912378, 4292912378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(4292912378n);
    input.add64(4292912378n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18429096685185614884n);
  });

  it('test operator "mul" overload (euint256, euint64) => euint256 test 4 (4292912378, 4292912378)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(4292912378n);
    input.add64(4292912378n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18429096685185614884n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578396814970518645, 18439393076952901085)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578396814970518645n);
    input.add64(18439393076952901085n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18438862826454918229n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 2 (18439393076952901081, 18439393076952901085)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18439393076952901081n);
    input.add64(18439393076952901085n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18439393076952901081n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 3 (18439393076952901085, 18439393076952901085)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18439393076952901085n);
    input.add64(18439393076952901085n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18439393076952901085n);
  });

  it('test operator "and" overload (euint256, euint64) => euint256 test 4 (18439393076952901085, 18439393076952901081)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18439393076952901085n);
    input.add64(18439393076952901081n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18439393076952901081n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575138878497255467, 18441296418739792919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575138878497255467n);
    input.add64(18441296418739792919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578657599316786239n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 2 (18441296418739792915, 18441296418739792919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18441296418739792915n);
    input.add64(18441296418739792919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18441296418739792919n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 3 (18441296418739792919, 18441296418739792919)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18441296418739792919n);
    input.add64(18441296418739792919n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18441296418739792919n);
  });

  it('test operator "or" overload (euint256, euint64) => euint256 test 4 (18441296418739792919, 18441296418739792915)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18441296418739792919n);
    input.add64(18441296418739792915n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18441296418739792919n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581449682348833995, 18444753228810258299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581449682348833995n);
    input.add64(18444753228810258299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039439141209004043664304n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 2 (18444753228810258295, 18444753228810258299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18444753228810258295n);
    input.add64(18444753228810258299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 3 (18444753228810258299, 18444753228810258299)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18444753228810258299n);
    input.add64(18444753228810258299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint64) => euint256 test 4 (18444753228810258299, 18444753228810258295)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18444753228810258299n);
    input.add64(18444753228810258295n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(12n);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579655327337947495, 18438962761364358079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579655327337947495n);
    input.add64(18438962761364358079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 2 (18438962761364358075, 18438962761364358079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18438962761364358075n);
    input.add64(18438962761364358079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 3 (18438962761364358079, 18438962761364358079)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18438962761364358079n);
    input.add64(18438962761364358079n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint64) => ebool test 4 (18438962761364358079, 18438962761364358075)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18438962761364358079n);
    input.add64(18438962761364358075n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457578997655721966093, 18437968835725654059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578997655721966093n);
    input.add64(18437968835725654059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 2 (18437968835725654055, 18437968835725654059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18437968835725654055n);
    input.add64(18437968835725654059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 3 (18437968835725654059, 18437968835725654059)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18437968835725654059n);
    input.add64(18437968835725654059n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint64) => ebool test 4 (18437968835725654059, 18437968835725654055)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18437968835725654059n);
    input.add64(18437968835725654055n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577778171434629241, 18443676400252575763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577778171434629241n);
    input.add64(18443676400252575763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint64) => ebool test 2 (18443676400252575759, 18443676400252575763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18443676400252575759n);
    input.add64(18443676400252575763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint64) => ebool test 3 (18443676400252575763, 18443676400252575763)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18443676400252575763n);
    input.add64(18443676400252575763n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint64) => ebool test 4 (18443676400252575763, 18443676400252575759)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18443676400252575763n);
    input.add64(18443676400252575759n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582386276584123553, 18443192077233247397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582386276584123553n);
    input.add64(18443192077233247397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint64) => ebool test 2 (18443192077233247393, 18443192077233247397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18443192077233247393n);
    input.add64(18443192077233247397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint64) => ebool test 3 (18443192077233247397, 18443192077233247397)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18443192077233247397n);
    input.add64(18443192077233247397n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint64) => ebool test 4 (18443192077233247397, 18443192077233247393)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18443192077233247397n);
    input.add64(18443192077233247393n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576194967854445761, 18441295092771573001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576194967854445761n);
    input.add64(18441295092771573001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint64) => ebool test 2 (18441295092771572997, 18441295092771573001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18441295092771572997n);
    input.add64(18441295092771573001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint64) => ebool test 3 (18441295092771573001, 18441295092771573001)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18441295092771573001n);
    input.add64(18441295092771573001n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint64) => ebool test 4 (18441295092771573001, 18441295092771572997)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18441295092771573001n);
    input.add64(18441295092771572997n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint64) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457577348858503949249, 18446356831740760619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577348858503949249n);
    input.add64(18446356831740760619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint64) => ebool test 2 (18446356831740760615, 18446356831740760619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18446356831740760615n);
    input.add64(18446356831740760619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint64) => ebool test 3 (18446356831740760619, 18446356831740760619)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18446356831740760619n);
    input.add64(18446356831740760619n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint64) => ebool test 4 (18446356831740760619, 18446356831740760615)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18446356831740760619n);
    input.add64(18446356831740760615n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579813847565218665, 18443636668516069273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579813847565218665n);
    input.add64(18443636668516069273n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18443636668516069273n);
  });

  it('test operator "min" overload (euint256, euint64) => euint256 test 2 (18443636668516069269, 18443636668516069273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18443636668516069269n);
    input.add64(18443636668516069273n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18443636668516069269n);
  });

  it('test operator "min" overload (euint256, euint64) => euint256 test 3 (18443636668516069273, 18443636668516069273)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18443636668516069273n);
    input.add64(18443636668516069273n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18443636668516069273n);
  });

  it('test operator "min" overload (euint256, euint64) => euint256 test 4 (18443636668516069273, 18443636668516069269)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18443636668516069273n);
    input.add64(18443636668516069269n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18443636668516069269n);
  });

  it('test operator "max" overload (euint256, euint64) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583257576614099953, 18438250957457099341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583257576614099953n);
    input.add64(18438250957457099341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583257576614099953n);
  });

  it('test operator "max" overload (euint256, euint64) => euint256 test 2 (18438250957457099337, 18438250957457099341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18438250957457099337n);
    input.add64(18438250957457099341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18438250957457099341n);
  });

  it('test operator "max" overload (euint256, euint64) => euint256 test 3 (18438250957457099341, 18438250957457099341)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18438250957457099341n);
    input.add64(18438250957457099341n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18438250957457099341n);
  });

  it('test operator "max" overload (euint256, euint64) => euint256 test 4 (18438250957457099341, 18438250957457099337)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(18438250957457099341n);
    input.add64(18438250957457099337n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(18438250957457099341n);
  });

  it('test operator "add" overload (euint256, euint128) => euint256 test 1 (170141183460469231731687303715884105729, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);
    input.add128(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(170141183460469231731687303715884105731n);
  });

  it('test operator "add" overload (euint256, euint128) => euint256 test 2 (170141183460469231731684278054029333276, 170141183460469231731684278054029333278)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731684278054029333276n);
    input.add128(170141183460469231731684278054029333278n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463368556108058666554n);
  });

  it('test operator "add" overload (euint256, euint128) => euint256 test 3 (170141183460469231731684278054029333278, 170141183460469231731684278054029333278)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731684278054029333278n);
    input.add128(170141183460469231731684278054029333278n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463368556108058666556n);
  });

  it('test operator "add" overload (euint256, euint128) => euint256 test 4 (170141183460469231731684278054029333278, 170141183460469231731684278054029333276)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731684278054029333278n);
    input.add128(170141183460469231731684278054029333276n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463368556108058666554n);
  });

  it('test operator "sub" overload (euint256, euint128) => euint256 test 1 (340282366920938463463373776145256730539, 340282366920938463463373776145256730539)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373776145256730539n);
    input.add128(340282366920938463463373776145256730539n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint128) => euint256 test 2 (340282366920938463463373776145256730539, 340282366920938463463373776145256730535)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373776145256730539n);
    input.add128(340282366920938463463373776145256730535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint128) => euint256 test 1 (85070591730234615865843651857942052865, 2)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(85070591730234615865843651857942052865n);
    input.add128(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(170141183460469231731687303715884105730n);
  });

  it('test operator "mul" overload (euint256, euint128) => euint256 test 2 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint256, euint128) => euint256 test 3 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "mul" overload (euint256, euint128) => euint256 test 4 (9223372036854775809, 9223372036854775809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(9223372036854775809n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(85070591730234615884290395931651604481n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582929951159672455, 340282366920938463463365847322826293787)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582929951159672455n);
    input.add128(340282366920938463463365847322826293787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463365636070292242947n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 2 (340282366920938463463365847322826293783, 340282366920938463463365847322826293787)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463365847322826293783n);
    input.add128(340282366920938463463365847322826293787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463365847322826293779n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 3 (340282366920938463463365847322826293787, 340282366920938463463365847322826293787)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463365847322826293787n);
    input.add128(340282366920938463463365847322826293787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463365847322826293787n);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 4 (340282366920938463463365847322826293787, 340282366920938463463365847322826293783)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463365847322826293787n);
    input.add128(340282366920938463463365847322826293783n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463365847322826293779n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457581776378743047187, 340282366920938463463373551062299472959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581776378743047187n);
    input.add128(340282366920938463463373551062299472959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457582951827808547903n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 2 (340282366920938463463373551062299472955, 340282366920938463463373551062299472959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373551062299472955n);
    input.add128(340282366920938463463373551062299472959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463373551062299472959n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 3 (340282366920938463463373551062299472959, 340282366920938463463373551062299472959)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373551062299472959n);
    input.add128(340282366920938463463373551062299472959n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463373551062299472959n);
  });

  it('test operator "or" overload (euint256, euint128) => euint256 test 4 (340282366920938463463373551062299472959, 340282366920938463463373551062299472955)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373551062299472959n);
    input.add128(340282366920938463463373551062299472955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463373551062299472959n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582608158223098943, 340282366920938463463373070389452427715)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582608158223098943n);
    input.add128(340282366920938463463373070389452427715n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907852929702298719625575994209836984672105980n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 2 (340282366920938463463373070389452427711, 340282366920938463463373070389452427715)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373070389452427711n);
    input.add128(340282366920938463463373070389452427715n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 3 (340282366920938463463373070389452427715, 340282366920938463463373070389452427715)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373070389452427715n);
    input.add128(340282366920938463463373070389452427715n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint128) => euint256 test 4 (340282366920938463463373070389452427715, 340282366920938463463373070389452427711)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373070389452427715n);
    input.add128(340282366920938463463373070389452427711n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(124n);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583147947133304483, 340282366920938463463373080930672512019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583147947133304483n);
    input.add128(340282366920938463463373080930672512019n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 2 (340282366920938463463373080930672512015, 340282366920938463463373080930672512019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373080930672512015n);
    input.add128(340282366920938463463373080930672512019n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 3 (340282366920938463463373080930672512019, 340282366920938463463373080930672512019)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373080930672512019n);
    input.add128(340282366920938463463373080930672512019n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint128) => ebool test 4 (340282366920938463463373080930672512019, 340282366920938463463373080930672512015)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373080930672512019n);
    input.add128(340282366920938463463373080930672512015n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457576673614745401783, 340282366920938463463373230674573043227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576673614745401783n);
    input.add128(340282366920938463463373230674573043227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 2 (340282366920938463463373230674573043223, 340282366920938463463373230674573043227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373230674573043223n);
    input.add128(340282366920938463463373230674573043227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 3 (340282366920938463463373230674573043227, 340282366920938463463373230674573043227)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373230674573043227n);
    input.add128(340282366920938463463373230674573043227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint128) => ebool test 4 (340282366920938463463373230674573043227, 340282366920938463463373230674573043223)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463373230674573043227n);
    input.add128(340282366920938463463373230674573043223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581781091933664799, 340282366920938463463369397462507033255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581781091933664799n);
    input.add128(340282366920938463463369397462507033255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint128) => ebool test 2 (340282366920938463463369397462507033251, 340282366920938463463369397462507033255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463369397462507033251n);
    input.add128(340282366920938463463369397462507033255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint128) => ebool test 3 (340282366920938463463369397462507033255, 340282366920938463463369397462507033255)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463369397462507033255n);
    input.add128(340282366920938463463369397462507033255n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint128) => ebool test 4 (340282366920938463463369397462507033255, 340282366920938463463369397462507033251)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463369397462507033255n);
    input.add128(340282366920938463463369397462507033251n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583305927195076375, 340282366920938463463370153073900549113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583305927195076375n);
    input.add128(340282366920938463463370153073900549113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint128) => ebool test 2 (340282366920938463463370153073900549109, 340282366920938463463370153073900549113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463370153073900549109n);
    input.add128(340282366920938463463370153073900549113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint128) => ebool test 3 (340282366920938463463370153073900549113, 340282366920938463463370153073900549113)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463370153073900549113n);
    input.add128(340282366920938463463370153073900549113n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint128) => ebool test 4 (340282366920938463463370153073900549113, 340282366920938463463370153073900549109)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463370153073900549113n);
    input.add128(340282366920938463463370153073900549109n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580916980779426235, 340282366920938463463367434270493244717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580916980779426235n);
    input.add128(340282366920938463463367434270493244717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint128) => ebool test 2 (340282366920938463463367434270493244713, 340282366920938463463367434270493244717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463367434270493244713n);
    input.add128(340282366920938463463367434270493244717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint128) => ebool test 3 (340282366920938463463367434270493244717, 340282366920938463463367434270493244717)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463367434270493244717n);
    input.add128(340282366920938463463367434270493244717n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint128) => ebool test 4 (340282366920938463463367434270493244717, 340282366920938463463367434270493244713)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463367434270493244717n);
    input.add128(340282366920938463463367434270493244713n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint128) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457582200169426284819, 340282366920938463463367486483424293599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582200169426284819n);
    input.add128(340282366920938463463367486483424293599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint128) => ebool test 2 (340282366920938463463367486483424293595, 340282366920938463463367486483424293599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463367486483424293595n);
    input.add128(340282366920938463463367486483424293599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint128) => ebool test 3 (340282366920938463463367486483424293599, 340282366920938463463367486483424293599)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463367486483424293599n);
    input.add128(340282366920938463463367486483424293599n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint128) => ebool test 4 (340282366920938463463367486483424293599, 340282366920938463463367486483424293595)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463367486483424293599n);
    input.add128(340282366920938463463367486483424293595n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578748486524440321, 340282366920938463463370032475222092647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578748486524440321n);
    input.add128(340282366920938463463370032475222092647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463370032475222092647n);
  });

  it('test operator "min" overload (euint256, euint128) => euint256 test 2 (340282366920938463463370032475222092643, 340282366920938463463370032475222092647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463370032475222092643n);
    input.add128(340282366920938463463370032475222092647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463370032475222092643n);
  });

  it('test operator "min" overload (euint256, euint128) => euint256 test 3 (340282366920938463463370032475222092647, 340282366920938463463370032475222092647)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463370032475222092647n);
    input.add128(340282366920938463463370032475222092647n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463370032475222092647n);
  });

  it('test operator "min" overload (euint256, euint128) => euint256 test 4 (340282366920938463463370032475222092647, 340282366920938463463370032475222092643)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463370032475222092647n);
    input.add128(340282366920938463463370032475222092643n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463370032475222092643n);
  });

  it('test operator "max" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580295384680552043, 340282366920938463463368511201134180813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580295384680552043n);
    input.add128(340282366920938463463368511201134180813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580295384680552043n);
  });

  it('test operator "max" overload (euint256, euint128) => euint256 test 2 (340282366920938463463368511201134180809, 340282366920938463463368511201134180813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463368511201134180809n);
    input.add128(340282366920938463463368511201134180813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463368511201134180813n);
  });

  it('test operator "max" overload (euint256, euint128) => euint256 test 3 (340282366920938463463368511201134180813, 340282366920938463463368511201134180813)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463368511201134180813n);
    input.add128(340282366920938463463368511201134180813n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463368511201134180813n);
  });

  it('test operator "max" overload (euint256, euint128) => euint256 test 4 (340282366920938463463368511201134180813, 340282366920938463463368511201134180809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(340282366920938463463368511201134180813n);
    input.add128(340282366920938463463368511201134180809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(340282366920938463463368511201134180813n);
  });

  it('test operator "add" overload (euint256, euint256) => euint256 test 1 (57896044618658097711785492504343953926634992332820282019728789013802824080146, 57896044618658097711785492504343953926634992332820282019728788109485910742180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728789013802824080146n);
    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577123288734822326n);
  });

  it('test operator "add" overload (euint256, euint256) => euint256 test 2 (57896044618658097711785492504343953926634992332820282019728788109485910742178, 57896044618658097711785492504343953926634992332820282019728788109485910742180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742178n);
    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576218971821484358n);
  });

  it('test operator "add" overload (euint256, euint256) => euint256 test 3 (57896044618658097711785492504343953926634992332820282019728788109485910742180, 57896044618658097711785492504343953926634992332820282019728788109485910742180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742180n);
    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576218971821484360n);
  });

  it('test operator "add" overload (euint256, euint256) => euint256 test 4 (57896044618658097711785492504343953926634992332820282019728788109485910742180, 57896044618658097711785492504343953926634992332820282019728788109485910742178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742180n);
    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576218971821484358n);
  });

  it('test operator "sub" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579667871013646565, 115792089237316195423570985008687907853269984665640564039457579667871013646565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579667871013646565n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579667871013646565n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579667871013646565, 115792089237316195423570985008687907853269984665640564039457579667871013646561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579667871013646565n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579667871013646561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, euint256) => euint256 test 1 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);
    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, euint256) => euint256 test 2 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);
    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, euint256) => euint256 test 3 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);
    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, euint256) => euint256 test 4 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);
    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580473854578773963, 115792089237316195423570985008687907853269984665640564039457579064265044491325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580473854578773963n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575671150140541961n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579064265044491321, 115792089237316195423570985008687907853269984665640564039457579064265044491325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491321n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579064265044491321n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457579064265044491325, 115792089237316195423570985008687907853269984665640564039457579064265044491325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491325n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579064265044491325n);
  });

  it('test operator "and" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457579064265044491325, 115792089237316195423570985008687907853269984665640564039457579064265044491321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491325n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579064265044491321n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579093390716184697, 115792089237316195423570985008687907853269984665640564039457580393822758166581)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184697n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580393822758166581n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457583914753535699069n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579093390716184693, 115792089237316195423570985008687907853269984665640564039457579093390716184697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184693n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579093390716184701n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457579093390716184697, 115792089237316195423570985008687907853269984665640564039457579093390716184697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184697n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579093390716184697n);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457579093390716184697, 115792089237316195423570985008687907853269984665640564039457579093390716184693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184697n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579093390716184701n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582495469225451093, 115792089237316195423570985008687907853269984665640564039457581801761034193859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582495469225451093n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(714633272808854n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457581801761034193855, 115792089237316195423570985008687907853269984665640564039457581801761034193859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193855n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457581801761034193859, 115792089237316195423570985008687907853269984665640564039457581801761034193859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193859n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457581801761034193859, 115792089237316195423570985008687907853269984665640564039457581801761034193855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193859n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(124n);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579028937132718443, 115792089237316195423570985008687907853269984665640564039457575281612927984431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579028937132718443n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575281612927984427, 115792089237316195423570985008687907853269984665640564039457575281612927984431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984427n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575281612927984431, 115792089237316195423570985008687907853269984665640564039457575281612927984431)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984431n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984431n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "eq" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575281612927984431, 115792089237316195423570985008687907853269984665640564039457575281612927984427)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984431n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575281612927984427n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.eq_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457581421114730215071, 115792089237316195423570985008687907853269984665640564039457577745710288733723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581421114730215071n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457577745710288733719, 115792089237316195423570985008687907853269984665640564039457577745710288733723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733719n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457577745710288733723, 115792089237316195423570985008687907853269984665640564039457577745710288733723)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733723n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733723n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ne" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457577745710288733723, 115792089237316195423570985008687907853269984665640564039457577745710288733719)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733723n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577745710288733719n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ne_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579759156646767465, 115792089237316195423570985008687907853269984665640564039457581776880742255263)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767465n);
    input.add256(115792089237316195423570985008687907853269984665640564039457581776880742255263n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457579759156646767461, 115792089237316195423570985008687907853269984665640564039457579759156646767465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767461n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "ge" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457579759156646767465, 115792089237316195423570985008687907853269984665640564039457579759156646767465)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767465n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767465n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "ge" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457579759156646767465, 115792089237316195423570985008687907853269984665640564039457579759156646767461)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767465n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579759156646767461n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.ge_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "gt" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580885855005541993, 115792089237316195423570985008687907853269984665640564039457582339622165148199)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541993n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582339622165148199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457580885855005541989, 115792089237316195423570985008687907853269984665640564039457580885855005541993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541989n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457580885855005541993, 115792089237316195423570985008687907853269984665640564039457580885855005541993)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541993n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "gt" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457580885855005541993, 115792089237316195423570985008687907853269984665640564039457580885855005541989)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541993n);
    input.add256(115792089237316195423570985008687907853269984665640564039457580885855005541989n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.gt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457583300217912499775, 115792089237316195423570985008687907853269984665640564039457582857952818399809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583300217912499775n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "le" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457582857952818399805, 115792089237316195423570985008687907853269984665640564039457582857952818399809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399805n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457582857952818399809, 115792089237316195423570985008687907853269984665640564039457582857952818399809)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399809n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "le" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457582857952818399809, 115792089237316195423570985008687907853269984665640564039457582857952818399805)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399809n);
    input.add256(115792089237316195423570985008687907853269984665640564039457582857952818399805n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.le_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457579345439988102233, 115792089237316195423570985008687907853269984665640564039457575518813902238261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579345439988102233n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457575518813902238257, 115792089237316195423570985008687907853269984665640564039457575518813902238261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238257n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(true);
  });

  it('test operator "lt" overload (euint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457575518813902238261, 115792089237316195423570985008687907853269984665640564039457575518813902238261)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238261n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "lt" overload (euint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457575518813902238261, 115792089237316195423570985008687907853269984665640564039457575518813902238257)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238261n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575518813902238257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.lt_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decryptBool(await this.contract10.resb());
    expect(res).to.equal(false);
  });

  it('test operator "min" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579091097262441647, 115792089237316195423570985008687907853269984665640564039457577447088911778291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579091097262441647n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577447088911778291n);
  });

  it('test operator "min" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577447088911778287, 115792089237316195423570985008687907853269984665640564039457577447088911778291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778287n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577447088911778287n);
  });

  it('test operator "min" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577447088911778291, 115792089237316195423570985008687907853269984665640564039457577447088911778291)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778291n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778291n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577447088911778291n);
  });

  it('test operator "min" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577447088911778291, 115792089237316195423570985008687907853269984665640564039457577447088911778287)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778291n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577447088911778287n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.min_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577447088911778287n);
  });

  it('test operator "max" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457577915867722108039, 115792089237316195423570985008687907853269984665640564039457577809030115355409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577915867722108039n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577915867722108039n);
  });

  it('test operator "max" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577809030115355405, 115792089237316195423570985008687907853269984665640564039457577809030115355409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355405n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
  });

  it('test operator "max" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577809030115355409, 115792089237316195423570985008687907853269984665640564039457577809030115355409)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
  });

  it('test operator "max" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577809030115355409, 115792089237316195423570985008687907853269984665640564039457577809030115355405)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577809030115355405n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.max_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577809030115355409n);
  });

  it('test operator "add" overload (euint256, uint256) => euint256 test 1 (57896044618658097711785492504343953926634992332820282019728789013802824080146, 57896044618658097711785492504343953926634992332820282019728788733583887971237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728789013802824080146n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_uint256(
      encryptedAmount.handles[0],
      57896044618658097711785492504343953926634992332820282019728788733583887971237n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577747386712051383n);
  });

  it('test operator "add" overload (euint256, uint256) => euint256 test 2 (57896044618658097711785492504343953926634992332820282019728788109485910742178, 57896044618658097711785492504343953926634992332820282019728788109485910742180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742178n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_uint256(
      encryptedAmount.handles[0],
      57896044618658097711785492504343953926634992332820282019728788109485910742180n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576218971821484358n);
  });

  it('test operator "add" overload (euint256, uint256) => euint256 test 3 (57896044618658097711785492504343953926634992332820282019728788109485910742180, 57896044618658097711785492504343953926634992332820282019728788109485910742180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742180n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_uint256(
      encryptedAmount.handles[0],
      57896044618658097711785492504343953926634992332820282019728788109485910742180n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576218971821484360n);
  });

  it('test operator "add" overload (euint256, uint256) => euint256 test 4 (57896044618658097711785492504343953926634992332820282019728788109485910742180, 57896044618658097711785492504343953926634992332820282019728788109485910742178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742180n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_euint256_uint256(
      encryptedAmount.handles[0],
      57896044618658097711785492504343953926634992332820282019728788109485910742178n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576218971821484358n);
  });

  it('test operator "add" overload (uint256, euint256) => euint256 test 1 (57896044618658097711785492504343953926634992332820282019728789372175656948236, 57896044618658097711785492504343953926634992332820282019728788733583887971237)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(57896044618658097711785492504343953926634992332820282019728788733583887971237n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_uint256_euint256(
      57896044618658097711785492504343953926634992332820282019728789372175656948236n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578105759544919473n);
  });

  it('test operator "add" overload (uint256, euint256) => euint256 test 2 (57896044618658097711785492504343953926634992332820282019728788109485910742178, 57896044618658097711785492504343953926634992332820282019728788109485910742180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_uint256_euint256(
      57896044618658097711785492504343953926634992332820282019728788109485910742178n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576218971821484358n);
  });

  it('test operator "add" overload (uint256, euint256) => euint256 test 3 (57896044618658097711785492504343953926634992332820282019728788109485910742180, 57896044618658097711785492504343953926634992332820282019728788109485910742180)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_uint256_euint256(
      57896044618658097711785492504343953926634992332820282019728788109485910742180n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576218971821484360n);
  });

  it('test operator "add" overload (uint256, euint256) => euint256 test 4 (57896044618658097711785492504343953926634992332820282019728788109485910742180, 57896044618658097711785492504343953926634992332820282019728788109485910742178)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(57896044618658097711785492504343953926634992332820282019728788109485910742178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.add_uint256_euint256(
      57896044618658097711785492504343953926634992332820282019728788109485910742180n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457576218971821484358n);
  });

  it('test operator "sub" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579667871013646565, 115792089237316195423570985008687907853269984665640564039457579667871013646565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579667871013646565n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579667871013646565n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579667871013646565, 115792089237316195423570985008687907853269984665640564039457579667871013646561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579667871013646565n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579667871013646561n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "sub" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579667871013646565, 115792089237316195423570985008687907853269984665640564039457579667871013646565)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579667871013646565n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579667871013646565n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "sub" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579667871013646565, 115792089237316195423570985008687907853269984665640564039457579667871013646561)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579667871013646561n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.sub_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579667871013646565n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "mul" overload (euint256, uint256) => euint256 test 1 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_uint256(
      encryptedAmount.handles[0],
      170141183460469231731687303715884105729n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, uint256) => euint256 test 2 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_uint256(
      encryptedAmount.handles[0],
      170141183460469231731687303715884105729n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, uint256) => euint256 test 3 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_uint256(
      encryptedAmount.handles[0],
      170141183460469231731687303715884105729n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (euint256, uint256) => euint256 test 4 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(170141183460469231731687303715884105729n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_euint256_uint256(
      encryptedAmount.handles[0],
      170141183460469231731687303715884105729n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (uint256, euint256) => euint256 test 1 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_uint256_euint256(
      170141183460469231731687303715884105729n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (uint256, euint256) => euint256 test 2 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_uint256_euint256(
      170141183460469231731687303715884105729n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (uint256, euint256) => euint256 test 3 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_uint256_euint256(
      170141183460469231731687303715884105729n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "mul" overload (uint256, euint256) => euint256 test 4 (170141183460469231731687303715884105729, 170141183460469231731687303715884105729)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(170141183460469231731687303715884105729n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.mul_uint256_euint256(
      170141183460469231731687303715884105729n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(28948022309329048855892746252171976963657778533331079473327770609410050621441n);
  });

  it('test operator "div" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457583310417243027629, 115792089237316195423570985008687907853269984665640564039457577919950085050983)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457583310417243027629n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.div_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577919950085050983n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457576535097376895415, 115792089237316195423570985008687907853269984665640564039457576535097376895419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576535097376895415n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.div_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576535097376895419n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "div" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457576535097376895419, 115792089237316195423570985008687907853269984665640564039457576535097376895419)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576535097376895419n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.div_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576535097376895419n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(1n);
  });

  it('test operator "div" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457576535097376895419, 115792089237316195423570985008687907853269984665640564039457576535097376895415)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457576535097376895419n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.div_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457576535097376895415n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(1n);
  });

  it('test operator "rem" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578502975350831453, 115792089237316195423570985008687907853269984665640564039457581127377285906497)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578502975350831453n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rem_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581127377285906497n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457578502975350831453n);
  });

  it('test operator "rem" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457577137781640958323, 115792089237316195423570985008687907853269984665640564039457577137781640958327)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577137781640958323n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rem_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577137781640958327n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457577137781640958323n);
  });

  it('test operator "rem" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457577137781640958327, 115792089237316195423570985008687907853269984665640564039457577137781640958327)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577137781640958327n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rem_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577137781640958327n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "rem" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457577137781640958327, 115792089237316195423570985008687907853269984665640564039457577137781640958323)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457577137781640958327n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.rem_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457577137781640958323n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(4n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580473854578773963, 115792089237316195423570985008687907853269984665640564039457581816489991853517)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457580473854578773963n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581816489991853517n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579557951374476745n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579064265044491321, 115792089237316195423570985008687907853269984665640564039457579064265044491325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491321n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579064265044491325n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579064265044491321n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457579064265044491325, 115792089237316195423570985008687907853269984665640564039457579064265044491325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491325n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579064265044491325n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579064265044491325n);
  });

  it('test operator "and" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457579064265044491325, 115792089237316195423570985008687907853269984665640564039457579064265044491321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491325n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579064265044491321n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579064265044491321n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457576516611416206839, 115792089237316195423570985008687907853269984665640564039457581816489991853517)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581816489991853517n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457576516611416206839n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457575038788798842309n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579064265044491321, 115792089237316195423570985008687907853269984665640564039457579064265044491325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579064265044491321n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579064265044491321n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457579064265044491325, 115792089237316195423570985008687907853269984665640564039457579064265044491325)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579064265044491325n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579064265044491325n);
  });

  it('test operator "and" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457579064265044491325, 115792089237316195423570985008687907853269984665640564039457579064265044491321)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579064265044491321n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.and_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579064265044491325n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579064265044491321n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579093390716184697, 115792089237316195423570985008687907853269984665640564039457575617602160724601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184697n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457575617602160724601n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579147311516134009n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579093390716184693, 115792089237316195423570985008687907853269984665640564039457579093390716184697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184693n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579093390716184697n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579093390716184701n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457579093390716184697, 115792089237316195423570985008687907853269984665640564039457579093390716184697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184697n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579093390716184697n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579093390716184697n);
  });

  it('test operator "or" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457579093390716184697, 115792089237316195423570985008687907853269984665640564039457579093390716184693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184697n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579093390716184693n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579093390716184701n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457580060042571469423, 115792089237316195423570985008687907853269984665640564039457575617602160724601)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457575617602160724601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580060042571469423n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457580622993567439487n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457579093390716184693, 115792089237316195423570985008687907853269984665640564039457579093390716184697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579093390716184693n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579093390716184701n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457579093390716184697, 115792089237316195423570985008687907853269984665640564039457579093390716184697)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184697n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579093390716184697n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579093390716184697n);
  });

  it('test operator "or" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457579093390716184697, 115792089237316195423570985008687907853269984665640564039457579093390716184693)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579093390716184693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.or_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579093390716184697n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(115792089237316195423570985008687907853269984665640564039457579093390716184701n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457582495469225451093, 115792089237316195423570985008687907853269984665640564039457579143464719512889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457582495469225451093n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457579143464719512889n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(5655612635835244n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457581801761034193855, 115792089237316195423570985008687907853269984665640564039457581801761034193859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193855n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581801761034193859n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457581801761034193859, 115792089237316195423570985008687907853269984665640564039457581801761034193859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193859n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581801761034193859n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (euint256, uint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457581801761034193859, 115792089237316195423570985008687907853269984665640564039457581801761034193855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193859n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_euint256_uint256(
      encryptedAmount.handles[0],
      115792089237316195423570985008687907853269984665640564039457581801761034193855n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457579014577469369511, 115792089237316195423570985008687907853269984665640564039457579143464719512889)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457579143464719512889n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457579014577469369511n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(269763301726622n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457581801761034193855, 115792089237316195423570985008687907853269984665640564039457581801761034193859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457581801761034193855n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(124n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457581801761034193859, 115792089237316195423570985008687907853269984665640564039457581801761034193859)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457581801761034193859n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(0n);
  });

  it('test operator "xor" overload (uint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457581801761034193859, 115792089237316195423570985008687907853269984665640564039457581801761034193855)', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contract10Address, this.signers.alice.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457581801761034193855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract10.xor_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457581801761034193859n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const res = await decrypt256(await this.contract10.res256());
    expect(res).to.equal(124n);
  });
});
