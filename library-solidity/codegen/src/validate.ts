import { type FheTypeInfo, type Operator, OperatorArguments, ReturnType } from './common';
import { ALL_FHE_TYPE_INFOS } from './fheTypeInfos';
import { ALL_OPERATORS } from './operators';

// Runtime type checks. Values are checked as `unknown`: their static types are not trusted at runtime.

/** `true` for a plain object (not `null`, not an array). */
function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

/** `true` for a string that is not empty or whitespace only. */
function isNonEmptyString(value: unknown): value is string {
  return typeof value === 'string' && value.trim() !== '';
}

/** `true` for an optional boolean field: absent (`undefined`) or a boolean. */
function isOptionalBoolean(value: unknown): value is boolean | undefined {
  return value === undefined || typeof value === 'boolean';
}

/**
 * `true` if `value` is one of the values of the numeric enum `enumObject`.
 * A numeric enum object also maps its values back to their names (e.g. `{ 0: 'Binary', Binary: 0 }`), so only the
 * numbers are enum values: a name such as `'Binary'` is rejected.
 */
function isNumericEnumValue(enumObject: Record<string, string | number>, value: unknown): boolean {
  return typeof value === 'number' && Object.values(enumObject).includes(value);
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
function validateFHETypeInfos(fheTypes: FheTypeInfo[]): void {
  // Runtime check: the static type is not trusted here, so the check runs on an `unknown` view of the value.
  // `Array.isArray` also rejects `undefined` and `null`.
  const fheTypesValue: unknown = fheTypes;
  if (!Array.isArray(fheTypesValue) || fheTypesValue.length === 0) {
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

  const seenTypes = new Set<string>();

  fheTypes.forEach((fheType) => {
    if (seenTypes.has(fheType.type)) {
      throw new Error(`Duplicate FheType found: ${fheType.type}`);
    }

    seenTypes.add(fheType.type);
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
  const operatorsValue: unknown = operators;
  if (!Array.isArray(operatorsValue) || operatorsValue.length === 0) {
    throw new Error('Operators is not defined or invalid');
  }

  const names = new Set<string>();
  // Checked once all the names are known: each must name another operator.
  const leftScalarInvertOps: Array<{ name: string; invertOp: string }> = [];

  operatorsValue.forEach((op: unknown) => {
    if (!isRecord(op)) {
      throw new Error(`Invalid operator: ${JSON.stringify(op)}`);
    }

    const name = op.name;
    if (!isNonEmptyString(name)) {
      throw new Error(`Invalid operator name: ${JSON.stringify(op)}`);
    }

    if (names.has(name)) {
      throw new Error(`Duplicate operator name found: ${name}`);
    }

    if (typeof op.hasScalar !== 'boolean') {
      throw new Error(`Invalid hasScalar value for operator: ${name}`);
    }

    if (typeof op.hasEncrypted !== 'boolean') {
      throw new Error(`Invalid hasEncrypted value for operator: ${name}`);
    }

    if (!isNumericEnumValue(OperatorArguments, op.arguments)) {
      throw new Error(`Invalid arguments for operator: ${name}`);
    }

    if (!isNumericEnumValue(ReturnType, op.returnType)) {
      throw new Error(`Invalid returnType for operator: ${name}`);
    }

    const invertOp = op.leftScalarInvertOp;
    if (invertOp !== undefined) {
      if (!isNonEmptyString(invertOp)) {
        throw new Error(`Invalid leftScalarInvertOp for operator: ${name}`);
      }
      leftScalarInvertOps.push({ name, invertOp });
    }

    if (!isOptionalBoolean(op.leftScalarEncrypt)) {
      throw new Error(`Invalid leftScalarEncrypt value for operator: ${name}`);
    }

    if (!isOptionalBoolean(op.leftScalarDisable)) {
      throw new Error(`Invalid leftScalarDisable value for operator: ${name}`);
    }

    if (!isNonEmptyString(op.fheLibName)) {
      throw new Error(`Invalid fheLibName for operator: ${name}`);
    }

    if (!isOptionalBoolean(op.shiftOperator)) {
      throw new Error(`Invalid shiftOperator value for operator: ${name}`);
    }

    if (!isOptionalBoolean(op.rotateOperator)) {
      throw new Error(`Invalid rotateOperator value for operator: ${name}`);
    }

    names.add(name);
  });

  leftScalarInvertOps.forEach(({ name, invertOp }) => {
    if (!names.has(invertOp)) {
      throw new Error(`Invalid leftScalarInvertOp for operator: ${name}. Unknown operator: ${invertOp}`);
    }
  });
}

export function validate(): void {
  // Validate the FHE types
  validateFHETypeInfos(ALL_FHE_TYPE_INFOS);
  // Validate the operators
  validateOperators(ALL_OPERATORS);
}
