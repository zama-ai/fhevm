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

describe('FHEVM operations 10', function () {
  before(async function () {
    this.signer = await getSigner(10);

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

  it('test operator "le" overload (euint128, uint128) => ebool test 1 (340282366920938463463369431786062180615, 340282366920938463463373127578809249203)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint128', value: 340282366920938463463369431786062180615n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463373127578809249203n,
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

  it('test operator "le" overload (euint128, uint128) => ebool test 2 (340282366920938463463367329768413053355, 340282366920938463463367329768413053359)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint128', value: 340282366920938463463367329768413053355n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367329768413053359n,
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

  it('test operator "le" overload (euint128, uint128) => ebool test 3 (340282366920938463463367329768413053359, 340282366920938463463367329768413053359)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint128', value: 340282366920938463463367329768413053359n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367329768413053359n,
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

  it('test operator "le" overload (euint128, uint128) => ebool test 4 (340282366920938463463367329768413053359, 340282366920938463463367329768413053355)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint128', value: 340282366920938463463367329768413053359n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.le_euint128_uint128(
      encryptedAmount.handles[0],
      340282366920938463463367329768413053355n,
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

  it('test operator "min" overload (euint32, euint8) => euint32 test 1 (4253082714, 41)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint32', value: 4253082714n },
        { type: 'uint8', value: 41n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 41n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 2 (37, 41)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint32', value: 37n },
        { type: 'uint8', value: 41n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 37n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 3 (41, 41)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint32', value: 41n },
        { type: 'uint8', value: 41n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 41n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "min" overload (euint32, euint8) => euint32 test 4 (41, 37)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint32', value: 41n },
        { type: 'uint8', value: 37n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.min_euint32_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 37n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 1 (49382, 103)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 49382n },
        { type: 'uint8', value: 103n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 2 (99, 103)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 99n },
        { type: 'uint8', value: 103n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 3 (103, 103)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 103n },
        { type: 'uint8', value: 103n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint8) => euint16 test 4 (103, 99)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 103n },
        { type: 'uint8', value: 99n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 1 (17, 7)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 17n },
        { type: 'uint8', value: 7n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 119n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 2 (12, 13)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 12n },
        { type: 'uint8', value: 13n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 156n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 3 (13, 13)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 13n },
        { type: 'uint8', value: 13n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 169n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint8, euint8) => euint8 test 4 (13, 12)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 13n },
        { type: 'uint8', value: 12n },
      ],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.mul_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 156n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (uint64, euint64) => ebool test 1 (18445616084724927657, 18445436475565736521)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18445436475565736521n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.le_uint64_euint64(
      18445616084724927657n,
      encryptedAmount.handles[0],
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

  it('test operator "le" overload (uint64, euint64) => ebool test 2 (18441120945513043029, 18441120945513043033)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18441120945513043033n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.le_uint64_euint64(
      18441120945513043029n,
      encryptedAmount.handles[0],
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

  it('test operator "le" overload (uint64, euint64) => ebool test 3 (18441120945513043033, 18441120945513043033)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18441120945513043033n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.le_uint64_euint64(
      18441120945513043033n,
      encryptedAmount.handles[0],
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

  it('test operator "le" overload (uint64, euint64) => ebool test 4 (18441120945513043033, 18441120945513043029)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18441120945513043029n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.le_uint64_euint64(
      18441120945513043033n,
      encryptedAmount.handles[0],
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

  it('test operator "neg" overload (euint32) => euint32 test 1 (3613399747)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint32', value: 3613399747n }],
      contractAddress: this.contract1Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract1.neg_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 681567549n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
