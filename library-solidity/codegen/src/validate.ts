import type { FheTypeInfo, Operator, PriceData } from './common.js';
import { FHE_OPERATOR_NAMES, FHE_TYPE_NAMES, OperatorArguments, ReturnType } from './common.js';
import { ALL_FHE_TYPE_INFOS } from './fheTypeInfos.js';
import { ALL_OPERATORS } from './operators.js';
import { ALL_OPERATORS_PRICES } from './operatorsPrices.js';

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

/** `true` for one of the FHE type names (`FheTypeName`). */
function isFheTypeName(value: unknown): boolean {
  return typeof value === 'string' && (FHE_TYPE_NAMES as readonly string[]).includes(value);
}

/** `true` for an optional number field: absent (`undefined`) or a number. */
function isOptionalNumber(value: unknown): value is number | undefined {
  return value === undefined || typeof value === 'number';
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
    if (!isFheTypeName(fheType.type) || typeof fheType.clearMatchingType !== 'string') {
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

/**
 * Validates a per-FHE-type cost map (`scalar`, `nonScalar`, `types` or `nBucketed` of a `PriceData` entry): optional,
 * keyed by FHE type names, each value checked by `isValidCost`.
 */
function validateCostMap(
  map: unknown,
  isValidCost: (cost: unknown) => boolean,
  operatorName: string,
  fieldName: string,
): void {
  if (map === undefined) {
    return;
  }
  if (!isRecord(map)) {
    throw new Error(`Invalid ${fieldName} prices for operator: ${operatorName}`);
  }
  for (const [typeName, cost] of Object.entries(map)) {
    if (!isFheTypeName(typeName)) {
      throw new Error(`Invalid ${fieldName} prices for operator: ${operatorName}. Unknown FHE type: ${typeName}`);
    }
    if (!isValidCost(cost)) {
      throw new Error(`Invalid ${fieldName} price for operator: ${operatorName}, FHE type: ${typeName}`);
    }
  }
}

/** `true` for an `NBucketedCost`: `le10` is required, the other buckets are optional. */
function isNBucketedCost(cost: unknown): boolean {
  if (!isRecord(cost) || typeof cost.le10 !== 'number') {
    return false;
  }
  const buckets = ['le10', 'le30', 'le60', 'le100', 'le128'];
  return Object.entries(cost).every(([bucket, value]) => buckets.includes(bucket) && isOptionalNumber(value));
}

/**
 * Validates the operators prices (`PriceData`, loaded from `operatorsPrices.json`).
 *
 * @param prices - The operators prices to validate.
 * @throws Will throw an error if an operator is missing or unknown, or if an entry has invalid properties.
 */
function validateOperatorsPrices(prices: PriceData): void {
  // Runtime check: the data comes from JSON, so its static type is not trusted here.
  const pricesValue: unknown = prices;
  if (!isRecord(pricesValue)) {
    throw new Error('Operators prices is not defined or invalid');
  }

  for (const operatorName of FHE_OPERATOR_NAMES) {
    if (!(operatorName in pricesValue)) {
      throw new Error(`Missing prices for operator: ${operatorName}`);
    }
  }

  for (const [operatorName, entry] of Object.entries(pricesValue)) {
    if (!(FHE_OPERATOR_NAMES as readonly string[]).includes(operatorName)) {
      throw new Error(`Unknown operator in prices: ${operatorName}`);
    }
    if (!isRecord(entry)) {
      throw new Error(`Invalid prices for operator: ${operatorName}`);
    }
    const fields = ['supportScalar', 'numberInputs', 'scalar', 'nonScalar', 'types', 'nBucketed'];
    const unknownField = Object.keys(entry).find((field) => !fields.includes(field));
    if (unknownField !== undefined) {
      throw new Error(`Unknown field in prices for operator: ${operatorName}. Field: ${unknownField}`);
    }
    if (typeof entry.supportScalar !== 'boolean') {
      throw new Error(`Invalid supportScalar value for operator: ${operatorName}`);
    }
    if (typeof entry.numberInputs !== 'number') {
      throw new Error(`Invalid numberInputs value for operator: ${operatorName}`);
    }
    const isNumber = (cost: unknown): boolean => typeof cost === 'number';
    validateCostMap(entry.scalar, isNumber, operatorName, 'scalar');
    validateCostMap(entry.nonScalar, isNumber, operatorName, 'nonScalar');
    validateCostMap(entry.types, isNumber, operatorName, 'types');
    validateCostMap(entry.nBucketed, isNBucketedCost, operatorName, 'nBucketed');
  }
}

export function validate(): void {
  // Validate the FHE types
  validateFHETypeInfos(ALL_FHE_TYPE_INFOS);
  // Validate the operators
  validateOperators(ALL_OPERATORS);
  // Validate the operators prices
  validateOperatorsPrices(ALL_OPERATORS_PRICES);
}
