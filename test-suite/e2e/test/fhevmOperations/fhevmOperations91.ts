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

describe('FHEVM operations 91', function () {
  before(async function () {
    this.signer = await getSigner(91);

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

  it('test operator "lt" overload (euint128, euint8) => ebool test 1 (340282366920938463463374451380981106537, 243)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint128', value: 340282366920938463463374451380981106537n },
        { type: 'uint8', value: 243n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 2 (239, 243)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint128', value: 239n },
        { type: 'uint8', value: 243n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 3 (243, 243)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint128', value: 243n },
        { type: 'uint8', value: 243n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint128, euint8) => ebool test 4 (243, 239)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint128', value: 243n },
        { type: 'uint8', value: 239n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.lt_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 1 (18441939623530402993, 18441931867559509173)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18441939623530402993n }],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441931867559509173n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10111232518148n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 2 (18441939623530402989, 18441939623530402993)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18441939623530402989n }],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441939623530402993n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 3 (18441939623530402993, 18441939623530402993)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18441939623530402993n }],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441939623530402993n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 4 (18441939623530402993, 18441939623530402989)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [{ type: 'uint64', value: 18441939623530402993n }],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441939623530402989n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 1 (24669, 340282366920938463463366292722973600473)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 24669n },
        { type: 'uint128', value: 340282366920938463463366292722973600473n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24665n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 2 (24665, 24669)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 24665n },
        { type: 'uint128', value: 24669n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24665n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 3 (24669, 24669)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 24669n },
        { type: 'uint128', value: 24669n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24669n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 4 (24669, 24665)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 24669n },
        { type: 'uint128', value: 24665n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24665n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 1 (57124, 115792089237316195423570985008687907853269984665640564039457580716762844836757)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 57124n },
        { type: 'uint256', value: 115792089237316195423570985008687907853269984665640564039457580716762844836757n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457580716762844889013n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 2 (57120, 57124)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 57120n },
        { type: 'uint256', value: 57124n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 57124n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 3 (57124, 57124)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 57124n },
        { type: 'uint256', value: 57124n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 57124n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint16, euint256) => euint256 test 4 (57124, 57120)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 57124n },
        { type: 'uint256', value: 57120n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.or_euint16_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 57124n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 1 (2, 9223372036854775809)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 2n },
        { type: 'uint128', value: 9223372036854775809n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9223372036854775811n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 2 (9221670991015262434, 9221670991015262436)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 9221670991015262434n },
        { type: 'uint128', value: 9221670991015262436n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443341982030524870n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 3 (9221670991015262436, 9221670991015262436)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 9221670991015262436n },
        { type: 'uint128', value: 9221670991015262436n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443341982030524872n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 4 (9221670991015262436, 9221670991015262434)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 9221670991015262436n },
        { type: 'uint128', value: 9221670991015262434n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443341982030524870n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 1 (18440098425586971115, 18440098425586971115)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 18440098425586971115n },
        { type: 'uint128', value: 18440098425586971115n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint128) => euint128 test 2 (18440098425586971115, 18440098425586971111)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 18440098425586971115n },
        { type: 'uint128', value: 18440098425586971111n },
      ],
      contractAddress: this.contract7Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract7.sub_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
