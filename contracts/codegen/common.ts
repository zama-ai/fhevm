import { strict as assert } from 'assert';

/**
 * Represents an operator with various properties and configurations.
 */
export type Operator = {
  /**
   * The name of the operator.
   */
  name: string;

  /**
   * Express left scalar operation as a different operation with arguments swapped.
   * Optional.
   */
  leftScalarInvertOp?: string;

  /**
   * The name of the precompiled function associated with this operator.
   */
  precompileName: string;

  /**
   * Indicates if the operator has a scalar operand.
   */
  hasScalar: boolean;

  /**
   * Indicates if the operator has an encrypted operand.
   */
  hasEncrypted: boolean;

  /**
   * The arguments required by the operator.
   */
  arguments: OperatorArguments;

  /**
   * The return type of the operator.
   */
  returnType: ReturnType;

  /**
   * If true, perform trivial encryption for the left scalar operand.
   * This is a workaround until tfhe-rs supports left scalar operands.
   * Optional.
   */
  leftScalarEncrypt?: boolean;

  /**
   * If true, disable the left scalar operator.
   * Optional.
   */
  leftScalarDisable?: boolean;

  /**
   * The name of the FHE library associated with this operator.
   * Optional.
   */
  fheLibName?: string;

  /**
   * The binary Solidity operator associated with this operator.
   * Optional.
   */
  binarySolidityOperator?: string;

  /**
   * The unary Solidity operator associated with this operator.
   * Optional.
   */
  unarySolidityOperator?: string;

  /**
   * Indicates if the operator is a shift operator.
   * Optional.
   */
  shiftOperator?: boolean;

  /**
   * Indicates if the operator is a rotate operator.
   * Optional.
   */
  rotateOperator?: boolean;
};

/**
 * Enum representing the types of operator arguments.
 *
 * @enum {number}
 * @property {number} Binary - Represents a binary operator argument.
 * @property {number} Unary - Represents a unary operator argument.
 */
export enum OperatorArguments {
  Binary,
  Unary,
}

/**
 * Enum representing the possible return types.
 */
export enum ReturnType {
  Uint,
  Ebool,
}

/**
 * An array of supported bit lengths for cryptographic operations.
 * The supported bit lengths include 8, 16, 32, 64, 128, and 256 bits.
 */
export const SUPPORTED_BITS: number[] = [8, 16, 32, 64, 128, 256];

/**
 * An array of supported unsigned integer bit lengths.
 */
export const SUPPORTED_UINT = [8, 16, 32, 64, 128, 256];

/**
 * A list of all supported operators with their respective properties.
 *
 * Each operator object contains the following properties:
 * - `name`: The name of the operator.
 * - `precompileName`: The precompiled name of the operator.
 * - `hasScalar`: A boolean indicating if the operator supports scalar values.
 * - `hasEncrypted`: A boolean indicating if the operator supports encrypted values.
 * - `arguments`: The type of arguments the operator accepts (binary or unary).
 * - `returnType`: The return type of the operator.
 * - `binarySolidityOperator` (optional): The corresponding binary operator in Solidity.
 * - `leftScalarEncrypt` (optional): A boolean indicating if the left scalar should be encrypted.
 * - `leftScalarDisable` (optional): A boolean indicating if the left scalar is disabled.
 * - `fheLibName` (optional): The corresponding function name in the FHE library.
 * - `shiftOperator` (optional): A boolean indicating if the operator is a shift operator.
 * - `rotateOperator` (optional): A boolean indicating if the operator is a rotate operator.
 * - `leftScalarInvertOp` (optional): The name of the inverted operator for the left scalar.
 * - `unarySolidityOperator` (optional): The corresponding unary operator in Solidity.
 */
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
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    fheLibName: 'fheBitAnd',
    binarySolidityOperator: '&',
  },
  {
    name: 'or',
    precompileName: 'BitwiseOr',
    hasScalar: true,
    hasEncrypted: true,
    arguments: OperatorArguments.Binary,
    returnType: ReturnType.Uint,
    fheLibName: 'fheBitOr',
    binarySolidityOperator: '|',
  },
  {
    name: 'xor',
    precompileName: 'BitwiseXor',
    hasScalar: true,
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

/**
 * Validates the list of operators to ensure there are no duplicate names or precompile names.
 *
 * @param operators - The list of operators to validate.
 * @returns The validated list of operators.
 * @throws Will throw an error if a duplicate operator name or precompile name is found.
 */
export function checks(operators: Operator[]): Operator[] {
  const nameMap: Record<string, boolean> = {};
  const precompNameMap: Record<string, boolean> = {};

  operators.forEach((op) => {
    assert(nameMap[op.name] == null, `Duplicate operator name found: ${op.name}`);
    assert(precompNameMap[op.precompileName] == null, `Duplicate precompileName found: ${op.precompileName}`);

    nameMap[op.name] = true;
    precompNameMap[op.precompileName] = true;
  });

  return operators;
}
