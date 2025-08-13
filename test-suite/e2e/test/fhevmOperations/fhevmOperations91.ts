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

  it('test operator "or" overload (euint8, euint16) => euint16 test 1 (180, 47944)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(180n);
    input.add16(47944n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 48124n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 2 (176, 180)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(176n);
    input.add16(180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 180n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 3 (180, 180)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(180n);
    input.add16(180n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 180n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint8, euint16) => euint16 test 4 (180, 176)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(180n);
    input.add16(176n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 180n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "neg" overload (euint64) => euint64 test 1 (18444498325551574601)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18444498325551574601n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.neg_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2245748157977015n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 1 (66, 1485831284)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(66n);
    input.add32(1485831284n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint8_euint32(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 2 (62, 66)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(62n);
    input.add32(66n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint8_euint32(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 3 (66, 66)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(66n);
    input.add32(66n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint8_euint32(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint8, euint32) => ebool test 4 (66, 62)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(66n);
    input.add32(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint8_euint32(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 1 (3660556922, 3017313315)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(3660556922n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint32_uint32(
      encryptedAmount.handles[0],
      3017313315n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4227837563n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 2 (2015649198, 2015649202)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2015649198n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint32_uint32(
      encryptedAmount.handles[0],
      2015649202n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2015649214n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 3 (2015649202, 2015649202)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2015649202n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint32_uint32(
      encryptedAmount.handles[0],
      2015649202n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2015649202n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint32, uint32) => euint32 test 4 (2015649202, 2015649198)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2015649202n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_euint32_uint32(
      encryptedAmount.handles[0],
      2015649198n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2015649214n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 1 (3772048245, 1322121314)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(3772048245n);
    input.add32(1322121314n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint32_euint32(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 2 (1322121310, 1322121314)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(1322121310n);
    input.add32(1322121314n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint32_euint32(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 3 (1322121314, 1322121314)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(1322121314n);
    input.add32(1322121314n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint32_euint32(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, euint32) => ebool test 4 (1322121314, 1322121310)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(1322121314n);
    input.add32(1322121310n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.eq_euint32_euint32(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 1 (43, 24)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(43n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint8_uint8(encryptedAmount.handles[0], 24n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 8n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 2 (39, 43)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(39n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint8_uint8(encryptedAmount.handles[0], 43n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 35n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 3 (43, 43)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(43n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint8_uint8(encryptedAmount.handles[0], 43n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 43n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, uint8) => euint8 test 4 (43, 39)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(43n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.and_euint8_uint8(encryptedAmount.handles[0], 39n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 35n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
