import type { Contract } from 'ethers';
import { ethers } from 'hardhat';

import { createInstance } from '../instance';
import { getSigner } from '../signers';

export const OVERSIZED_SHIFT_64 = 70n;
export const SHIFT_ROTATE_VALUE_64 = 0x123456789abcdef0n;

export const SHIFT_CASES = [
  { bits: 8n, valueType: 'uint8', value: 0xa5n, amounts: [0n, 7n, 8n, 255n] },
  { bits: 16n, valueType: 'uint16', value: 0xa5c3n, amounts: [0n, 15n, 16n, 255n] },
  { bits: 32n, valueType: 'uint32', value: 0xa5c3f00fn, amounts: [0n, 31n, 32n, 255n] },
  { bits: 64n, valueType: 'uint64', value: SHIFT_ROTATE_VALUE_64, amounts: [0n, 63n, 64n, OVERSIZED_SHIFT_64, 255n] },
  {
    bits: 128n,
    valueType: 'uint128',
    value: 0x123456789abcdef0fedcba9876543210n,
    amounts: [0n, 127n, 128n, 255n],
  },
  {
    // The shift amount is a uint8/euint8, so amount >= 256 is unrepresentable here.
    bits: 256n,
    valueType: 'uint256',
    value: 0x123456789abcdef0fedcba9876543210123456789abcdef0fedcba9876543210n,
    amounts: [0n, 255n],
  },
] as const;

// Under the legacy modulo the amount == bits rows make every operator the identity; they
// only become discriminating once OVERSHIFT_RETURNS_ZERO flips.
export const WIDTHS = SHIFT_CASES.map(({ bits }) => bits);

// div/rem and add/sub/mul are only defined up to euint128
export const NARROW_CASES = SHIFT_CASES.filter(({ bits }) => bits <= 128n);

// Deployed once per suite: every fixture entry point starts with `delete _resBatch`.
export function useOperatorEdgeCaseFixture(): void {
  before(async function () {
    this.signer = await getSigner(119);
    this.instance = await createInstance();
    const factory = await ethers.getContractFactory('FHEVMOperatorEdgeCaseTestSuite');
    const contract = await factory.connect(this.signer).deploy();
    await contract.waitForDeployment();
    this.edge = contract as unknown as Contract;
    this.edgeAddress = await contract.getAddress();
  });
}

export async function decryptBatch(
  instance: Awaited<ReturnType<typeof createInstance>>,
  contract: Contract,
): Promise<bigint[]> {
  const handles = [...(await contract.resBatch())] as string[];
  const res = await instance.publicDecrypt(handles);
  return handles.map((h) => res.clearValues[h as `0x${string}`] as bigint);
}
