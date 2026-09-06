import { assert, expect } from 'chai';

import { NARROW_CASES, decryptBatch, useOperatorEdgeCaseFixture } from './operatorEdgeCases';

describe('FHEVM manual operations - div and rem edge cases', function () {
  useOperatorEdgeCaseFixture();

  NARROW_CASES.forEach(({ bits, valueType, value }) => {
    const max = (1n << bits) - 1n;
    (
      [
        [value, 1n],
        [value, value],
        [value, max],
        [max, max],
        [0n, value],
      ] as const
    ).forEach(([dividend, divisor]) => {
      it(`div/rem(euint${bits}(${dividend}), ${divisor}) matches reference semantics`, async function () {
        const encryptedAmount = await this.instance.encryptTypedValues({
          values: [{ type: valueType, value: dividend }],
          contractAddress: this.edgeAddress,
          userAddress: this.signer.address,
        });
        const tx = await this.edge[`test_divrem_euint${bits}`](
          encryptedAmount.handles[0],
          divisor,
          encryptedAmount.inputProof,
        );
        await tx.wait();
        const values = await decryptBatch(this.instance, this.edge);
        assert.deepEqual(values, [dividend / divisor, dividend % divisor]);
      });
    });

    it(`div/rem(euint${bits}, 0) reverts`, async function () {
      const encryptedAmount = await this.instance.encryptTypedValues({
        values: [{ type: valueType, value }],
        contractAddress: this.edgeAddress,
        userAddress: this.signer.address,
      });
      // FHEVMExecutor reverts with DivisionByZero() before any FHE work
      await expect(this.edge[`test_divrem_euint${bits}`](encryptedAmount.handles[0], 0n, encryptedAmount.inputProof)).to
        .be.reverted;
    });
  });
});
