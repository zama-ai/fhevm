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

describe('FHEVM operations 92', function () {
  before(async function () {
    this.signer = await getSigner(92);

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

  it('test operator "min" overload (euint128, euint16) => euint128 test 1 (340282366920938463463371721861920561581, 36382)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463371721861920561581n);
    input.add16(36382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36382n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 2 (36378, 36382)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(36378n);
    input.add16(36382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36378n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 3 (36382, 36382)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(36382n);
    input.add16(36382n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36382n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint16) => euint128 test 4 (36382, 36378)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(36382n);
    input.add16(36378n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 36378n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 1 (340282366920938463463373707854917926449, 2194466261)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(340282366920938463463373707854917926449n);
    input.add32(2194466261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2194466261n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 2 (2194466257, 2194466261)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(2194466257n);
    input.add32(2194466261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2194466257n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 3 (2194466261, 2194466261)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(2194466261n);
    input.add32(2194466261n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2194466261n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "min" overload (euint128, euint32) => euint128 test 4 (2194466261, 2194466257)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add128(2194466261n);
    input.add32(2194466257n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.min_euint128_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 2194466257n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 1 (2765606203, 340282366920938463463370625319605465543)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2765606203n);
    input.add128(340282366920938463463370625319605465543n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 2 (2765606199, 2765606203)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2765606199n);
    input.add128(2765606203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 3 (2765606203, 2765606203)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2765606203n);
    input.add128(2765606203n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint32, euint128) => ebool test 4 (2765606203, 2765606199)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(2765606203n);
    input.add128(2765606199n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.ge_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 1 (3851745212, 340282366920938463463367710830475720487)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(3851745212n);
    input.add128(340282366920938463463367710830475720487n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 2 (3851745208, 3851745212)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(3851745208n);
    input.add128(3851745212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 3 (3851745212, 3851745212)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(3851745212n);
    input.add128(3851745212n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "gt" overload (euint32, euint128) => ebool test 4 (3851745212, 3851745208)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add32(3851745212n);
    input.add128(3851745208n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.gt_euint32_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 1 (214, 206)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(214n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint8_uint8(encryptedAmount.handles[0], 206n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 8n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 2 (102, 106)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(102n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint8_uint8(encryptedAmount.handles[0], 106n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 102n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 3 (106, 106)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(106n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint8_uint8(encryptedAmount.handles[0], 106n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rem" overload (euint8, uint8) => euint8 test 4 (106, 102)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);
    input.add8(106n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.rem_euint8_uint8(encryptedAmount.handles[0], 102n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract7.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 1 (340282366920938463463368016240094657015, 340282366920938463463371426091241704701)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463371426091241704701n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint128_euint128(
      340282366920938463463368016240094657015n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463373683700160445951n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 2 (340282366920938463463370372732293038459, 340282366920938463463370372732293038463)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463370372732293038463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint128_euint128(
      340282366920938463463370372732293038459n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370372732293038463n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 3 (340282366920938463463370372732293038463, 340282366920938463463370372732293038463)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463370372732293038463n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint128_euint128(
      340282366920938463463370372732293038463n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370372732293038463n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (uint128, euint128) => euint128 test 4 (340282366920938463463370372732293038463, 340282366920938463463370372732293038459)', async function () {
    const input = this.instance.createEncryptedInput(this.contract7Address, this.signer.address);

    input.add128(340282366920938463463370372732293038459n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract7.or_uint128_euint128(
      340282366920938463463370372732293038463n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract7.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 340282366920938463463370372732293038463n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
