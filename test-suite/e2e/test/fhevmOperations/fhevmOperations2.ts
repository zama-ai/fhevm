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

describe('FHEVM operations 2', function () {
  before(async function () {
    this.signer = await getSigner(2);

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

  it('test operator "xor" overload (euint8, euint64) => euint64 test 1 (77, 18442442052149344105)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(77n);
    input.add64(18442442052149344105n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442442052149344036n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 2 (73, 77)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(73n);
    input.add64(77n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 3 (77, 77)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(77n);
    input.add64(77n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint64) => euint64 test 4 (77, 73)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(77n);
    input.add64(73n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint8_euint64(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 1 (340282366920938463463371539642781067539, 340282366920938463463366340355001501819)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add128(340282366920938463463366340355001501819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_uint128_euint128(
      340282366920938463463371539642781067539n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463371539642781067539n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 2 (340282366920938463463369029112380855845, 340282366920938463463369029112380855849)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add128(340282366920938463463369029112380855849n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_uint128_euint128(
      340282366920938463463369029112380855845n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369029112380855849n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 3 (340282366920938463463369029112380855849, 340282366920938463463369029112380855849)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add128(340282366920938463463369029112380855849n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_uint128_euint128(
      340282366920938463463369029112380855849n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369029112380855849n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint128, euint128) => euint128 test 4 (340282366920938463463369029112380855849, 340282366920938463463369029112380855845)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add128(340282366920938463463369029112380855845n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_uint128_euint128(
      340282366920938463463369029112380855849n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463369029112380855849n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 1 (64569, 39863)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(64569n);
    input.add16(39863n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26510n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 2 (39859, 39863)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(39859n);
    input.add16(39863n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint16(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 3 (39863, 39863)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(39863n);
    input.add16(39863n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint16(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint16) => euint16 test 4 (39863, 39859)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add16(39863n);
    input.add16(39859n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_euint16_euint16(
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
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 1 (18439745494209672317, 41053)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(18439745494209672317n);
    input.add16(41053n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 8285n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 2 (41049, 41053)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(41049n);
    input.add16(41053n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 41049n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 3 (41053, 41053)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(41053n);
    input.add16(41053n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 41053n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint64, euint16) => euint64 test 4 (41053, 41049)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add64(41053n);
    input.add16(41049n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 41049n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 1 (59, 340282366920938463463373366978817530885)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(59n);
    input.add128(340282366920938463463373366978817530885n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373366978817530885n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 2 (55, 59)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(55n);
    input.add128(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 59n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 3 (59, 59)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(59n);
    input.add128(59n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 59n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, euint128) => euint128 test 4 (59, 55)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(59n);
    input.add128(55n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 59n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 1 (126, 191)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(191n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_uint8_euint8(126n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 193n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 2 (57, 61)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_uint8_euint8(57n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 3 (61, 61)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(61n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_uint8_euint8(61n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint8, euint8) => euint8 test 4 (61, 57)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);

    input.add8(57n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.xor_uint8_euint8(61n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
