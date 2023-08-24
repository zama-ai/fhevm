import assert from 'assert';

export type Operator = {
  name: string;
  precompileName: string;
  bits: number[];
  hasScalar: boolean;
  hasEncrypted: boolean;
  arguments: OperatorArguments;
};

export type Precompile = {
  name: string;
  code: number;
};

export enum OperatorArguments {
  Binary,
  Unary,
}

export const SUPPORTED_BITS = [8, 16, 32];

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
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'sub',
    precompileName: 'Subtract',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'mul',
    precompileName: 'Multiply',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'div',
    precompileName: 'Divide',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: false,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'and',
    precompileName: 'BitwiseAnd',
    bits: SUPPORTED_BITS,
    hasScalar: false,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'or',
    precompileName: 'BitwiseOr',
    bits: SUPPORTED_BITS,
    hasScalar: false,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'xor',
    precompileName: 'BitwiseXor',
    bits: SUPPORTED_BITS,
    hasScalar: false,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'shl',
    precompileName: 'ShiftLeft',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'shr',
    precompileName: 'ShiftRight',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'eq',
    precompileName: 'Equal',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'ne',
    precompileName: 'NotEqual',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'ge',
    precompileName: 'GreaterThanOrEqual',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'gt',
    precompileName: 'GreaterThan',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'le',
    precompileName: 'LessThanOrEqual',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'lt',
    precompileName: 'LessThan',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'min',
    precompileName: 'Min',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'max',
    precompileName: 'Max',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
  },
  {
    name: 'neg',
    precompileName: 'Negate',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Unary,
  },
  {
    name: 'not',
    precompileName: 'Not',
    bits: SUPPORTED_BITS,
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Unary,
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
