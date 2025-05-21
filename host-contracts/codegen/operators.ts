import { Operator, OperatorArguments, ReturnType } from './common';

/**
 * A list of all supported operators with their respective properties.
 *
 * Each operator object contains the following properties:
 * - `name`: The name of the operator.
 * - `hasScalar`: A boolean indicating if the operator supports scalar values.
 * - `hasEncrypted`: A boolean indicating if the operator supports encrypted values.
 * - `arguments`: The type of arguments the operator accepts (binary or unary).
 * - `returnType`: The return type of the operator.
 * - `fheLibName`: The corresponding function name in the FHE library.
 * - `leftScalarEncrypt` (optional): A boolean indicating if the left scalar should be encrypted.
 * - `leftScalarDisable` (optional): A boolean indicating if the left scalar is disabled.
 * - `shiftOperator` (optional): A boolean indicating if the operator is a shift operator.
 * - `rotateOperator` (optional): A boolean indicating if the operator is a rotate operator.
 * - `leftScalarInvertOp` (optional): The name of the inverted operator for the left scalar.
 */
export const ALL_OPERATORS: Operator[] = [
  {
    name: 'add',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    fheLibName: 'fheAdd',
  },
  {
    name: 'sub',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    leftScalarEncrypt: true,
    fheLibName: 'fheSub',
  },
  {
    name: 'mul',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    fheLibName: 'fheMul',
  },
  {
    name: 'div',
    hasScalar: true,
    hasEncrypted: false,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    leftScalarDisable: true,
    fheLibName: 'fheDiv',
  },
  {
    name: 'rem',
    hasScalar: true,
    hasEncrypted: false,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    leftScalarDisable: true,
    fheLibName: 'fheRem',
  },
  {
    name: 'and',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    fheLibName: 'fheBitAnd',
  },
  {
    name: 'or',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    fheLibName: 'fheBitOr',
  },
  {
    name: 'xor',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    fheLibName: 'fheBitXor',
  },
  {
    name: 'shl',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    leftScalarEncrypt: true,
    shiftOperator: true,
    fheLibName: 'fheShl',
  },
  {
    name: 'shr',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    leftScalarEncrypt: true,
    shiftOperator: true,
    fheLibName: 'fheShr',
  },
  {
    name: 'rotl',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    leftScalarEncrypt: true,
    rotateOperator: true,
    fheLibName: 'fheRotl',
  },
  {
    name: 'rotr',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    leftScalarEncrypt: true,
    rotateOperator: true,
    fheLibName: 'fheRotr',
  },
  {
    name: 'eq',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    fheLibName: 'fheEq',
  },
  {
    name: 'ne',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    fheLibName: 'fheNe',
  },
  {
    name: 'ge',
    leftScalarInvertOp: 'le',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    fheLibName: 'fheGe',
  },
  {
    name: 'gt',
    leftScalarInvertOp: 'lt',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    fheLibName: 'fheGt',
  },
  {
    name: 'le',
    leftScalarInvertOp: 'ge',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    fheLibName: 'fheLe',
  },
  {
    name: 'lt',
    leftScalarInvertOp: 'gt',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Ebool,
    fheLibName: 'fheLt',
  },
  {
    name: 'min',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    fheLibName: 'fheMin',
  },
  {
    name: 'max',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Euint,
    fheLibName: 'fheMax',
  },
  {
    name: 'neg',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Unary,
    returnType: ReturnType.Euint,
    fheLibName: 'fheNeg',
  },
  {
    name: 'not',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Unary,
    returnType: ReturnType.Euint,
    fheLibName: 'fheNot',
  },
];
