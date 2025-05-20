/**
 * Enum representing different types of arguments.
 */
export enum ArgumentType {
  /**
   * Represents an encrypted boolean argument type.
   */
  Ebool,

  /**
   * Represents an encrypted unsigned integer argument type.
   */
  Euint,

  /**
   * Represents a generic unsigned integer argument type.
   */
  Uint,
}

export type FunctionType = {
  /**
   * The type of the function argument.
   */
  type: ArgumentType;

  /**
   * The bit length of the function argument.
   */
  bits: number;
};

export type OverloadSignature = {
  /**
   * The name of the overload signature.
   */
  name: string;

  /**
   * The arguments of the overload signature.
   */
  arguments: FunctionType[];

  /**
   * The return type of the overload signature.
   */
  returnType: FunctionType;

  /**
   * The binary operator associated with the overload signature.
   * Optional.
   */
  binaryOperator?: string;

  /**
   * The unary operator associated with the overload signature.
   * Optional.
   */
  unaryOperator?: string;
};

export type OverloadShard = {
  /**
   * The shard number of the overload.
   */
  shardNumber: number;

  /**
   * The overload signatures in the shard.
   */
  overloads: OverloadSignature[];
};

/**
 * Represents a Fully Homomorphic Encryption (FHE) type definition.
 * This interface defines the structure of an FHE type, including its
 * properties, supported operators, and related metadata.
 */
export interface FheType {
  /**
   * The name or identifier of the FHE type.
   */
  type: string;

  /**
   * A list of operators that are supported by this FHE type.
   */
  supportedOperators: string[];

  /**
   * The bit length of the FHE type, representing its size in bits.
   */
  bitLength: number;

  /**
   * The corresponding clear (non-encrypted) type that matches this FHE type.
   */
  clearMatchingType: string;

  /**
   * The value associated with this FHE type.
   */
  value: number;

  /**
   * An optional list of alias types that are associated with this FHE type.
   */
  aliases?: AliasFheType[];
}

/**
 * Represents an alias for a Fully Homomorphic Encryption (FHE) type.
 * This interface provides a way to define alternative names or representations
 * for an FHE type, along with its supported operators and the corresponding
 * clear (unencrypted) matching type.
 */
export interface AliasFheType {
  /**
   * The name or identifier of the FHE type.
   */
  type: string;

  /**
   * A list of operators that are supported by this FHE type.
   */
  supportedOperators: string[];

  /**
   * The corresponding clear (non-encrypted) type that matches this FHE type.
   */
  clearMatchingType: string;
}

/**
 * Represents an adjusted Fully Homomorphic Encryption (FHE) type with metadata
 * about its properties, supported operations, and related type information.
 */
export interface AdjustedFheType {
  /**
   * The name of the FHE type.
   */
  type: string;

  /**
   * A list of operators supported by this FHE type.
   */
  supportedOperators: string[];

  /**
   * The bit length of the FHE type, indicating its size or precision.
   */
  bitLength: number;

  /**
   * The corresponding clear (non-encrypted) type that matches this FHE type.
   */
  clearMatchingType: string;

  /**
   * (Optional) A specific value associated with this FHE type.
   */
  value?: number;

  /**
   * (Optional) Indicates whether this type is an alias for another type.
   */
  isAlias?: boolean;

  /**
   * (Optional) The name of the type this alias refers to, if applicable.
   */
  aliasType?: string;

  /**
   * (Optional) The corresponding clear type for the alias, if applicable.
   */
  clearMatchingTypeAlias?: string;
}

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
   */
  fheLibName: string;

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
  Euint,
  Ebool,
}

/**
 * Validates the FHE (Fully Homomorphic Encryption) types.
 *
 * This function ensures that the provided FHE types are correctly defined,
 * have valid properties, and do not contain duplicates.
 *
 * @param fheTypes - An array of FHE types to validate.
 * @throws Will throw an error if the FHE types array is undefined, not an array, or empty.
 * Will throw an error if any FHE type has invalid properties or if duplicate FHE types are found.
 */
