import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { assert } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMTestSuite1 } from '../../types/contracts/operations/FHEVMTestSuite1';
import type { FHEVMTestSuite2 } from '../../types/contracts/operations/FHEVMTestSuite2';
import type { FHEVMTestSuite3 } from '../../types/contracts/operations/FHEVMTestSuite3';
import type { FHEVMTestSuite4 } from '../../types/contracts/operations/FHEVMTestSuite4';
import type { FHEVMTestSuite5 } from '../../types/contracts/operations/FHEVMTestSuite5';
import type { FHEVMTestSuite6 } from '../../types/contracts/operations/FHEVMTestSuite6';
import type { FHEVMTestSuite7 } from '../../types/contracts/operations/FHEVMTestSuite7';
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

describe('FHEVM operations 13', function () {
  before(async function () {
    this.signer = await getSigner(13);

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

  it('test operator "mul" overload (euint16, euint64) => euint64 test 1 (2, 32765)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 2n },
        { type: 'uint64', value: 32765n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 65530n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 2 (150, 150)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 150n },
        { type: 'uint64', value: 150n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22500n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 3 (150, 150)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 150n },
        { type: 'uint64', value: 150n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22500n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint64) => euint64 test 4 (150, 150)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 150n },
        { type: 'uint64', value: 150n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.mul_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22500n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 1 (3349058566, 340282366920938463463366038125678950263)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint32', value: 3349058566n },
        { type: 'uint128', value: 340282366920938463463366038125678950263n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.eq_euint32_euint128(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 2 (3349058562, 3349058566)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint32', value: 3349058562n },
        { type: 'uint128', value: 3349058566n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.eq_euint32_euint128(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 3 (3349058566, 3349058566)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint32', value: 3349058566n },
        { type: 'uint128', value: 3349058566n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.eq_euint32_euint128(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint128) => ebool test 4 (3349058566, 3349058562)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint32', value: 3349058566n },
        { type: 'uint128', value: 3349058562n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.eq_euint32_euint128(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (109, 28)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint8', value: 109n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.rem_euint8_uint8(encryptedAmount.handles[0], 28n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 25n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (40, 44)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint8', value: 40n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.rem_euint8_uint8(encryptedAmount.handles[0], 44n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 40n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (44, 44)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint8', value: 44n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.rem_euint8_uint8(encryptedAmount.handles[0], 44n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (44, 40)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint8', value: 44n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.rem_euint8_uint8(encryptedAmount.handles[0], 40n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 1 (122, 122)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 122n },
        { type: 'uint8', value: 122n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.sub_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint8, euint8) => euint8 test 2 (122, 118)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 122n },
        { type: 'uint8', value: 118n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.sub_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 1 (931283036, 931283036)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 931283036n },
        { type: 'uint32', value: 931283036n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.sub_euint64_euint32(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint32) => euint64 test 2 (931283036, 931283032)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 931283036n },
        { type: 'uint32', value: 931283032n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.sub_euint64_euint32(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 1 (250, 72)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint8', value: 250n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.ge_euint8_uint8(encryptedAmount.handles[0], 72n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 2 (67, 71)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint8', value: 67n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.ge_euint8_uint8(encryptedAmount.handles[0], 71n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 3 (71, 71)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint8', value: 71n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.ge_euint8_uint8(encryptedAmount.handles[0], 71n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint8, uint8) => ebool test 4 (71, 67)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint8', value: 71n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.ge_euint8_uint8(encryptedAmount.handles[0], 67n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
