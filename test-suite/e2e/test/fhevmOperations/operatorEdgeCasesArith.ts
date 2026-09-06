import { assert } from 'chai';

import { NARROW_CASES, decryptBatch, useOperatorEdgeCaseFixture } from './operatorEdgeCases';

describe('FHEVM manual operations - arithmetic edge cases', function () {
  useOperatorEdgeCaseFixture();

  NARROW_CASES.forEach(({ bits, valueType }) => {
    const modulus = 1n << bits;
    const max = modulus - 1n;
    const wrap = (v: bigint) => ((v % modulus) + modulus) % modulus;
    (
      [
        [max, 1n],
        [0n, 1n],
        [max, max],
        [max, 2n],
        [0n, 0n],
      ] as const
    ).forEach(([lhs, rhs]) => {
      it(`add/sub/mul/neg/not(euint${bits}(${lhs}), ${rhs}) wraps`, async function () {
        const encryptedAmount = await this.instance.encryptTypedValues({
          values: [
            { type: valueType, value: lhs },
            { type: valueType, value: rhs },
          ],
          contractAddress: this.edgeAddress,
          userAddress: this.signer.address,
        });
        const tx = await this.edge[`test_arith_euint${bits}`](
          encryptedAmount.handles[0],
          rhs,
          encryptedAmount.handles[1],
          encryptedAmount.inputProof,
        );
        await tx.wait();
        const values = await decryptBatch(this.instance, this.edge);
        const binary = [wrap(lhs + rhs), wrap(lhs - rhs), wrap(lhs * rhs)];
        assert.deepEqual(values, [...binary, ...binary, wrap(-lhs), max - lhs]);
      });
    });
  });
});
