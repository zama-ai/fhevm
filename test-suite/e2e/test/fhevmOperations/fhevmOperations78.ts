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

describe('FHEVM operations 78', function () {
  before(async function () {
    this.signer = await getSigner(78);

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

  it('test operator "min" overload (uint16, euint16) => euint16 test 1 (27034, 54305)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add16(54305n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(27034n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 27034n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 2 (41764, 41768)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add16(41768n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(41764n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 41764n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 3 (41768, 41768)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add16(41768n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(41768n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 41768n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (uint16, euint16) => euint16 test 4 (41768, 41764)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add16(41764n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.min_uint16_euint16(41768n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 41764n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 1 (5514, 51030)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(5514n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 51030n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51030n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 2 (5510, 5514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(5510n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 5514n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5514n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 3 (5514, 5514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(5514n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 5514n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5514n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint16, uint16) => euint16 test 4 (5514, 5510)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add16(5514n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_euint16_uint16(encryptedAmount.handles[0], 5510n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5514n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 1 (10738, 51030)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add16(51030n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(10738n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 51030n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 2 (5510, 5514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add16(5514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(5510n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5514n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 3 (5514, 5514)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add16(5514n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(5514n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5514n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (uint16, euint16) => euint16 test 4 (5514, 5510)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add16(5510n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.max_uint16_euint16(5514n, encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract6.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5514n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 1 (2056964365, 2047425512)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(2056964365n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      2047425512n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4104389877n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 2 (2056964361, 2056964365)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(2056964361n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      2056964365n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928726n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 3 (2056964365, 2056964365)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(2056964365n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      2056964365n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928730n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint32, uint32) => euint32 test 4 (2056964365, 2056964361)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(2056964365n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_euint32_uint32(
      encryptedAmount.handles[0],
      2056964361n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928726n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 1 (1839189334, 1023712757)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(1023712757n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      1839189334n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2862902091n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 2 (2056964361, 2056964365)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(2056964365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      2056964361n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928726n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 3 (2056964365, 2056964365)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(2056964365n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      2056964365n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928730n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (uint32, euint32) => euint32 test 4 (2056964365, 2056964361)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);

    input.add32(2056964361n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.add_uint32_euint32(
      2056964365n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4113928726n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 1 (2271219408, 2271219408)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(2271219408n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_uint32(
      encryptedAmount.handles[0],
      2271219408n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint32, uint32) => euint32 test 2 (2271219408, 2271219404)', async function () {
    const input = this.instance.createEncryptedInput(this.contract6Address, this.signer.address);
    input.add32(2271219408n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract6.sub_euint32_uint32(
      encryptedAmount.handles[0],
      2271219404n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract6.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
