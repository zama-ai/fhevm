import { assert } from 'chai';

import { SHIFT_CASES, decryptBatch, useOperatorEdgeCaseFixture } from './operatorEdgeCases';
import { expectedRotl, expectedRotr, expectedShl, expectedShr } from './shiftSemantics';

describe('FHEVM manual operations - shift and rotate edge cases', function () {
  useOperatorEdgeCaseFixture();

  SHIFT_CASES.forEach(({ bits, valueType, value, amounts }) => {
    amounts.forEach((amount) => {
      it(`shl/shr/rotl/rotr(euint${bits}, ${amount}) matches reference semantics`, async function () {
        const encryptedAmount = await this.instance.encryptTypedValues({
          values: [
            { type: valueType, value },
            { type: 'uint8', value: amount },
          ],
          contractAddress: this.edgeAddress,
          userAddress: this.signer.address,
        });
        const tx = await this.edge[`test_shifts_euint${bits}`](
          encryptedAmount.handles[0],
          amount,
          encryptedAmount.handles[1],
          encryptedAmount.inputProof,
        );
        await tx.wait();
        const values = await decryptBatch(this.instance, this.edge);
        const expected = [
          expectedShl(value, amount, bits),
          expectedShr(value, amount, bits),
          expectedRotl(value, amount, bits),
          expectedRotr(value, amount, bits),
        ];
        assert.deepEqual(values, [...expected, ...expected]);
      });
    });
  });
});
