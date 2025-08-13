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

describe('FHEVM operations 60', function () {
  before(async function () {
    this.signer = await getSigner(60);

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

  it('test operator "lt" overload (euint64, euint128) => ebool test 1 (18444256851706074271, 340282366920938463463371872359075868597)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18444256851706074271n);
    input.add128(340282366920938463463371872359075868597n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
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

  it('test operator "lt" overload (euint64, euint128) => ebool test 2 (18444256851706074267, 18444256851706074271)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18444256851706074267n);
    input.add128(18444256851706074271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
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

  it('test operator "lt" overload (euint64, euint128) => ebool test 3 (18444256851706074271, 18444256851706074271)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18444256851706074271n);
    input.add128(18444256851706074271n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
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

  it('test operator "lt" overload (euint64, euint128) => ebool test 4 (18444256851706074271, 18444256851706074267)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(18444256851706074271n);
    input.add128(18444256851706074267n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.lt_euint64_euint128(
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

  it('test operator "add" overload (euint64, euint128) => euint128 test 1 (2, 9223372036854775809)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(2n);
    input.add128(9223372036854775809n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 9223372036854775811n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 2 (9219146308852063854, 9219146308852063856)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(9219146308852063854n);
    input.add128(9219146308852063856n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438292617704127710n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 3 (9219146308852063856, 9219146308852063856)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(9219146308852063856n);
    input.add128(9219146308852063856n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438292617704127712n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "add" overload (euint64, euint128) => euint128 test 4 (9219146308852063856, 9219146308852063854)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add64(9219146308852063856n);
    input.add128(9219146308852063854n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.add_euint64_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18438292617704127710n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "eq" overload (uint128, euint128) => ebool test 1 (340282366920938463463366528658094296109, 340282366920938463463372503569758885355)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add128(340282366920938463463372503569758885355n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint128_euint128(
      340282366920938463463366528658094296109n,
      encryptedAmount.handles[0],
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

  it('test operator "eq" overload (uint128, euint128) => ebool test 2 (340282366920938463463369457203716781685, 340282366920938463463369457203716781689)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add128(340282366920938463463369457203716781689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint128_euint128(
      340282366920938463463369457203716781685n,
      encryptedAmount.handles[0],
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

  it('test operator "eq" overload (uint128, euint128) => ebool test 3 (340282366920938463463369457203716781689, 340282366920938463463369457203716781689)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add128(340282366920938463463369457203716781689n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint128_euint128(
      340282366920938463463369457203716781689n,
      encryptedAmount.handles[0],
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

  it('test operator "eq" overload (uint128, euint128) => ebool test 4 (340282366920938463463369457203716781689, 340282366920938463463369457203716781685)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);

    input.add128(340282366920938463463369457203716781685n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.eq_uint128_euint128(
      340282366920938463463369457203716781689n,
      encryptedAmount.handles[0],
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

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (31318, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(31318n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 30n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (6, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(6n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (10, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shr_euint16_uint8(encryptedAmount.handles[0], 10n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (10, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add16(10n);

    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.shr_euint16_uint8(encryptedAmount.handles[0], 6n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract4.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 1 (340282366920938463463371455311449119019, 51054)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(340282366920938463463371455311449119019n);
    input.add16(51054n);
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
      [handle]: 340282366920938463463371455311449119019n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 2 (51050, 51054)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(51050n);
    input.add16(51054n);
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
      [handle]: 51054n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 3 (51054, 51054)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(51054n);
    input.add16(51054n);
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
      [handle]: 51054n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "max" overload (euint128, euint16) => euint128 test 4 (51054, 51050)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add128(51054n);
    input.add16(51050n);
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
      [handle]: 51054n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 1 (76, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(76n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 19n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 2 (6, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 129n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 3 (10, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 130n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "rotr" overload (euint8, euint8) => euint8 test 4 (10, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract4Address, this.signer.address);
    input.add8(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract4.rotr_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract4.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 40n,
    };
    assert.deepEqual(res, expectedRes);
  });
});