export function validateFHETypes(fheTypes: FheType[]): void {
  if (!fheTypes || !Array.isArray(fheTypes) || fheTypes.length === 0) {
    throw new Error('fheTypes is not defined or invalid');
  }
  fheTypes.forEach((fheType) => {
    if (typeof fheType.type !== 'string' || typeof fheType.clearMatchingType !== 'string') {
      throw new Error(`Invalid FHE type: ${JSON.stringify(fheType)}`);
    }

    if (!Array.isArray(fheType.supportedOperators) || fheType.supportedOperators.some((op) => typeof op !== 'string')) {
      throw new Error(`Invalid supportedOperators for FHE type: ${fheType.type}`);
    }

    if (typeof fheType.bitLength !== 'number' || fheType.bitLength < 0) {
      throw new Error(`Invalid bitLength for FHE type: ${fheType.type}`);
    }

    if (typeof fheType.value !== 'number' || fheType.value < 0) {
      throw new Error(`Invalid value for FHE type: ${fheType.type}`);
    }

    if (
      fheType.aliases &&
      (!Array.isArray(fheType.aliases) ||
        fheType.aliases.some(
          (alias) =>
            typeof alias.type !== 'string' ||
            !Array.isArray(alias.supportedOperators) ||
            alias.supportedOperators.some((op) => typeof op !== 'string') ||
            typeof alias.clearMatchingType !== 'string',
        ))
    ) {
      throw new Error(`Invalid aliases for FHE type: ${fheType.type}`);
    }

    if (fheType.aliases) {
      fheType.aliases.forEach((alias) => {
        if (typeof alias.type !== 'string') {
          throw new Error(`Invalid alias type: ${JSON.stringify(alias)}`);
        }

        if (!Array.isArray(alias.supportedOperators) || alias.supportedOperators.some((op) => typeof op !== 'string')) {
          throw new Error(`Invalid supportedOperators for alias: ${alias.type}`);
        }

        if (typeof alias.clearMatchingType !== 'string') {
          throw new Error(`Invalid clearMatchingType for alias: ${alias.type}`);
        }
      });
    }
  });

  const nameMap: Record<string, boolean> = {};

  fheTypes.forEach((fheType) => {
    if (nameMap[fheType.type] != null) {
      throw new Error(`Duplicate FheType found: ${fheType.type}`);
    }

    nameMap[fheType.type] = true;
  });
}

/**
 * Validates an array of operators to ensure they are correctly defined and have unique names.
 *
 * @param operators - An array of Operator objects to validate.
 * @throws Will throw an error if the operators array is undefined, not an array, or empty.
 * @throws Will throw an error if any operator has invalid properties or if duplicate operator names are found.
 */
export function validateOperators(operators: Operator[]): void {
  if (!operators || !Array.isArray(operators) || operators.length === 0) {
    throw new Error('Operators is not defined or invalid');
  }

  const nameMap: Record<string, boolean> = {};

  operators.forEach((op) => {
    if (typeof op.name !== 'string' || op.name.trim() === '') {
      throw new Error(`Invalid operator name: ${JSON.stringify(op)}`);
    }

    if (nameMap[op.name] != null) {
      throw new Error(`Duplicate operator name found: ${op.name}`);
    }

    if (typeof op.hasScalar !== 'boolean') {
      throw new Error(`Invalid hasScalar value for operator: ${op.name}`);
    }

    if (typeof op.hasEncrypted !== 'boolean') {
      throw new Error(`Invalid hasEncrypted value for operator: ${op.name}`);
    }

    if (op.arguments === null || !Object.values(OperatorArguments).includes(op.arguments)) {
      throw new Error(`Invalid arguments for operator: ${op.name}`);
    }

    if (op.returnType === null || !Object.values(ReturnType).includes(op.returnType)) {
      throw new Error(`Invalid returnType for operator: ${op.name}`);
    }

    if (op.leftScalarInvertOp && typeof op.leftScalarInvertOp !== 'string') {
      throw new Error(`Invalid leftScalarInvertOp for operator: ${op.name}`);
    }

    if (op.leftScalarEncrypt != null && typeof op.leftScalarEncrypt !== 'boolean') {
      throw new Error(`Invalid leftScalarEncrypt value for operator: ${op.name}`);
    }

    if (op.leftScalarDisable != null && typeof op.leftScalarDisable !== 'boolean') {
      throw new Error(`Invalid leftScalarDisable value for operator: ${op.name}`);
    }

    if (typeof op.fheLibName !== 'string' || op.fheLibName.trim() === '') {
      throw new Error(`Invalid fheLibName for operator: ${op.name}`);
    }

    if (op.shiftOperator != null && typeof op.shiftOperator !== 'boolean') {
      throw new Error(`Invalid shiftOperator value for operator: ${op.name}`);
    }

    if (op.rotateOperator != null && typeof op.rotateOperator !== 'boolean') {
      throw new Error(`Invalid rotateOperator value for operator: ${op.name}`);
    }

    nameMap[op.name] = true;
  });
}
