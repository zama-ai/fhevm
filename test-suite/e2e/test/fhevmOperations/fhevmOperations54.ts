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

describe('FHEVM operations 54', function () {
  before(async function () {
    this.signer = await getSigner(54);

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

  it('test operator "min" overload (uint8, euint8) => euint8 test 1 (62, 160)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add8(160n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint8_euint8(62n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 62n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 2 (24, 28)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint8_euint8(24n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 3 (28, 28)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add8(28n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint8_euint8(28n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint8, euint8) => euint8 test 4 (28, 24)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add8(24n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_uint8_euint8(28n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 1 (2883474093, 18441614575890308501)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(2883474093n);
    input.add64(18441614575890308501n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 2 (2883474089, 2883474093)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(2883474089n);
    input.add64(2883474093n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 3 (2883474093, 2883474093)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(2883474093n);
    input.add64(2883474093n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint32, euint64) => ebool test 4 (2883474093, 2883474089)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(2883474093n);
    input.add64(2883474089n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 1 (41814, 49)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(41814n);
    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 2 (45, 49)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(45n);
    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 3 (49, 49)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(49n);
    input.add8(49n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint16, euint8) => ebool test 4 (49, 45)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(49n);
    input.add8(45n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.gt_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 1 (18445401558326212315, 340282366920938463463368019691021308423)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18445401558326212315n);
    input.add128(340282366920938463463368019691021308423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 2 (18445401558326212311, 18445401558326212315)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18445401558326212311n);
    input.add128(18445401558326212315n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 3 (18445401558326212315, 18445401558326212315)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18445401558326212315n);
    input.add128(18445401558326212315n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint64, euint128) => ebool test 4 (18445401558326212315, 18445401558326212311)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18445401558326212315n);
    input.add128(18445401558326212311n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ge_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 1 (115792089237316195423570985008687907853269984665640564039457580704963586072301, 115792089237316195423570985008687907853269984665640564039457577901737098089495)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577901737098089495n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457580704963586072301n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 2 (115792089237316195423570985008687907853269984665640564039457577752484848025849, 115792089237316195423570985008687907853269984665640564039457577752484848025853)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577752484848025853n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577752484848025849n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 3 (115792089237316195423570985008687907853269984665640564039457577752484848025853, 115792089237316195423570985008687907853269984665640564039457577752484848025853)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577752484848025853n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577752484848025853n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (uint256, euint256) => ebool test 4 (115792089237316195423570985008687907853269984665640564039457577752484848025853, 115792089237316195423570985008687907853269984665640564039457577752484848025849)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add256(115792089237316195423570985008687907853269984665640564039457577752484848025849n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_uint256_euint256(
      115792089237316195423570985008687907853269984665640564039457577752484848025853n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 1 (2, 4611686018427387905)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(2n);
    input.add128(4611686018427387905n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9223372036854775810n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 2 (4293178787, 4293178787)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(4293178787n);
    input.add128(4293178787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18431384097146791369n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 3 (4293178787, 4293178787)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(4293178787n);
    input.add128(4293178787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18431384097146791369n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint128) => euint128 test 4 (4293178787, 4293178787)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(4293178787n);
    input.add128(4293178787n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18431384097146791369n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
