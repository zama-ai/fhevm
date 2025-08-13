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

describe('FHEVM operations 76', function () {
  before(async function () {
    this.signer = await getSigner(76);

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

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (212, 42)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint8_euint8(212n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (136, 140)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add8(140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint8_euint8(136n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (140, 140)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add8(140n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint8_euint8(140n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (140, 136)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add8(136n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint8_euint8(140n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 1 (340282366920938463463366937329217102177, 18438109052585414657)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463366937329217102177n);
    input.add64(18438109052585414657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463366937329217102177n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 2 (18438109052585414653, 18438109052585414657)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(18438109052585414653n);
    input.add64(18438109052585414657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438109052585414657n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 3 (18438109052585414657, 18438109052585414657)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(18438109052585414657n);
    input.add64(18438109052585414657n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438109052585414657n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint64) => euint128 test 4 (18438109052585414657, 18438109052585414653)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(18438109052585414657n);
    input.add64(18438109052585414653n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint128_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438109052585414657n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "neg" overload (euint32) => euint32 test 1 (1859970404)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(1859970404n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.neg_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2434996892n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 1 (18442100042053626655, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18442100042053626655n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36019726644635989n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 2 (5, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 3 (9, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint64, uint8) => euint64 test 4 (9, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.shr_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 1 (97, 2690237631)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(97n);
    input.add32(2690237631n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 2 (93, 97)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(93n);
    input.add32(97n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 3 (97, 97)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(97n);
    input.add32(97n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (euint8, euint32) => ebool test 4 (97, 93)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(97n);
    input.add32(93n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint8_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 1 (89, 192)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(89n);
    input.add8(192n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 2 (85, 89)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(85n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 3 (89, 89)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(89n);
    input.add8(89n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint8) => ebool test 4 (89, 85)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(89n);
    input.add8(85n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });
});
