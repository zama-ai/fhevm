import { assert } from 'chai';

import { SHIFT_CASES, WIDTHS, decryptBatch, useOperatorEdgeCaseFixture } from './operatorEdgeCases';

describe('FHEVM manual operations - cast edge cases', function () {
  useOperatorEdgeCaseFixture();

  const MODULUS_256 = 1n << 256n;
  [0n, MODULUS_256 - 1n].forEach((operand) => {
    it(`neg/not(euint256(${operand === 0n ? '0' : 'MAX'})) wraps`, async function () {
      const encryptedAmount = await this.instance.encryptTypedValues({
        values: [{ type: 'uint256', value: operand }],
        contractAddress: this.edgeAddress,
        userAddress: this.signer.address,
      });
      const tx = await this.edge.test_negnot_euint256(encryptedAmount.handles[0], encryptedAmount.inputProof);
      await tx.wait();
      const values = await decryptBatch(this.instance, this.edge);
      assert.deepEqual(values, [(MODULUS_256 - operand) % MODULUS_256, MODULUS_256 - 1n - operand]);
    });
  });

  SHIFT_CASES.filter(({ bits }) => bits > 8n).forEach(({ bits, valueType, value }) => {
    it(`asEuintX(euint${bits}) truncates`, async function () {
      const encryptedAmount = await this.instance.encryptTypedValues({
        values: [{ type: valueType, value }],
        contractAddress: this.edgeAddress,
        userAddress: this.signer.address,
      });
      const tx = await this.edge[`test_narrow_euint${bits}`](encryptedAmount.handles[0], encryptedAmount.inputProof);
      await tx.wait();
      const values = await decryptBatch(this.instance, this.edge);
      assert.deepEqual(
        values,
        WIDTHS.filter((w) => w < bits)
          .reverse()
          .map((w) => value & ((1n << w) - 1n)),
      );
    });
  });
});
