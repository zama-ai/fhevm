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

describe('FHEVM operations 83', function () {
  before(async function () {
    this.signer = await getSigner(83);

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

  it('test operator "and" overload (euint16, euint64) => euint64 test 1 (4340, 18444141774127327107)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(4340n);
    input.add64(18444141774127327107n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 2 (4336, 4340)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(4336n);
    input.add64(4340n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4336n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 3 (4340, 4340)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(4340n);
    input.add64(4340n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4340n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint64) => euint64 test 4 (4340, 4336)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(4340n);
    input.add64(4336n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint16_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4336n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 1 (63955, 63955)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(63955n);
    input.add16(63955n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint16) => euint64 test 2 (63955, 63951)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(63955n);
    input.add16(63951n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 1 (146, 186)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(146n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint16_uint16(encryptedAmount.handles[0], 186n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 27156n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 2 (222, 222)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(222n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint16_uint16(encryptedAmount.handles[0], 222n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49284n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 3 (222, 222)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(222n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint16_uint16(encryptedAmount.handles[0], 222n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49284n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, uint16) => euint16 test 4 (222, 222)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(222n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.mul_euint16_uint16(encryptedAmount.handles[0], 222n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 49284n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint8, euint128) => euint128 test 1 (215, 215)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(215n);
    input.add128(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint8, euint128) => euint128 test 2 (215, 211)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(215n);
    input.add128(211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 1 (340282366920938463463371535967321767657, 30040)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(340282366920938463463371535967321767657n);
    input.add16(30040n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 2 (30036, 30040)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(30036n);
    input.add16(30040n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 3 (30040, 30040)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(30040n);
    input.add16(30040n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint128, euint16) => ebool test 4 (30040, 30036)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add128(30040n);
    input.add16(30036n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.ge_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 1 (43, 206)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(43n);
    input.add8(206n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 2 (39, 43)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(39n);
    input.add8(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 35n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 3 (43, 43)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(43n);
    input.add8(43n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 43n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint8) => euint8 test 4 (43, 39)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add8(43n);
    input.add8(39n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 35n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
