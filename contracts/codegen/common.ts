import { strict as assert } from 'node:assert';

export enum Network {
  Evmos,
  Network1,
}

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
  shiftOperator?: boolean;
  rotateOperator?: boolean;
};

export type CodegenContext = {
  libFheAddress: string;
};

export enum OperatorArguments {
  Binary,
  Unary,
}

export enum ReturnType {
  Uint,
  Ebool,
}

export const SUPPORTED_BITS: number[] = [4, 8, 16, 32, 64];

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
    shiftOperator: true,
  },
  {
    name: 'shr',
    precompileName: 'ShiftRight',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    leftScalarEncrypt: true,
    shiftOperator: true,
  },
  {
    name: 'rotl',
    precompileName: 'RotateLeft',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    leftScalarEncrypt: true,
    rotateOperator: true,
  },
  {
    name: 'rotr',
    precompileName: 'RotateRight',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    leftScalarEncrypt: true,
    rotateOperator: true,
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

export function networkCodegenContext(network: Network): CodegenContext {
  switch (network) {
    case Network.Evmos:
      return {
        libFheAddress: '0x000000000000000000000000000000000000005d',
      };
    case Network.Network1:
      return {
        libFheAddress: '0x010000000000000000000000000000000000005D',
      };
  }
}
