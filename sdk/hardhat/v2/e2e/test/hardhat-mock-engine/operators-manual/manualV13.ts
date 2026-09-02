import { expect } from 'chai';
import { ethers } from 'hardhat';
import * as hre from 'hardhat';

import type { FHEVMManualTestSuite } from '../../../typechain-types';
import { Signers, getSigners, initSigners } from '../signers';

/**
 * Manual-operation tests for the operators @fhevm/solidity 0.13.3 added: `FHE.isIn`, `FHE.sum`, and
 * the euint64 shift/rotate forms taking an encrypted or plain shift amount.
 *
 * Ported from the fhevm repository's e2e suite
 * (<fhevm>/test-suite/e2e/test/fhevmOperations/manual.ts), adapted to this plugin's API:
 * `instance.encryptTypedValues({ values })` becomes the `createEncryptedInput(...).addNN(...)`
 * builder, and `instance.publicDecrypt` becomes `hre.fhevm.publicDecrypt` — which returns the same
 * handle-keyed `clearValues` shape.
 *
 * Upstream's `mulDiv` and `toExternalE*` tests are deliberately absent: those operators are not in
 * 0.13.3 and are expected in v14, so the contract does not declare them either.
 */

const UINT64_MASK = (1n << 64n) - 1n;
const OVERSIZED_SHIFT_64 = 70n;
const REDUCED_SHIFT_64 = 6n;
const SHIFT_ROTATE_VALUE_64 = 0x123456789abcdef0n;

function rotl64(value: bigint, shift: bigint): bigint {
  const normalized = shift % 64n;
  return ((value << normalized) | (value >> (64n - normalized))) & UINT64_MASK;
}

function rotr64(value: bigint, shift: bigint): bigint {
  const normalized = shift % 64n;
  return ((value >> normalized) | (value << (64n - normalized))) & UINT64_MASK;
}

/** Runs the transaction, then publicly decrypts the euint64 result the suite stored. */
async function decrypt64Result(
  contract: FHEVMManualTestSuite,
  txPromise: Promise<{ wait(): Promise<unknown> }>,
): Promise<bigint> {
  await (await txPromise).wait();
  const handle = (await contract.resEuint64()) as `0x${string}`;
  const res = await hre.fhevm.publicDecrypt([handle]);
  return res.clearValues[handle] as bigint;
}

async function deployFHEVMManualTestFixture(): Promise<FHEVMManualTestSuite> {
  const signers = await getSigners();
  const contractFactory = await ethers.getContractFactory('FHEVMManualTestSuite');
  const contract = await contractFactory.connect(signers.alice).deploy();
  await contract.waitForDeployment();
  return contract;
}

