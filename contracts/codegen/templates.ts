import { assert } from 'console';

import { AdjustedFheType, AliasFheType, FheType, Operator, OperatorArguments, ReturnType } from './common';
import { getUint } from './utils';

/**
 * Generates Solidity type aliases from an array of FHE types.
 *
 * This function filters the provided FHE types to include only those that are supported for
 * binary or unary operations. It then maps these types to Solidity type aliases, where each
 * type is represented as a `bytes32`. Additionally, it includes a predefined alias for
 * `einput`, which is represented as `bytes32`.
 *
 * @param fheTypes - An array of FHE types to generate Solidity type aliases from.
 * @returns A string containing the Solidity type aliases, each on a new line.
 */
export function createSolidityTypeAliasesFromFheTypes(fheTypes: FheType[]): string {
  let res = fheTypes
    .filter((fheType: FheType) => fheType.supportedOperators.length > 0)
    .map((fheType: FheType) => `type e${fheType.type.toLowerCase()} is bytes32;`);

  res = res.concat(
    fheTypes
      .map((fheType: FheType) =>
        (fheType.aliases?.filter((fheTypeAlias: AliasFheType) => fheTypeAlias.supportedOperators.length > 0) ?? []).map(
          (fheTypeAlias: AliasFheType) => `type e${fheTypeAlias.type.toLowerCase()} is bytes32;`,
        ),
      )
      .flat(),
  );

  return res.concat(['type einput is bytes32;']).join('\n');
}

/**
 * Generates a Solidity enum definition from an array of FheType objects.
 *
 * @param {FheType[]} fheTypes - An array of FheType objects to be converted into a Solidity enum.
 * @returns {string} A string representing the Solidity enum definition.
 */
export function createSolidityEnumFromFheTypes(fheTypes: FheType[]): string {
  return `enum FheType {
    ${fheTypes
      .map((fheType: FheType, index: number) => `${fheType.type}${index < fheTypes.length - 1 ? ',' : ''}`)
      .join('\n')}
}`;
}

