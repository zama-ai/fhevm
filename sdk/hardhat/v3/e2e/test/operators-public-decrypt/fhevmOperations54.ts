import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { assert } from 'chai';
import { network } from 'hardhat';

import type { FHEVMPublicDecryptTestSuite4 } from '../../types/ethers-contracts/index.ts';
import { type Signers, getSigners } from '../utils/signers.ts';

// The v2 file, mechanically adapted: `hre.fhevm` → the connection's `fhevm`, handles narrowed with `at`,
// result handles typed as Hex.
const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;

// `handles` is `Hex[]`: the i-th handle, or a loud failure.
function at(handles: readonly Hex[], i: number): Hex {
  const handle = handles[i];
  if (handle === undefined) throw new Error(`encrypt() returned no handle #${String(i)}`);
  return handle;
}

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
  let contract4Address: Hex;

  before(async function () {
    signers = await getSigners(connection);
    signer = signers.alice;

    contract4 = await deployFHEVMTestFixture4(signer);
    contract4Address = (await contract4.getAddress()) as Hex;
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 1 (18446307955039574325, 45271)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add64(18446307955039574325n);
    input.add16(45271n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_euint64_euint16(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint64()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18446307955039574325n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 2 (45267, 45271)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add64(45267n);
    input.add16(45271n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_euint64_euint16(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint64()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45271n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 3 (45271, 45271)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add64(45271n);
    input.add16(45271n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_euint64_euint16(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint64()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45271n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (euint64, euint16) => euint64 test 4 (45271, 45267)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add64(45271n);
    input.add16(45267n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_euint64_euint16(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint64()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 45271n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 1 (213, 26)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add8(213n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.or_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint8()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 223n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 2 (22, 26)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add8(22n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.or_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint8()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 30n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 3 (26, 26)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add8(26n);
    input.add8(26n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.or_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint8()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 26n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "or" overload (euint8, euint8) => euint8 test 4 (26, 22)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add8(26n);
    input.add8(22n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.or_euint8_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint8()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 30n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 1 (41810, 8)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add16(41810n);

    const encryptedAmount = await input.encrypt();
    const tx = await contract4.shr_euint16_uint8(at(encryptedAmount.handles, 0), 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = (await contract4.resEuint16()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 163n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 2 (4, 8)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add16(4n);

    const encryptedAmount = await input.encrypt();
    const tx = await contract4.shr_euint16_uint8(at(encryptedAmount.handles, 0), 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = (await contract4.resEuint16()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 3 (8, 8)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await contract4.shr_euint16_uint8(at(encryptedAmount.handles, 0), 8n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = (await contract4.resEuint16()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "shr" overload (euint16, uint8) => euint16 test 4 (8, 4)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add16(8n);

    const encryptedAmount = await input.encrypt();
    const tx = await contract4.shr_euint16_uint8(at(encryptedAmount.handles, 0), 4n, encryptedAmount.inputProof);
    await tx.wait();
    const handle = (await contract4.resEuint16()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 1 (18445863906332305427, 18443180247415512627)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);

    input.add64(18443180247415512627n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_uint64_euint64(
      18445863906332305427n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint64()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18445863906332305427n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 2 (18439875117400843375, 18439875117400843379)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);

    input.add64(18439875117400843379n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_uint64_euint64(
      18439875117400843375n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint64()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 3 (18439875117400843379, 18439875117400843379)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);

    input.add64(18439875117400843379n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_uint64_euint64(
      18439875117400843379n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint64()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "max" overload (uint64, euint64) => euint64 test 4 (18439875117400843379, 18439875117400843375)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);

    input.add64(18439875117400843375n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.max_uint64_euint64(
      18439875117400843379n,
      at(encryptedAmount.handles, 0),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint64()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 18439875117400843379n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 1 (34, 115792089237316195423570985008687907853269984665640564039457577752723022763875)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add8(34n);
    input.add256(115792089237316195423570985008687907853269984665640564039457577752723022763875n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.eq_euint8_euint256(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEbool()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 2 (30, 34)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add8(30n);
    input.add256(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.eq_euint8_euint256(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEbool()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 3 (34, 34)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add8(34n);
    input.add256(34n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.eq_euint8_euint256(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEbool()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "eq" overload (euint8, euint256) => ebool test 4 (34, 30)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add8(34n);
    input.add256(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.eq_euint8_euint256(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEbool()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 1 (91, 2)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add16(91n);
    input.add8(2n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.mul_euint16_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint16()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 182n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 2 (14, 16)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add16(14n);
    input.add8(16n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.mul_euint16_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint16()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 224n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 3 (9, 9)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add16(9n);
    input.add8(9n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.mul_euint16_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint16()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 81n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "mul" overload (euint16, euint8) => euint16 test 4 (16, 14)', async function () {
    const input = fhevm.createEncryptedInput(contract4Address, signer.address as Hex);
    input.add16(16n);
    input.add8(14n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract4.mul_euint16_euint8(
      at(encryptedAmount.handles, 0),
      at(encryptedAmount.handles, 1),
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = (await contract4.resEuint16()) as Hex;
    const res = await fhevm.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 224n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });
});
