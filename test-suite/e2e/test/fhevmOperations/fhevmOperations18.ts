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

describe('FHEVM operations 18', function () {
  before(async function () {
    this.signer = await getSigner(18);

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

  it('test operator "max" overload (euint16, euint16) => euint16 test 1 (5514, 51325)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(5514n);
    input.add16(51325n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51325n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 2 (5510, 5514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(5510n);
    input.add16(5514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5514n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 3 (5514, 5514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(5514n);
    input.add16(5514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5514n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, euint16) => euint16 test 4 (5514, 5510)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(5514n);
    input.add16(5510n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint16_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5514n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 1 (2, 61819)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(2n);
    input.add32(61819n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 61821n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 2 (10080, 10084)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(10080n);
    input.add32(10084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 20164n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 3 (10084, 10084)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(10084n);
    input.add32(10084n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 20168n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint16, euint32) => euint32 test 4 (10084, 10080)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(10084n);
    input.add32(10080n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 20164n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 1 (27746, 27746)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(27746n);
    input.add32(27746n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint32) => euint32 test 2 (27746, 27742)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(27746n);
    input.add32(27742n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 1 (2, 20459)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(2n);
    input.add32(20459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 40918n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 2 (215, 215)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(215n);
    input.add32(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 46225n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 3 (215, 215)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(215n);
    input.add32(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 46225n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint32) => euint32 test 4 (215, 215)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(215n);
    input.add32(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.mul_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 46225n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 1 (28986, 730827779)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(28986n);
    input.add32(730827779n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 2 (28982, 28986)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(28982n);
    input.add32(28986n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28978n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 3 (28986, 28986)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(28986n);
    input.add32(28986n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28986n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint32) => euint32 test 4 (28986, 28982)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(28986n);
    input.add32(28982n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.and_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28978n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 1 (1008, 1894332575)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(1008n);
    input.add32(1894332575n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1894333439n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 2 (1004, 1008)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(1004n);
    input.add32(1008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1020n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 3 (1008, 1008)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(1008n);
    input.add32(1008n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1008n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint16, euint32) => euint32 test 4 (1008, 1004)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add16(1008n);
    input.add32(1004n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.or_euint16_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1020n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
