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

describe('FHEVM operations 3', function () {
  before(async function () {
    this.signer = await getSigner(3);

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

  it('test operator "min" overload (euint8, euint8) => euint8 test 1 (82, 233)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(82n);
    input.add8(233n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 82n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 2 (78, 82)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(78n);
    input.add8(82n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 78n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 3 (82, 82)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(82n);
    input.add8(82n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 82n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint8, euint8) => euint8 test 4 (82, 78)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(82n);
    input.add8(78n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.min_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 78n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 1 (163, 114)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(163n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 163n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 2 (110, 114)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(110n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 114n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 3 (114, 114)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(114n);
    input.add8(114n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 114n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint8, euint8) => euint8 test 4 (114, 110)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(114n);
    input.add8(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.max_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 114n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 1 (2, 146)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(2n);
    input.add16(146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 148n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 2 (124, 126)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(124n);
    input.add16(126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 250n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 3 (126, 126)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(126n);
    input.add16(126n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 252n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint8, euint16) => euint16 test 4 (126, 124)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(126n);
    input.add16(124n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.add_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 250n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint8, euint16) => euint16 test 1 (125, 125)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(125n);
    input.add16(125n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint16(
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

  it('test operator "sub" overload (euint8, euint16) => euint16 test 2 (125, 121)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(125n);
    input.add16(121n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.sub_euint8_euint16(
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

  it('test operator "mul" overload (euint8, euint16) => euint16 test 1 (2, 110)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(2n);
    input.add16(110n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 220n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 2 (10, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 100n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 3 (10, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 100n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint8, euint16) => euint16 test 4 (10, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(10n);
    input.add16(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.mul_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 100n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 1 (250, 35535)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(250n);
    input.add16(35535n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 202n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 2 (246, 250)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(246n);
    input.add16(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 242n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 3 (250, 250)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(250n);
    input.add16(250n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 250n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint8, euint16) => euint16 test 4 (250, 246)', async function () {
    const input = this.instance.createEncryptedInput(this.contract1Address, this.signer.address);
    input.add8(250n);
    input.add16(246n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract1.and_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract1.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 242n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
