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

describe('FHEVM operations 73', function () {
  before(async function () {
    this.signer = await getSigner(73);

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

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18440149341511828991, 1754071866)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 18440149341511828991n },
        { type: 'uint32', value: 1754071866n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440149341511828991n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (1754071862, 1754071866)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 1754071862n },
        { type: 'uint32', value: 1754071866n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1754071866n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (1754071866, 1754071866)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 1754071866n },
        { type: 'uint32', value: 1754071866n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1754071866n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (1754071866, 1754071862)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 1754071866n },
        { type: 'uint32', value: 1754071862n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1754071866n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (42686, 34855)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 42686n },
        { type: 'uint16', value: 34855n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 11929n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (34851, 34855)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 34851n },
        { type: 'uint16', value: 34855n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (34855, 34855)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 34855n },
        { type: 'uint16', value: 34855n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (34855, 34851)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 34855n },
        { type: 'uint16', value: 34851n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (137, 137)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 137n },
        { type: 'uint8', value: 137n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (137, 133)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint16', value: 137n },
        { type: 'uint8', value: 133n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 1 (18444237080447470479, 18439875117400843379)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 18444237080447470479n },
        { type: 'uint64', value: 18439875117400843379n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18444237080447470479n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 2 (18439875117400843375, 18439875117400843379)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 18439875117400843375n },
        { type: 'uint64', value: 18439875117400843379n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 3 (18439875117400843379, 18439875117400843379)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 18439875117400843379n },
        { type: 'uint64', value: 18439875117400843379n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint64) => euint64 test 4 (18439875117400843379, 18439875117400843375)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 18439875117400843379n },
        { type: 'uint64', value: 18439875117400843375n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.max_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 1 (340282366920938463463370565926781235887, 385755231)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint128', value: 340282366920938463463370565926781235887n },
        { type: 'uint32', value: 385755231n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 2 (385755227, 385755231)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint128', value: 385755227n },
        { type: 'uint32', value: 385755231n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 3 (385755231, 385755231)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint128', value: 385755231n },
        { type: 'uint32', value: 385755231n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "gt" overload (euint128, euint32) => ebool test 4 (385755231, 385755227)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint128', value: 385755231n },
        { type: 'uint32', value: 385755227n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.gt_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 1 (18442457919507223175, 56)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 18442457919507223175n },
        { type: 'uint8', value: 56n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 2 (52, 56)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 52n },
        { type: 'uint8', value: 56n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 3 (56, 56)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 56n },
        { type: 'uint8', value: 56n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "lt" overload (euint64, euint8) => ebool test 4 (56, 52)', async function () {
    const encryptedAmount = await this.instance.encryptTypedValues({
      values: [
        { type: 'uint64', value: 56n },
        { type: 'uint8', value: 52n },
      ],
      contractAddress: this.contract5Address,
      userAddress: this.signer.address,
    });
    const tx = await this.contract5.lt_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
