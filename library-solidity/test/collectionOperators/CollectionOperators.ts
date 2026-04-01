import type { FHEVMGasProfileSuite } from '../../typechain-types/examples/tests/FHEVMGasProfileSuite';
import { getTxHCUFromTxReceipt } from '../coprocessorUtils';
import { getSigners, initSigners } from '../signers';
import { deployFHEVMGasProfileSuiteFixture } from './CollectionOperators.fixture';

// Array sizes to benchmark.
const SIZES = [8, 16, 32, 64, 128] as const;
const TYPES: FheType[] = ['euint8', 'euint16', 'euint32', 'euint64', 'euint128'];
const ISIN_EXTRA_TYPES: IsInExtraType[] = ['eaddress', 'euint256'];

// Safe upper bound per (op, type), derived from maxHCUPerTx=20M.
// Cases above these limits are covered in the boundary tests below.
function maxSafeSize(op: Operation, type: FheType): number {
  if (op === 'sum' && (type === 'euint64' || type === 'euint128')) return 64;
  return 128;
}

// Boundary cases: all expected to revert (size guard or HCU limit exceeded).
const BOUNDARY_CASES: { op: Operation; type: FheType; size: number }[] = [
  { op: 'isIn', type: 'euint8', size: 256 },
  { op: 'isIn', type: 'euint16', size: 256 },
  { op: 'isIn', type: 'euint32', size: 256 },
  { op: 'isIn', type: 'euint64', size: 256 },
  { op: 'isIn', type: 'euint128', size: 256 },
  { op: 'sum', type: 'euint8', size: 256 },
  { op: 'sum', type: 'euint16', size: 256 },
  { op: 'sum', type: 'euint32', size: 256 },
  { op: 'sum', type: 'euint64', size: 128 },
  { op: 'sum', type: 'euint128', size: 128 },
];

type FheType = 'euint8' | 'euint16' | 'euint32' | 'euint64' | 'euint128';
type IsInExtraType = 'eaddress' | 'euint256';
type Operation = 'isIn' | 'sum';

describe('Gas Profile: isIn and sum', function () {
  this.timeout(300_000);

  before(async function () {
    await initSigners(1);
    this.signers = await getSigners();
    this.contract = await deployFHEVMGasProfileSuiteFixture();
  });

  // ------------------------------------------------------------------
  // isIn benchmarks
  // ------------------------------------------------------------------

  for (const size of SIZES) {
    for (const type of TYPES) {
      if (size > maxSafeSize('isIn', type)) continue;
      it(`isIn(${type}, n=${size})`, async function () {
        const tx = await (this.contract as any)[`profile_isIn_${type}`](size);
        const receipt = await tx.wait();
        const { globalTxHCU, maxTxHCUDepth } = getTxHCUFromTxReceipt(receipt!);
        console.log(`isIn(${type}, n=${size}) — EVM gas: ${receipt!.gasUsed}, Total HCU: ${globalTxHCU}, HCU Depth: ${maxTxHCUDepth}`);
      });
    }
  }

  // isIn benchmarks — eaddress and euint256

  for (const size of SIZES) {
    for (const type of ISIN_EXTRA_TYPES) {
      it(`isIn(${type}, n=${size})`, async function () {
        const tx = await (this.contract as any)[`profile_isIn_${type}`](size);
        const receipt = await tx.wait();
        const { globalTxHCU, maxTxHCUDepth } = getTxHCUFromTxReceipt(receipt!);
        console.log(`isIn(${type}, n=${size}) — EVM gas: ${receipt!.gasUsed}, Total HCU: ${globalTxHCU}, HCU Depth: ${maxTxHCUDepth}`);
      });
    }
  }

  // ------------------------------------------------------------------
  // sum benchmarks
  // ------------------------------------------------------------------

  for (const size of SIZES) {
    for (const type of TYPES) {
      if (size > maxSafeSize('sum', type)) continue;
      it(`sum(${type}, n=${size})`, async function () {
        const tx = await (this.contract as any)[`profile_sum_${type}`](size);
        const receipt = await tx.wait();
        const { globalTxHCU, maxTxHCUDepth } = getTxHCUFromTxReceipt(receipt!);
        console.log(`sum(${type}, n=${size}) — EVM gas: ${receipt!.gasUsed}, Total HCU: ${globalTxHCU}, HCU Depth: ${maxTxHCUDepth}`);
      });
    }
  }

  // ------------------------------------------------------------------
  // Boundary cases — expected to revert (FHECollectionSizeInvalid guard or HCU limit)
  // ------------------------------------------------------------------

  describe('Boundary cases (expected revert)', function () {
    for (const { op, type, size } of BOUNDARY_CASES) {
      it(`${op}(${type}, n=${size}) reverts`, async function () {
        const fn = `profile_${op}_${type}`;
        try {
          const tx = await (this.contract as any)[fn](size);
          await tx.wait();
          throw new Error(`Expected revert but transaction succeeded`);
        } catch (err: any) {
          if (err.message === `Expected revert but transaction succeeded`) throw err;
          // Test passes — any revert (size guard or HCU limit) is the expected outcome.
        }
      });
    }
  });
});
