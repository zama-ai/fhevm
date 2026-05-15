import { assert, expect } from 'chai';
import { ethers } from 'hardhat';

import type { FHEVMManualTestSuite } from '../../types/contracts/operations/FHEVMManualTestSuite';
import { createInstance } from '../instance';
import { getSigner } from '../signers';
import { bigIntToBytes256 } from '../utils';

async function deployFHEVMManualTestFixture(): Promise<FHEVMManualTestSuite> {
  const admin = await getSigner(119);

  const contractFactory = await ethers.getContractFactory('FHEVMManualTestSuite');
  const contract = await contractFactory.connect(admin).deploy();
  await contract.waitForDeployment();

  return contract;
}

const UINT64_MASK = (1n << 64n) - 1n;
const OVERSIZED_SHIFT_64 = 70n;
const REDUCED_SHIFT_64 = 6n;
const SHIFT_ROTATE_VALUE_64 = 0x123456789abcdef0n;
const addr = (value: string) => ethers.getAddress(value);
const ADDR_A = addr('0x8ba1f109551bd432803012645ac136ddd64dba72');
const ADDR_B = addr('0x8881f109551bd432803012645ac136ddd64dba72');
const ADDR_C = addr('0x9ba1f109551bd432803012645ac136ddd64dba72');
const ADDR_D = addr('0x9aa1f109551bd432803012645ac136ddd64dba72');

function rotl64(value: bigint, shift: bigint): bigint {
  const normalized = shift % 64n;
  return ((value << normalized) | (value >> (64n - normalized))) & UINT64_MASK;
}

function rotr64(value: bigint, shift: bigint): bigint {
  const normalized = shift % 64n;
  return ((value >> normalized) | (value << (64n - normalized))) & UINT64_MASK;
}

async function decrypt64Result(
  instance: Awaited<ReturnType<typeof createInstance>>,
  contract: FHEVMManualTestSuite,
  txPromise: Promise<{ wait(): Promise<unknown> }>,
): Promise<bigint> {
  await (await txPromise).wait();
  const handle = await contract.resEuint64();
  const res = await instance.publicDecrypt([handle]);
  return res.clearValues[handle] as bigint;
}

