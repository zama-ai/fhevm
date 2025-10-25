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

describe('FHEVM operations 82', function () {
  before(async function () {
    this.signer = await getSigner(82);

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

  it('test operator "gt" overload (uint32, euint32) => ebool test 1 (3528012460, 3170827740)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(3170827740n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      3528012460n,
      encryptedAmount.handles[0],
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

  it('test operator "gt" overload (uint32, euint32) => ebool test 2 (953438493, 953438497)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(953438497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      953438493n,
      encryptedAmount.handles[0],
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

  it('test operator "gt" overload (uint32, euint32) => ebool test 3 (953438497, 953438497)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(953438497n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      953438497n,
      encryptedAmount.handles[0],
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

  it('test operator "gt" overload (uint32, euint32) => ebool test 4 (953438497, 953438493)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(953438493n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.gt_uint32_euint32(
      953438497n,
      encryptedAmount.handles[0],
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

  it('test operator "le" overload (euint32, uint32) => ebool test 1 (2247755504, 2968171169)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(2247755504n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      2968171169n,
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

  it('test operator "le" overload (euint32, uint32) => ebool test 2 (1603469839, 1603469843)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(1603469839n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      1603469843n,
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

  it('test operator "le" overload (euint32, uint32) => ebool test 3 (1603469843, 1603469843)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(1603469843n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      1603469843n,
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

  it('test operator "le" overload (euint32, uint32) => ebool test 4 (1603469843, 1603469839)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(1603469843n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_euint32_uint32(
      encryptedAmount.handles[0],
      1603469839n,
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

  it('test operator "le" overload (uint32, euint32) => ebool test 1 (955768418, 2968171169)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(2968171169n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      955768418n,
      encryptedAmount.handles[0],
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

  it('test operator "le" overload (uint32, euint32) => ebool test 2 (1603469839, 1603469843)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(1603469843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      1603469839n,
      encryptedAmount.handles[0],
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

  it('test operator "le" overload (uint32, euint32) => ebool test 3 (1603469843, 1603469843)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(1603469843n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      1603469843n,
      encryptedAmount.handles[0],
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

  it('test operator "le" overload (uint32, euint32) => ebool test 4 (1603469843, 1603469839)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(1603469839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.le_uint32_euint32(
      1603469843n,
      encryptedAmount.handles[0],
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

  it('test operator "lt" overload (euint32, uint32) => ebool test 1 (1993066230, 4102504073)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(1993066230n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      4102504073n,
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

  it('test operator "lt" overload (euint32, uint32) => ebool test 2 (1993066226, 1993066230)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(1993066226n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      1993066230n,
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

  it('test operator "lt" overload (euint32, uint32) => ebool test 3 (1993066230, 1993066230)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(1993066230n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      1993066230n,
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

  it('test operator "lt" overload (euint32, uint32) => ebool test 4 (1993066230, 1993066226)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(1993066230n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_euint32_uint32(
      encryptedAmount.handles[0],
      1993066226n,
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

  it('test operator "lt" overload (uint32, euint32) => ebool test 1 (4115873166, 4102504073)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(4102504073n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      4115873166n,
      encryptedAmount.handles[0],
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

  it('test operator "lt" overload (uint32, euint32) => ebool test 2 (1993066226, 1993066230)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(1993066230n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      1993066226n,
      encryptedAmount.handles[0],
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

  it('test operator "lt" overload (uint32, euint32) => ebool test 3 (1993066230, 1993066230)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(1993066230n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      1993066230n,
      encryptedAmount.handles[0],
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

  it('test operator "lt" overload (uint32, euint32) => ebool test 4 (1993066230, 1993066226)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(1993066226n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.lt_uint32_euint32(
      1993066230n,
      encryptedAmount.handles[0],
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

  it('test operator "min" overload (euint32, uint32) => euint32 test 1 (1441663254, 422146200)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(1441663254n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      422146200n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 422146200n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 2 (122228748, 122228752)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(122228748n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      122228752n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 122228748n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 3 (122228752, 122228752)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(122228752n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      122228752n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 122228752n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, uint32) => euint32 test 4 (122228752, 122228748)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(122228752n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_euint32_uint32(
      encryptedAmount.handles[0],
      122228748n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 122228748n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
