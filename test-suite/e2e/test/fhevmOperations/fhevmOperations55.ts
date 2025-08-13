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

describe('FHEVM operations 55', function () {
  before(async function () {
    this.signer = await getSigner(55);

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

  it('test operator "max" overload (uint128, euint128) => euint128 test 1 (340282366920938463463371751978062779129, 340282366920938463463373751683342743839)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add128(340282366920938463463373751683342743839n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint128_euint128(
      340282366920938463463371751978062779129n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373751683342743839n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 2 (340282366920938463463365683663523221971, 340282366920938463463365683663523221975)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add128(340282366920938463463365683663523221975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint128_euint128(
      340282366920938463463365683663523221971n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463365683663523221975n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 3 (340282366920938463463365683663523221975, 340282366920938463463365683663523221975)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add128(340282366920938463463365683663523221975n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint128_euint128(
      340282366920938463463365683663523221975n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463365683663523221975n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 4 (340282366920938463463365683663523221975, 340282366920938463463365683663523221971)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add128(340282366920938463463365683663523221971n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_uint128_euint128(
      340282366920938463463365683663523221975n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463365683663523221975n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 1 (950780529, 938213546)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(950780529n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_uint32(
      encryptedAmount.handles[0],
      938213546n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 816316448n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 2 (950780525, 950780529)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(950780525n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_uint32(
      encryptedAmount.handles[0],
      950780529n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 950780513n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 3 (950780529, 950780529)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(950780529n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_uint32(
      encryptedAmount.handles[0],
      950780529n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 950780529n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint32, uint32) => euint32 test 4 (950780529, 950780525)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(950780529n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.and_euint32_uint32(
      encryptedAmount.handles[0],
      950780525n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 950780513n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 1 (2147443143, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(2147443143n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4294886286n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 2 (37609, 37609)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(37609n);
    input.add32(37609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1414436881n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 3 (37609, 37609)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(37609n);
    input.add32(37609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1414436881n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint32) => euint64 test 4 (37609, 37609)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(37609n);
    input.add32(37609n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.mul_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1414436881n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 1 (18442684280855507527, 18444930736089945979)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18442684280855507527n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18444930736089945979n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 2 (18442684280855507523, 18442684280855507527)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18442684280855507523n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18442684280855507527n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 3 (18442684280855507527, 18442684280855507527)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18442684280855507527n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18442684280855507527n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, uint64) => ebool test 4 (18442684280855507527, 18442684280855507523)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18442684280855507527n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.ne_euint64_uint64(
      encryptedAmount.handles[0],
      18442684280855507523n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 1 (340282366920938463463372063437639994843, 3380679538)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463372063437639994843n);
    input.add32(3380679538n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463372063437639994843n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 2 (3380679534, 3380679538)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(3380679534n);
    input.add32(3380679538n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3380679538n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 3 (3380679538, 3380679538)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(3380679538n);
    input.add32(3380679538n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3380679538n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint32) => euint128 test 4 (3380679538, 3380679534)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(3380679538n);
    input.add32(3380679534n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3380679538n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 1 (943370818, 18443069382736677797)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(943370818n);
    input.add64(18443069382736677797n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443069383138720231n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 2 (943370814, 943370818)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(943370814n);
    input.add64(943370818n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 124n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 3 (943370818, 943370818)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(943370818n);
    input.add64(943370818n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint32, euint64) => euint64 test 4 (943370818, 943370814)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add32(943370818n);
    input.add64(943370814n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.xor_euint32_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 124n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
