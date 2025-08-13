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

describe('FHEVM operations 67', function () {
  before(async function () {
    this.signer = await getSigner(67);

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

  it('test operator "sub" overload (euint16, euint8) => euint16 test 1 (215, 215)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(215n);
    input.add8(215n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "sub" overload (euint16, euint8) => euint16 test 2 (215, 211)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(215n);
    input.add8(211n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.sub_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 1 (340282366920938463463373114976830652183, 509)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463373114976830652183n);
    input.add16(509n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373114976830652138n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 2 (505, 509)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(505n);
    input.add16(509n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 3 (509, 509)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(509n);
    input.add16(509n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint128, euint16) => euint128 test 4 (509, 505)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(509n);
    input.add16(505n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 1 (340282366920938463463368509253758463595, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(340282366920938463463368509253758463595n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463461813473861272759295n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 2 (4, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(4n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1024n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 3 (8, 8)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(8n);
    input.add8(8n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2048n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotl" overload (euint128, euint8) => euint128 test 4 (8, 4)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add128(8n);
    input.add8(4n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.rotl_euint128_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 1 (18439852880390706283, 85)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(18439852880390706283n);
    input.add8(85n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 2 (81, 85)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(81n);
    input.add8(85n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 3 (85, 85)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(85n);
    input.add8(85n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint64, euint8) => ebool test 4 (85, 81)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add64(85n);
    input.add8(81n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.ne_euint64_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 1 (62, 15242)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(62n);
    input.add16(15242n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 15284n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 2 (58, 62)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(58n);
    input.add16(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 3 (62, 62)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(62n);
    input.add16(62n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint8, euint16) => euint16 test 4 (62, 58)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add8(62n);
    input.add16(58n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint8_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 1 (65074, 340282366920938463463368271355386546231)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(65074n);
    input.add128(340282366920938463463368271355386546231n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463368271355386574341n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 2 (65070, 65074)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(65070n);
    input.add128(65074n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 3 (65074, 65074)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(65074n);
    input.add128(65074n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "xor" overload (euint16, euint128) => euint128 test 4 (65074, 65070)', async function () {
    const input = this.instance.createEncryptedInput(this.contract5Address, this.signer.address);
    input.add16(65074n);
    input.add128(65070n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract5.xor_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract5.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 28n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
