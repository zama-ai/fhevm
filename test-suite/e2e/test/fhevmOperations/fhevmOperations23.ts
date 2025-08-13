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

describe('FHEVM operations 23', function () {
  before(async function () {
    this.signer = await getSigner(23);

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

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 1 (11023, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(11023n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rotl_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 7766n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 2 (5, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rotl_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2560n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 3 (9, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rotl_euint16_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4608n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint16, uint8) => euint16 test 4 (9, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rotl_euint16_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 288n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 1 (18442798313793586165, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18442798313793586165n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rotr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 15564378659693747215n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 2 (2, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(2n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rotr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 576460752303423488n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 3 (6, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rotr_euint64_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1729382256910270464n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 4 (6, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.rotr_euint64_uint8(encryptedAmount.handles[0], 2n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9223372036854775809n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 1 (56280, 31135)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(56280n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_uint16(encryptedAmount.handles[0], 31135n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 64479n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 2 (11256, 11260)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(11256n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_uint16(encryptedAmount.handles[0], 11260n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 11260n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 3 (11260, 11260)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(11260n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_uint16(encryptedAmount.handles[0], 11260n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 11260n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, uint16) => euint16 test 4 (11260, 11256)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(11260n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_uint16(encryptedAmount.handles[0], 11256n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 11260n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 1 (3772048245, 2323152457)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(3772048245n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_uint32(
      encryptedAmount.handles[0],
      2323152457n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 2 (1322121310, 1322121314)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(1322121310n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_uint32(
      encryptedAmount.handles[0],
      1322121314n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 3 (1322121314, 1322121314)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(1322121314n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_uint32(
      encryptedAmount.handles[0],
      1322121314n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint32, uint32) => ebool test 4 (1322121314, 1322121310)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(1322121314n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.eq_euint32_uint32(
      encryptedAmount.handles[0],
      1322121310n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 1 (2, 129)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(2n);
    input.add64(129n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 131n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 2 (92, 94)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(92n);
    input.add64(94n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 186n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 3 (94, 94)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(94n);
    input.add64(94n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 188n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint64) => euint64 test 4 (94, 92)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add8(94n);
    input.add64(92n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 186n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 1 (18441892899049733305, 18438079348518990817)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18441892899049733305n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18438079348518990817n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 2 (18441892899049733301, 18441892899049733305)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18441892899049733301n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18441892899049733305n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 3 (18441892899049733305, 18441892899049733305)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18441892899049733305n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18441892899049733305n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint64, uint64) => ebool test 4 (18441892899049733305, 18441892899049733301)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add64(18441892899049733305n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.gt_euint64_uint64(
      encryptedAmount.handles[0],
      18441892899049733301n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });
});
