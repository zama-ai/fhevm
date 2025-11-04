import { expect } from 'chai';

import { createInstances, getTotalBits } from '../instance';
import { getSigners, initSigners } from '../signers';
import { bigIntToBytes64, bigIntToBytes128 } from '../utils';

describe('Testing fhevmjs/fhevmjsMocked', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();
    this.contractAddress = '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2';
    this.instances = await createInstances(this.signers);
  });

  it('should be able to pack up to 256 ebools', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    for (let i = 0; i < 256; i++) {
      input.addBool(false);
    }
    const total = getTotalBits(input);
    expect(total).to.eq(2048);
    await input.encrypt();
  });

  it('should be unable to pack more than 256 ebools', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    for (let i = 0; i < 256; i++) {
      input.addBool(true);
    }
    const total = getTotalBits(input);
    expect(total).to.eq(2048);
    expect(() => input.addBool(false)).to.throw(
      'Packing more than 256 variables in a single input ciphertext is unsupported',
    );
  });

  it('should be able to pack up to 32 euint64s', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    for (let i = 0; i < 32; i++) {
      input.add64(1024n);
    }
    const total = getTotalBits(input);
    expect(total).to.eq(2048);
    await input.encrypt();
  });

  it('should be unable to pack more than 32 euint64s', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    for (let i = 0; i < 32; i++) {
      input.add64(37n);
    }
    const total = getTotalBits(input);
    expect(total).to.eq(2048);
    expect(() => input.add64(1n)).to.throw('Packing more than 2048 bits in a single input ciphertext is unsupported');
  });

  it('should be able to pack up to 8 euint256s', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    for (let i = 0; i < 8; i++) {
      input.add256(797979n);
    }
    const total = getTotalBits(input);
    expect(total).to.eq(2048);
    await input.encrypt();
  });

  it('should not be able to pack more than 8 euint256s', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    for (let i = 0; i < 8; i++) {
      input.add256(797979n);
    }
    const total = getTotalBits(input);
    expect(total).to.eq(2048);
    expect(() => input.addBool(false)).to.throw(
      'Packing more than 2048 bits in a single input ciphertext is unsupported',
    );
  });

  it('should be able to pack up to 2048 bits but not more', async function () {
    const input = this.instances.alice.createEncryptedInput(this.contractAddress, this.signers.alice.address);
    input.add256(797979n);
    input.add256(797978n);
    input.add256(797977n);
    input.add256(797976n);
    input.add256(797975n);
    input.add256(797974n);
    input.add256(6887n);
    input.add128(6887n);
    input.add64(6887n);
    input.add64(6887n);
    const total = getTotalBits(input);
    expect(total).to.eq(2048);
    expect(() => input.addBool(false)).to.throw(
      'Packing more than 2048 bits in a single input ciphertext is unsupported',
    );
  });
});
