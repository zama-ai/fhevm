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

  it('test operator "lt" overload (euint64, euint32) => ebool test 1 (18443888068781654083, 496066044)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18443888068781654083n);
    input.add32(496066044n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "lt" overload (euint64, euint32) => ebool test 2 (496066040, 496066044)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(496066040n);
    input.add32(496066044n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "lt" overload (euint64, euint32) => ebool test 3 (496066044, 496066044)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(496066044n);
    input.add32(496066044n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "lt" overload (euint64, euint32) => ebool test 4 (496066044, 496066040)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(496066044n);
    input.add32(496066040n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.lt_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "min" overload (euint64, euint32) => euint64 test 1 (18439380057149500047, 3056369901)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18439380057149500047n);
    input.add32(3056369901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3056369901n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 2 (3056369897, 3056369901)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(3056369897n);
    input.add32(3056369901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3056369897n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 3 (3056369901, 3056369901)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(3056369901n);
    input.add32(3056369901n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3056369901n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint64, euint32) => euint64 test 4 (3056369901, 3056369897)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(3056369901n);
    input.add32(3056369897n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.min_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3056369897n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 1 (18440149341511828991, 1754071866)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18440149341511828991n);
    input.add32(1754071866n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18440149341511828991n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 2 (1754071862, 1754071866)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(1754071862n);
    input.add32(1754071866n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1754071866n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 3 (1754071866, 1754071866)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(1754071866n);
    input.add32(1754071866n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1754071866n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint64, euint32) => euint64 test 4 (1754071866, 1754071862)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(1754071866n);
    input.add32(1754071862n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.max_euint64_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1754071866n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 1 (9219094923281824695, 9219147465732255085)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(9219094923281824695n);
    input.add64(9219147465732255085n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438242389014079780n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 2 (9219094923281824693, 9219094923281824695)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(9219094923281824693n);
    input.add64(9219094923281824695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438189846563649388n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 3 (9219094923281824695, 9219094923281824695)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(9219094923281824695n);
    input.add64(9219094923281824695n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438189846563649390n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint64) => euint64 test 4 (9219094923281824695, 9219094923281824693)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(9219094923281824695n);
    input.add64(9219094923281824693n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.add_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438189846563649388n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 1 (18443553360840341423, 18443553360840341423)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18443553360840341423n);
    input.add64(18443553360840341423n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint64, euint64) => euint64 test 2 (18443553360840341423, 18443553360840341419)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(18443553360840341423n);
    input.add64(18443553360840341419n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.sub_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 1 (4294910698, 4293270308)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(4294910698n);
    input.add64(4293270308n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439212575234954984n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 2 (4293270308, 4293270308)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(4293270308n);
    input.add64(4293270308n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18432169937554414864n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 3 (4293270308, 4293270308)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(4293270308n);
    input.add64(4293270308n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18432169937554414864n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "mul" overload (euint64, euint64) => euint64 test 4 (4293270308, 4293270308)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add64(4293270308n);
    input.add64(4293270308n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.mul_euint64_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18432169937554414864n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
