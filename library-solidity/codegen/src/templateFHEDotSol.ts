import { assert } from 'console';
import { readFileSync } from 'fs';

import type { AdjustedFheType, FheTypeInfo, Operator } from './common';
import { OperatorArguments, ReturnType } from './common';
import { resolveTemplatePath } from './paths';
import { getUint, removeTemplateComments } from './utils';

export function generateSolidityFHELib(
  operators: Operator[],
  fheTypes: FheTypeInfo[],
  fheTypeDotSol: string,
  implDotSol: string,
): string {
  // Placeholders:
  // =============
  // $${ImplDotSol}$$
  // $${FheTypeDotSol}$$
  // $${FHEOperators}$$
  // $${ACLFunctions}$$
  // $${FHEtoBytes32}$$
  const file = resolveTemplatePath('FHE.sol-template');
  const template = readFileSync(file, 'utf8');

  let code = removeTemplateComments(template);

  code = code.replace('$${ImplDotSol}$$', implDotSol);
  code = code.replace('$${FheTypeDotSol}$$', fheTypeDotSol);

  // Exclude types that do not support any operators.
  const adjustedFheTypes = generateAdjustedFheTypeArray(fheTypes);

  code = code.replace('$${FHEOperators}$$', generateFHEOperators(operators, adjustedFheTypes));
  code = code.replace('$${ACLFunctions}$$', generateSolidityACLMethods(adjustedFheTypes));
  code = code.replace('$${FHEtoBytes32}$$', generateToBytes32(adjustedFheTypes));

  return code;
}

function generateFHEOperators(operators: Operator[], adjustedFheTypes: AdjustedFheType[]): string {
  const res: string[] = [];

  // 1. Generate isInitialized function for all supported types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => {
    res.push(handleSolidityTFHEIsInitialized(fheType));
  });

  // 2. Handle encrypted operators for two encrypted types
  adjustedFheTypes.forEach((lhsFheType: AdjustedFheType) => {
    adjustedFheTypes.forEach((rhsFheType: AdjustedFheType) => {
      operators.forEach((operator) => {
        res.push(handleSolidityTFHEEncryptedOperatorForTwoEncryptedTypes(lhsFheType, rhsFheType, operator));
      });
    });
  });

  // 3. Handle scalar operators for all supported types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => {
    operators.forEach((operator) => {
      res.push(generateSolidityTFHEScalarOperator(fheType, operator));
    });
  });

  // 4. Handle shift & rotate operators for all supported types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => {
    operators.forEach((operator) => {
      res.push(handleSolidityTFHEShiftOperator(fheType, operator));
    });
  });

  // 5. Handle ternary operator (i.e., select) for all supported types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => res.push(handleSolidityTFHESelect(fheType)));

  // 6. Handle custom casting (1) between euint types and (2) between an euint type and ebool.
  adjustedFheTypes.forEach((outputFheType: AdjustedFheType) => {
    adjustedFheTypes.forEach((inputFheType: AdjustedFheType) => {
      res.push(handleSolidityTFHECustomCastBetweenTwoEuint(inputFheType, outputFheType));
    });
    res.push(handleSolidityTFHECustomCastBetweenEboolAndEuint(outputFheType));
  });

  // 7. Handle unary operators for all supported types.
  adjustedFheTypes.forEach((fheType: AdjustedFheType) =>
    res.push(handleSolidityTFHEUnaryOperators(fheType, operators)),
  );

  // 8. Handle conversion from plaintext and externalEXXX to all supported types (e.g., externalEbool --> ebool, uint32 --> euint32)
  adjustedFheTypes.forEach((fheType: AdjustedFheType) =>
    res.push(handleSolidityTFHEConvertPlaintextAndEinputToRespectiveType(fheType)),
  );

  // 9. Handle rand/randBounded for all supported types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => res.push(handleSolidityTFHERand(fheType)));

  return res.join('');
}

