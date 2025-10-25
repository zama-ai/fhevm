import { type FheTypeInfo, type Operator, OperatorArguments, ReturnType } from './common.js';
import { ALL_FHE_TYPE_INFOS } from './fheTypeInfos.js';
import { ALL_OPERATORS } from './operators.js';

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
function validateFHETypeInfos(fheTypes: FheTypeInfo[]): void {
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
function validateOperators(operators: Operator[]): void {
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

export function validate() {
  // Validate the FHE types
  validateFHETypeInfos(ALL_FHE_TYPE_INFOS);
  // Validate the operators
  validateOperators(ALL_OPERATORS);
}
