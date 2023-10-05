import { strict as assert } from 'node:assert';

export type Operator = {
  name: string;
  // express left scalar operation as different operation with arguments swapped
  leftScalarInvertOp?: string;
  precompileName: string;
  hasScalar: boolean;
  hasEncrypted: boolean;
  arguments: OperatorArguments;
  returnType: ReturnType;
  // if true do trivial encryption for left scalar operand, this is workaround until tfhe-rs supports left scalar operands
  leftScalarEncrypt?: boolean;
  // disable left scalar operator
  leftScalarDisable?: boolean;
  fheLibName?: string;
  binarySolidityOperator?: string;
  unarySolidityOperator?: string;
};

export type Precompile = {
  name: string;
  code: number;
};

export enum OperatorArguments {
  Binary,
  Unary,
}

export enum ReturnType {
  Uint,
  Ebool,
}

export const SUPPORTED_BITS: number[] = [8, 16, 32];

export const ALL_PRECOMPILES: Precompile[] = [
  { name: 'Add', code: 65 },
  { name: 'Verify', code: 66 },
  { name: 'Reencrypt', code: 67 },
  { name: 'FhePubKey', code: 68 },
  { name: 'LessThanOrEqual', code: 70 },
  { name: 'Subtract', code: 71 },
  { name: 'Multiply', code: 72 },
  { name: 'LessThan', code: 73 },
  { name: 'Rand', code: 74 },
  { name: 'OptimisticRequire', code: 75 },
  { name: 'Cast', code: 76 },
  { name: 'TrivialEncrypt', code: 77 },
  { name: 'BitwiseAnd', code: 78 },
  { name: 'BitwiseOr', code: 79 },
  { name: 'BitwiseXor', code: 80 },
  { name: 'Equal', code: 81 },
  { name: 'GreaterThanOrEqual', code: 82 },
  { name: 'GreaterThan', code: 83 },
  { name: 'ShiftLeft', code: 84 },
  { name: 'ShiftRight', code: 85 },
  { name: 'NotEqual', code: 86 },
  { name: 'Min', code: 87 },
  { name: 'Max', code: 88 },
  { name: 'Negate', code: 89 },
  { name: 'Not', code: 90 },
  { name: 'Decrypt', code: 91 },
  { name: 'Divide', code: 92 },
  { name: 'Rem', code: 94 },
];

export const ALL_OPERATORS: Operator[] = [
  {
    name: 'add',
    precompileName: 'Add',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    binarySolidityOperator: '+',
  },
  {
    name: 'sub',
    precompileName: 'Subtract',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    leftScalarEncrypt: true,
    binarySolidityOperator: '-',
  },
  {
    name: 'mul',
    precompileName: 'Multiply',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    binarySolidityOperator: '*',
  },
  {
    name: 'div',
    precompileName: 'Divide',
    hasScalar: true,
    hasEncrypted: false,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    leftScalarDisable: true,
  },
  {
    name: 'rem',
    precompileName: 'Rem',
    hasScalar: true,
    hasEncrypted: false,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    leftScalarDisable: true,
  },
  {
    name: 'and',
    precompileName: 'BitwiseAnd',
    hasScalar: false,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    fheLibName: 'fheBitAnd',
    binarySolidityOperator: '&',
  },
  {
    name: 'or',
    precompileName: 'BitwiseOr',
    hasScalar: false,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    fheLibName: 'fheBitOr',
    binarySolidityOperator: '|',
  },
  {
    name: 'xor',
    precompileName: 'BitwiseXor',
    hasScalar: false,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    fheLibName: 'fheBitXor',
    binarySolidityOperator: '^',
  },
  {
    name: 'shl',
    precompileName: 'ShiftLeft',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    leftScalarEncrypt: true,
  },
  {
    name: 'shr',
    precompileName: 'ShiftRight',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    leftScalarEncrypt: true,
  },
  {
    name: 'eq',
    precompileName: 'Equal',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
  },
  {
    name: 'ne',
    precompileName: 'NotEqual',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
  },
  {
    name: 'ge',
    leftScalarInvertOp: 'le',
    precompileName: 'GreaterThanOrEqual',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
  },
  {
    name: 'gt',
    leftScalarInvertOp: 'lt',
    precompileName: 'GreaterThan',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
  },
  {
    name: 'le',
    leftScalarInvertOp: 'ge',
    precompileName: 'LessThanOrEqual',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
  },
  {
    name: 'lt',
    leftScalarInvertOp: 'gt',
    precompileName: 'LessThan',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
  },
  {
    name: 'min',
    precompileName: 'Min',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
  },
  {
    name: 'max',
    precompileName: 'Max',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
  },
  {
    name: 'neg',
    precompileName: 'Negate',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Unary,
    returnType: ReturnType.Uint,
    unarySolidityOperator: '-',
  },
  {
    name: 'not',
    precompileName: 'Not',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Unary,
    returnType: ReturnType.Uint,
    unarySolidityOperator: '~',
  },
];

export function checks(operators: Operator[]): Operator[] {
  const nameMap: { [key: string]: boolean } = {};
  const precompNameMap: { [key: string]: boolean } = {};

  operators.forEach((op) => {
    assert(nameMap[op.name] == null);
    nameMap[op.name] = true;

    assert(precompNameMap[op.precompileName] == null);
    precompNameMap[op.precompileName] = true;
  });

  return operators;
}
