// Ported from `@fhevm/mock-utils` (`fhevm/coprocessor/hcu.ts`). HCU is a cost model applied to the
// FHEVMExecutor's operator events, so it survived the move to an on-chain cleartext stack unchanged:
// the v13 `CleartextFHEVMExecutor` emits the same events. Only the imports were rehomed.
import type { ethers as EthersT } from 'ethers';

import { HardhatFhevmError } from '../../error';
import { assertHHFhevm as assertFhevm } from '../error';
import { parseFhevmHandle } from '../fhevmHandle';
import {
  assertEventArgIsArrayOfBytes32String,
  assertEventArgIsBigUint8,
  assertEventArgIsBigUint256,
  assertEventArgIsBytes1String,
  assertEventArgIsBytes16String,
  assertEventArgIsBytes32String,
} from './eventArgs';
import { type HCUOperator, HCUByOperator, getHCU } from './HCUByOperator';
import { type FheTypeName, getFheTypeName, getFheTypeNameFromByte } from './fheTypeName';

export type FhevmTransactionHCUInfo = {
  transactionHash: `0x${string}`;
  globalHCU: number;
  maxHCUDepth: number;
  HCUDepthByHandle: Record<`0x${string}`, number>;
};

function _getTypeOperatorHCU(op: HCUOperator, typeName: FheTypeName): number {
  return op.types?.[typeName] ?? 0;
}

function _getOperatorHCU(op: HCUOperator, scalar: boolean, typeName: FheTypeName): number {
  assertFhevm(op.supportScalar);
  const prices: Partial<Record<FheTypeName, number>> | undefined = scalar ? op.scalar : op.nonScalar;
  return prices?.[typeName] ?? 0;
}

function _getScalarOperatorHCU(op: HCUOperator, typeName: FheTypeName): number {
  return op.scalar?.[typeName] ?? 0;
}

// `op` is the price entry this log's event resolved to, carried so callers need not re-index
// `HCUByOperator`, whose string index signature makes every lookup optional again.
type FHELogEntry = { name: keyof typeof HCUByOperator; args: EthersT.Result; op: HCUOperator };

function _filterOperatorsHCUsLogs(
  coprocessorAddress: `0x${string}`,
  coprocessorContractInterface: EthersT.Interface,
  logs: readonly EthersT.Log[],
): FHELogEntry[] {
  const res: FHELogEntry[] = [];
  for (const log of logs) {
    if (log.address.toLowerCase() !== coprocessorAddress.toLowerCase()) {
      continue;
    }
    try {
      const parsedLog: EthersT.LogDescription | null = coprocessorContractInterface.parseLog({
        topics: log.topics,
        data: log.data,
      });

      if (!parsedLog || !(parsedLog.name in HCUByOperator)) {
        continue;
      }

      const eventName = parsedLog.name;
      const opPrices = HCUByOperator[eventName];
      if (opPrices == null || typeof opPrices !== 'object') {
        continue;
      }

      res.push({
        name: eventName,
        args: parsedLog.args,
        op: opPrices,
      });
    } catch {
      //
    }
  }
  return res;
}