describe('FHEVM manual operations', function () {
  beforeEach(async function () {
    this.signer = await getSigner(119);

    const contract = await deployFHEVMManualTestFixture();
    this.contractAddress = await contract.getAddress();
    this.contract = contract;
    const instance = await createInstance();
    this.instance = instance;
  });

  // Keep this regression isolated so operators CI can target only the
  // oversized-index path without pulling the whole manual suite.
  describe('FHEVM oversized shift and rotate indexes', function () {
    it('shr(euint64, uint8) applies modulo semantics for indexes > bit width', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add64(SHIFT_ROTATE_VALUE_64);
      const encryptedAmount = await input.encrypt();
      const res = await decrypt64Result(
        this.instance,
        this.contract,
        this.contract.test_shr_euint64_uint8(
          encryptedAmount.handles[0],
          OVERSIZED_SHIFT_64,
          encryptedAmount.inputProof,
        ),
      );
      assert.equal(res, SHIFT_ROTATE_VALUE_64 >> REDUCED_SHIFT_64);
    });

    it('shr(euint64, euint8) applies modulo semantics for indexes > bit width', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add64(SHIFT_ROTATE_VALUE_64);
      input.add8(OVERSIZED_SHIFT_64);
      const encryptedAmount = await input.encrypt();
      const res = await decrypt64Result(
        this.instance,
        this.contract,
        this.contract.test_shr_euint64_euint8(
          encryptedAmount.handles[0],
          encryptedAmount.handles[1],
          encryptedAmount.inputProof,
        ),
      );
      assert.equal(res, SHIFT_ROTATE_VALUE_64 >> REDUCED_SHIFT_64);
    });

    it('shl(euint64, uint8) applies modulo semantics for indexes > bit width', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add64(SHIFT_ROTATE_VALUE_64);
      const encryptedAmount = await input.encrypt();
      const res = await decrypt64Result(
        this.instance,
        this.contract,
        this.contract.test_shl_euint64_uint8(
          encryptedAmount.handles[0],
          OVERSIZED_SHIFT_64,
          encryptedAmount.inputProof,
        ),
      );
      assert.equal(res, (SHIFT_ROTATE_VALUE_64 << REDUCED_SHIFT_64) & UINT64_MASK);
    });

    it('shl(euint64, euint8) applies modulo semantics for indexes > bit width', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add64(SHIFT_ROTATE_VALUE_64);
      input.add8(OVERSIZED_SHIFT_64);
      const encryptedAmount = await input.encrypt();
      const res = await decrypt64Result(
        this.instance,
        this.contract,
        this.contract.test_shl_euint64_euint8(
          encryptedAmount.handles[0],
          encryptedAmount.handles[1],
          encryptedAmount.inputProof,
        ),
      );
      assert.equal(res, (SHIFT_ROTATE_VALUE_64 << REDUCED_SHIFT_64) & UINT64_MASK);
    });

    it('rotl(euint64, uint8) applies modulo semantics for indexes > bit width', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add64(SHIFT_ROTATE_VALUE_64);
      const encryptedAmount = await input.encrypt();
      const res = await decrypt64Result(
        this.instance,
        this.contract,
        this.contract.test_rotl_euint64_uint8(
          encryptedAmount.handles[0],
          OVERSIZED_SHIFT_64,
          encryptedAmount.inputProof,
        ),
      );
      assert.equal(res, rotl64(SHIFT_ROTATE_VALUE_64, REDUCED_SHIFT_64));
    });

    it('rotr(euint64, uint8) applies modulo semantics for indexes > bit width', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add64(SHIFT_ROTATE_VALUE_64);
      const encryptedAmount = await input.encrypt();
      const res = await decrypt64Result(
        this.instance,
        this.contract,
        this.contract.test_rotr_euint64_uint8(
          encryptedAmount.handles[0],
          OVERSIZED_SHIFT_64,
          encryptedAmount.inputProof,
        ),
      );
      assert.equal(res, rotr64(SHIFT_ROTATE_VALUE_64, REDUCED_SHIFT_64));
    });

    it('rotr(euint64, euint8) applies modulo semantics for indexes > bit width', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add64(SHIFT_ROTATE_VALUE_64);
      input.add8(OVERSIZED_SHIFT_64);
      const encryptedAmount = await input.encrypt();
      const res = await decrypt64Result(
        this.instance,
        this.contract,
        this.contract.test_rotr_euint64_euint8(
          encryptedAmount.handles[0],
          encryptedAmount.handles[1],
          encryptedAmount.inputProof,
        ),
      );
      assert.equal(res, rotr64(SHIFT_ROTATE_VALUE_64, REDUCED_SHIFT_64));
    });

    it('rotl(euint64, euint8) applies modulo semantics for indexes > bit width', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add64(SHIFT_ROTATE_VALUE_64);
      input.add8(OVERSIZED_SHIFT_64);
      const encryptedAmount = await input.encrypt();
      const res = await decrypt64Result(
        this.instance,
        this.contract,
        this.contract.test_rotl_euint64_euint8(
          encryptedAmount.handles[0],
          encryptedAmount.handles[1],
          encryptedAmount.inputProof,
        ),
      );
      assert.equal(res, rotl64(SHIFT_ROTATE_VALUE_64, REDUCED_SHIFT_64));
    });
  });

  it('Select works returning if false', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addBool(false);
    input.add32(3);
    input.add32(4);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_select(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.handles[2],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 4n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('Select works returning if true', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addBool(true);
    input.add32(3);
    input.add32(4);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_select(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.handles[2],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 3n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('Select ebool', async function () {
    const tx = await this.contract.test_select_ebool(true, false, true);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const tx2 = await this.contract.test_select_ebool(false, false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle, handle2]);
    const expectedRes = {
      [handle]: false,
      [handle2]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('Select works for eaddress returning if false', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addBool(false);
    input.addAddress(ADDR_A);
    input.addAddress(ADDR_B);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_select_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.handles[2],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resAdd();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: ADDR_B,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('Select works for eaddress returning if true', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addBool(true);
    input.addAddress(ADDR_A);
    input.addAddress(ADDR_B);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_select_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.handles[2],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resAdd();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: ADDR_A,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool eq ebool', async function () {
    const tx = await this.contract.eqEbool(true, true);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const tx2 = await this.contract.eqEbool(false, false);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();
    const tx3 = await this.contract.eqEbool(false, true);
    await tx3.wait();
    const handle3 = await await this.contract.resEbool();
    const tx4 = await this.contract.eqEbool(true, false);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: true,
      [handle2]: true,
      [handle3]: false,
      [handle4]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool eq ebool - ScalarL', async function () {
    const tx = await this.contract.eqEboolScalarL(true, true);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const tx2 = await this.contract.eqEboolScalarL(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();
    const tx3 = await this.contract.eqEboolScalarL(false, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();
    const tx4 = await this.contract.eqEboolScalarL(true, false);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: true,
      [handle2]: false,
      [handle3]: true,
      [handle4]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool eq ebool - ScalarR', async function () {
    const tx = await this.contract.eqEboolScalarL(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const tx2 = await this.contract.eqEboolScalarL(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();
    const tx3 = await this.contract.eqEboolScalarL(true, true);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();
    const tx4 = await this.contract.eqEboolScalarL(true, false);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: true,
      [handle2]: false,
      [handle3]: true,
      [handle4]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool ne ebool', async function () {
    const tx = await this.contract.neEbool(true, true);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const tx2 = await this.contract.neEbool(false, false);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();
    const tx3 = await this.contract.neEbool(false, true);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();
    const tx4 = await this.contract.neEbool(true, false);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: false,
      [handle3]: true,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool ne ebool - ScalarL', async function () {
    const tx = await this.contract.neEboolScalarL(true, true);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const tx2 = await this.contract.neEboolScalarL(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();
    const tx3 = await this.contract.neEboolScalarL(false, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();
    const tx4 = await this.contract.neEboolScalarL(true, false);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: true,
      [handle3]: false,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool ne ebool - ScalarR', async function () {
    const tx = await this.contract.neEboolScalarL(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const tx2 = await this.contract.neEboolScalarL(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();
    const tx3 = await this.contract.neEboolScalarL(true, true);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();
    const tx4 = await this.contract.neEboolScalarL(true, false);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: true,
      [handle3]: false,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress eq eaddress,eaddress true', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    input.addAddress(ADDR_A);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_eaddress_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress eq eaddress,eaddress false', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    input.addAddress(ADDR_C);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_eaddress_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress eq scalar eaddress,address true', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    const b = ADDR_A;
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_eaddress_address(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress eq scalar eaddress,address false', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    const b = ADDR_D;
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_eaddress_address(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress eq scalar address,eaddress true', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    const b = ADDR_A;
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_address_eaddress(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress eq scalar address,eaddress false', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    const b = ADDR_D;
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_eq_address_eaddress(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress ne eaddress,eaddress false', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    input.addAddress(ADDR_A);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_eaddress_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress ne eaddress,eaddress true', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    input.addAddress(ADDR_C);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_eaddress_eaddress(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress ne scalar eaddress,address false', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    const b = ADDR_A;
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_eaddress_address(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress ne scalar eaddress,address true', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    const b = ADDR_D;
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_eaddress_address(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress ne scalar address,eaddress false', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    const b = ADDR_A;
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_address_eaddress(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('eaddress ne scalar address,eaddress true', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress(ADDR_A);
    const b = ADDR_D;
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_ne_address_eaddress(encryptedAmount.handles[0], b, encryptedAmount.inputProof);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint8 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint8_cast(true);
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint8 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint8_cast(false);
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint16 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint16_cast(true);
    await tx.wait();
    const handle = await this.contract.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint16 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint16_cast(false);
    await tx.wait();
    const handle = await this.contract.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint32 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint32_cast(true);
    await tx.wait();
    const handle = await this.contract.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint32 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint32_cast(false);
    await tx.wait();
    const handle = await this.contract.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint64 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint64_cast(true);
    await tx.wait();
    const handle = await this.contract.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint64 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint64_cast(false);
    await tx.wait();
    const handle = await this.contract.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint128 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint128_cast(true);
    await tx.wait();
    const handle = await this.contract.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint128 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint128_cast(false);
    await tx.wait();
    const handle = await this.contract.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint256 casting works with true', async function () {
    const tx = await this.contract.test_ebool_to_euint256_cast(true);
    await tx.wait();
    const handle = await this.contract.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 1n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool to euint256 casting works with false', async function () {
    const tx = await this.contract.test_ebool_to_euint256_cast(false);
    await tx.wait();
    const handle = await this.contract.resEuint256();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 0n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('euint128 to euint8 casting works', async function () {
    const tx = await this.contract.test_euint128_to_euint8_cast(7668756464674969496544n);
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: 224n,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool not for false is true', async function () {
    const tx = await this.contract.test_ebool_not(false);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool not for true is false', async function () {
    const tx = await this.contract.test_ebool_not(true);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    const expectedRes = {
      [handle]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool and', async function () {
    const tx = await this.contract.test_ebool_and(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();

    const tx2 = await this.contract.test_ebool_and(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();

    const tx3 = await this.contract.test_ebool_and(true, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();

    const tx4 = await this.contract.test_ebool_and(true, true);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();

    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: false,
      [handle3]: false,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool or', async function () {
    const tx = await this.contract.test_ebool_or(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();

    const tx2 = await this.contract.test_ebool_or(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();

    const tx3 = await this.contract.test_ebool_or(true, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();

    const tx4 = await this.contract.test_ebool_or(true, true);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();

    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: true,
      [handle3]: true,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool xor', async function () {
    const tx = await this.contract.test_ebool_xor(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();

    const tx2 = await this.contract.test_ebool_xor(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();

    const tx3 = await this.contract.test_ebool_xor(true, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();

    const tx4 = await this.contract.test_ebool_xor(true, true);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();

    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: true,
      [handle3]: true,
      [handle4]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool xor scalarL', async function () {
    const tx = await this.contract.test_ebool_xor_scalarL(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();

    const tx2 = await this.contract.test_ebool_xor_scalarL(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();

    const tx3 = await this.contract.test_ebool_xor_scalarL(true, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();

    const tx4 = await this.contract.test_ebool_xor_scalarL(true, true);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();

    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: true,
      [handle3]: true,
      [handle4]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool xor scalarR', async function () {
    const tx = await this.contract.test_ebool_xor_scalarR(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();

    const tx2 = await this.contract.test_ebool_xor_scalarR(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();

    const tx3 = await this.contract.test_ebool_xor_scalarR(true, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();

    const tx4 = await this.contract.test_ebool_xor_scalarR(true, true);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();

    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: true,
      [handle3]: true,
      [handle4]: false,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool or scalarL', async function () {
    const tx = await this.contract.test_ebool_or_scalarL(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();

    const tx2 = await this.contract.test_ebool_or_scalarL(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();

    const tx3 = await this.contract.test_ebool_or_scalarL(true, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();

    const tx4 = await this.contract.test_ebool_or_scalarL(true, true);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();

    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: true,
      [handle3]: true,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool or scalarR', async function () {
    const tx = await this.contract.test_ebool_or_scalarR(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();

    const tx2 = await this.contract.test_ebool_or_scalarR(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();

    const tx3 = await this.contract.test_ebool_or_scalarR(true, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();

    const tx4 = await this.contract.test_ebool_or_scalarR(true, true);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();

    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: true,
      [handle3]: true,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool and scalarL', async function () {
    const tx = await this.contract.test_ebool_and_scalarL(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();

    const tx2 = await this.contract.test_ebool_and_scalarL(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();

    const tx3 = await this.contract.test_ebool_and_scalarL(true, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();

    const tx4 = await this.contract.test_ebool_and_scalarL(true, true);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();

    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: false,
      [handle3]: false,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool and scalarR', async function () {
    const tx = await this.contract.test_ebool_and_scalarR(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();

    const tx2 = await this.contract.test_ebool_and_scalarR(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();

    const tx3 = await this.contract.test_ebool_and_scalarR(true, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();

    const tx4 = await this.contract.test_ebool_and_scalarR(true, true);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();

    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: false,
      [handle3]: false,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool ne ebool', async function () {
    const tx = await this.contract.neEbool(true, true);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const tx2 = await this.contract.neEbool(false, false);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();
    const tx3 = await this.contract.neEbool(false, true);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();
    const tx4 = await this.contract.neEbool(true, false);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: false,
      [handle3]: true,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool ne ebool - ScalarL', async function () {
    const tx = await this.contract.neEboolScalarL(true, true);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const tx2 = await this.contract.neEboolScalarL(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();
    const tx3 = await this.contract.neEboolScalarL(false, false);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();
    const tx4 = await this.contract.neEboolScalarL(true, false);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: true,
      [handle3]: false,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('ebool ne ebool - ScalarR', async function () {
    const tx = await this.contract.neEboolScalarL(false, false);
    await tx.wait();
    const handle = await this.contract.resEbool();
    const tx2 = await this.contract.neEboolScalarL(false, true);
    await tx2.wait();
    const handle2 = await this.contract.resEbool();
    const tx3 = await this.contract.neEboolScalarL(true, true);
    await tx3.wait();
    const handle3 = await this.contract.resEbool();
    const tx4 = await this.contract.neEboolScalarL(true, false);
    await tx4.wait();
    const handle4 = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle, handle2, handle3, handle4]);
    const expectedRes = {
      [handle]: false,
      [handle2]: true,
      [handle3]: false,
      [handle4]: true,
    };
    assert.deepEqual(res.clearValues, expectedRes);
  });

  it('test operator "sum" euint16 - two elements', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add16(1000n);
    input.add16(2000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_sum_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 3000n);
  });

  it('test operator "sum" euint32 - two elements', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add32(100000n);
    input.add32(200000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_sum_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 300000n);
  });

  it('test operator "sum" euint8 - three elements', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(10n);
    input.add8(20n);
    input.add8(30n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_sum_euint8(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.handles[2],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 60n);
  });

  it('test operator "sum" euint64 - two elements', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add64(1000000n);
    input.add64(2000000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_sum_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 3000000n);
  });

  it('test operator "sum" euint128 - two elements', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add128(100000000000000000000n);
    input.add128(200000000000000000000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_sum_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint128();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 300000000000000000000n);
  });

  it('test operator "sum" euint8 - duplicate handle counted twice', async function () {
    const value = 7;
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(value);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_sum_euint8_duplicate(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], value * 2);
  });

  it('test operator "sum" euint8 - uninitialized element treated as 0', async function () {
    const tx = await this.contract.test_sum_euint8_uninitialized();
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 5n);
  });

  it('test operator "sum" euint8 - empty array returns 0', async function () {
    const tx = await this.contract.test_sum_euint8_empty();
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 0n);
  });

  it('test operator "sum" euint8 - single element returns fresh handle', async function () {
    const value = 42;
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(value);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_sum_euint8_single(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint8();
    assert.notEqual(handle, encryptedAmount.handles[0]);
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], BigInt(value));
  });

  it('test operator "sum" euint8 - 100 elements at max array size', async function () {
    const tx = await this.contract.test_sum_euint8_max_array();
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 100n);
  });

  it('test operator "isIn" euint8 - value found in set', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(20n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_isIn_euint8_found(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" euint8 - value not found in set', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_isIn_euint8_not_found(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], false);
  });

  it('test operator "isIn" euint16 - value found in set', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add16(1000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_isIn_euint16(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" euint32 - value found in set', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add32(100000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_isIn_euint32(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" euint64 - value found in set', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add64(1000000000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_isIn_euint64(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" euint128 - value found in set', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add128(10000000000000000000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_isIn_euint128(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" euint8 - uninitialized value treated as 0 (found)', async function () {
    const tx = await this.contract.test_isIn_euint8_uninitialized();
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" euint8 - single element set, found', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_isIn_euint8_single_element(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" euint8 - 100 elements at max array size', async function () {
    const tx = await this.contract.test_isIn_euint8_max_array();
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" euint8 - empty set returns false', async function () {
    const tx = await this.contract.test_isIn_euint8_empty_set();
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], false);
  });

  it('test operator "isIn" euint8 - zero-initialized set, enc(0) found', async function () {
    const tx = await this.contract.test_isIn_euint8_zero_initialized_set();
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" euint8 - max type value (255) found in set', async function () {
    const tx = await this.contract.test_isIn_euint8_max_value_found();
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" euint8 - single element set, not found', async function () {
    const tx = await this.contract.test_isIn_euint8_single_element_not_found();
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], false);
  });

  it('test operator "isIn" eaddress - value found in set', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress('0x2222222222222222222222222222222222222222');
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_isIn_eaddress_found(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" eaddress - value not found in set', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.addAddress('0x4444444444444444444444444444444444444444');
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_isIn_eaddress_not_found(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], false);
  });

  it('test operator "isIn" euint256 - value found in set', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add256(42n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_isIn_euint256_found(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], true);
  });

  it('test operator "isIn" euint256 - value not found in set', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add256(99n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_isIn_euint256_not_found(
      encryptedAmount.handles[0],
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEbool();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], false);
  });

  // euint8: 200 * 200 / 200 = 200 (intermediate 40000 overflows uint8, widening required)
  it('test operator "mulDiv" euint8 enc*enc: (200 * 200) / 200 = 200', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(200n);
    input.add8(200n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint8_enc_enc(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      200n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 200n);
  });

  it('test operator "mulDiv" euint8 enc*scalar: (50 * 3) / 5 = 30', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(50n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint8_enc_scalar(
      encryptedAmount.handles[0],
      3n,
      5n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 30n);
  });

  // euint16: 60000 * 60000 / 60000 = 60000 (intermediate overflows uint16)
  it('test operator "mulDiv" euint16 enc*enc: (60000 * 60000) / 60000 = 60000', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add16(60000n);
    input.add16(60000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint16_enc_enc(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      60000n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 60000n);
  });

  it('test operator "mulDiv" euint16 enc*scalar: (1000 * 3) / 5 = 600', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add16(1000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint16_enc_scalar(
      encryptedAmount.handles[0],
      3n,
      5n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint16();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 600n);
  });

  it('test operator "mulDiv" euint32 enc*enc: (300000 * 300000) / 300000 = 300000', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add32(300000n);
    input.add32(300000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint32_enc_enc(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      300000n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 300000n);
  });

  it('test operator "mulDiv" euint32 enc*scalar: (1000000 * 3) / 5 = 600000', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add32(1000000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint32_enc_scalar(
      encryptedAmount.handles[0],
      3n,
      5n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint32();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 600000n);
  });

  // euint64: 10^10 * 10^10 / 10^10 = 10^10 (intermediate 10^20 overflows uint64)
  it('test operator "mulDiv" euint64 enc*enc: (10^10 * 10^10) / 10^10 = 10^10', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add64(10000000000n);
    input.add64(10000000000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint64_enc_enc(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      10000000000n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 10000000000n);
  });

  it('test operator "mulDiv" euint64 enc*scalar: (10^9 * 3) / 5 = 6*10^8', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add64(1000000000n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint64_enc_scalar(
      encryptedAmount.handles[0],
      3n,
      5n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint64();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 600000000n);
  });

  // Edge cases
  it('test operator "mulDiv" - division by zero reverts', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(100n);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    // divisor = 0 -> FHEVMExecutor reverts with DivisionByZero()
    const promise = this.contract.test_mulDiv_euint8_enc_enc(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      0n,
      encryptedAmount.inputProof,
    );
    await expect(promise).to.be.reverted;
  });

  it('test operator "mulDiv" euint8 enc*enc: (0 * 100) / 50 = 0 (zero lhs)', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(0n);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint8_enc_enc(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      50n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 0n);
  });

  it('test operator "mulDiv" euint8 enc*enc: (100 * 0) / 50 = 0 (zero rhs encrypted)', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(100n);
    input.add8(0n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint8_enc_enc(
      encryptedAmount.handles[0],
      encryptedAmount.handles[1],
      50n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 0n);
  });

  it('test operator "mulDiv" euint8 enc*scalar: (100 * 0) / 50 = 0 (zero rhs scalar)', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(100n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint8_enc_scalar(
      encryptedAmount.handles[0],
      0n, // scalar b = 0
      50n,
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 0n);
  });

  it('test operator "mulDiv" euint8 enc*scalar: (7 * 3) / 4 = 5 (truncating division)', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(7n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint8_enc_scalar(
      encryptedAmount.handles[0],
      3n,
      4n, // 21 / 4 = 5 (truncated, not 5.25)
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 5n);
  });

  it('test operator "mulDiv" euint8 enc*scalar: (1 * 1) / 2 = 0 (truncation to zero)', async function () {
    const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
    input.add8(1n);
    const encryptedAmount = await input.encrypt();
    const tx = await this.contract.test_mulDiv_euint8_enc_scalar(
      encryptedAmount.handles[0],
      1n,
      2n, // 1 / 2 = 0 (integer truncation)
      encryptedAmount.inputProof,
    );
    await tx.wait();
    const handle = await this.contract.resEuint8();
    const res = await this.instance.publicDecrypt([handle]);
    assert.equal(res.clearValues[handle], 0n);
  });

  // -----------------------------------------------------------------------
  // Multi-output sample ops (proof-of-concept).
  //
  // Validates the full multi-output pipeline end-to-end:
  //   Solidity executor → event → host-listener → DB (group_id +
  //   output_index) → worker grouping pass → tfhe-rs multi-output
  //   execution → N ciphertexts → public decryption.
  // -----------------------------------------------------------------------

  describe('FHEVM sample multi-output ops', function () {
    it('sampleMultiOutput euint32 - non-zero input: value = ct + 1, found = true', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add32(41);
      const encryptedAmount = await input.encrypt();
      const tx = await this.contract.test_sampleMultiOutput_euint32(
        encryptedAmount.handles[0],
        encryptedAmount.inputProof,
      );
      await tx.wait();

      const valueHandle = await this.contract.resEuint32();
      const foundHandle = await this.contract.resEbool();
      assert.notEqual(valueHandle, encryptedAmount.handles[0]);
      assert.notEqual(valueHandle, foundHandle);

      const res = await this.instance.publicDecrypt([valueHandle, foundHandle]);
      assert.equal(res.clearValues[valueHandle], 42n); // 41 + 1
      assert.equal(res.clearValues[foundHandle], true); // 41 != 0
    });

    it('sampleMultiOutput euint32 - zero input: value = 1, found = false', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add32(0);
      const encryptedAmount = await input.encrypt();
      const tx = await this.contract.test_sampleMultiOutput_euint32(
        encryptedAmount.handles[0],
        encryptedAmount.inputProof,
      );
      await tx.wait();

      const valueHandle = await this.contract.resEuint32();
      const foundHandle = await this.contract.resEbool();

      const res = await this.instance.publicDecrypt([valueHandle, foundHandle]);
      assert.equal(res.clearValues[valueHandle], 1n); // 0 + 1
      assert.equal(res.clearValues[foundHandle], false); // 0 != 0 is false
    });

    it('sampleMultiOutput100 euint32 - slot 0 returns ct + 1', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add32(100);
      const encryptedAmount = await input.encrypt();
      const tx = await this.contract.test_sampleMultiOutput100_euint32(
        encryptedAmount.handles[0],
        0,
        encryptedAmount.inputProof,
      );
      await tx.wait();

      assert.equal(await this.contract.resSampleMulti100SlotIndex(), 0);
      const handle = await this.contract.resSampleMulti100Slot();
      const res = await this.instance.publicDecrypt([handle]);
      assert.equal(res.clearValues[handle], 101n); // 100 + (0 + 1)
    });

    it('sampleMultiOutput100 euint32 - slot 42 returns ct + 43', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add32(1000);
      const encryptedAmount = await input.encrypt();
      const tx = await this.contract.test_sampleMultiOutput100_euint32(
        encryptedAmount.handles[0],
        42,
        encryptedAmount.inputProof,
      );
      await tx.wait();

      assert.equal(await this.contract.resSampleMulti100SlotIndex(), 42);
      const handle = await this.contract.resSampleMulti100Slot();
      const res = await this.instance.publicDecrypt([handle]);
      assert.equal(res.clearValues[handle], 1043n); // 1000 + (42 + 1)
    });

    it('sampleMultiOutput100 euint32 - slot 99 returns ct + 100', async function () {
      const input = this.instance.createEncryptedInput(this.contractAddress, this.signer.address);
      input.add32(7);
      const encryptedAmount = await input.encrypt();
      const tx = await this.contract.test_sampleMultiOutput100_euint32(
        encryptedAmount.handles[0],
        99,
        encryptedAmount.inputProof,
      );
      await tx.wait();

      assert.equal(await this.contract.resSampleMulti100SlotIndex(), 99);
      const handle = await this.contract.resSampleMulti100Slot();
      const res = await this.instance.publicDecrypt([handle]);
      assert.equal(res.clearValues[handle], 107n); // 7 + (99 + 1)
    });
  });
});