/**
 * Generates an array of adjusted FHE (Fully Homomorphic Encryption) types based on the provided FHE types.
 *
 * This function processes an array of `FheType` objects and creates a new array of `AdjustedFheType` objects.
 * It includes the original FHE types that have supported operators and also processes their aliases, if any,
 * to include them in the adjusted array with additional alias-specific properties.
 *
 * @param fheTypes - An array of `FheType` objects to be adjusted.
 *
 * @returns An array of `AdjustedFheType` objects containing the adjusted FHE types and their aliases.
 *
 * @remarks
 * - Only FHE types with supported operators are included in the result.
 * - Aliases are processed separately and marked with the `isAlias` property.
 * - The `aliasType` property indicates the original type for an alias.
 * - The `clearMatchingTypeAlias` property is included for aliases to reference the original clear matching type.
 */
function generateAdjustedFheTypeArray(fheTypes: FheTypeInfo[]): AdjustedFheType[] {
  const adjustedFheTypes: AdjustedFheType[] = [];

  for (let i = 0; i < fheTypes.length; i++) {
    const fheType = fheTypes[i];

    if (fheType.supportedOperators.length > 0) {
      adjustedFheTypes.push({
        type: fheType.type,
        bitLength: fheType.bitLength,
        supportedOperators: fheType.supportedOperators,
        clearMatchingType: fheType.clearMatchingType,
        value: fheType.value,
      });
    }

    if (fheType.aliases !== undefined && fheType.aliases.length > 0) {
      for (let i = 0; i < fheType.aliases.length; i++) {
        if (fheType.aliases[i].supportedOperators.length > 0) {
          adjustedFheTypes.push({
            type: fheType.aliases[i].type,
            bitLength: fheType.bitLength,
            supportedOperators: fheType.aliases[i].supportedOperators,
            clearMatchingType: fheType.aliases[i].clearMatchingType,
            value: fheType.value,
            isAlias: true,
            aliasType: fheType.type,
            clearMatchingTypeAlias: fheType.clearMatchingType,
          });
        }
      }
    }
  }

  return adjustedFheTypes;
}

function handleSolidityTFHEEncryptedOperatorForTwoEncryptedTypes(
  lhsFheType: AdjustedFheType,
  rhsFheType: AdjustedFheType,
  operator: Operator,
): string {
  const res: string[] = [];

  if (operator.shiftOperator || operator.rotateOperator) {
    return '';
  }

  if (!operator.hasEncrypted || operator.arguments != OperatorArguments.Binary) {
    return '';
  }

  if (
    !lhsFheType.supportedOperators.includes(operator.name) ||
    !rhsFheType.supportedOperators.includes(operator.name)
  ) {
    return '';
  }

  if (lhsFheType.type.startsWith('Uint') && rhsFheType.type.startsWith('Uint')) {
    // Determine the maximum number of bits between lhsBits and rhsBits
    const outputBits = Math.max(lhsFheType.bitLength, rhsFheType.bitLength);
    const castLeftToRight = lhsFheType.bitLength < rhsFheType.bitLength;
    const castRightToLeft = lhsFheType.bitLength > rhsFheType.bitLength;

    const returnType =
      operator.returnType == ReturnType.Euint
        ? `euint${outputBits}`
        : operator.returnType == ReturnType.Ebool
          ? `ebool`
          : assert(false, 'Unknown return type');

    const scalarFlag = operator.hasEncrypted && operator.hasScalar ? ', false' : '';
    const leftExpr = castLeftToRight ? `asEuint${outputBits}(a)` : 'a';
    const rightExpr = castRightToLeft ? `asEuint${outputBits}(b)` : 'b';
    const implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(${leftExpr}), euint${outputBits}.unwrap(${rightExpr})${scalarFlag})`;

    res.push(`
    /**
    * @dev Evaluates ${operator.name}(e${lhsFheType.type.toLowerCase()} a, e${rhsFheType.type.toLowerCase()} b)  and returns the result.
    */
    function ${operator.name}(e${lhsFheType.type.toLowerCase()} a, e${rhsFheType.type.toLowerCase()} b) internal returns (${returnType}) {
        if (!isInitialized(a)) {
            a = asE${lhsFheType.type.toLowerCase()}(0);
        }
        if (!isInitialized(b)) {
            b = asE${rhsFheType.type.toLowerCase()}(0);
        }
        return ${returnType}.wrap(${implExpression});
    }
`);
  } else if (lhsFheType.type === 'Bool' && rhsFheType.type === 'Bool') {
    res.push(`
    /**
    * @dev Evaluates ${operator.name}(ebool a, ebool b) and returns the result.
    */
    function ${operator.name}(ebool a, ebool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.${operator.name}(ebool.unwrap(a), ebool.unwrap(b), false));
    }