describe('FHEVM manual operations (isIn / sum / shift-rotate)', function () {
  let signers: Signers;
  let contractAddress: string;
  let contract: FHEVMManualTestSuite;

  beforeEach(async function () {
    await initSigners();
    signers = await getSigners();
    contract = await deployFHEVMManualTestFixture();
    contractAddress = await contract.getAddress();
  });

  it('shr(euint64, uint8) applies modulo semantics for indexes > bit width', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add64(SHIFT_ROTATE_VALUE_64);
    const encryptedAmount = await input.encrypt();
    const res = await decrypt64Result(
      contract,
      contract.test_shr_euint64_uint8(encryptedAmount.handles[0], OVERSIZED_SHIFT_64, encryptedAmount.inputProof),
    );
    expect(res).to.equal(SHIFT_ROTATE_VALUE_64 >> REDUCED_SHIFT_64);
  });

  it('shr(euint64, euint8) applies modulo semantics for indexes > bit width', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add64(SHIFT_ROTATE_VALUE_64);
    input.add8(OVERSIZED_SHIFT_64);
    const encryptedAmount = await input.encrypt();
    const res = await decrypt64Result(
      contract,
      contract.test_shr_euint64_euint8(
        encryptedAmount.handles[0],
        encryptedAmount.handles[1],
        encryptedAmount.inputProof,
      ),
    );
    expect(res).to.equal(SHIFT_ROTATE_VALUE_64 >> REDUCED_SHIFT_64);
  });

  it('shl(euint64, uint8) applies modulo semantics for indexes > bit width', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add64(SHIFT_ROTATE_VALUE_64);
    const encryptedAmount = await input.encrypt();
    const res = await decrypt64Result(
      contract,
      contract.test_shl_euint64_uint8(encryptedAmount.handles[0], OVERSIZED_SHIFT_64, encryptedAmount.inputProof),
    );
    expect(res).to.equal((SHIFT_ROTATE_VALUE_64 << REDUCED_SHIFT_64) & UINT64_MASK);
  });

  it('shl(euint64, euint8) applies modulo semantics for indexes > bit width', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add64(SHIFT_ROTATE_VALUE_64);
    input.add8(OVERSIZED_SHIFT_64);
    const encryptedAmount = await input.encrypt();
    const res = await decrypt64Result(
      contract,
      contract.test_shl_euint64_euint8(
        encryptedAmount.handles[0],
        encryptedAmount.handles[1],
        encryptedAmount.inputProof,
      ),
    );
    expect(res).to.equal((SHIFT_ROTATE_VALUE_64 << REDUCED_SHIFT_64) & UINT64_MASK);
  });

  it('rotl(euint64, uint8) applies modulo semantics for indexes > bit width', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add64(SHIFT_ROTATE_VALUE_64);
    const encryptedAmount = await input.encrypt();
    const res = await decrypt64Result(
      contract,
      contract.test_rotl_euint64_uint8(encryptedAmount.handles[0], OVERSIZED_SHIFT_64, encryptedAmount.inputProof),
    );
    expect(res).to.equal(rotl64(SHIFT_ROTATE_VALUE_64, REDUCED_SHIFT_64));
  });

  it('rotr(euint64, uint8) applies modulo semantics for indexes > bit width', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add64(SHIFT_ROTATE_VALUE_64);
    const encryptedAmount = await input.encrypt();
    const res = await decrypt64Result(
      contract,
      contract.test_rotr_euint64_uint8(encryptedAmount.handles[0], OVERSIZED_SHIFT_64, encryptedAmount.inputProof),
    );
    expect(res).to.equal(rotr64(SHIFT_ROTATE_VALUE_64, REDUCED_SHIFT_64));
  });

  it('rotr(euint64, euint8) applies modulo semantics for indexes > bit width', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add64(SHIFT_ROTATE_VALUE_64);
    input.add8(OVERSIZED_SHIFT_64);
    const encryptedAmount = await input.encrypt();
    const res = await decrypt64Result(
      contract,
      contract.test_rotr_euint64_euint8(
        encryptedAmount.handles[0],
        encryptedAmount.handles[1],
        encryptedAmount.inputProof,
      ),
    );
    expect(res).to.equal(rotr64(SHIFT_ROTATE_VALUE_64, REDUCED_SHIFT_64));
  });

  it('rotl(euint64, euint8) applies modulo semantics for indexes > bit width', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add64(SHIFT_ROTATE_VALUE_64);
    input.add8(OVERSIZED_SHIFT_64);
    const encryptedAmount = await input.encrypt();
    const res = await decrypt64Result(
      contract,
      contract.test_rotl_euint64_euint8(
        encryptedAmount.handles[0],
        encryptedAmount.handles[1],
        encryptedAmount.inputProof,
      ),
    );
    expect(res).to.equal(rotl64(SHIFT_ROTATE_VALUE_64, REDUCED_SHIFT_64));
  });

  it('test operator "sum" euint16 - two elements', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add16(1000n);
    input.add16(2000n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_sum_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract.resEuint16();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(3000n);
  });

  it('test operator "sum" euint32 - two elements', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add32(100000n);
    input.add32(200000n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_sum_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract.resEuint32();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(300000n);
  });

  it('test operator "sum" euint8 - three elements', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add8(10n);
    input.add8(20n);
    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_sum_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.handles[2],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract.resEuint8();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(60n);
  });

  it('test operator "sum" euint64 - two elements', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add64(1000000n);
    input.add64(2000000n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_sum_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract.resEuint64();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(3000000n);
  });

  it('test operator "sum" euint128 - two elements', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add128(100000000000000000000n);
    input.add128(200000000000000000000n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_sum_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await contract.resEuint128();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(300000000000000000000n);
  });

  it('test operator "sum" euint8 - duplicate handle counted twice', async function () {
    const value = 7;
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add8(value);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_sum_euint8_duplicate(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEuint8();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(value * 2);
  });

  it('test operator "sum" euint8 - uninitialized element treated as 0', async function () {
    const tx = await contract.test_sum_euint8_uninitialized();
    await tx.wait();
    const handle = await contract.resEuint8();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(5n);
  });

  it('test operator "sum" euint8 - empty array returns 0', async function () {
    const tx = await contract.test_sum_euint8_empty();
    await tx.wait();
    const handle = await contract.resEuint8();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(0n);
  });

  it('test operator "sum" euint8 - single element returns fresh handle', async function () {
    const value = 42;
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add8(value);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_sum_euint8_single(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEuint8();
    expect(handle).to.not.equal(encryptedAmount.handles[0]);
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(BigInt(value));
  });

  it('test operator "sum" euint8 - 100 elements at max array size', async function () {
    const tx = await contract.test_sum_euint8_max_array();
    await tx.wait();
    const handle = await contract.resEuint8();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(100n);
  });

  it('test operator "isIn" euint8 - value found in set', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add8(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_isIn_euint8_found(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" euint8 - value not found in set', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add8(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_isIn_euint8_not_found(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(false);
  });

  it('test operator "isIn" euint16 - value found in set', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add16(1000n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_isIn_euint16(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" euint32 - value found in set', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add32(100000n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_isIn_euint32(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" euint64 - value found in set', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add64(1000000000n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_isIn_euint64(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" euint128 - value found in set', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add128(10000000000000000000n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_isIn_euint128(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" euint8 - uninitialized value treated as 0 (found)', async function () {
    const tx = await contract.test_isIn_euint8_uninitialized();
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" euint8 - single element set, found', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_isIn_euint8_single_element(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" euint8 - 100 elements at max array size', async function () {
    const tx = await contract.test_isIn_euint8_max_array();
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" euint8 - empty set returns false', async function () {
    const tx = await contract.test_isIn_euint8_empty_set();
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(false);
  });

  it('test operator "isIn" euint8 - zero-initialized set, enc(0) found', async function () {
    const tx = await contract.test_isIn_euint8_zero_initialized_set();
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" euint8 - max type value (255) found in set', async function () {
    const tx = await contract.test_isIn_euint8_max_value_found();
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" euint8 - single element set, not found', async function () {
    const tx = await contract.test_isIn_euint8_single_element_not_found();
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(false);
  });

  it('test operator "isIn" eaddress - value found in set', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.addAddress('0x2222222222222222222222222222222222222222');
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_isIn_eaddress_found(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" eaddress - value not found in set', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.addAddress('0x4444444444444444444444444444444444444444');
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_isIn_eaddress_not_found(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(false);
  });

  it('test operator "isIn" euint256 - value found in set', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add256(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_isIn_euint256_found(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(true);
  });

  it('test operator "isIn" euint256 - value not found in set', async function () {
    const input = hre.fhevm.createEncryptedInput(contractAddress, signers.alice.address);
    input.add256(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await contract.test_isIn_euint256_not_found(encryptedAmount.handles[0], encryptedAmount.inputProof);
    await tx.wait();
    const handle = await contract.resEbool();
    const res = await hre.fhevm.publicDecrypt([handle]);
    expect(res.clearValues[handle]).to.equal(false);
  });
});
