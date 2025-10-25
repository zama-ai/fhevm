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

describe('FHEVM operations 85', function () {
  before(async function () {
    this.signer = await getSigner(85);

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

  it('test operator "and" overload (uint64, euint64) => euint64 test 1 (18440829243277354959, 18437858841139870577)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18437858841139870577n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18440829243277354959n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18437856085838570305n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 2 (18443935695415273509, 18443935695415273513)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18443935695415273513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18443935695415273509n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443935695415273505n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 3 (18443935695415273513, 18443935695415273513)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18443935695415273513n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18443935695415273513n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443935695415273513n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (uint64, euint64) => euint64 test 4 (18443935695415273513, 18443935695415273509)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18443935695415273509n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.and_uint64_euint64(
      18443935695415273513n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443935695415273505n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 1 (18443816109613900561, 18437955621632859185)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18443816109613900561n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18437955621632859185n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18443894119169261361n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 2 (18442438487998006519, 18442438487998006523)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18442438487998006519n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18442438487998006523n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442438487998006527n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 3 (18442438487998006523, 18442438487998006523)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18442438487998006523n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18442438487998006523n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442438487998006523n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint64, uint64) => euint64 test 4 (18442438487998006523, 18442438487998006519)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18442438487998006523n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_euint64_uint64(
      encryptedAmount.handles[0],
      18442438487998006519n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442438487998006527n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 1 (18445392183738577967, 18437955621632859185)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18437955621632859185n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18445392183738577967n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18445608242225085503n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 2 (18442438487998006519, 18442438487998006523)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18442438487998006523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18442438487998006519n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442438487998006527n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 3 (18442438487998006523, 18442438487998006523)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18442438487998006523n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18442438487998006523n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442438487998006523n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint64, euint64) => euint64 test 4 (18442438487998006523, 18442438487998006519)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18442438487998006519n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.or_uint64_euint64(
      18442438487998006523n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18442438487998006527n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 1 (18441939623530402993, 18441931867559509173)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18441939623530402993n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441931867559509173n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 10111232518148n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 2 (18441939623530402989, 18441939623530402993)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18441939623530402989n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441939623530402993n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint64, uint64) => euint64 test 3 (18441939623530402993, 18441939623530402993)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18441939623530402993n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441939623530402993n,
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

  it('test operator "xor" overload (euint64, uint64) => euint64 test 4 (18441939623530402993, 18441939623530402989)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18441939623530402993n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_euint64_uint64(
      encryptedAmount.handles[0],
      18441939623530402989n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 1 (18438880122339881845, 18441931867559509173)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18441931867559509173n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18438880122339881845n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3070480001224640n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 2 (18441939623530402989, 18441939623530402993)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18441939623530402993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18441939623530402989n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (uint64, euint64) => euint64 test 3 (18441939623530402993, 18441939623530402993)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18441939623530402993n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18441939623530402993n,
      encryptedAmount.handles[0],
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

  it('test operator "xor" overload (uint64, euint64) => euint64 test 4 (18441939623530402993, 18441939623530402989)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add64(18441939623530402989n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.xor_uint64_euint64(
      18441939623530402993n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (euint64, uint64) => ebool test 1 (18442977952698121359, 18437852794148046425)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18442977952698121359n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18437852794148046425n,
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

  it('test operator "eq" overload (euint64, uint64) => ebool test 2 (18439679158105806075, 18439679158105806079)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18439679158105806075n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18439679158105806079n,
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

  it('test operator "eq" overload (euint64, uint64) => ebool test 3 (18439679158105806079, 18439679158105806079)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18439679158105806079n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18439679158105806079n,
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

  it('test operator "eq" overload (euint64, uint64) => ebool test 4 (18439679158105806079, 18439679158105806075)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add64(18439679158105806079n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.eq_euint64_uint64(
      encryptedAmount.handles[0],
      18439679158105806075n,
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
});