`);
  } else if (lhsFheType.type.startsWith('Address') && rhsFheType.type.startsWith('Address')) {
    res.push(`
      /**
      * @dev Evaluates ${operator.name}(eaddress a, eaddress b) and returns the result.
      */
      function ${operator.name}(eaddress a, eaddress b) internal returns (ebool) {
          if (!isInitialized(a)) {
              a = asEaddress(address(0));
          }
          if (!isInitialized(b)) {
              b = asEaddress(address(0));
          }
          return ebool.wrap(Impl.${operator.name}(eaddress.unwrap(a), eaddress.unwrap(b), false));
      }
  `);
  } else if (lhsFheType.type.startsWith('Int') && rhsFheType.type.startsWith('Int')) {
    throw new Error('Int types are not supported!');
  }

  return res.join('');
}

function generateSolidityTFHEScalarOperator(fheType: AdjustedFheType, operator: Operator): string {
  const res: string[] = [];

  if (operator.shiftOperator || operator.rotateOperator) {
    return '';
  }

  if (operator.arguments != OperatorArguments.Binary) {
    return '';
  }

  if (!operator.hasScalar) {
    return '';
  }

  if (!fheType.supportedOperators.includes(operator.name)) {
    return '';
  }

  const returnType =
    operator.returnType == ReturnType.Euint
      ? `e${fheType.type.toLowerCase()} `
      : operator.returnType == ReturnType.Ebool
        ? `ebool`
        : assert(false, 'Unknown return type');

  let scalarFlag = operator.hasEncrypted && operator.hasScalar ? ', true' : '';
  const leftOpName = operator.leftScalarInvertOp ?? operator.name;

  let implExpressionA;

  if (fheType.type == 'Bool') {
    implExpressionA = `Impl.${operator.name}(e${fheType.type.toLowerCase()}.unwrap(a), bytes32(uint256(b?1:0))${scalarFlag})`;
  } else if (fheType.type.startsWith('Int')) {
    throw new Error('Int types are not supported!');
  } else {
    implExpressionA = `Impl.${operator.name}(e${fheType.type.toLowerCase()}.unwrap(a), bytes32(uint256(${
      fheType.isAlias && fheType.clearMatchingTypeAlias !== undefined
        ? `${fheType.clearMatchingTypeAlias.toLowerCase()}(b)`
        : 'b'
    }))${scalarFlag})`;
  }

  let implExpressionB;

  if (fheType.type == 'Bool') {
    implExpressionB = `Impl.${leftOpName}(e${fheType.type.toLowerCase()}.unwrap(b), bytes32(uint256(a?1:0))${scalarFlag})`;
  } else if (fheType.type.startsWith('Int')) {
    throw new Error('Int types are not supported yet!');
  } else {
    implExpressionB = `Impl.${leftOpName}(e${fheType.type.toLowerCase()}.unwrap(b), bytes32(uint256(${
      fheType.isAlias && fheType.clearMatchingTypeAlias !== undefined
        ? `${fheType.clearMatchingTypeAlias.toLowerCase()}(a)`
        : 'a'
    }))${scalarFlag})`;
  }

  let maybeEncryptLeft = '';

  if (operator.leftScalarEncrypt) {
    // workaround until tfhe-rs left scalar support:
    // do the trivial encryption and preserve order of operations
    scalarFlag = ', false';
    maybeEncryptLeft = `e${fheType.type.toLowerCase()} aEnc = asE${fheType.type.toLowerCase()}(a);`;
    implExpressionB = `Impl.${leftOpName}(e${fheType.type.toLowerCase()}.unwrap(aEnc), e${fheType.type.toLowerCase()}.unwrap(b)${scalarFlag})`;
  }

  const clearMatchingType =
    fheType.type === 'Address'
      ? fheType.clearMatchingType
      : fheType.isAlias && fheType.clearMatchingTypeAlias !== undefined
        ? fheType.clearMatchingTypeAlias
        : fheType.clearMatchingType;

  // rhs scalar
  res.push(`
    
    /**
    * @dev Evaluates ${operator.name}(e${fheType.type.toLowerCase()} a, ${clearMatchingType.toLowerCase()} b) and returns the result.
    */
    function ${operator.name}(e${fheType.type.toLowerCase()} a, ${clearMatchingType.toLowerCase()} b) internal returns (${returnType}) {
        if (!isInitialized(a)) {
            a = asE${fheType.type.toLowerCase()}(${
              fheType.type == 'Bool' ? 'false' : fheType.type == 'Address' ? `${clearMatchingType.toLowerCase()}(0)` : 0
            });
        }
        return ${returnType}.wrap(${implExpressionA});
    }
