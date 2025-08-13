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

describe('FHEVM operations 3', function () {
  before(async function () {
    this.signer = await getSigner(3);

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

  it('test operator "le" overload (euint16, euint32) => ebool test 1 (59642, 3263355736)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(59642n);
    input.add32(3263355736n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint32(
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

  it('test operator "le" overload (euint16, euint32) => ebool test 2 (59638, 59642)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(59638n);
    input.add32(59642n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint32(
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

  it('test operator "le" overload (euint16, euint32) => ebool test 3 (59642, 59642)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(59642n);
    input.add32(59642n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint32(
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

  it('test operator "le" overload (euint16, euint32) => ebool test 4 (59642, 59638)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(59642n);
    input.add32(59638n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint16_euint32(
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

  it('test operator "sub" overload (euint64, euint8) => euint64 test 1 (225, 225)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(225n);
    input.add8(225n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint8) => euint64 test 2 (225, 221)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(225n);
    input.add8(221n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 1 (53960, 24861)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add32(24861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint32_euint32(53960n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1341499560n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 2 (61103, 61103)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add32(61103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint32_euint32(61103n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3733576609n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 3 (61103, 61103)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add32(61103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint32_euint32(61103n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3733576609n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (uint32, euint32) => euint32 test 4 (61103, 61103)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add32(61103n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_uint32_euint32(61103n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3733576609n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 1 (18438100183900742705, 18438100183900742705)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add64(18438100183900742705n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_uint64_euint64(
      18438100183900742705n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (uint64, euint64) => euint64 test 2 (18438100183900742705, 18438100183900742701)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add64(18438100183900742701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_uint64_euint64(
      18438100183900742705n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 1 (29024, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(29024n);
    input.add16(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 58048n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 2 (228, 228)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(228n);
    input.add16(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51984n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 3 (228, 228)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(228n);
    input.add16(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51984n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint32, euint16) => euint32 test 4 (228, 228)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(228n);
    input.add16(228n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51984n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 1 (2147483649, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(2147483649n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2147483651n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 2 (646615753, 646615757)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(646615753n);
    input.add32(646615757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1293231510n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 3 (646615757, 646615757)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(646615757n);
    input.add32(646615757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1293231514n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 4 (646615757, 646615753)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add128(646615757n);
    input.add32(646615753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1293231510n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
