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

describe('FHEVM operations 40', function () {
  before(async function () {
    this.signer = await getSigner(40);

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

  it('test operator "ne" overload (euint128, euint32) => ebool test 1 (340282366920938463463368210852058581855, 3867497129)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463368210852058581855n);
    input.add32(3867497129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 2 (3867497125, 3867497129)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(3867497125n);
    input.add32(3867497129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 3 (3867497129, 3867497129)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(3867497129n);
    input.add32(3867497129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint128, euint32) => ebool test 4 (3867497129, 3867497125)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(3867497129n);
    input.add32(3867497125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 1 (3041687534, 214)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(3041687534n);
    input.add8(214n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3041687550n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 2 (210, 214)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(210n);
    input.add8(214n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 214n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 3 (214, 214)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(214n);
    input.add8(214n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 214n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, euint8) => euint32 test 4 (214, 210)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add32(214n);
    input.add8(210n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 214n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 1 (157, 40)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add8(40n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_uint8_euint8(157n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 189n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 2 (79, 83)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_uint8_euint8(79n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 95n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 3 (83, 83)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add8(83n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_uint8_euint8(83n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 83n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint8, euint8) => euint8 test 4 (83, 79)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add8(79n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_uint8_euint8(83n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 95n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 1 (340282366920938463463371657968684580303, 115792089237316195423570985008687907853269984665640564039457575816303071569023)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463371657968684580303n);
    input.add256(115792089237316195423570985008687907853269984665640564039457575816303071569023n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 2 (340282366920938463463371657968684580299, 340282366920938463463371657968684580303)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463371657968684580299n);
    input.add256(340282366920938463463371657968684580303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 3 (340282366920938463463371657968684580303, 340282366920938463463371657968684580303)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463371657968684580303n);
    input.add256(340282366920938463463371657968684580303n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint128, euint256) => ebool test 4 (340282366920938463463371657968684580303, 340282366920938463463371657968684580299)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463371657968684580303n);
    input.add256(340282366920938463463371657968684580299n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_euint128_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (27229, 101)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(27229n);
    input.add8(101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 27192n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (97, 101)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(97n);
    input.add8(101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (101, 101)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(101n);
    input.add8(101n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (101, 97)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(101n);
    input.add8(97n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.xor_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 1 (18442798313793586165, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18442798313793586165n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 15564378659693747215n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 2 (2, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 576460752303423488n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 3 (6, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1729382256910270464n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 4 (6, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9223372036854775809n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