`);

  // lhs scalar
  if (!operator.leftScalarDisable) {
    res.push(`

    /**
    * @dev Evaluates ${operator.name}(${clearMatchingType.toLowerCase()} a, e${fheType.type.toLowerCase()} b) and returns the result.
    */
    function ${operator.name}(${clearMatchingType.toLowerCase()} a, e${fheType.type.toLowerCase()} b) internal returns (${returnType}) {
        ${maybeEncryptLeft}
        if (!isInitialized(b)) {
            b = asE${fheType.type.toLowerCase()}(${
              fheType.type == 'Bool' ? 'false' : fheType.type == 'Address' ? `${clearMatchingType.toLowerCase()}(0)` : 0
            });
        }
        return ${returnType}.wrap(${implExpressionB});
    }
        `);
  }

  return res.join('');
}

function handleSolidityTFHEIsInitialized(fheType: AdjustedFheType): string {
  return `
      /**
      * @dev Returns true if the encrypted integer is initialized and false otherwise.
      */
      function isInitialized(e${fheType.type.toLowerCase()} v) internal pure returns (bool) {
          return e${fheType.type.toLowerCase()}.unwrap(v) != 0;
      }
    `;
}

function handleSolidityTFHEShiftOperator(fheType: AdjustedFheType, operator: Operator): string {
  const res: string[] = [];

  if (!operator.shiftOperator && !operator.rotateOperator) {
    return res.join();
  }

  if (fheType.supportedOperators.includes(operator.name)) {
    const lhsBits = fheType.bitLength;
    const rhsBits = 8;
    const castRightToLeft = lhsBits > rhsBits;

    let scalarFlag = ', false';

    const leftExpr = 'a';
    const rightExpr = castRightToLeft ? `asE${fheType.type.toLowerCase()}(b)` : 'b';
    let implExpression: string = `Impl.${operator.name}(e${fheType.type.toLowerCase()}.unwrap(${leftExpr}), e${fheType.type.toLowerCase()}.unwrap(${rightExpr})${scalarFlag})`;

    res.push(`
    /** 
     * @dev Evaluates ${operator.name}(euint${lhsBits} a, euint${rhsBits} b) and returns the result.
     */
    function ${operator.name}(euint${lhsBits} a, euint${rhsBits} b) internal returns (e${fheType.type.toLowerCase()}) {
        if (!isInitialized(a)) {
            a = asEuint${lhsBits}(0);
        }
        if (!isInitialized(b)) {
            b = asEuint${rhsBits}(0);
        }
        return e${fheType.type.toLowerCase()}.wrap(${implExpression});
    }
