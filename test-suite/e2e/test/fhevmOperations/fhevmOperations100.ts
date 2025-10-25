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

describe('FHEVM operations 100', function () {
  before(async function () {
    this.signer = await getSigner(100);

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

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 1 (18438499424116504505, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18438499424116504505n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 16336113777889491455n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 2 (4, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 3 (8, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2048n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint64, uint8) => euint64 test 4 (8, 4)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotl_euint64_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 1 (18444264105617150445, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18444264105617150445n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 17798220883680519720n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 2 (5, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(5n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 180143985094819840n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 3 (9, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 324259173170675712n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, euint8) => euint64 test 4 (9, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(9n);
    input.add8(5n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5188146770730811392n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 1 (18444264105617150445, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(18444264105617150445n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 17798220883680519720n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 2 (5, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(5n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 180143985094819840n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 3 (9, 9)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 9n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 324259173170675712n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint64, uint8) => euint64 test 4 (9, 5)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add64(9n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rotr_euint64_uint8(encryptedAmount.handles[0], 5n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5188146770730811392n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463367825108556783067, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463367825108556783067n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463449484409494762870784n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 2 (7, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(7n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14336n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 3 (11, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(11n);
    input.add8(11n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, euint8) => euint128 test 4 (11, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(11n);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1408n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 1 (340282366920938463463367825108556783067, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463367825108556783067n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463449484409494762870784n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 2 (7, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(7n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 14336n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 3 (11, 11)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 11n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 22528n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint128, uint8) => euint128 test 4 (11, 7)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(11n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shl_euint128_uint8(encryptedAmount.handles[0], 7n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1408n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 1 (340282366920938463463369869815014359481, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463369869815014359481n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 5316911983139663491615154215859599366n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 2 (2, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(2n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 3 (6, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(6n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint128, euint8) => euint128 test 4 (6, 2)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(6n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.shr_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
