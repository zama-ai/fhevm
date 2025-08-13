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

describe('FHEVM operations 38', function () {
  before(async function () {
    this.signer = await getSigner(38);

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

  it('test operator "or" overload (euint256, euint256) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457578328316009366477, 115792089237316195423570985008687907853269984665640564039457579838666730423367)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578328316009366477n);
    input.add256(115792089237316195423570985008687907853269984665640564039457579838666730423367n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457582867168732497871n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 2 (115792089237316195423570985008687907853269984665640564039457578328316009366473, 115792089237316195423570985008687907853269984665640564039457578328316009366477)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578328316009366473n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578328316009366477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457578328316009366477n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 3 (115792089237316195423570985008687907853269984665640564039457578328316009366477, 115792089237316195423570985008687907853269984665640564039457578328316009366477)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578328316009366477n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578328316009366477n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457578328316009366477n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "or" overload (euint256, euint256) => euint256 test 4 (115792089237316195423570985008687907853269984665640564039457578328316009366477, 115792089237316195423570985008687907853269984665640564039457578328316009366473)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457578328316009366477n);
    input.add256(115792089237316195423570985008687907853269984665640564039457578328316009366473n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.or_euint256_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 115792089237316195423570985008687907853269984665640564039457578328316009366477n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ge" overload (euint128, euint64) => ebool test 1 (340282366920938463463365721076714685915, 18445075217321741283)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(340282366920938463463365721076714685915n);
    input.add64(18445075217321741283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint128_euint64(
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

  it('test operator "ge" overload (euint128, euint64) => ebool test 2 (18445075217321741279, 18445075217321741283)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(18445075217321741279n);
    input.add64(18445075217321741283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint128_euint64(
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

  it('test operator "ge" overload (euint128, euint64) => ebool test 3 (18445075217321741283, 18445075217321741283)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(18445075217321741283n);
    input.add64(18445075217321741283n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint128_euint64(
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

  it('test operator "ge" overload (euint128, euint64) => ebool test 4 (18445075217321741283, 18445075217321741279)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add128(18445075217321741283n);
    input.add64(18445075217321741279n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ge_euint128_euint64(
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

  it('test operator "and" overload (euint16, euint128) => euint128 test 1 (37268, 340282366920938463463372636374506682063)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(37268n);
    input.add128(340282366920938463463372636374506682063n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4228n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 2 (37264, 37268)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(37264n);
    input.add128(37268n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 37264n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 3 (37268, 37268)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(37268n);
    input.add128(37268n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 37268n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint16, euint128) => euint128 test 4 (37268, 37264)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add16(37268n);
    input.add128(37264n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint16_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 37264n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 1 (115792089237316195423570985008687907853269984665640564039457575065969206793813, 1232554331)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(115792089237316195423570985008687907853269984665640564039457575065969206793813n);
    input.add32(1232554331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1080033361n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 2 (1232554327, 1232554331)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(1232554327n);
    input.add32(1232554331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1232554323n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 3 (1232554331, 1232554331)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(1232554331n);
    input.add32(1232554331n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1232554331n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "and" overload (euint256, euint32) => euint256 test 4 (1232554331, 1232554327)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add256(1232554331n);
    input.add32(1232554327n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.and_euint256_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1232554323n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 1 (197, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(197n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 20n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 2 (6, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(6n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 24n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 3 (10, 10)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(10n);
    input.add8(10n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 40n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "shl" overload (euint8, euint8) => euint8 test 4 (10, 6)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(10n);
    input.add8(6n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.shl_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract3.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 128n,
    };
    assert.deepEqual(res, expectedRes);
  });

  it('test operator "ne" overload (euint8, euint16) => ebool test 1 (112, 20102)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(112n);
    input.add16(20102n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint8_euint16(
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

  it('test operator "ne" overload (euint8, euint16) => ebool test 2 (108, 112)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(108n);
    input.add16(112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint8_euint16(
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

  it('test operator "ne" overload (euint8, euint16) => ebool test 3 (112, 112)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(112n);
    input.add16(112n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint8_euint16(
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

  it('test operator "ne" overload (euint8, euint16) => ebool test 4 (112, 108)', async function () {
    const input = this.instance.createEncryptedInput(this.contract3Address, this.signer.address);
    input.add8(112n);
    input.add16(108n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract3.ne_euint8_euint16(
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
});
