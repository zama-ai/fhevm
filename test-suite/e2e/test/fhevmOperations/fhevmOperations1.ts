import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
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

describe('FHEVM operations 1', function () {
  before(async function () {
    this.signer = await getSigner(1);

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

  it('test operator "xor" overload (uint8, euint8) => euint8 test 1 (173, 102)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_uint8_euint8(173n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 203n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 2 (4, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_uint8_euint8(4n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 3 (8, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_uint8_euint8(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 4 (8, 4)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_uint8_euint8(8n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 12n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint64) => euint64 test 1 (1488611147, 1488611147)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1488611147n);
    input.add64(1488611147n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint32_euint64(
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

  it('test operator "sub" overload (euint32, euint64) => euint64 test 2 (1488611147, 1488611143)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1488611147n);
    input.add64(1488611143n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint32_euint64(
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

  it('test operator "add" overload (euint16, euint8) => euint16 test 1 (167, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(167n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 169n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 2 (96, 100)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(96n);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 196n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 3 (100, 100)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(100n);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 200n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint16, euint8) => euint16 test 4 (100, 96)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(100n);
    input.add8(96n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 196n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 1 (61612, 47173)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(61612n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint16_uint16(encryptedAmount.handles[0], 47173n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 2 (61310, 61314)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(61310n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint16_uint16(encryptedAmount.handles[0], 61314n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 3 (61314, 61314)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(61314n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint16_uint16(encryptedAmount.handles[0], 61314n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "div" overload (euint16, uint16) => euint16 test 4 (61314, 61310)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(61314n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.div_euint16_uint16(encryptedAmount.handles[0], 61310n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 1 (181, 142)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(181n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_uint8(encryptedAmount.handles[0], 142n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 2 (5, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 3 (9, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "le" overload (euint8, uint8) => ebool test 4 (9, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.le_euint8_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 1 (2, 2147483649)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(2n);
    input.add128(2147483649n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint32_euint128(
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
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 2 (1607734426, 1607734428)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1607734426n);
    input.add128(1607734428n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3215468854n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 3 (1607734428, 1607734428)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1607734428n);
    input.add128(1607734428n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3215468856n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "add" overload (euint32, euint128) => euint128 test 4 (1607734428, 1607734426)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add32(1607734428n);
    input.add128(1607734426n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3215468854n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
