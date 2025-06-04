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

describe('FHEVM operations 45', function () {
  before(async function () {
    this.signer = await getSigner(45);

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

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (45240, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(45240n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 44n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (6, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (10, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (10, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shr_euint16_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 1 (48214, 7073)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add16(7073n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(48214n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 2 (45201, 45205)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add16(45205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(45201n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 3 (45205, 45205)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add16(45205n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(45205n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "le" overload (uint16, euint16) => ebool test 4 (45205, 45201)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add16(45201n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.le_uint16_euint16(45205n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 1 (184, 116)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add8(116n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint8_euint8(184n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 2 (154, 158)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint8_euint8(154n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 3 (158, 158)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add8(158n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint8_euint8(158n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "lt" overload (uint8, euint8) => ebool test 4 (158, 154)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add8(154n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_uint8_euint8(158n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 1 (18437839614113114093, 18439553596948450583)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add64(18439553596948450583n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint64_euint64(
      18437839614113114093n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 2 (18441474664501921447, 18441474664501921451)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add64(18441474664501921451n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint64_euint64(
      18441474664501921447n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 3 (18441474664501921451, 18441474664501921451)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add64(18441474664501921451n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint64_euint64(
      18441474664501921451n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint64, euint64) => ebool test 4 (18441474664501921451, 18441474664501921447)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);

    input.add64(18441474664501921447n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.eq_uint64_euint64(
      18441474664501921451n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (2147453857, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(2147453857n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4294907714n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (50376, 50376)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(50376n);
    input.add32(50376n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2537741376n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (50376, 50376)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(50376n);
    input.add32(50376n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2537741376n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (50376, 50376)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(50376n);
    input.add32(50376n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2537741376n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 1 (18441576190276316565, 108)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18441576190276316565n);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18441576190276316669n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 2 (104, 108)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(104n);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 108n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 3 (108, 108)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(108n);
    input.add8(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 108n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, euint8) => euint64 test 4 (108, 104)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(108n);
    input.add8(104n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 108n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
