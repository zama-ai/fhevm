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

  it('test operator "le" overload (euint128, euint16) => ebool test 1 (340282366920938463463373061358223662361, 20178)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463373061358223662361n);
    input.add16(20178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint128, euint16) => ebool test 2 (20174, 20178)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(20174n);
    input.add16(20178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint128, euint16) => ebool test 3 (20178, 20178)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(20178n);
    input.add16(20178n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "le" overload (euint128, euint16) => ebool test 4 (20178, 20174)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(20178n);
    input.add16(20174n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.le_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "lt" overload (euint128, euint16) => ebool test 1 (340282366920938463463370460778833589529, 53631)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463370460778833589529n);
    input.add16(53631n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "lt" overload (euint128, euint16) => ebool test 2 (53627, 53631)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(53627n);
    input.add16(53631n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "lt" overload (euint128, euint16) => ebool test 3 (53631, 53631)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(53631n);
    input.add16(53631n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "lt" overload (euint128, euint16) => ebool test 4 (53631, 53627)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(53631n);
    input.add16(53627n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
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

  it('test operator "min" overload (euint128, euint16) => euint128 test 1 (340282366920938463463366145918203138607, 58861)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463366145918203138607n);
    input.add16(58861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 58861n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 2 (58857, 58861)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(58857n);
    input.add16(58861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 58857n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 3 (58861, 58861)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(58861n);
    input.add16(58861n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 58861n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 4 (58861, 58857)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(58861n);
    input.add16(58857n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 58857n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 1 (340282366920938463463368659222487539321, 15753)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463368659222487539321n);
    input.add16(15753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368659222487539321n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 2 (15749, 15753)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(15749n);
    input.add16(15753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 15753n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 3 (15753, 15753)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(15753n);
    input.add16(15753n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 15753n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 4 (15753, 15749)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(15753n);
    input.add16(15749n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.max_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 15753n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 1 (2147483649, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(2147483649n);
    input.add32(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2147483651n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 2 (468914479, 468914483)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(468914479n);
    input.add32(468914483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 937828962n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 3 (468914483, 468914483)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(468914483n);
    input.add32(468914483n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 937828966n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint128, euint32) => euint128 test 4 (468914483, 468914479)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(468914483n);
    input.add32(468914479n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 937828962n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 1 (3806541193, 3806541193)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(3806541193n);
    input.add32(3806541193n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint128, euint32) => euint128 test 2 (3806541193, 3806541189)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(3806541193n);
    input.add32(3806541189n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.sub_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
