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

describe('FHEVM operations 75', function () {
  before(async function () {
    this.signer = await getSigner(75);

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

  it('test operator "and" overload (uint16, euint16) => euint16 test 1 (23488, 49271)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(49271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(23488n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 16448n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 2 (26114, 26118)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(26118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(26114n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26114n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 3 (26118, 26118)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(26118n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(26118n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26118n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint16, euint16) => euint16 test 4 (26118, 26114)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(26114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.and_uint16_euint16(26118n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26114n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 1 (14010, 21508)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(14010n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 21508n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 30398n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 2 (14006, 14010)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(14006n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 14010n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14014n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 3 (14010, 14010)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(14010n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 14010n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14010n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 4 (14010, 14006)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(14010n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_euint16_uint16(encryptedAmount.handles[0], 14006n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14014n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 1 (15445, 21508)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(21508n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(15445n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 31829n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 2 (14006, 14010)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(14010n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(14006n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14014n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 3 (14010, 14010)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(14010n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(14010n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14010n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint16, euint16) => euint16 test 4 (14010, 14006)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(14006n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.or_uint16_euint16(14010n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14014n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 1 (42686, 45262)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(42686n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 45262n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5744n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 2 (34851, 34855)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(34851n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 34855n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 3 (34855, 34855)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(34855n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 34855n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, uint16) => euint16 test 4 (34855, 34851)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(34855n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_uint16(encryptedAmount.handles[0], 34851n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 1 (28636, 45262)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(45262n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(28636n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 57106n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 2 (34851, 34855)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(34855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(34851n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 3 (34855, 34855)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(34855n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(34855n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint16, euint16) => euint16 test 4 (34855, 34851)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);

    input.add16(34851n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_uint16_euint16(34855n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 1 (343, 59905)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(343n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 59905n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 2 (339, 343)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(339n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 343n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 3 (343, 343)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(343n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 343n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint16, uint16) => ebool test 4 (343, 339)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(343n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.eq_euint16_uint16(encryptedAmount.handles[0], 339n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });
});
