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

describe('FHEVM operations 46', function () {
  before(async function () {
    this.signer = await getSigner(46);

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

  it('test operator "ne" overload (euint8, euint128) => ebool test 1 (168, 340282366920938463463372272325931514701)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 168n },
        { type: 'uint128', value: 340282366920938463463372272325931514701n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.ne_euint8_euint128(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 2 (164, 168)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 164n },
        { type: 'uint128', value: 168n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.ne_euint8_euint128(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 3 (168, 168)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 168n },
        { type: 'uint128', value: 168n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.ne_euint8_euint128(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ne" overload (euint8, euint128) => ebool test 4 (168, 164)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 168n },
        { type: 'uint128', value: 164n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.ne_euint8_euint128(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 1 (18444516960769536337, 19791)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 18444516960769536337n },
        { type: 'uint16', value: 19791n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.lt_euint64_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 2 (19787, 19791)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 19787n },
        { type: 'uint16', value: 19791n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.lt_euint64_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 3 (19791, 19791)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 19791n },
        { type: 'uint16', value: 19791n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.lt_euint64_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint16) => ebool test 4 (19791, 19787)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 19791n },
        { type: 'uint16', value: 19787n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.lt_euint64_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 1 (18441793874479678031, 18444669461336692671)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18441793874479678031n }],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18444669461336692671n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 2 (18438850655314522435, 18438850655314522439)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18438850655314522435n }],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18438850655314522439n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 3 (18438850655314522439, 18438850655314522439)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18438850655314522439n }],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18438850655314522439n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "ge" overload (euint64, uint64) => ebool test 4 (18438850655314522439, 18438850655314522435)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18438850655314522439n }],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.ge_euint64_uint64(
      encryptedAmount.handles[0],
      18438850655314522435n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578104479961759913, 340282366920938463463372582172919627471)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint256', value: 115792089237316195423570985008687907853269984665640564039457578104479961759913n },
        { type: 'uint128', value: 340282366920938463463372582172919627471n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463367858670350123145n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 2 (340282366920938463463372582172919627467, 340282366920938463463372582172919627471)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint256', value: 340282366920938463463372582172919627467n },
        { type: 'uint128', value: 340282366920938463463372582172919627471n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372582172919627467n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 3 (340282366920938463463372582172919627471, 340282366920938463463372582172919627471)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint256', value: 340282366920938463463372582172919627471n },
        { type: 'uint128', value: 340282366920938463463372582172919627471n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372582172919627471n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint256, euint128) => euint256 test 4 (340282366920938463463372582172919627471, 340282366920938463463372582172919627467)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint256', value: 340282366920938463463372582172919627471n },
        { type: 'uint128', value: 340282366920938463463372582172919627467n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.and_euint256_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372582172919627467n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 1 (43, 34333)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 43n },
        { type: 'uint16', value: 34333n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.gt_euint8_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 2 (39, 43)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 39n },
        { type: 'uint16', value: 43n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.gt_euint8_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 3 (43, 43)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 43n },
        { type: 'uint16', value: 43n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.gt_euint8_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint16) => ebool test 4 (43, 39)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 43n },
        { type: 'uint16', value: 39n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.gt_euint8_euint16(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (2, 146)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 2n },
        { type: 'uint16', value: 146n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 148n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (124, 126)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 124n },
        { type: 'uint16', value: 126n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 250n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (126, 126)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 126n },
        { type: 'uint16', value: 126n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 252n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (126, 124)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint8', value: 126n },
        { type: 'uint16', value: 124n },
      ],
      contractAddress: this.contract4Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract4.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 250n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
