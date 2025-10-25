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

describe('FHEVM operations 30', function () {
  before(async function () {
    this.signer = await getSigner(30);

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

  it('test operator "le" overload (euint32, euint16) => ebool test 1 (553680951, 54026)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(553680951n);
    input.add16(54026n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint32, euint16) => ebool test 2 (54022, 54026)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(54022n);
    input.add16(54026n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint32, euint16) => ebool test 3 (54026, 54026)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(54026n);
    input.add16(54026n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint32, euint16) => ebool test 4 (54026, 54022)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(54026n);
    input.add16(54022n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.le_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "lt" overload (euint32, euint16) => ebool test 1 (471619784, 4203)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(471619784n);
    input.add16(4203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "lt" overload (euint32, euint16) => ebool test 2 (4199, 4203)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(4199n);
    input.add16(4203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "lt" overload (euint32, euint16) => ebool test 3 (4203, 4203)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(4203n);
    input.add16(4203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "lt" overload (euint32, euint16) => ebool test 4 (4203, 4199)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(4203n);
    input.add16(4199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.lt_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "min" overload (euint32, euint16) => euint32 test 1 (2321629392, 21828)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2321629392n);
    input.add16(21828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 21828n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 2 (21824, 21828)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(21824n);
    input.add16(21828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 21824n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 3 (21828, 21828)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(21828n);
    input.add16(21828n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 21828n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint32, euint16) => euint32 test 4 (21828, 21824)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(21828n);
    input.add16(21824n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.min_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 21824n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 1 (1036813849, 57146)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(1036813849n);
    input.add16(57146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1036813849n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 2 (57142, 57146)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(57142n);
    input.add16(57146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 57146n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 3 (57146, 57146)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(57146n);
    input.add16(57146n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 57146n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint32, euint16) => euint32 test 4 (57146, 57142)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(57146n);
    input.add16(57142n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.max_euint32_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 57146n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 1 (1028482183, 2052334060)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(1028482183n);
    input.add32(2052334060n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3080816243n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 2 (2056964361, 2056964365)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2056964361n);
    input.add32(2056964365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928726n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 3 (2056964365, 2056964365)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2056964365n);
    input.add32(2056964365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928730n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, euint32) => euint32 test 4 (2056964365, 2056964361)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2056964365n);
    input.add32(2056964361n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.add_euint32_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract2.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928726n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, euint32) => euint32 test 1 (2271219408, 2271219408)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2271219408n);
    input.add32(2271219408n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint32(
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

  it('test operator "sub" overload (euint32, euint32) => euint32 test 2 (2271219408, 2271219404)', async function () {
    const input = this.instance.createEncryptedInput(this.contract2Address, this.signer.address);
    input.add32(2271219408n);
    input.add32(2271219404n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract2.sub_euint32_euint32(
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
});
