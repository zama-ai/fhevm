import { assert } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMTestSuite1 } from '../../types/contracts/tests/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../types/contracts/tests/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../types/contracts/tests/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../types/contracts/tests/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../types/contracts/tests/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../types/contracts/tests/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../types/contracts/tests/FHEVMTestSuite7';
import { createInstance } from '../instance';
import { getSigner, getSigners, initSigners } from '../signers';

async function deployFHEVMTestFixture1(signer: HardhatEthersSigner): Promise<FHEVMTestSuite1> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite1');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture2(signer: HardhatEthersSigner): Promise<FHEVMTestSuite2> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite2');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture3(signer: HardhatEthersSigner): Promise<FHEVMTestSuite3> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite3');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture4(signer: HardhatEthersSigner): Promise<FHEVMTestSuite4> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture5(signer: HardhatEthersSigner): Promise<FHEVMTestSuite5> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite5');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture6(signer: HardhatEthersSigner): Promise<FHEVMTestSuite6> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite6');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

async function deployFHEVMTestFixture7(signer: HardhatEthersSigner): Promise<FHEVMTestSuite7> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMTestSuite7');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('FHEVM operations 14', function () {
  before(async function () {
    this.signer = await getSigner(14);

    const contract1 = await deployFHEVMTestFixture1(this.signer);
    this.contract1Address = await contract1.getAddress();
    this.contract1 = contract1;

    const contract2 = await deployFHEVMTestFixture2(this.signer);
    this.contract2Address = await contract2.getAddress();
    this.contract2 = contract2;

    const contract3 = await deployFHEVMTestFixture3(this.signer);
    this.contract3Address = await contract3.getAddress();
    this.contract3 = contract3;

    const contract4 = await deployFHEVMTestFixture4(this.signer);
    this.contract4Address = await contract4.getAddress();
    this.contract4 = contract4;

    const contract5 = await deployFHEVMTestFixture5(this.signer);
    this.contract5Address = await contract5.getAddress();
    this.contract5 = contract5;

    const contract6 = await deployFHEVMTestFixture6(this.signer);
    this.contract6Address = await contract6.getAddress();
    this.contract6 = contract6;

    const contract7 = await deployFHEVMTestFixture7(this.signer);
    this.contract7Address = await contract7.getAddress();
    this.contract7 = contract7;

    const instance = await createInstance();
    this.instance = instance;
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 1 (45661, 80)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(45661n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 80n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 2 (76, 80)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(76n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 64n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 3 (80, 80)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(80n);
    input.add8(80n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 80n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint8) => euint16 test 4 (80, 76)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(80n);
    input.add8(76n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 64n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 1 (43384, 227)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(43384n);
    input.add8(227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 43515n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 2 (223, 227)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(223n);
    input.add8(227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 255n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 3 (227, 227)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(227n);
    input.add8(227n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 227n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint8) => euint16 test 4 (227, 223)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(227n);
    input.add8(223n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.or_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 255n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (49382, 103)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(49382n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49281n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (99, 103)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(99n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (103, 103)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(103n);
    input.add8(103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (103, 99)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(103n);
    input.add8(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 1 (53755, 122)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(53755n);
    input.add8(122n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 2 (118, 122)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(118n);
    input.add8(122n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 3 (122, 122)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(122n);
    input.add8(122n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, euint8) => ebool test 4 (122, 118)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(122n);
    input.add8(118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.eq_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 1 (34416, 159)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(34416n);
    input.add8(159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 2 (155, 159)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(155n);
    input.add8(159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 3 (159, 159)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(159n);
    input.add8(159n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint16, euint8) => ebool test 4 (159, 155)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(159n);
    input.add8(155n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ne_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 1 (43195, 201)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(43195n);
    input.add8(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 2 (197, 201)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(197n);
    input.add8(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 3 (201, 201)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(201n);
    input.add8(201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint16, euint8) => ebool test 4 (201, 197)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(201n);
    input.add8(197n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.ge_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });
});
