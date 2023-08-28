import assert from 'assert';

export type Operator = {
  name: string;
  leftScalarInvertOp?: string;
  precompileName: string;
  hasScalar: boolean;
  hasEncrypted: boolean;
  arguments: OperatorArguments;
  returnType: ReturnType;
  tfheSolOrder: number;
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
  { name: 'Require', code: 69 },
  { name: 'LessThanOrEqual', code: 70 },
  { name: 'Subtract', code: 71 },
  { name: 'Multiply', code: 72 },
  { name: 'LessThan', code: 73 },
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
];

export const ALL_OPERATORS: Operator[] = [
  {
    name: 'add',
    precompileName: 'Add',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 7,
  },
  {
    name: 'sub',
    precompileName: 'Subtract',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 8,
  },
  {
    name: 'mul',
    precompileName: 'Multiply',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 9,
  },
  {
    name: 'div',
    precompileName: 'Divide',
    hasScalar: true,
    hasEncrypted: false,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 17,
  },
  {
    name: 'and',
    precompileName: 'BitwiseAnd',
    hasScalar: false,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 10,
  },
  {
    name: 'or',
    precompileName: 'BitwiseOr',
    hasScalar: false,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 11,
  },
  {
    name: 'xor',
    precompileName: 'BitwiseXor',
    hasScalar: false,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 12,
  },
  {
    name: 'shl',
    precompileName: 'ShiftLeft',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 13,
  },
  {
    name: 'shr',
    precompileName: 'ShiftRight',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 14,
  },
  {
    name: 'eq',
    precompileName: 'Equal',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    tfheSolOrder: 1,
  },
  {
    name: 'ne',
    precompileName: 'NotEqual',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    tfheSolOrder: 2,
  },
  {
    name: 'ge',
    leftScalarInvertOp: 'le',
    precompileName: 'GreaterThanOrEqual',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    tfheSolOrder: 3,
  },
  {
    name: 'gt',
    leftScalarInvertOp: 'lt',
    precompileName: 'GreaterThan',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    tfheSolOrder: 4,
  },
  {
    name: 'le',
    leftScalarInvertOp: 'ge',
    precompileName: 'LessThanOrEqual',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    tfheSolOrder: 5,
  },
  {
    name: 'lt',
    leftScalarInvertOp: 'gt',
    precompileName: 'LessThan',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    tfheSolOrder: 6,
  },
  {
    name: 'min',
    precompileName: 'Min',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 15,
  },
  {
    name: 'max',
    precompileName: 'Max',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 16,
  },
  {
    name: 'neg',
    precompileName: 'Negate',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Unary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 18,
  },
  {
    name: 'not',
    precompileName: 'Not',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Unary,
    returnType: ReturnType.Uint,
    tfheSolOrder: 19,
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