`);

    // Code and test for shift(euint{inputBits},uint8}
    scalarFlag = ', true';
    implExpression = `Impl.${operator.name}(e${fheType.type.toLowerCase()}.unwrap(a), bytes32(uint256(b))${scalarFlag})`;

    res.push(`
    /** 
     * @dev Evaluates ${operator.name}(e${fheType.type.toLowerCase()} a, ${getUint(rhsBits)}) and returns the result.
     */
    function ${operator.name}(e${fheType.type.toLowerCase()} a, ${getUint(rhsBits)} b) internal returns (e${fheType.type.toLowerCase()}) {
        if (!isInitialized(a)) {
            a = asE${fheType.type.toLowerCase()}(0);
        }
        return e${fheType.type.toLowerCase()}.wrap(${implExpression});
    }
  `);
  }
  return res.join('');
}

function checkInitialized(varname: string, type: string) {
  if (type === 'Bool') {
    return `if (!isInitialized(${varname})) { ${varname} = asEbool(false); }`;
  } else if (type.startsWith('Uint') || type.startsWith('Uint')) {
    return `if (!isInitialized(${varname})) { ${varname} = asE${type.toLowerCase()}(0); }`;
  } else if (type.startsWith('Address')) {
    return `if (!isInitialized(${varname})) { ${varname} = asEaddress(address(0)); }`;
  } else {
    throw new Error(`Unsupported type ${type}`);
  }
}

function handleSolidityTFHESelect(fheType: AdjustedFheType): string {
  let res = '';

  if (fheType.supportedOperators.includes('select')) {
    res += `
    /**
    * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
    *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
    */
    function select(ebool control, e${fheType.type.toLowerCase()} a, e${fheType.type.toLowerCase()} b) internal returns (e${fheType.type.toLowerCase()}) {
        ${checkInitialized('control', 'Bool')}
        ${checkInitialized('a', fheType.type)}
        ${checkInitialized('b', fheType.type)}
        return e${fheType.type.toLowerCase()}.wrap(Impl.select(ebool.unwrap(control), e${fheType.type.toLowerCase()}.unwrap(a), e${fheType.type.toLowerCase()}.unwrap(b)));
    }
    `;
  }

  return res;
}

function handleSolidityTFHECustomCastBetweenTwoEuint(
  inputFheType: AdjustedFheType,
  outputFheType: AdjustedFheType,
): string {
  if (
    inputFheType.type == outputFheType.type ||
    !inputFheType.type.startsWith('Uint') ||
    !outputFheType.type.startsWith('Uint')
  ) {
    return '';
  }

  return `
  /**
    * @dev Casts an encrypted integer from 'e${inputFheType.type.toLowerCase()}' to 'e${outputFheType.type.toLowerCase()}'.
    */
    function asE${outputFheType.type.toLowerCase()}(e${inputFheType.type.toLowerCase()} value) internal returns (e${outputFheType.type.toLowerCase()}) {
        return e${outputFheType.type.toLowerCase()}.wrap(Impl.cast(e${inputFheType.type.toLowerCase()}.unwrap(value), FheType.${outputFheType.type}));
    }
    `;
}

function handleSolidityTFHECustomCastBetweenEboolAndEuint(fheType: AdjustedFheType): string {
  const res: string[] = [];

  if (fheType.type.startsWith('Uint')) {
    res.push(`
    /**
    /** 
     * @dev Converts an 'ebool' to an 'e${fheType.type.toLowerCase()}'.
     */
    function asE${fheType.type.toLowerCase()}(ebool b) internal returns (e${fheType.type.toLowerCase()}) {
        return e${fheType.type.toLowerCase()}.wrap(Impl.cast(ebool.unwrap(b), FheType.${fheType.type}));
    }
    `);

    if (fheType.supportedOperators.includes('ne')) {
      res.push(`
      /**
      * @dev Casts an encrypted integer from 'e${fheType.type.toLowerCase()}' to 'ebool'.
      */
      function asEbool(e${fheType.type.toLowerCase()} value) internal returns (ebool) {
          return ne(value, 0);
      }
      `);
    }
  }

  return res.join('');
}

function handleSolidityTFHEUnaryOperators(fheType: AdjustedFheType, operators: Operator[]): string {
  const res: string[] = [];

  operators.forEach((op) => {
    if (op.arguments == OperatorArguments.Unary && fheType.supportedOperators.includes(op.name)) {
      res.push(`
          /**
           * @dev Evaluates ${op.name}(e${fheType.type.toLowerCase()} value) and returns the result.
           */
        function ${op.name}(e${fheType.type.toLowerCase()} value) internal returns (e${fheType.type.toLowerCase()}) {
            ${checkInitialized('value', fheType.type)}
            return e${fheType.type.toLowerCase()}.wrap(Impl.${op.name}(e${fheType.type.toLowerCase()}.unwrap(value)));
        }
      `);
    }
  });

  return res.join('\n');
}

/**
 * Generates Solidity functions to convert plaintext and encrypted input handles to their respective encrypted types.
 *
 * @param {AdjustedFheType} fheType - The Fully Homomorphic Encryption (FHE) type information.
 * @returns {string} - The Solidity code for the conversion functions.
 *
 * The generated functions include:
 * - A function to convert an `einput` handle and its proof to an encrypted type.
 * - If the type is `Bool`, an additional function to convert a plaintext boolean to an encrypted boolean.
 * - For other types, a function to convert a plaintext value to the respective encrypted type.
 */
function handleSolidityTFHEConvertPlaintextAndEinputToRespectiveType(fheType: AdjustedFheType): string {
  let result = `
    /** 
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted e${fheType.type.toLowerCase()} integer.
     */
    function fromExternal(externalE${fheType.type.toLowerCase()} inputHandle, bytes memory inputProof) internal returns (e${fheType.type.toLowerCase()}) {
        return e${fheType.type.toLowerCase()}.wrap(Impl.verify(externalE${fheType.type.toLowerCase()}.unwrap(inputHandle), inputProof, FheType.${fheType.isAlias ? fheType.aliasType : fheType.type}));
    }

    `;

  /// If boolean, add also the asEbool function that allows casting bool
  if (fheType.type.startsWith('Bool')) {
    result += `
      /** 
     * @dev Converts a plaintext boolean to an encrypted boolean.
     */
      function asEbool(bool value) internal returns (ebool) {
        return ebool.wrap(Impl.trivialEncrypt(value? 1 : 0, FheType.Bool));
    }

    `;
  } else {
    const value =
      fheType.isAlias && fheType.clearMatchingTypeAlias !== undefined
        ? `${fheType.clearMatchingTypeAlias.toLowerCase()}(value)`
        : 'value';

    result += `
    /** 
     * @dev Convert a plaintext value to an encrypted e${fheType.type.toLowerCase()} integer.
    */
    function asE${fheType.type.toLowerCase()}(${fheType.clearMatchingType} value) internal returns (e${fheType.type.toLowerCase()}) {
        return e${fheType.type.toLowerCase()}.wrap(Impl.trivialEncrypt(uint256(${value}), FheType.${fheType.isAlias ? fheType.aliasType : fheType.type}));
    }

    `;
  }
  return result;
}

/**
 * Generates Solidity ACL (Access Control List) methods for the provided FHE types.
 *
 * @param {AdjustedFheType[]} fheTypes - An array of FHE types for which to generate the ACL methods.
 * @returns {string} A string containing the generated Solidity code for the ACL methods.
 */
function generateSolidityACLMethods(fheTypes: AdjustedFheType[]): string {
  const res: string[] = [];

  fheTypes.forEach((fheType: AdjustedFheType) =>
    res.push(`
    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(e${fheType.type.toLowerCase()} value, address account) internal view returns (bool) {
      return Impl.isAllowed(e${fheType.type.toLowerCase()}.unwrap(value), account);
    }

    /**
    * @dev Returns whether the sender is allowed to use the value.
    */
    function isSenderAllowed(e${fheType.type.toLowerCase()} value) internal view returns (bool) {
      return Impl.isAllowed(e${fheType.type.toLowerCase()}.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(e${fheType.type.toLowerCase()} value, address account) internal returns(e${fheType.type.toLowerCase()}) {
      Impl.allow(e${fheType.type.toLowerCase()}.unwrap(value), account);
      return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(e${fheType.type.toLowerCase()} value) internal returns(e${fheType.type.toLowerCase()}) {
      Impl.allow(e${fheType.type.toLowerCase()}.unwrap(value), address(this));
      return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(e${fheType.type.toLowerCase()} value, address account) internal returns(e${fheType.type.toLowerCase()}) {
      Impl.allowTransient(e${fheType.type.toLowerCase()}.unwrap(value), account);
      return value;
    }

    /**
     * @dev Makes the value publicly decryptable.
     */
    function makePubliclyDecryptable(e${fheType.type.toLowerCase()} value) internal returns(e${fheType.type.toLowerCase()}) {
      Impl.makePubliclyDecryptable(e${fheType.type.toLowerCase()}.unwrap(value));
      return value;
    }

    /**
     * @dev Returns whether the the value is publicly decryptable.
     */
    function isPubliclyDecryptable(e${fheType.type.toLowerCase()} value) internal view returns (bool) {
      return Impl.isPubliclyDecryptable(e${fheType.type.toLowerCase()}.unwrap(value));
    }

    `),
  );

  return res.join('');
}

function generateToBytes32(fheTypes: AdjustedFheType[]): string {
  const res: string[] = [];
  fheTypes.forEach((fheType: AdjustedFheType) =>
    res.push(`
    /**
     * @dev Converts handle from its custom type to the underlying bytes32. Used when requesting a decryption.
     */
    function toBytes32(e${fheType.type.toLowerCase()} value) internal pure returns (bytes32 ct) {
      ct = e${fheType.type.toLowerCase()}.unwrap(value);
    }
    `),
  );
  return res.join('');
}

function handleSolidityTFHERand(fheType: AdjustedFheType): string {
  let res = '';

  if (fheType.supportedOperators.includes('rand')) {
    res += `
    /**
    * @dev Generates a random encrypted value.
    */
    function randE${fheType.type.toLowerCase()}() internal returns (e${fheType.type.toLowerCase()}) {
      return e${fheType.type.toLowerCase()}.wrap(Impl.rand(FheType.${fheType.isAlias ? fheType.aliasType : fheType.type}));
    }

    `;
  }

  if (fheType.supportedOperators.includes('randBounded')) {
    res += `
    /**
    * @dev Generates a random encrypted ${fheType.bitLength}-bit unsigned integer in the [0, upperBound) range.
    *      The upperBound must be a power of 2.
    */
    function randE${fheType.type.toLowerCase()}(uint${fheType.bitLength} upperBound) internal returns (e${fheType.type.toLowerCase()}) {
      return e${fheType.type.toLowerCase()}.wrap(Impl.randBounded(upperBound, FheType.${fheType.isAlias ? fheType.aliasType : fheType.type}));
    }

    `;
  }

  return res;
}