export function getTxHCUFromTxReceipt(
  coprocessorAddress: `0x${string}`,
  coprocessorContractInterface: EthersT.Interface,
  receipt: EthersT.TransactionReceipt,
): FhevmTransactionHCUInfo {
  if (receipt.status === 0) {
    throw new HardhatFhevmError('Transaction reverted');
  }

  function readFromHCUMap(handle: `0x${string}`): number {
    if (hcuMap[handle] === undefined) {
      return 0;
    }
    return hcuMap[handle];
  }

  const hcuMap: Record<string, number> = {};

  const FHELogs = _filterOperatorsHCUsLogs(coprocessorAddress, coprocessorContractInterface, receipt.logs);

  let totalHCUConsumed = 0;

  for (const event of FHELogs) {
    let hcuConsumed: number;

    switch (event.name) {
      case 'TrivialEncrypt': {
        // event TrivialEncrypt(address indexed caller, uint256 pt, FheType toType, bytes32 result);
        const toFheType: unknown = event.args[2];
        const resultBytes32: unknown = event.args[3];

        assertEventArgIsBigUint256(toFheType, 'TrivialEncrypt', 2);
        assertEventArgIsBytes32String(resultBytes32, 'TrivialEncrypt', 3);

        // HCU is computed using the toType arg
        const toFheTypeName: FheTypeName = getFheTypeNameFromByte(Number(toFheType));

        hcuConsumed = _getTypeOperatorHCU(event.op, toFheTypeName);
        totalHCUConsumed += hcuConsumed;

        hcuMap[resultBytes32] = hcuConsumed;
        break;
      }

      case 'Cast': {
        // "event Cast(address indexed caller, bytes32 ct, uint8 toType, bytes32 result)",
        const ctBytes32: unknown = event.args[1 as keyof typeof event.args];
        const toTypeUint8: unknown = event.args[2 as keyof typeof event.args];
        const resultBytes32: unknown = event.args[3 as keyof typeof event.args];

        assertEventArgIsBytes32String(ctBytes32, event.name, 1);
        assertEventArgIsBigUint8(toTypeUint8, event.name, 2);
        assertEventArgIsBytes32String(resultBytes32, event.name, 3);

        // HCU is computed using the ct bytes32 arg
        const ctFheTypeName: FheTypeName = getFheTypeName(parseFhevmHandle(ctBytes32).fhevmType);

        hcuConsumed = _getTypeOperatorHCU(event.op, ctFheTypeName);
        totalHCUConsumed += hcuConsumed;

        hcuMap[resultBytes32] = hcuConsumed + readFromHCUMap(ctBytes32);
        break;
      }

      case 'FheAdd':
      case 'FheSub':
      case 'FheMul':
      case 'FheBitAnd':
      case 'FheBitOr':
      case 'FheBitXor':
      case 'FheShl':
      case 'FheShr':
      case 'FheRotl':
      case 'FheRotr':
      case 'FheMax':
      case 'FheMin': {
        // event <Event Name>(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
        const lhsBytes32: unknown = event.args[1 as keyof typeof event.args];
        const rhsBytes32: unknown = event.args[2 as keyof typeof event.args];
        const scalarBytes1: unknown = event.args[3 as keyof typeof event.args];
        const resultBytes32: unknown = event.args[4 as keyof typeof event.args];

        assertEventArgIsBytes32String(lhsBytes32, event.name, 1);
        assertEventArgIsBytes32String(rhsBytes32, event.name, 2);
        assertEventArgIsBytes1String(scalarBytes1, event.name, 3);
        assertEventArgIsBytes32String(resultBytes32, event.name, 4);

        const scalar: boolean = scalarBytes1 === '0x01';

        // HCU is computed using the result bytes32 type
        const resultFheTypeName: FheTypeName = getFheTypeName(parseFhevmHandle(resultBytes32).fhevmType);

        hcuConsumed = _getOperatorHCU(event.op, scalar, resultFheTypeName);
        totalHCUConsumed += hcuConsumed;

        hcuMap[resultBytes32] =
          hcuConsumed + Math.max(readFromHCUMap(lhsBytes32), scalar ? 0 : readFromHCUMap(rhsBytes32));
        break;
      }

      // Return boolean
      case 'FheEq':
      case 'FheNe':
      case 'FheGe':
      case 'FheGt':
      case 'FheLe':
      case 'FheLt': {
        // event <Event Name>(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
        const lhsBytes32: unknown = event.args[1 as keyof typeof event.args];
        const rhsBytes32: unknown = event.args[2 as keyof typeof event.args];
        const scalarBytes1: unknown = event.args[3 as keyof typeof event.args];
        const resultBytes32: unknown = event.args[4 as keyof typeof event.args];

        assertEventArgIsBytes32String(lhsBytes32, event.name, 1);
        assertEventArgIsBytes32String(rhsBytes32, event.name, 2);
        assertEventArgIsBytes1String(scalarBytes1, event.name, 3);
        assertEventArgIsBytes32String(resultBytes32, event.name, 4);

        const scalar: boolean = scalarBytes1 === '0x01';

        // HCU is computed using lhs bytes32 type
        const lhsFheTypeName: FheTypeName = getFheTypeName(parseFhevmHandle(lhsBytes32).fhevmType);

        hcuConsumed = _getOperatorHCU(event.op, scalar, lhsFheTypeName);
        totalHCUConsumed += hcuConsumed;

        hcuMap[resultBytes32] =
          hcuConsumed + Math.max(readFromHCUMap(lhsBytes32), scalar ? 0 : readFromHCUMap(rhsBytes32));
        break;
      }

      case 'FheDiv':
      case 'FheRem': {
        // event <Event Name>(address indexed caller, bytes32 lhs, bytes32 rhs, bytes1 scalarByte, bytes32 result);
        const lhsBytes32: unknown = event.args[1 as keyof typeof event.args];
        const rhsBytes32: unknown = event.args[2 as keyof typeof event.args];
        const scalarBytes1: unknown = event.args[3 as keyof typeof event.args];
        const resultBytes32: unknown = event.args[4 as keyof typeof event.args];

        assertEventArgIsBytes32String(lhsBytes32, event.name, 1);
        assertEventArgIsBytes32String(rhsBytes32, event.name, 2);
        assertEventArgIsBytes1String(scalarBytes1, event.name, 3);
        assertEventArgIsBytes32String(resultBytes32, event.name, 4);

        const scalar: boolean = scalarBytes1 === '0x01';
        if (!scalar) {
          throw new HardhatFhevmError(`Non-scalar ${event.name} not implemented yet`);
        }

        // HCU is computed using the result bytes32 type
        const resultFheTypeName: FheTypeName = getFheTypeName(parseFhevmHandle(resultBytes32).fhevmType);

        hcuConsumed = _getScalarOperatorHCU(event.op, resultFheTypeName);
        totalHCUConsumed += hcuConsumed;

        hcuMap[resultBytes32] = hcuConsumed + readFromHCUMap(lhsBytes32);
        break;
      }

      case 'FheNot':
      case 'FheNeg': {
        // "event <Event Name>(address indexed caller, bytes32 ct, bytes32 result)",
        const ctBytes32: unknown = event.args[1 as keyof typeof event.args];
        const resultBytes32: unknown = event.args[2 as keyof typeof event.args];

        assertEventArgIsBytes32String(ctBytes32, event.name, 1);
        assertEventArgIsBytes32String(resultBytes32, event.name, 2);

        // HCU is computed using the ct bytes32 arg
        const ctFheTypeName: FheTypeName = getFheTypeName(parseFhevmHandle(ctBytes32).fhevmType);

        hcuConsumed = _getTypeOperatorHCU(event.op, ctFheTypeName);
        totalHCUConsumed += hcuConsumed;

        hcuMap[resultBytes32] = hcuConsumed + readFromHCUMap(ctBytes32);
        break;
      }

      case 'FheIfThenElse': {
        // "event FheIfThenElse(address indexed caller, bytes32 control, bytes32 ifTrue, bytes32 ifFalse, bytes32 result)",
        const controlBytes32: unknown = event.args[1 as keyof typeof event.args];
        const ifTrueBytes32: unknown = event.args[2 as keyof typeof event.args];
        const ifFalseBytes32: unknown = event.args[3 as keyof typeof event.args];
        const resultBytes32: unknown = event.args[4 as keyof typeof event.args];

        assertEventArgIsBytes32String(controlBytes32, event.name, 1);
        assertEventArgIsBytes32String(ifTrueBytes32, event.name, 2);
        assertEventArgIsBytes32String(ifFalseBytes32, event.name, 3);
        assertEventArgIsBytes32String(resultBytes32, event.name, 4);

        // HCU is computed using the result bytes32 type
        const resultFheTypeName: FheTypeName = getFheTypeName(parseFhevmHandle(resultBytes32).fhevmType);

        hcuConsumed = _getTypeOperatorHCU(event.op, resultFheTypeName);
        totalHCUConsumed += hcuConsumed;

        hcuMap[resultBytes32] =
          hcuConsumed +
          Math.max(
            // Compute max
            readFromHCUMap(controlBytes32),
            readFromHCUMap(ifTrueBytes32),
            readFromHCUMap(ifFalseBytes32),
          );
        break;
      }

      case 'FheRand': {
        // "event FheRand(address indexed caller, uint8 randType, bytes16 seed, bytes32 result)",
        const randTypeUint8: unknown = event.args[1 as keyof typeof event.args];
        const seedBytes16: unknown = event.args[2 as keyof typeof event.args];
        const resultBytes32: unknown = event.args[3 as keyof typeof event.args];

        assertEventArgIsBigUint8(randTypeUint8, event.name, 1);
        assertEventArgIsBytes16String(seedBytes16, event.name, 2);
        assertEventArgIsBytes32String(resultBytes32, event.name, 3);

        // HCU is computed using the randType uint8 arg
        const randTypeFheTypeName: FheTypeName = getFheTypeNameFromByte(Number(randTypeUint8));

        hcuConsumed = _getTypeOperatorHCU(event.op, randTypeFheTypeName);
        totalHCUConsumed += hcuConsumed;

        hcuMap[resultBytes32] = hcuConsumed;
        break;
      }

      case 'FheRandBounded': {
        // "event FheRandBounded(address indexed caller, uint256 upperBound, uint8 randType, bytes16 seed, bytes32 result)",
        const upperBoundUint256: unknown = event.args[1 as keyof typeof event.args];
        const randTypeUint8: unknown = event.args[2 as keyof typeof event.args];
        const seedBytes16: unknown = event.args[3 as keyof typeof event.args];
        const resultBytes32: unknown = event.args[4 as keyof typeof event.args];

        assertEventArgIsBigUint256(upperBoundUint256, event.name, 1);
        assertEventArgIsBigUint8(randTypeUint8, event.name, 2);
        assertEventArgIsBytes16String(seedBytes16, event.name, 3);
        assertEventArgIsBytes32String(resultBytes32, event.name, 4);

        // Price is computed using the result bytes32 type
        const resultFheTypeName: FheTypeName = getFheTypeName(parseFhevmHandle(resultBytes32).fhevmType);

        hcuConsumed = _getTypeOperatorHCU(event.op, resultFheTypeName);
        totalHCUConsumed += hcuConsumed;

        hcuMap[resultBytes32] = hcuConsumed;
        break;
      }

      case 'FheSum': {
        // event FheSum(address indexed caller, bytes32[] values, bytes32 result);
        const values: unknown = event.args[1 as keyof typeof event.args];
        const resultBytes32: unknown = event.args[2 as keyof typeof event.args];
        assertEventArgIsArrayOfBytes32String(values, event.name, 1);
        assertEventArgIsBytes32String(resultBytes32, event.name, 2);

        // Priced on the result type and bucketed by how many elements were summed.
        const resultFheTypeName: FheTypeName = getFheTypeName(parseFhevmHandle(resultBytes32).fhevmType);
        hcuConsumed = getHCU(event.name, resultFheTypeName, { n: values.length });
        totalHCUConsumed += hcuConsumed;

        // Depth carries the deepest input, as for any other n-ary operator.
        hcuMap[resultBytes32] = hcuConsumed + Math.max(0, ...values.map((v) => readFromHCUMap(v)));
        break;
      }

      case 'FheIsIn': {
        // event FheIsIn(address indexed caller, bytes32 value, bytes32[] values, bytes32 result);
        const valueBytes32: unknown = event.args[1 as keyof typeof event.args];
        const values: unknown = event.args[2 as keyof typeof event.args];
        const resultBytes32: unknown = event.args[3 as keyof typeof event.args];
        assertEventArgIsBytes32String(valueBytes32, event.name, 1);
        assertEventArgIsArrayOfBytes32String(values, event.name, 2);
        assertEventArgIsBytes32String(resultBytes32, event.name, 3);

        // Priced on the needle's type — the result is an ebool — and bucketed by haystack size.
        const valueFheTypeName: FheTypeName = getFheTypeName(parseFhevmHandle(valueBytes32).fhevmType);
        hcuConsumed = getHCU(event.name, valueFheTypeName, { n: values.length });
        totalHCUConsumed += hcuConsumed;

        hcuMap[resultBytes32] =
          hcuConsumed + Math.max(readFromHCUMap(valueBytes32), ...values.map((v) => readFromHCUMap(v)));
        break;
      }
    }
  }

  // Deepest single handle. `Object.values` is typed `number[]`, so no entry needs an undefined check.
  const maxDepthHCU = Math.max(0, ...Object.values(hcuMap));

  return {
    transactionHash: receipt.hash as `0x${string}`,
    globalHCU: totalHCUConsumed,
    maxHCUDepth: maxDepthHCU,
    HCUDepthByHandle: hcuMap,
  };
}
