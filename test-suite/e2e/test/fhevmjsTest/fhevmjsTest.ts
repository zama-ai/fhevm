import { expect } from 'chai';

import { createInstances } from '../instance';
import type { TypedValue } from '../sdk/types';
import { getSigners, initSigners } from '../signers';

const bitWidth = (typedValue: TypedValue): number => {
  switch (typedValue.type) {
    case 'bool':
      return 2;
    case 'uint8':
      return 8;
    case 'uint16':
      return 16;
    case 'uint32':
      return 32;
    case 'uint64':
      return 64;
    case 'uint128':
      return 128;
    case 'uint256':
      return 256;
    case 'address':
      return 160;
  }
};

describe('Testing fhevmjs/fhevmjsMocked', function () {
  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();
    this.contractAddress = '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2';
    this.instances = await createInstances(this.signers);
  });

  it('should be able to pack up to 255 ebools', async function () {
    await this.instances.alice.encryptTypedValues({
      values: Array.from({ length: 255 }, () => ({ type: 'bool' as const, value: false })),
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });
  });

  it('should be unable to pack more than 255 ebools', async function () {
    await expect(
      this.instances.alice.encryptTypedValues({
        values: [
          ...Array.from({ length: 256 }, () => ({ type: 'bool' as const, value: true })),
          { type: 'bool' as const, value: false },
        ],
        contractAddress: this.contractAddress,
        userAddress: this.signers.alice.address,
      }),
    ).to.be.rejectedWith('Packing more than 256 variables in a single input ciphertext is unsupported');
  });

  it('should be able to pack up to 32 euint64s', async function () {
    await this.instances.alice.encryptTypedValues({
      values: Array.from({ length: 32 }, () => ({ type: 'uint64' as const, value: 1024n })),
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });
  });

  it('should be unable to pack more than 32 euint64s', async function () {
    await expect(
      this.instances.alice.encryptTypedValues({
        values: [
          ...Array.from({ length: 32 }, () => ({ type: 'uint64' as const, value: 37n })),
          { type: 'uint64' as const, value: 1n },
        ],
        contractAddress: this.contractAddress,
        userAddress: this.signers.alice.address,
      }),
    ).to.be.rejectedWith('Packing more than 2048 bits in a single input ciphertext is unsupported');
  });

  it('should be able to pack up to 8 euint256s', async function () {
    await this.instances.alice.encryptTypedValues({
      values: Array.from({ length: 8 }, () => ({ type: 'uint256' as const, value: 797979n })),
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });
  });

  it('should not be able to pack more than 8 euint256s', async function () {
    await expect(
      this.instances.alice.encryptTypedValues({
        values: [
          ...Array.from({ length: 8 }, () => ({ type: 'uint256' as const, value: 797979n })),
          { type: 'bool' as const, value: false },
        ],
        contractAddress: this.contractAddress,
        userAddress: this.signers.alice.address,
      }),
    ).to.be.rejectedWith('Packing more than 2048 bits in a single input ciphertext is unsupported');
  });

  it('should be able to pack up to 2048 bits but not more', async function () {
    const values: TypedValue[] = [
      { type: 'uint256', value: 797979n },
      { type: 'uint256', value: 797978n },
      { type: 'uint256', value: 797977n },
      { type: 'uint256', value: 797976n },
      { type: 'uint256', value: 797975n },
      { type: 'uint256', value: 797974n },
      { type: 'uint256', value: 6887n },
      { type: 'uint128', value: 6887n },
      { type: 'uint64', value: 6887n },
      { type: 'uint64', value: 6887n },
    ];
    await this.instances.alice.encryptTypedValues({
      values,
      contractAddress: this.contractAddress,
      userAddress: this.signers.alice.address,
    });
    const total = values.reduce((sum, typedValue) => sum + bitWidth(typedValue), 0);
    expect(total).to.eq(2048);
    await expect(
      this.instances.alice.encryptTypedValues({
        values: [...values, { type: 'bool' as const, value: false }],
        contractAddress: this.contractAddress,
        userAddress: this.signers.alice.address,
      }),
    ).to.be.rejectedWith('Packing more than 2048 bits in a single input ciphertext is unsupported');
  });
});
