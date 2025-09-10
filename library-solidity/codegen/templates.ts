import { assert } from 'console';

import { AdjustedFheType, FheType, Operator, OperatorArguments, ReturnType } from './common';
import { getUint } from './utils';

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
    function ${op.name}(bytes32 lhs, bytes32 rhs${scalarArg}) internal returns (bytes32 result) {
        ${scalarSection}
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).${op.fheLibName}(lhs, rhs, scalarByte);
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

import {FheType} from "./FheType.sol";

${generateImplCoprocessorInterface(operators)}

${generateACLInterface()}

${generateInputVerifierInterface()}

/**
 * @title   Impl
 * @notice  This library is the core implementation for computing FHE operations (e.g. add, sub, xor).
 */
library Impl {
  /// keccak256(abi.encode(uint256(keccak256("confidential.storage.config")) - 1)) & ~bytes32(uint256(0xff))
  bytes32 private constant CoprocessorConfigLocation = 0x9e7b61f58c47dc699ac88507c4f5bb9f121c03808c5676a8078fe583e4649700;

  /// keccak256(abi.encode(uint256(keccak256("confidential.storage.decryptionRequests")) - 1)) & ~bytes32(uint256(0xff));
  bytes32 private constant DecryptionRequestsStorageLocation = 0x878245876662ba28a480c5ea71726db859fb50222b0a3d7cbbc21cfa336faf00;

  /**
   * @dev Returns the Coprocessor config.
   */
  function getCoprocessorConfig() internal pure returns (CoprocessorConfig storage $) {
      assembly {
          $.slot := CoprocessorConfigLocation
      }
  }

  /**
   * @dev Returns the DecryptionRequests storage struct.
   */
  function getDecryptionRequests() internal pure returns (DecryptionRequests storage $) {
      assembly {
          $.slot := DecryptionRequestsStorageLocation
      }
  }

  /**
   * @notice                  Sets the coprocessor addresses.
   * @param coprocessorConfig Coprocessor config struct that contains contract addresses.
  */
  function setCoprocessor(CoprocessorConfig memory coprocessorConfig) internal {
      CoprocessorConfig storage $ = getCoprocessorConfig();
      $.ACLAddress = coprocessorConfig.ACLAddress;
      $.CoprocessorAddress = coprocessorConfig.CoprocessorAddress;
      $.DecryptionOracleAddress = coprocessorConfig.DecryptionOracleAddress;
      $.KMSVerifierAddress = coprocessorConfig.KMSVerifierAddress;
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
     * @title   CoprocessorConfig
     * @notice  This struct contains all addresses of core contracts, which are needed in a typical dApp.
     */
    struct CoprocessorConfig {
        address ACLAddress;
        address CoprocessorAddress;
        address DecryptionOracleAddress;
        address KMSVerifierAddress;
    }

    /**
     * @title   DecryptionRequests
     * @notice  This struct contains the internal counter for requestIDs generated by the dapp, 
     *          and the mapping from internal requestIDs to list of handles requested for decryption.
     */
    struct DecryptionRequests {
        uint256 counterRequest;
        mapping(uint256 => bytes32[]) requestedHandles;
    }

    /**
    * @title   IFHEVMExecutor
    * @notice  This interface contains all functions to conduct FHE operations.
    */
    interface IFHEVMExecutor {`);
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

    /**
     * @notice                      Returns the address of the InputVerifier contract used by the coprocessor.
     * @return inputVerifierAddress Address of the InputVerifier.
     */
    function getInputVerifierAddress() external view returns (address);
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
     *      Account Abstraction when bundling several UserOps calling the FHEVMExecutor Coprocessor.
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

    /**
     * @notice                  Returns wether a handle is allowed to be publicly decrypted.
     * @param handle            Handle.
     * @return isDecryptable    Whether the handle can be publicly decrypted.
     */
    function isAllowedForDecryption(bytes32 handle) external view returns(bool);
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
   *      Account Abstraction when bundling several UserOps calling the FHEVMExecutor Coprocessor.
   */
  function cleanTransientStorage() external;
  }
  `;
}

function generateKMSVerifierInterface(): string {
  return `
  /** 
   * @title IKMSVerifier 
   * @notice This interface contains the only function required from KMSVerifier. 
   */
  interface IKMSVerifier {
    function verifyDecryptionEIP712KMSSignatures(
        bytes32[] memory handlesList,
        bytes memory decryptedResult,
        bytes memory decryptionProof
    ) external returns (bool);
  }
  `;
}

function generateDecryptionOracleInterface(): string {
  return `
  /** 
   * @title IDecryptionOracle 
   * @notice This interface contains the only function required from DecryptionOracle. 
   */
  interface IDecryptionOracle {
    function requestDecryption(uint256 requestID, bytes32[] calldata ctsHandles, bytes4 callbackSelector) external payable;
  }
  `;
}

export function generateSolidityFHELib(operators: Operator[], fheTypes: FheType[]): string {
  const res: string[] = [];

  res.push(`// SPDX-License-Identifier: BSD-3-Clause-Clear
  pragma solidity ^0.8.24;

  import "./Impl.sol";
  import {FheType} from "./FheType.sol";

  import "encrypted-types/EncryptedTypes.sol";

  ${generateKMSVerifierInterface()}

  ${generateDecryptionOracleInterface()}

  /**
   * @title   FHE
   * @notice  This library is the interaction point for all smart contract developers
   *          that interact with the FHEVM protocol.
   */
  library FHE {
    /// @notice Returned if some handles were already saved for corresponding ID.
    error HandlesAlreadySavedForRequestID();

    /// @notice Returned if there was not handle found for the requested ID.
    error NoHandleFoundForRequestID();

    /// @notice Returned if the returned KMS signatures are not valid.
    error InvalidKMSSignatures();

    /// @notice This event is emitted when requested decryption has been fulfilled.
    event DecryptionFulfilled(uint256 indexed requestID);

    /**
     * @notice                  Sets the coprocessor addresses.
     * @param coprocessorConfig Coprocessor config struct that contains contract addresses.
    */
    function setCoprocessor(CoprocessorConfig memory coprocessorConfig) internal {
        Impl.setCoprocessor(coprocessorConfig);
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

  // 9. Handle conversion from plaintext and externalEXXX to all supported types (e.g., externalEbool --> ebool, uint32 --> euint32)
  adjustedFheTypes.forEach((fheType: AdjustedFheType) =>
    res.push(handleSolidityTFHEConvertPlaintextAndEinputToRespectiveType(fheType)),
  );

  // 10. Handle rand/randBounded for all supported types
  adjustedFheTypes.forEach((fheType: AdjustedFheType) => res.push(handleSolidityTFHERand(fheType)));

  // 11. Push ACL Solidity methods
  res.push(generateSolidityACLMethods(adjustedFheTypes));

  // 12. Push DecryptionOracle Solidity methods
  res.push(generateSolidityDecryptionOracleMethods(adjustedFheTypes));

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
 * Generates the implementation of a unary operator function.
 *
 * @param op - The operator for which the implementation is generated.
 * @returns The string representation of the unary operator function.
 */
function handleUnaryOperatorForImpl(op: Operator): string {
  return `
    function ${op.name}(bytes32 ct) internal returns (bytes32 result) {
      CoprocessorConfig storage $ = getCoprocessorConfig();
      result = IFHEVMExecutor($.CoprocessorAddress).${op.fheLibName}(ct);
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
     *      UserOps calling the FHEVMExecutor.
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

/**
 * Generates Solidity DecryptionOracle methods for the provided FHE types.
 *
 * @param {AdjustedFheType[]} fheTypes - An array of FHE types for which to generate the ACL methods.
 * @returns {string} A string containing the generated Solidity code for the ACL methods.
 */
function generateSolidityDecryptionOracleMethods(fheTypes: AdjustedFheType[]): string {
  const res: string[] = [];

  res.push(
    `
    /**
     * @dev Recovers the stored array of handles corresponding to requestID.
     */
    function loadRequestedHandles(uint256 requestID) internal view returns (bytes32[] memory) {
      DecryptionRequests storage $ = Impl.getDecryptionRequests();
      if ($.requestedHandles[requestID].length == 0) {
          revert NoHandleFoundForRequestID();
      }
      return $.requestedHandles[requestID];
    }

    /**
     * @dev     Calls the DecryptionOracle contract to request the decryption of a list of handles.
     * @notice  Also does the needed call to ACL::allowForDecryption with requested handles.
     */
    function requestDecryption(
        bytes32[] memory ctsHandles,
        bytes4 callbackSelector
    ) internal returns (uint256 requestID) {
      requestID = requestDecryption(ctsHandles, callbackSelector, 0);
    }

    /**
     * @dev     Calls the DecryptionOracle contract to request the decryption of a list of handles, with a custom msgValue.
     * @notice  Also does the needed call to ACL::allowForDecryption with requested handles.
     */
    function requestDecryption(
        bytes32[] memory ctsHandles,
        bytes4 callbackSelector,
        uint256 msgValue
    ) internal returns (uint256 requestID) {
      DecryptionRequests storage $ = Impl.getDecryptionRequests();
      requestID = $.counterRequest;
      CoprocessorConfig storage $$ = Impl.getCoprocessorConfig();
      IACL($$.ACLAddress).allowForDecryption(ctsHandles);
      IDecryptionOracle($$.DecryptionOracleAddress).requestDecryption{value: msgValue}(requestID, ctsHandles, callbackSelector);
      saveRequestedHandles(requestID, ctsHandles);
      $.counterRequest++;
    }

    /**
     * @dev     MUST be called inside the callback function the dApp contract to verify the signatures, 
     * @dev     otherwise fake decryption results could be submitted.
     * @notice  Warning: MUST be called directly in the callback function called by the relayer.
     */
    function checkSignatures(uint256 requestID, bytes memory cleartexts, bytes memory decryptionProof) internal {
        bytes32[] memory handlesList = loadRequestedHandles(requestID);
        bool isVerified = verifySignatures(handlesList, cleartexts, decryptionProof);
        if (!isVerified) {
            revert InvalidKMSSignatures();
        }
        emit DecryptionFulfilled(requestID);
    }

    /**
     * @dev Private low-level function used to link in storage an array of handles to its associated requestID.
     */
    function saveRequestedHandles(uint256 requestID, bytes32[] memory handlesList) private {
      DecryptionRequests storage $ = Impl.getDecryptionRequests();
      if ($.requestedHandles[requestID].length != 0) {
          revert HandlesAlreadySavedForRequestID();
      }
      $.requestedHandles[requestID] = handlesList;
    }

    /**
     * @dev Private low-level function used to extract the decryptedResult bytes array and verify the KMS signatures.
     * @notice  Warning: MUST be called directly in the callback function called by the relayer.
     * @dev The callback function has the following signature:
     * - requestID (static uint256)
     * - cleartexts (dynamic bytes)
     * - decryptionProof (dynamic bytes)
     *
     * This means that the calldata is encoded in the following way:
     * - 4 bytes: selector
     * - 32 bytes: requestID
     * - 32 bytes: offset of the cleartexts
     * - 32 bytes: offset of the decryptionProof 
     * - 32 bytes: length of the cleartexts (total number of bytes)
     * - n*32 bytes: the "n" cleartext values, with "n" the number of handles
     * - 32 bytes: length of the decryptionProof (total number of bytes)
     * - ... the data of the decryptionProof (signatures, extra data)
     */
    function verifySignatures(
        bytes32[] memory handlesList,
        bytes memory cleartexts,
        bytes memory decryptionProof
    ) private returns (bool) {
        // Compute the signature offset
        // This offset is computed by considering the format encoded by the KMS when creating the
        // "decryptedResult" bytes array (see comment below), which is the following:
        // - requestID: 32 bytes
        // - all "n" decrypted values (which is "cleartexts" itself): n*32 bytes ("cleartexts.length" bytes)
        // - offset of the signatures: 32 bytes
        // - the rest of signature values (lengths, offsets, values)
        // This means the expected offset to concatenate to the "decryptedResult" bytes array has
        // the following value: 32 + n*32 + 32
        // See https://docs.soliditylang.org/en/latest/abi-spec.html#use-of-dynamic-types for more details.
        // The signature offset will most likely be removed in the future,
        // see https://github.com/zama-ai/fhevm-internal/issues/345
        uint256 signaturesOffset = 32 + cleartexts.length + 32;

        // Built the "decryptedResult" bytes array
        // Currently, the "decryptedResult" is encoded (by the KMS) in the following format:
        // - n*32 bytes: the "n" decrypted values, "cleartexts" itself
        // - 32 bytes: offset of the signatures, as explained above
        // This is equivalent to concatenating the cleartexts and the signatures offset, which can
        // be done using abi.encoded in a gas efficient way.
        // The signature offset will most likely be removed in the future,
        // see https://github.com/zama-ai/fhevm-internal/issues/345
        // Here we can use "encodePacked" instead of "abi.encode" to save gas, as the cleartexts
        // and the signaturesOffset are already 32 bytes aligned (ie, no padding needed).
        bytes memory decryptedResult = abi.encodePacked(cleartexts, signaturesOffset);

        CoprocessorConfig storage $ = Impl.getCoprocessorConfig();
        return
            IKMSVerifier($.KMSVerifierAddress).verifyDecryptionEIP712KMSSignatures(
                handlesList,
                decryptedResult,
                decryptionProof
            );
    }
  `,
  );

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

function generateCustomMethodsForImpl(): string {
  return `
    /**
    * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
    *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
    */
    function select(bytes32 control, bytes32 ifTrue, bytes32 ifFalse) internal returns (bytes32 result) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheIfThenElse(control, ifTrue, ifFalse);
    }

    /**
     * @notice              Verifies the ciphertext (FHEVMExecutor) and allows transient (ACL).
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
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).verifyCiphertext(inputHandle, msg.sender, inputProof, toType);
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
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).cast(ciphertext, toType);
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
      CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).trivialEncrypt(value, toType);
    }

    function rand(FheType randType) internal returns(bytes32 result) {
      CoprocessorConfig storage $ = getCoprocessorConfig();
      result = IFHEVMExecutor($.CoprocessorAddress).fheRand(randType);
    }

    function randBounded(uint256 upperBound, FheType randType) internal returns(bytes32 result) {
      CoprocessorConfig storage $ = getCoprocessorConfig();
      result = IFHEVMExecutor($.CoprocessorAddress).fheRandBounded(upperBound, randType);
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
      CoprocessorConfig storage $ = getCoprocessorConfig();
      IACL($.ACLAddress).allowTransient(handle, account);
    }


    /**
     * @notice              Allows the use of handle for the address account.
     * @dev                 The caller must be allowed to use handle for allow() to succeed. If not, allow() reverts.
     * @param handle        Handle.
     * @param account       Address of the account.
     */
    function allow(bytes32 handle, address account) internal {
      CoprocessorConfig storage $ = getCoprocessorConfig();
      IACL($.ACLAddress).allow(handle, account);
    }

    /**
     * @notice              Allows the handle to be publicly decryptable.
     * @dev                 The caller must be allowed to use handle for makePubliclyDecryptable() to succeed. 
     *                      If not, makePubliclyDecryptable() reverts.
     * @param handle        Handle.
     */
    function makePubliclyDecryptable(bytes32 handle) internal {
      CoprocessorConfig storage $ = getCoprocessorConfig();
      bytes32[] memory handleArray = new bytes32[](1);
      handleArray[0] = handle;
      IACL($.ACLAddress).allowForDecryption(handleArray);
    }

    /**
     * @dev This function removes the transient allowances in the ACL, which could be useful for integration
     *      with Account Abstraction when bundling several UserOps calling the FHEVMExecutor Coprocessor.
     */
    function cleanTransientStorageACL() internal {
      CoprocessorConfig storage $ = getCoprocessorConfig();
      IACL($.ACLAddress).cleanTransientStorage();
    }


    /**
     * @dev This function removes the transient proofs in the InputVerifier, which could be useful for integration 
     *      with Account Abstraction when bundling several UserOps calling the FHEVMExecutor Coprocessor.
     */
    function cleanTransientStorageInputVerifier() internal {
      CoprocessorConfig storage $ = getCoprocessorConfig();
      address inputVerifierAddress = IFHEVMExecutor($.CoprocessorAddress).getInputVerifierAddress();
      IInputVerifier(inputVerifierAddress).cleanTransientStorage();
    }


    /**
     * @notice              Returns whether the account is allowed to use the handle, either due to
     *                      allowTransient() or allow().
     * @param handle        Handle.
     * @param account       Address of the account.
     * @return isAllowed    Whether the account can access the handle.
     */
    function isAllowed(bytes32 handle, address account) internal view returns (bool) {
      CoprocessorConfig storage $ = getCoprocessorConfig();
      return IACL($.ACLAddress).isAllowed(handle, account);
    }

    /**
     * @notice              Returns whether the handle is allowed to be publicly decrypted.
     * @param handle        Handle.
     * @return isAllowed    Whether the handle can be publicly decrypted.
     */
    function isPubliclyDecryptable(bytes32 handle) internal view returns (bool) {
      CoprocessorConfig storage $ = getCoprocessorConfig();
      return IACL($.ACLAddress).isAllowedForDecryption(handle);
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