export function generateSolidityFheType(fheTypes: FheType[]): string {
  return `
    // SPDX-License-Identifier: BSD-3-Clause-Clear
    pragma solidity ^0.8.24;

    ${createSolidityEnumFromFheTypes(fheTypes)}
`;
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
function generateAdjustedFheTypeArray(fheTypes: FheType[]): AdjustedFheType[] {
  let adjustedFheTypes: AdjustedFheType[] = [];

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

/**
 * Generates the implementation of a binary operator function for Impl.sol.
 *
 * @param op - The operator for which the implementation is generated.
 * @returns The string representation of the binary operator function.
 */
function handleSolidityBinaryOperatorForImpl(op: Operator): string {
  const scalarArg = op.hasScalar && op.hasEncrypted ? ', bool scalar' : '';
  const scalarByte = op.hasScalar ? '0x01' : '0x00';
  const scalarSection =
    op.hasScalar && op.hasEncrypted
      ? `bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }`
      : `bytes1 scalarByte = ${scalarByte};`;
  return (
    `
    /**
     * @dev Returns the HTTPZ config.
     */
    function ${op.name}(bytes32 lhs, bytes32 rhs${scalarArg}) internal returns (bytes32 result) {
        ${scalarSection}
        HTTPZConfigStruct storage $ = getHTTPZConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).${op.fheLibName}(lhs, rhs, scalarByte);
    }` + '\n'
  );
}

/**
 * Generates the Solidity implementation (Impl.sol) library for FHE operations.
 *
 * @param operators - An array of Operator objects representing the supported operations.
 * @returns A string containing the Solidity implementation library code.
 */
export function generateSolidityImplLib(operators: Operator[]): string {
  const res: string[] = [];

  res.push(`
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "../contracts/FheType.sol";

${generateImplCoprocessorInterface(operators)}

${generateACLInterface()}

${generateInputVerifierInterface()}

/**
 * @title   Impl
 * @notice  This library is the core implementation for computing FHE operations (e.g. add, sub, xor).
 */
library Impl {
  /// keccak256(abi.encode(uint256(keccak256("httpz.storage.HTTPZConfig")) - 1)) & ~bytes32(uint256(0xff))
  bytes32 private constant HTTPZConfigLocation = 0x15b1d18ad3df4183245a6a11b17d9fa31dc4c35ffbf591bdfd0f9704a799c300;

  /**
   * @dev Returns the HTTPZ config.
   */
  function getHTTPZConfig() internal pure returns (HTTPZConfigStruct storage $) {
      assembly {
          $.slot := HTTPZConfigLocation
      }
  }

  /**
   * @notice            Sets the coprocessor addresses.
   * @param httpzConfig HTTPZ config struct that contains contract addresses.
  */
  function setCoprocessor(HTTPZConfigStruct memory httpzConfig) internal {
      HTTPZConfigStruct storage $ = getHTTPZConfig();
      $.ACLAddress = httpzConfig.ACLAddress;
      $.TFHEExecutorAddress = httpzConfig.TFHEExecutorAddress;
      $.KMSVerifierAddress = httpzConfig.KMSVerifierAddress;
      $.InputVerifierAddress = httpzConfig.InputVerifierAddress;
  }
`);

  operators.forEach((op) => {
    switch (op.arguments) {
      case OperatorArguments.Binary:
        res.push(handleSolidityBinaryOperatorForImpl(op));
        break;
      case OperatorArguments.Unary:
        res.push(handleUnaryOperatorForImpl(op));
        break;
    }
  });

  res.push(generateCustomMethodsForImpl());

  res.push('}\n');

  return res.join('');
}

function generateImplCoprocessorInterface(operators: Operator[]): string {
  const res: string[] = [];

  res.push(`
    /**
     * @title   HTTPZConfigStruct
     * @notice  This struct contains all addresses of core contracts, which are needed in a typical dApp.
     */
    struct HTTPZConfigStruct {
        address ACLAddress;
        address TFHEExecutorAddress;
        address KMSVerifierAddress;
        address InputVerifierAddress;
    }

    /**
    * @title   ITFHEExecutor
    * @notice  This interface contains all functions to conduct FHE operations.
    */
    interface ITFHEExecutor {`);
  operators.forEach((op) => {
    const tail = 'external returns (bytes32 result);';
    let functionArguments: string;
    switch (op.arguments) {
      case OperatorArguments.Binary:
        functionArguments = '(bytes32 lhs, bytes32 rhs, bytes1 scalarByte)';
        res.push(`  
          
          /**
           * @notice              Computes ${op.fheLibName} operation.
           * @param lhs           LHS.
           * @param rhs           RHS.
           * @param scalarByte    Scalar byte.
           * @return result       Result.
           */
          function ${op.fheLibName}${functionArguments} ${tail}`);
        break;
      case OperatorArguments.Unary:
        functionArguments = '(bytes32 ct)';
        res.push(`  

           /**
           * @notice              Computes ${op.fheLibName} operation.
           * @param ct            Ct
           * @return result       Result.
           */
          function ${op.fheLibName}${functionArguments} ${tail}`);
        break;
    }
  });

  res.push(coprocessorInterfaceCustomFunctions());

  res.push('}');

  return res.join('');
}

function coprocessorInterfaceCustomFunctions(): string {
  return `
    /**
     * @notice                Verifies the ciphertext.
     * @param inputHandle     Input handle.
     * @param callerAddress   Address of the caller.
     * @param inputProof      Input proof.
     * @param inputType       Input type.
     * @return result         Result.
     */
    function verifyCiphertext(bytes32 inputHandle, address callerAddress, bytes memory inputProof, FheType inputType) external returns (bytes32 result);

    /**
     * @notice          Performs the casting to a target type.
     * @param ct        Value to cast.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function cast(bytes32 ct, FheType toType) external returns (bytes32 result);

     /**
     * @notice          Does trivial encryption.
     * @param ct        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(uint256 ct, FheType toType) external returns (bytes32 result);

    /**
     * @notice          Does trivial encryption.
     * @param ct        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(bytes memory ct, FheType toType) external returns (bytes32 result);

    /**
     * @notice              Computes FHEEq operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheEq(bytes32 lhs, bytes memory rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes FHENe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheNe(bytes32 lhs, bytes memory rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes FHEIfThenElse operation.
     * @param control       Control value.
     * @param ifTrue        If true.
     * @param ifFalse       If false.
     * @return result       Result.
     */
    function fheIfThenElse(bytes32 control, bytes32 ifTrue, bytes32 ifFalse) external returns (bytes32 result);

    /**
     * @notice              Computes FHERand operation.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRand(FheType randType) external returns (bytes32 result);

    /**
     * @notice              Computes FHERandBounded operation.
     * @param upperBound    Upper bound value.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRandBounded(uint256 upperBound, FheType randType) external returns (bytes32 result);
  `;
}

function generateACLInterface(): string {
  return `
  /**
  * @title   IACL.
  * @notice  This interface contains all functions that are used to conduct operations
  *          with the ACL contract.
  */
  interface IACL {
    /**
     * @notice              Allows the use of handle by address account for this transaction.
     * @dev                 The caller must be allowed to use handle for allowTransient() to succeed.
     *                      If not, allowTransient() reverts.
     *                      The Coprocessor contract can always allowTransient(), contrarily to allow().
     * @param ciphertext    Ciphertext.
     * @param account       Address of the account.
     */
    function allowTransient(bytes32 ciphertext, address account) external;

    /**
     * @notice              Allows the use of handle for the address account.
     * @dev                 The caller must be allowed to use handle for allow() to succeed. If not, allow() reverts.
     * @param handle        Handle.
     * @param account       Address of the account.
     */
    function allow(bytes32 handle, address account) external;

    /**
     * @dev This function removes the transient allowances, which could be useful for integration with
     *      Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor.
     */
    function cleanTransientStorage() external;

    /**
     * @notice              Returns whether the account is allowed to use the handle, either due to
     *                      allowTransient() or allow().
     * @param handle        Handle.
     * @param account       Address of the account.
     * @return isAllowed    Whether the account can access the handle.
     */
    function isAllowed(bytes32 handle, address account) external view returns(bool);

    /**
     * @notice              Allows a list of handles to be decrypted.
     * @param handlesList   List of handles.
     */
    function allowForDecryption(bytes32[] memory handlesList) external;
  }
  `;
}

function generateInputVerifierInterface(): string {
  return `
  /** 
   * @title IInputVerifier 
   * @notice This interface contains the only function required from InputVerifier. 
   */
  interface IInputVerifier {

  /**
   * @dev This function removes the transient allowances, which could be useful for integration with
   *      Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor.
   */
  function cleanTransientStorage() external;
  }
  `;
}

export function generateSolidityHTTPZLib(operators: Operator[], fheTypes: FheType[]): string {
  const res: string[] = [];

  res.push(`// SPDX-License-Identifier: BSD-3-Clause-Clear
  pragma solidity ^0.8.24;

  import "./Impl.sol";
  import {FheType} from "../contracts/FheType.sol";

  ${createSolidityTypeAliasesFromFheTypes(fheTypes)}

  /**
   * @title   HTTPZ
   * @notice  This library is the interaction point for all smart contract developers
   *          that interact with the HTTPZ protocol.
   */
  library HTTPZ {

  /// @notice Returned if the input's length is greater than 64 bytes.
  error InputLengthAbove64Bytes(uint256 inputLength);

  /// @notice Returned if the input's length is greater than 128 bytes.
  error InputLengthAbove128Bytes(uint256 inputLength);

  /// @notice Returned if the input's length is greater than 256 bytes.
  error InputLengthAbove256Bytes(uint256 inputLength);

    /**
     * @notice            Sets the coprocessor addresses.
     * @param httpzConfig HTTPZ config struct that contains contract addresses.
    */
    function setCoprocessor(HTTPZConfigStruct memory httpzConfig) internal {
        Impl.setCoprocessor(httpzConfig);
    }
  `);

  // 1. Exclude types that do not support any operators.
  const adjustedFheTypes = generateAdjustedFheTypeArray(fheTypes);

  // 2. Generate isInitialized function for all supported types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => {
    res.push(handleSolidityTFHEIsInitialized(fheType));
  });

  // 3. Handle encrypted operators for two encrypted types
  adjustedFheTypes.forEach((lhsFheType: AdjustedFheType) => {
    adjustedFheTypes.forEach((rhsFheType: AdjustedFheType) => {
      operators.forEach((operator) => {
        res.push(handleSolidityTFHEEncryptedOperatorForTwoEncryptedTypes(lhsFheType, rhsFheType, operator));
      });
    });
  });

  // 4. Handle scalar operators for all supported types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => {
    operators.forEach((operator) => {
      res.push(generateSolidityTFHEScalarOperator(fheType, operator));
    });
  });

  // 5. Handle shift & rotate operators for all supported types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => {
    operators.forEach((operator) => {
      res.push(handleSolidityTFHEShiftOperator(fheType, operator));
    });
  });

  // 6. Handle ternary operator (i.e., select) for all supported types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => res.push(handleSolidityTFHESelect(fheType)));

  // 7. Handle custom casting (1) between euint types and (2) between an euint type and ebool.
  adjustedFheTypes.forEach((outputFheType: AdjustedFheType) => {
    adjustedFheTypes.forEach((inputFheType: AdjustedFheType) => {
      res.push(handleSolidityTFHECustomCastBetweenTwoEuint(inputFheType, outputFheType));
    });
    res.push(handleSolidityTFHECustomCastBetweenEboolAndEuint(outputFheType));
  });

  // 8. Handle unary operators for all supported types.
  adjustedFheTypes.forEach((fheType: AdjustedFheType) =>
    res.push(handleSolidityTFHEUnaryOperators(fheType, operators)),
  );

  // 9. Handle conversion from plaintext and einput to all supported types (e.g., einput --> ebool, bytes memory --> ebytes64, uint32 --> euint32)
  adjustedFheTypes.forEach((fheType: AdjustedFheType) =>
    res.push(handleSolidityTFHEConvertPlaintextAndEinputToRespectiveType(fheType)),
  );

  // 10. Handle rand/randBounded for all supported types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => res.push(handleSolidityTFHERand(fheType)));

  // 11. Add padding to bytes for all ebytes types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => res.push(handleTFHEPadToBytesForEbytes(fheType)));

  // 12. Push ACL Solidity methods
  res.push(generateSolidityACLMethods(adjustedFheTypes));

  res.push('}\n');

  return res.join('');
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
    let implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(${leftExpr}), euint${outputBits}.unwrap(${rightExpr})${scalarFlag})`;

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
  } else if (lhsFheType.type == rhsFheType.type && rhsFheType.type.startsWith('Bytes')) {
    const bytesLength = lhsFheType.bitLength / 8;

    res.push(`
      /**
      * @dev Evaluates ${operator.name}(e${lhsFheType.type.toLowerCase()} a, e${rhsFheType.type.toLowerCase()} b) and returns the result.
      */
      function ${operator.name}(e${lhsFheType.type.toLowerCase()} a, e${rhsFheType.type.toLowerCase()} b) internal returns (ebool) {
          if (!isInitialized(a)) {
              a = asEbytes${bytesLength}(padToBytes${bytesLength}(hex""));
          }
          if (!isInitialized(b)) {
              b = asEbytes${bytesLength}(padToBytes${bytesLength}(hex""));
          }
          return ebool.wrap(Impl.${operator.name}(e${lhsFheType.type.toLowerCase()}.unwrap(a), e${rhsFheType.type.toLowerCase()}.unwrap(b), false));
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
  } else if (fheType.type.startsWith('Bytes')) {
    implExpressionA = `Impl.${operator.name}(e${fheType.type.toLowerCase()}.unwrap(a), b${scalarFlag})`;
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
  } else if (fheType.type.startsWith('Bytes')) {
    implExpressionB = `Impl.${leftOpName}(e${fheType.type.toLowerCase()}.unwrap(b), a${scalarFlag})`;
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
              fheType.type == 'Bool'
                ? 'false'
                : fheType.type.startsWith('Bytes')
                  ? `padToBytes${fheType.bitLength / 8}(hex"")`
                  : fheType.type == 'Address'
                    ? `${clearMatchingType.toLowerCase()}(0)`
                    : 0
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
              fheType.type == 'Bool'
                ? 'false'
                : fheType.type.startsWith('Bytes')
                  ? `padToBytes${fheType.bitLength / 8}(hex"")`
                  : fheType.type == 'Address'
                    ? `${clearMatchingType.toLowerCase()}(0)`
                    : 0
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

function handleSolidityTFHESelect(fheType: AdjustedFheType): string {
  let res = '';

  if (fheType.supportedOperators.includes('select')) {
    res += `
    /**
    * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
    *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
    */
    function select(ebool control, e${fheType.type.toLowerCase()} a, e${fheType.type.toLowerCase()} b) internal returns (e${fheType.type.toLowerCase()}) {
        return e${fheType.type.toLowerCase()}.wrap(Impl.select(ebool.unwrap(control), e${fheType.type.toLowerCase()}.unwrap(a), e${fheType.type.toLowerCase()}.unwrap(b)));
    }`;
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
 * - If the type is `Bytes`, an additional function to convert plaintext bytes to the respective encrypted type.
 * - For other types, a function to convert a plaintext value to the respective encrypted type.
 */
function handleSolidityTFHEConvertPlaintextAndEinputToRespectiveType(fheType: AdjustedFheType): string {
  let result = `
    /** 
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted e${fheType.type.toLowerCase()} integer.
     */
    function asE${fheType.type.toLowerCase()}(einput inputHandle, bytes memory inputProof) internal returns (e${fheType.type.toLowerCase()}) {
        return e${fheType.type.toLowerCase()}.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.${fheType.isAlias ? fheType.aliasType : fheType.type}));
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
  } else if (fheType.type.startsWith('Bytes')) {
    result += `
      /**
        * @dev Convert the plaintext bytes to a e${fheType.type.toLowerCase()} value.
      */
      function asE${fheType.type.toLowerCase()}(${fheType.clearMatchingTypeAlias} value) internal returns (e${fheType.type.toLowerCase()}) {
        return e${fheType.type.toLowerCase()}.wrap(Impl.trivialEncrypt(value, FheType.${fheType.isAlias ? fheType.aliasType : fheType.type}));
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
 * Generates the implementation of a unary operator function.
 *
 * @param op - The operator for which the implementation is generated.
 * @returns The string representation of the unary operator function.
 */
function handleUnaryOperatorForImpl(op: Operator): string {
  return `
    function ${op.name}(bytes32 ct) internal returns (bytes32 result) {
      HTTPZConfigStruct storage $ = getHTTPZConfig();
      result = ITFHEExecutor($.TFHEExecutorAddress).${op.fheLibName}(ct);
    }
  `;
}

/**
 * Generates Solidity ACL (Access Control List) methods for the provided FHE types.
 *
 * @param {AdjustedFheType[]} fheTypes - An array of FHE types for which to generate the ACL methods.
 * @returns {string} A string containing the generated Solidity code for the ACL methods.
 */
function generateSolidityACLMethods(fheTypes: AdjustedFheType[]): string {
  const res: string[] = [];

  res.push(
    `
    /**
     * @dev This function cleans the transient storage for the ACL (accounts) and the InputVerifier
     *      (input proofs).
     *      This could be useful for integration with Account Abstraction when bundling several 
     *      UserOps calling the TFHEExecutor.
     */
    function cleanTransientStorage() internal {
      Impl.cleanTransientStorageACL();
      Impl.cleanTransientStorageInputVerifier();
    }
  `,
  );

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

    `),
  );

  return res.join('');
}

function handleTFHEPadToBytesForEbytes(fheType: AdjustedFheType): string {
  if (!fheType.type.startsWith('Bytes')) {
    return '';
  }

  const bytesLength = fheType.bitLength / 8;

  return `
        /**
         * @dev Left-pad a bytes array with zeros such that it becomes of length ${bytesLength}.
         */
        function padToBytes${bytesLength}(bytes memory input) internal pure returns (bytes memory) {
          uint256 inputLength = input.length;
    
          if (inputLength > ${bytesLength}) {
              revert InputLengthAbove${bytesLength}Bytes(inputLength);
          }
    
          bytes memory result = new bytes(${bytesLength});
          uint256 paddingLength = ${bytesLength} - inputLength;
    
          for (uint256 i = 0; i < paddingLength; i++) {
              result[i] = 0;
          }

          for (uint256 i = 0; i < inputLength; i++) {
              result[paddingLength + i] = input[i];
          }
          return result;
        }
      `;
}

function generateCustomMethodsForImpl(): string {
  return `
    /**
    * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
    *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
    */
    function select(bytes32 control, bytes32 ifTrue, bytes32 ifFalse) internal returns (bytes32 result) {
        HTTPZConfigStruct storage $ = getHTTPZConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheIfThenElse(control, ifTrue, ifFalse);
    }

    /**
     * @notice              Verifies the ciphertext (TFHEExecutor) and allows transient (ACL).
     * @param inputHandle   Input handle.
     * @param inputProof    Input proof.
     * @param toType        Input type.
     * @return result       Result.
     */
    function verify(
        bytes32 inputHandle,
        bytes memory inputProof,
        FheType toType
    ) internal returns (bytes32 result) {
        HTTPZConfigStruct storage $ = getHTTPZConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).verifyCiphertext(inputHandle, msg.sender, inputProof, toType);
        IACL($.ACLAddress).allowTransient(result, msg.sender);
    }

    /**
     * @notice            Performs the casting to a target type.
     * @param ciphertext  Ciphertext to cast.
     * @param toType      Target type.
     * @return result     Result value of the target type.
     */
    function cast(
        bytes32 ciphertext,
        FheType toType
    ) internal returns (bytes32 result) {
        HTTPZConfigStruct storage $ = getHTTPZConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).cast(ciphertext, toType);
    }

    /**
     * @notice          Does trivial encryption.
     * @param value     Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(
        uint256 value,
        FheType toType
    ) internal returns (bytes32 result) {
      HTTPZConfigStruct storage $ = getHTTPZConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).trivialEncrypt(value, toType);
    }

    /**
     * @notice          Does trivial encryption.
     * @param value     Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(
      bytes memory value,
      FheType toType
    ) internal returns (bytes32 result) {
      HTTPZConfigStruct storage $ = getHTTPZConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).trivialEncrypt(value, toType);
    }

    /**
     * @notice              Computes FHEEq operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalar        Scalar byte.
     * @return result       Result.
     */
    function eq(bytes32 lhs, bytes memory rhs, bool scalar) internal returns (bytes32 result) {
      bytes1 scalarByte;
      if (scalar) {
          scalarByte = 0x01;
      } else {
          scalarByte = 0x00;
      }
      HTTPZConfigStruct storage $ = getHTTPZConfig();
      result = ITFHEExecutor($.TFHEExecutorAddress).fheEq(lhs, rhs, scalarByte);
  }

  /**
   * @notice              Computes FHENe operation.
   * @param lhs           LHS.
   * @param rhs           RHS.
   * @param scalar        Scalar byte.
   * @return result       Result.
  */
  function ne(bytes32 lhs, bytes memory rhs, bool scalar) internal returns (bytes32 result) {
      bytes1 scalarByte;
      if (scalar) {
          scalarByte = 0x01;
      } else {
          scalarByte = 0x00;
      }
      HTTPZConfigStruct storage $ = getHTTPZConfig();
      result = ITFHEExecutor($.TFHEExecutorAddress).fheNe(lhs, rhs, scalarByte);
  }

    function rand(FheType randType) internal returns(bytes32 result) {
      HTTPZConfigStruct storage $ = getHTTPZConfig();
      result = ITFHEExecutor($.TFHEExecutorAddress).fheRand(randType);
    }

    function randBounded(uint256 upperBound, FheType randType) internal returns(bytes32 result) {
      HTTPZConfigStruct storage $ = getHTTPZConfig();
      result = ITFHEExecutor($.TFHEExecutorAddress).fheRandBounded(upperBound, randType);
    }

    /**
     * @notice              Allows the use of handle by address account for this transaction.
     * @dev                 The caller must be allowed to use handle for allowTransient() to succeed.
     *                      If not, allowTransient() reverts.
     *                      The Coprocessor contract can always allowTransient(), contrarily to allow().
     * @param handle        Handle.
     * @param account       Address of the account.
     */
    function allowTransient(bytes32 handle, address account) internal {
      HTTPZConfigStruct storage $ = getHTTPZConfig();
      IACL($.ACLAddress).allowTransient(handle, account);
    }


    /**
     * @notice              Allows the use of handle for the address account.
     * @dev                 The caller must be allowed to use handle for allow() to succeed. If not, allow() reverts.
     * @param handle        Handle.
     * @param account       Address of the account.
     */
    function allow(bytes32 handle, address account) internal {
      HTTPZConfigStruct storage $ = getHTTPZConfig();
      IACL($.ACLAddress).allow(handle, account);
    }

    /**
     * @dev This function removes the transient allowances in the ACL, which could be useful for integration
     *      with Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor.
     */
    function cleanTransientStorageACL() internal {
      HTTPZConfigStruct storage $ = getHTTPZConfig();
      IACL($.ACLAddress).cleanTransientStorage();
    }


    /**
     * @dev This function removes the transient proofs in the InputVerifier, which could be useful for integration 
     *      with Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor.
     */
    function cleanTransientStorageInputVerifier() internal {
      HTTPZConfigStruct storage $ = getHTTPZConfig();
      IInputVerifier($.InputVerifierAddress).cleanTransientStorage();
    }


    /**
     * @notice              Returns whether the account is allowed to use the handle, either due to
     *                      allowTransient() or allow().
     * @param handle        Handle.
     * @param account       Address of the account.
     * @return isAllowed    Whether the account can access the handle.
     */
    function isAllowed(bytes32 handle, address account) internal view returns (bool) {
      HTTPZConfigStruct storage $ = getHTTPZConfig();
      return IACL($.ACLAddress).isAllowed(handle, account);
    }
    `;
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
