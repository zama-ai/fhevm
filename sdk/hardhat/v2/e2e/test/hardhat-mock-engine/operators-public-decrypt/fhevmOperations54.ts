import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { assert } from 'chai';
import * as hre from 'hardhat';
import { ethers } from 'hardhat';

import type { FHEVMPublicDecryptTestSuite4 } from '../../../typechain-types/contracts/operators-public-decrypt/FHEVMPublicDecryptTestSuite4';
import { Signers, getSigners, initSigners } from '../signers';

async function deployFHEVMTestFixture4(signer: HardhatEthersSigner): Promise<FHEVMPublicDecryptTestSuite4> {
  const admin = signer;

  const contractFactory = await ethers.getContractFactory('FHEVMPublicDecryptTestSuite4');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

describe('FHEVM operations 54', function () {
  let signers: Signers;
  let signer: HardhatEthersSigner;
  let contract4: FHEVMPublicDecryptTestSuite4;
  let contract4Address: string;

  before(async function () {
    await initSigners();
    signers = await getSigners();
    signer = signers.alice;

    contract4 = await deployFHEVMTestFixture4(signer);
    contract4Address = await contract4.getAddress();
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18446307955039574325, 45271)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add64(18446307955039574325n);
    input.add16(45271n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint64();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18446307955039574325n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (45267, 45271)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add64(45267n);
    input.add16(45271n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint64();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45271n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (45271, 45271)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add64(45271n);
    input.add16(45271n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint64();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45271n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (45271, 45267)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add64(45271n);
    input.add16(45267n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_euint64_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint64();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45271n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (213, 26)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add8(213n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint8();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 223n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (22, 26)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add8(22n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint8();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 30n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (26, 26)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add8(26n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint8();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (26, 22)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add8(26n);
    input.add8(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.or_euint8_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint8();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 30n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (41810, 8)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add16(41810n);

    const encryptedAmount = await input.encrypt();
    const tx = await contract4.shr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract4.resEuint16();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 163n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await contract4.shr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract4.resEuint16();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await contract4.shr_euint16_uint8(encryptedAmount.handles[0], 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract4.resEuint16();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await contract4.shr_euint16_uint8(encryptedAmount.handles[0], 4n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract4.resEuint16();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18445863906332305427, 18443180247415512627)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);

    input.add64(18443180247415512627n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_uint64_euint64(
      18445863906332305427n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint64();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18445863906332305427n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18439875117400843375, 18439875117400843379)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);

    input.add64(18439875117400843379n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_uint64_euint64(
      18439875117400843375n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint64();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18439875117400843379, 18439875117400843379)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);

    input.add64(18439875117400843379n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_uint64_euint64(
      18439875117400843379n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint64();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18439875117400843379, 18439875117400843375)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);

    input.add64(18439875117400843375n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_uint64_euint64(
      18439875117400843379n,
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint64();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 1 (34, 115792089237316195423570985008687907853269984665640564039457577752723022763875)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add8(34n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577752723022763875n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 2 (30, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add8(30n);
    input.add256(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 3 (34, 34)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add8(34n);
    input.add256(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 4 (34, 30)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add8(34n);
    input.add256(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.eq_euint8_euint256(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (91, 2)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add16(91n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint16();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 182n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (14, 16)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add16(14n);
    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint16();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 224n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (9, 9)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add16(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint16();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 81n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (16, 14)', async function () {
    const input = hre.fhevm.createEncryptedInput(contract4Address, signer.address);
    input.add16(16n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.mul_euint16_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract4.resEuint16();
    const res = await hre.fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 224n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
