import { assert } from 'console';

import { Operator, OperatorArguments, ReturnType } from './common';
import { ArgumentType, OverloadSignature } from './testgen';
import { getUint } from './utils';

export function commonSolLib(): string {
  return `
type ebool is bytes32;
type euint4 is bytes32;
type euint8 is bytes32;
type euint16 is bytes32;
type euint32 is bytes32;
type euint64 is bytes32;
type euint128 is bytes32;
type euint256 is bytes32;
type eaddress is bytes32;
type ebytes64 is bytes32;
type ebytes128 is bytes32;
type ebytes256 is bytes32;
type einput is bytes32;

/**
 * @title   Common
 * @notice  This library contains all the values used to communicate types to the run time.
 */
library Common {

    /// @notice Runtime type for encrypted boolean.
    uint8 internal constant ebool_t = 0;

    /// @notice Runtime type for encrypted uint4.
    uint8 internal constant euint4_t = 1;

    /// @notice Runtime type for encrypted uint8.
    uint8 internal constant euint8_t = 2;

    /// @notice Runtime type for encrypted uint16.
    uint8 internal constant euint16_t = 3;

    /// @notice Runtime type for encrypted uint32.
    uint8 internal constant euint32_t = 4;
    /// @notice Runtime type for encrypted uint64.
    uint8 internal constant euint64_t = 5;
    /// @notice Runtime type for encrypted uint128.
    uint8 internal constant euint128_t = 6;
    /// @notice Runtime type for encrypted addresses.
    uint8 internal constant euint160_t = 7;

    /// @notice Runtime type for encrypted uint256.
    uint8 internal constant euint256_t = 8;

    /// @notice Runtime type for encrypted bytes64.
    uint8 internal constant ebytes64_t = 9;

    /// @notice Runtime type for encrypted bytes128.
    uint8 internal constant ebytes128_t = 10;

    /// @notice Runtime type for encrypted bytes256.
    uint8 internal constant ebytes256_t = 11;
}
`;
}

function binaryOperatorImpl(op: Operator): string {
  const fname = operatorFheLibFunction(op);
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
     * @dev Returns the FHEVM config.
     */
    function ${op.name}(bytes32 lhs, bytes32 rhs${scalarArg}) internal returns (bytes32 result) {
        ${scalarSection}
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).${fname}(lhs, rhs, scalarByte);
    }` + '\n'
  );
}

export function implSol(operators: Operator[]): string {
  const res: string[] = [];

  const coprocessorInterface = generateImplCoprocessorInterface(operators);
  const aclInterface = generateACLInterface();
  const inputVerifierInterface = generateInputVerifierInterface();

  res.push(`
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./TFHE.sol";

${coprocessorInterface}

${aclInterface}

${inputVerifierInterface}

/**
 * @title   Impl
 * @notice  This library is the core implementation for computing FHE operations (e.g. add, sub, xor).
 */
library Impl {
  /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.FHEVMConfig")) - 1)) & ~bytes32(uint256(0xff))
  bytes32 private constant FHEVMConfigLocation = 0xed8d60e34876f751cc8b014c560745351147d9de11b9347c854e881b128ea600;

  /**
   * @dev Returns the FHEVM config.
   */
  function getFHEVMConfig() internal pure returns (FHEVMConfigStruct storage $) {
      assembly {
          $.slot := FHEVMConfigLocation
      }
  }

  /**
   * @notice            Sets the FHEVM addresses.
   * @param fhevmConfig FHEVM config struct that contains contract addresses.
  */
  function setFHEVM(FHEVMConfigStruct memory fhevmConfig) internal {
      FHEVMConfigStruct storage $ = getFHEVMConfig();
      $.ACLAddress = fhevmConfig.ACLAddress;
      $.TFHEExecutorAddress = fhevmConfig.TFHEExecutorAddress;
      $.KMSVerifierAddress = fhevmConfig.KMSVerifierAddress;
      $.InputVerifierAddress = fhevmConfig.InputVerifierAddress;
  }
`);

  operators.forEach((op) => {
    switch (op.arguments) {
      case OperatorArguments.Binary:
        res.push(binaryOperatorImpl(op));
        break;
      case OperatorArguments.Unary:
        res.push(unaryOperatorImpl(op));
        break;
    }
  });

  res.push(implCustomMethods());

  res.push('}\n');

  return res.join('');
}

export function fhevmLibSol(operators: Operator[]): string {
  const res: string[] = [];

  const fheLibInterface = generateImplFhevmLibInterface(operators);

  res.push(`
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

${fheLibInterface}

`);

  return res.join('');
}

function operatorFheLibFunction(op: Operator): string {
  if (op.fheLibName) {
    return op.fheLibName;
  }
  return `fhe${capitalizeFirstLetter(op.name)}`;
}

function capitalizeFirstLetter(input: string): string {
  const firstLetter = input.toUpperCase().charAt(0);
  const theRest = input.substring(1);
  return `${firstLetter}${theRest}`;
}

function generateImplFhevmLibInterface(operators: Operator[]): string {
  const res: string[] = [];

  res.push('interface FhevmLib {');
  res.push(fheLibCustomInterfaceFunctions());
  res.push('}');

  return res.join('\n');
}

function generateImplCoprocessorInterface(operators: Operator[]): string {
  const res: string[] = [];

  res.push(`
    /**
     * @title   FHEVMConfigStruct
     * @notice  This struct contains all addresses of core contracts, which are needed in a typical dApp.
     */
    struct FHEVMConfigStruct {
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
    let functionName = operatorFheLibFunction(op);
    const tail = 'external returns (bytes32 result);';
    let functionArguments: string;
    switch (op.arguments) {
      case OperatorArguments.Binary:
        functionArguments = '(bytes32 lhs, bytes32 rhs, bytes1 scalarByte)';
        res.push(`  
          
          /**
           * @notice              Computes ${functionName} operation.
           * @param lhs           LHS.
           * @param rhs           RHS.
           * @param scalarByte    Scalar byte.
           * @return result       Result.
           */
          function ${functionName}${functionArguments} ${tail}`);
        break;
      case OperatorArguments.Unary:
        functionArguments = '(bytes32 ct)';
        res.push(`  

           /**
           * @notice              Computes ${functionName} operation.
           * @param ct            Ct
           * @return result       Result.
           */
          function ${functionName}${functionArguments} ${tail}`);
        break;
    }
  });

  res.push(coprocessorInterfaceCustomFunctions());

  res.push('}');

  return res.join('');
}

function fheLibCustomInterfaceFunctions(): string {
  return `
    /**
     * @notice                Verifies the ciphertext.
     * @param inputHandle     Input handle.
     * @param callerAddress   Address of the caller.
     * @param inputProof      Input proof.
     * @param inputType       Input type.
     * @return result         Result.
     */
    function verifyCiphertext(bytes32 inputHandle, address callerAddress, address contractAddress, bytes memory inputProof, bytes1 inputType) external pure returns (bytes32 result);
  `;
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
    function verifyCiphertext(bytes32 inputHandle, address callerAddress, bytes memory inputProof, bytes1 inputType) external returns (bytes32 result);

    /**
     * @notice          Performs the casting to a target type.
     * @param ct        Value to cast.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function cast(bytes32 ct, bytes1 toType) external returns (bytes32 result);

     /**
     * @notice          Does trivial encryption.
     * @param ct        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(uint256 ct, bytes1 toType) external returns (bytes32 result);

    /**
     * @notice          Does trivial encryption.
     * @param ct        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(bytes memory ct, bytes1 toType) external returns (bytes32 result);

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
    function fheRand(bytes1 randType) external returns (bytes32 result);

    /**
     * @notice              Computes FHERandBounded operation.
     * @param upperBound    Upper bound value.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRandBounded(uint256 upperBound, bytes1 randType) external returns (bytes32 result);
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

export function tfheSol(
  operators: Operator[],
  supportedBits: number[],
  mocked: boolean,
): [string, OverloadSignature[]] {
  const signatures: OverloadSignature[] = [];
  const res: string[] = [];

  res.push(`// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./Impl.sol";


${commonSolLib()}


/**
 * @title   TFHE
 * @notice  This library is the interaction point for all smart contract developers
 *          that interact with TFHE.
 */
library TFHE {

/// @notice Returned if the input's length is greater than 64 bytes.
error InputLengthAbove64Bytes(uint256 inputLength);

/// @notice Returned if the input's length is greater than 128 bytes.
error InputLengthAbove128Bytes(uint256 inputLength);

/// @notice Returned if the input's length is greater than 256 bytes.
error InputLengthAbove256Bytes(uint256 inputLength);

  /**
   * @notice            Sets the FHEVM addresses.
   * @param fhevmConfig FHEVM config struct that contains contract addresses.
  */
  function setFHEVM(FHEVMConfigStruct memory fhevmConfig) internal {
      Impl.setFHEVM(fhevmConfig);
  }
`);

  if (mocked) {
    res.push(`
    /**
    * @dev Returns true if the encrypted bool is initialized and false otherwise.
    */
    function isInitialized(ebool /*v*/) internal pure returns (bool) {
        return true;
    }
  `);
    supportedBits.forEach((b) => {
      res.push(`
      /**
      * @dev Returns true if the encrypted integer is initialized and false otherwise.
      */
      function isInitialized(euint${b} /*v*/) internal pure returns (bool) {
          return true;
      }
    `);
    });
  } else {
    res.push(`
      /**
      * @dev Returns true if the encrypted integer is initialized and false otherwise.
      */
    function isInitialized(ebool v) internal pure returns (bool) {
        return ebool.unwrap(v) != 0;
    }
  `);
    supportedBits.forEach((b) => {
      res.push(`
      /**
      * @dev Returns true if the encrypted integer is initialized and false otherwise.
      */
      function isInitialized(euint${b} v) internal pure returns (bool) {
          return euint${b}.unwrap(v) != 0;
      }
    `);
    });
  }

  supportedBits.forEach((lhsBits) => {
    supportedBits.forEach((rhsBits) => {
      operators.forEach((operator) => {
        if (!operator.shiftOperator && !operator.rotateOperator)
          res.push(tfheEncryptedOperator(lhsBits, rhsBits, operator, signatures));
      });
    });
    operators.forEach((operator) => {
      if (!operator.shiftOperator && !operator.rotateOperator)
        res.push(tfheScalarOperator(lhsBits, lhsBits, operator, signatures));
    });
  });

  supportedBits.forEach((bits) => {
    operators.forEach((operator) => {
      if (operator.shiftOperator || operator.rotateOperator)
        res.push(tfheShiftOperators(bits, operator, signatures, !!operator.rotateOperator, mocked));
    });
  });

  // TODO: Decide whether we want to have mixed-inputs for CMUX/Select
  supportedBits.forEach((bits) => res.push(tfheSelect(bits)));
  supportedBits.forEach((outputBits) => {
    supportedBits.forEach((inputBits) => {
      res.push(tfheAsEboolCustomCast(inputBits, outputBits));
    });
    res.push(tfheAsEboolUnaryCast(outputBits));
  });
  supportedBits.forEach((bits) => res.push(tfheUnaryOperators(bits, operators, signatures)));
  supportedBits.forEach((bits) => res.push(tfheCustomUnaryOperators(bits, signatures, mocked)));

  res.push(tfheCustomMethods());

  res.push(tfheAclMethods(supportedBits));

  res.push('}\n');

  return [res.join(''), signatures];
}

function tfheEncryptedOperator(
  lhsBits: number,
  rhsBits: number,
  operator: Operator,
  signatures: OverloadSignature[],
): string {
  if (!operator.hasEncrypted || operator.arguments != OperatorArguments.Binary) {
    return '';
  }

  const res: string[] = [];

  const outputBits = Math.max(lhsBits, rhsBits);
  const castLeftToRight = lhsBits < rhsBits;
  const castRightToLeft = lhsBits > rhsBits;
  const returnType =
    operator.returnType == ReturnType.Uint
      ? `euint${outputBits}`
      : operator.returnType == ReturnType.Ebool
        ? `ebool`
        : assert(false, 'Unknown return type');
  const returnTypeOverload: ArgumentType =
    operator.returnType == ReturnType.Uint ? ArgumentType.EUint : ArgumentType.Ebool;
  const scalarFlag = operator.hasEncrypted && operator.hasScalar ? ', false' : '';

  const leftExpr = castLeftToRight ? `asEuint${outputBits}(a)` : 'a';
  const rightExpr = castRightToLeft ? `asEuint${outputBits}(b)` : 'b';
  let implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(${leftExpr}), euint${outputBits}.unwrap(${rightExpr})${scalarFlag})`;
  signatures.push({
    name: operator.name,
    arguments: [
      { type: ArgumentType.EUint, bits: lhsBits },
      { type: ArgumentType.EUint, bits: rhsBits },
    ],
    returnType: { type: returnTypeOverload, bits: outputBits },
  });
  res.push(`
    /**
    * @dev Evaluates ${operator.name}(a, b) and returns the result.
    */
    function ${operator.name}(euint${lhsBits} a, euint${rhsBits} b) internal returns (${returnType}) {
        if (!isInitialized(a)) {
            a = asEuint${lhsBits}(0);
        }
        if (!isInitialized(b)) {
            b = asEuint${rhsBits}(0);
        }
        return ${returnType}.wrap(${implExpression});
    }
`);

  return res.join('');
}

function tfheScalarOperator(
  lhsBits: number,
  rhsBits: number,
  operator: Operator,
  signatures: OverloadSignature[],
): string {
  if (operator.arguments != OperatorArguments.Binary) {
    return '';
  }

  if (!operator.hasScalar || lhsBits != rhsBits) {
    return '';
  }

  const res: string[] = [];

  const outputBits = Math.max(lhsBits, rhsBits);
  const returnType =
    operator.returnType == ReturnType.Uint
      ? `euint${outputBits}`
      : operator.returnType == ReturnType.Ebool
        ? `ebool`
        : assert(false, 'Unknown return type');
  const returnTypeOverload = operator.returnType == ReturnType.Uint ? ArgumentType.EUint : ArgumentType.Ebool;
  var scalarFlag = operator.hasEncrypted && operator.hasScalar ? ', true' : '';
  const leftOpName = operator.leftScalarInvertOp ?? operator.name;
  var implExpressionA = `Impl.${operator.name}(euint${outputBits}.unwrap(a), bytes32(uint256(b))${scalarFlag})`;
  var implExpressionB = `Impl.${leftOpName}(euint${outputBits}.unwrap(b), bytes32(uint256(a))${scalarFlag})`;
  var maybeEncryptLeft = '';
  if (operator.leftScalarEncrypt) {
    // workaround until tfhe-rs left scalar support:
    // do the trivial encryption and preserve order of operations
    scalarFlag = ', false';
    maybeEncryptLeft = `euint${outputBits} aEnc = asEuint${outputBits}(a);`;
    implExpressionB = `Impl.${leftOpName}(euint${outputBits}.unwrap(aEnc), euint${outputBits}.unwrap(b)${scalarFlag})`;
  }
  signatures.push({
    name: operator.name,
    arguments: [
      { type: ArgumentType.EUint, bits: lhsBits },
      { type: ArgumentType.Uint, bits: rhsBits },
    ],
    returnType: { type: returnTypeOverload, bits: outputBits },
  });

  // rhs scalar
  res.push(`
    /**
    * @dev Evaluates ${operator.name}(a, b) and returns the result.
    */
    function ${operator.name}(euint${lhsBits} a, ${getUint(rhsBits)} b) internal returns (${returnType}) {
        if (!isInitialized(a)) {
            a = asEuint${lhsBits}(0);
        }
        return ${returnType}.wrap(${implExpressionA});
    }
`);

  // lhs scalar
  if (!operator.leftScalarDisable) {
    signatures.push({
      name: operator.name,
      arguments: [
        { type: ArgumentType.Uint, bits: rhsBits },
        { type: ArgumentType.EUint, bits: lhsBits },
      ],
      returnType: { type: returnTypeOverload, bits: outputBits },
    });

    res.push(`

    /**
    * @dev Evaluates ${operator.name}(a, b) and returns the result.
    */
    function ${operator.name}(${getUint(lhsBits)} a, euint${rhsBits} b) internal returns (${returnType}) {
        ${maybeEncryptLeft}
        if (!isInitialized(b)) {
            b = asEuint${rhsBits}(0);
        }
        return ${returnType}.wrap(${implExpressionB});
    }
        `);
  }

  return res.join('');
}

function tfheShiftOperators(
  inputBits: number,
  operator: Operator,
  signatures: OverloadSignature[],
  rotate: boolean,
  mocked: boolean,
): string {
  const res: string[] = [];

  // Code and test for shift(euint{inputBits},euint8}
  const outputBits = inputBits;
  const lhsBits = inputBits;
  const rhsBits = 8;
  const castRightToLeft = lhsBits > rhsBits;

  const returnType = `euint${outputBits}`;

  const returnTypeOverload: ArgumentType = ArgumentType.EUint;
  let scalarFlag = ', false';

  const leftExpr = 'a';
  const rightExpr = castRightToLeft ? `asEuint${outputBits}(b)` : 'b';
  let implExpression: string;
  if (mocked) {
    if (rotate) {
      implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(${leftExpr}), euint${outputBits}.unwrap(${rightExpr}) % ${lhsBits}, ${lhsBits}${scalarFlag})`;
    } else {
      implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(${leftExpr}), euint${outputBits}.unwrap(${rightExpr}) % ${lhsBits}${scalarFlag})`;
    }
  } else {
    implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(${leftExpr}), euint${outputBits}.unwrap(${rightExpr})${scalarFlag})`;
  }

  if (inputBits >= 8) {
    signatures.push({
      name: operator.name,
      arguments: [
        { type: ArgumentType.EUint, bits: lhsBits },
        { type: ArgumentType.EUint, bits: rhsBits },
      ],
      returnType: { type: returnTypeOverload, bits: outputBits },
    });

    res.push(`
    /** 
     * @dev Evaluates ${operator.name}(a, b) and returns the result.
     */
    function ${operator.name}(euint${lhsBits} a, euint${rhsBits} b) internal returns (${returnType}) {
        if (!isInitialized(a)) {
            a = asEuint${lhsBits}(0);
        }
        if (!isInitialized(b)) {
            b = asEuint${rhsBits}(0);
        }
        return ${returnType}.wrap(${implExpression});
    }
`);
  }

  // Code and test for shift(euint{inputBits},uint8}
  scalarFlag = ', true';
  implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(a), bytes32(uint256(b))${scalarFlag})`;
  if (mocked) {
    if (rotate) {
      implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(a), bytes32(uint256(b)) % ${lhsBits}, ${lhsBits}${scalarFlag})`;
    } else {
      implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(a), bytes32(uint256(b)) % ${lhsBits}${scalarFlag})`;
    }
  }
  signatures.push({
    name: operator.name,
    arguments: [
      { type: ArgumentType.EUint, bits: lhsBits },
      { type: ArgumentType.Uint, bits: rhsBits },
    ],
    returnType: { type: returnTypeOverload, bits: outputBits },
  });
  res.push(`
    /** 
     * @dev Evaluates ${operator.name}(a, b) and returns the result.
     */
    function ${operator.name}(euint${lhsBits} a, ${getUint(rhsBits)} b) internal returns (${returnType}) {
        if (!isInitialized(a)) {
            a = asEuint${lhsBits}(0);
        }
        return ${returnType}.wrap(${implExpression});
    }
  `);
  return res.join('');
}

function tfheSelect(inputBits: number): string {
  return `
    /**
    * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
    *         If 'control's value is 'false', the result has the same value as 'ifFalse'.
    */
    function select(ebool control, euint${inputBits} a, euint${inputBits} b) internal returns (euint${inputBits}) {
        return euint${inputBits}.wrap(Impl.select(ebool.unwrap(control), euint${inputBits}.unwrap(a), euint${inputBits}.unwrap(b)));
    }`;
}

function tfheAsEboolCustomCast(inputBits: number, outputBits: number): string {
  if (inputBits == outputBits) {
    return '';
  }

  return `
  /**
    * @dev Casts an encrypted integer from euint${inputBits} to euint${outputBits}.
    */
    function asEuint${outputBits}(euint${inputBits} value) internal returns (euint${outputBits}) {
        return euint${outputBits}.wrap(Impl.cast(euint${inputBits}.unwrap(value), Common.euint${outputBits}_t));
    }
    `;
}

function tfheAsEboolUnaryCast(bits: number): string {
  const res: string[] = [];
  res.push(`
    /**
    * @dev Casts an encrypted integer from euint${bits} to ebool.
    */
    function asEbool(euint${bits} value) internal returns (ebool) {
        return ne(value, 0);
    }
    `);

  if (bits == 8) {
    res.push(`
    /**
    * @dev Converts an inputHandle with corresponding inputProof to an encrypted boolean.
    */
    function asEbool(einput inputHandle, bytes memory inputProof) internal returns (ebool) {
        return ebool.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, Common.ebool_t));
    }

    /**
    * @dev Converts a plaintext value to an encrypted boolean.
    */
    function asEbool(uint256 value) internal returns (ebool) {
        return ebool.wrap(Impl.trivialEncrypt(value, Common.ebool_t));
    }

    /** 
     * @dev Converts a plaintext boolean to an encrypted boolean.
     */
    function asEbool(bool value) internal returns (ebool) {
        if (value) {
            return asEbool(uint256(1));
        } else {
            return asEbool(uint256(0));
        }
    }

    /** 
     * @dev Converts an 'ebool' to an 'euint8'.
    */
     function asEuint8(ebool value) internal returns (euint8) {
      return euint8.wrap(Impl.cast(ebool.unwrap(value), Common.euint8_t));
    }

     /** 
     * @dev Evaluates and(a, b) and returns the result.
     */
     function and(ebool a, ebool b) internal returns (ebool) {
        return ebool.wrap(Impl.and(ebool.unwrap(a), ebool.unwrap(b), false));
    }

    /** 
     * @dev Evaluates and(a, b) and returns the result.
    */
     function and(ebool a, bool b) internal returns (ebool) {
        return ebool.wrap(Impl.and(ebool.unwrap(a), bytes32(uint256(b?1:0)), true));
    }

     /** 
     * @dev Evaluates and(a, b) and returns the result.
    */
     function and(bool a, ebool b) internal returns (ebool) {
        return ebool.wrap(Impl.and(ebool.unwrap(b), bytes32(uint256(a?1:0)), true));
    }

     /** 
     * @dev Evaluates or(a, b) and returns the result.
    */
     function or(ebool a, ebool b) internal returns (ebool) {
        return ebool.wrap(Impl.or(ebool.unwrap(a), ebool.unwrap(b), false));
    }

     /** 
     * @dev Evaluates or(a, b) and returns the result.
     */
     function or(ebool a, bool b) internal returns (ebool) {
        return ebool.wrap(Impl.or(ebool.unwrap(a), bytes32(uint256(b?1:0)), true));
    }

    /** 
     * @dev Evaluates or(a, b) and returns the result.
     */
    function or(bool a, ebool b) internal returns (ebool) {
        return ebool.wrap(Impl.or(ebool.unwrap(b), bytes32(uint256(a?1:0)), true));
    }

    /** 
     * @dev Evaluates xor(a, b) and returns the result.
     */
    function xor(ebool a, ebool b) internal returns (ebool) {
        return ebool.wrap(Impl.xor(ebool.unwrap(a), ebool.unwrap(b), false));
    }

    /** 
     * @dev Evaluates xor(a, b) and returns the result.
     */ 
    function xor(ebool a, bool b) internal returns (ebool) {
        return ebool.wrap(Impl.xor(ebool.unwrap(a), bytes32(uint256(b?1:0)), true));
    }

    /** 
     * @dev Evaluates xor(a, b) and returns the result.
     */
    function xor(bool a, ebool b) internal returns (ebool) {
        return ebool.wrap(Impl.xor(ebool.unwrap(b), bytes32(uint256(a?1:0)), true));
    }

    function not(ebool a) internal returns (ebool) {
        return ebool.wrap(Impl.not(ebool.unwrap(a)));
    }
    `);
  } else {
    res.push(`
    /** 
     * @dev Converts an 'ebool' to an 'euint${bits}'.
     */
    function asEuint${bits}(ebool b) internal returns (euint${bits}) {
        return euint${bits}.wrap(Impl.cast(ebool.unwrap(b), Common.euint${bits}_t));
    }
    `);
  }

  return res.join('');
}

function tfheUnaryOperators(bits: number, operators: Operator[], signatures: OverloadSignature[]): string {
  const res: string[] = [];

  operators.forEach((op) => {
    if (op.arguments == OperatorArguments.Unary) {
      signatures.push({
        name: op.name,
        arguments: [{ type: ArgumentType.EUint, bits }],
        returnType: { type: ArgumentType.EUint, bits },
      });

      res.push(`
        function ${op.name}(euint${bits} value) internal returns (euint${bits}) {
            return euint${bits}.wrap(Impl.${op.name}(euint${bits}.unwrap(value)));
        }
      `);
    }
  });

  return res.join('\n');
}

function tfheCustomUnaryOperators(bits: number, signatures: OverloadSignature[], mocked: boolean): string {
  let result = `
    /** 
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted euint${bits} integer.
     */
    function asEuint${bits}(einput inputHandle, bytes memory inputProof) internal returns (euint${bits}) {
        return euint${bits}.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, Common.euint${bits}_t));
    }

    /** 
     * @dev Convert a plaintext value to an encrypted euint${bits} integer.
    */
    function asEuint${bits}(uint256 value) internal returns (euint${bits}) {
        return euint${bits}.wrap(Impl.trivialEncrypt(value, Common.euint${bits}_t));
    }

    `;
  return result;
}

function unaryOperatorImpl(op: Operator): string {
  let fname = operatorFheLibFunction(op);
  return `
    function ${op.name}(bytes32 ct) internal returns (bytes32 result) {
      FHEVMConfigStruct storage $ = getFHEVMConfig();
      result = ITFHEExecutor($.TFHEExecutorAddress).${fname}(ct);
    }
  `;
}

function tfheAclMethods(supportedBits: number[]): string {
  const res: string[] = [];

  res.push(
    `
    /**
     * @dev This function cleans the transient storage for the ACL (accounts) and the InputVerifier
     *      (input proofs).
     *      This could be useful for integration with Account Abstraction when bundling several 
     *      UserOps calling the TFHEExecutorCoprocessor.
     */
    function cleanTransientStorage() internal {
      Impl.cleanTransientStorageACL();
      Impl.cleanTransientStorageInputVerifier();
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(ebool value, address account) internal view returns (bool) {
      return Impl.isAllowed(ebool.unwrap(value), account);
    }
  `,
  );

  supportedBits.forEach((bits) =>
    res.push(`
    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(euint${bits} value, address account) internal view returns (bool) {
      return Impl.isAllowed(euint${bits}.unwrap(value), account);
    }`),
  );

  res.push(
    `
    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(eaddress value, address account) internal view returns(bool) {
      return Impl.isAllowed(eaddress.unwrap(value), account);
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(ebytes256 value, address account) internal view returns (bool) {
      return Impl.isAllowed(ebytes256.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(ebool value) internal view returns (bool) {
      return Impl.isAllowed(ebool.unwrap(value), msg.sender);
    }
    `,
  );

  supportedBits.forEach((bits) =>
    res.push(
      `
      /**
      * @dev Returns whether the sender is allowed to use the value.
      */
      function isSenderAllowed(euint${bits} value) internal view returns (bool) {
        return Impl.isAllowed(euint${bits}.unwrap(value), msg.sender);
      }
      `,
    ),
  );

  res.push(
    `
    /**
    * @dev Returns whether the sender is allowed to use the value.
    */
    function isSenderAllowed(eaddress value) internal view returns(bool) {
      return Impl.isAllowed(eaddress.unwrap(value), msg.sender);
    }

    /**
    * @dev Returns whether the sender is allowed to use the value.
    */
    function isSenderAllowed(ebytes256 value) internal view returns(bool) {
      return Impl.isAllowed(ebytes256.unwrap(value), msg.sender);
    }
    `,
  );

  res.push(
    `
    function allow(ebool value, address account) internal {
      Impl.allow(ebool.unwrap(value), account);
    }

    function allowThis(ebool value) internal {
      Impl.allow(ebool.unwrap(value), address(this));
    }
    `,
  );

  supportedBits.forEach((bits) =>
    res.push(
      `
    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(euint${bits} value, address account) internal {
      Impl.allow(euint${bits}.unwrap(value), account);
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(euint${bits} value) internal {
      Impl.allow(euint${bits}.unwrap(value), address(this));
    }
    \n`,
    ),
  );

  res.push(
    `
    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(eaddress value, address account) internal {
      Impl.allow(eaddress.unwrap(value), account);
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(eaddress value) internal {
      Impl.allow(eaddress.unwrap(value), address(this));
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(ebytes64 value, address account) internal {
      Impl.allow(ebytes64.unwrap(value), account);
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(ebytes64 value) internal {
      Impl.allow(ebytes64.unwrap(value), address(this));
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(ebytes128 value, address account) internal {
      Impl.allow(ebytes128.unwrap(value), account);
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(ebytes128 value) internal {
      Impl.allow(ebytes128.unwrap(value), address(this));
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(ebytes256 value, address account) internal {
      Impl.allow(ebytes256.unwrap(value), account);
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(ebytes256 value) internal {
      Impl.allow(ebytes256.unwrap(value), address(this));
    }
    `,
  );

  res.push(
    `
    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(ebool value, address account) internal {
      Impl.allowTransient(ebool.unwrap(value), account);
    }
    `,
  );

  supportedBits.forEach((bits) =>
    res.push(
      `
    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(euint${bits} value, address account) internal {
      Impl.allowTransient(euint${bits}.unwrap(value), account);
    }
    \n`,
    ),
  );

  res.push(
    `
    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(eaddress value, address account) internal {
      Impl.allowTransient(eaddress.unwrap(value), account);
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(ebytes64 value, address account) internal {
      Impl.allowTransient(ebytes64.unwrap(value), account);
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(ebytes128 value, address account) internal {
      Impl.allowTransient(ebytes128.unwrap(value), account);
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(ebytes256 value, address account) internal {
      Impl.allowTransient(ebytes256.unwrap(value), account);
    }
    `,
  );

  return res.join('');
}

function tfheCustomMethods(): string {
  let result = `
    /**
    * @dev Generates a random encrypted boolean.
    */
    function randEbool() internal returns (ebool) {
      return ebool.wrap(Impl.rand(Common.ebool_t));
    }

    /**
    * @dev Generates a random encrypted 4-bit unsigned integer.
    */
    function randEuint4() internal returns (euint4) {
      return euint4.wrap(Impl.rand(Common.euint4_t));
    }

    /**
    * @dev Generates a random encrypted 4-bit unsigned integer in the [0, upperBound) range.
    *      The upperBound must be a power of 2.
    */
    function randEuint4(uint8 upperBound) internal returns (euint4) {
      return euint4.wrap(Impl.randBounded(upperBound, Common.euint4_t));
    }

    /**
    * @dev Generates a random encrypted 8-bit unsigned integer.
    */
    function randEuint8() internal returns (euint8) {
      return euint8.wrap(Impl.rand(Common.euint8_t));
    }

    /**
    * @dev Generates a random encrypted 8-bit unsigned integer in the [0, upperBound) range.
    *      The upperBound must be a power of 2.
    */
    function randEuint8(uint8 upperBound) internal returns (euint8) {
      return euint8.wrap(Impl.randBounded(upperBound, Common.euint8_t));
    }

    /**
    * @dev Generates a random encrypted 16-bit unsigned integer.
    */
    function randEuint16() internal returns (euint16) {
      return euint16.wrap(Impl.rand(Common.euint16_t));
    }

    /**
    * @dev Generates a random encrypted 16-bit unsigned integer in the [0, upperBound) range.
    *      The upperBound must be a power of 2.
    */
    function randEuint16(uint16 upperBound) internal returns (euint16) {
      return euint16.wrap(Impl.randBounded(upperBound, Common.euint16_t));
    }

    /**
    * @dev Generates a random encrypted 32-bit unsigned integer.
    */
    function randEuint32() internal returns (euint32) {
      return euint32.wrap(Impl.rand(Common.euint32_t));
    }

    /**
    * @dev Generates a random encrypted 32-bit unsigned integer in the [0, upperBound) range.
    *      The upperBound must be a power of 2.
    */
    function randEuint32(uint32 upperBound) internal returns (euint32) {
      return euint32.wrap(Impl.randBounded(upperBound, Common.euint32_t));
    }

    /**
    * @dev Generates a random encrypted 64-bit unsigned integer.
    */
    function randEuint64() internal returns (euint64) {
      return euint64.wrap(Impl.rand(Common.euint64_t));
    }

    /**
    * @dev Generates a random encrypted 64-bit unsigned integer in the [0, upperBound) range.
    *      The upperBound must be a power of 2.
    */
    function randEuint64(uint64 upperBound) internal returns (euint64) {
      return euint64.wrap(Impl.randBounded(upperBound, Common.euint64_t));
    }

    /**
    * @dev Generates a random encrypted 128-bit unsigned integer.
    */
    function randEuint128() internal returns (euint128) {
      return euint128.wrap(Impl.rand(Common.euint128_t));
    }

    /**
    * @dev Generates a random encrypted 128-bit unsigned integer in the [0, upperBound) range.
    *      The upperBound must be a power of 2.
    */
    function randEuint128(uint128 upperBound) internal returns (euint128) {
      return euint128.wrap(Impl.randBounded(upperBound, Common.euint128_t));
    }

    /**
    * @dev Generates a random encrypted 256-bit unsigned integer.
    */
    function randEuint256() internal returns (euint256) {
      return euint256.wrap(Impl.rand(Common.euint256_t));
    }

    /**
    * @dev Generates a random encrypted 256-bit unsigned integer in the [0, upperBound) range.
    *      The upperBound must be a power of 2.
    */
    function randEuint256(uint256 upperBound) internal returns (euint256) {
      return euint256.wrap(Impl.randBounded(upperBound, Common.euint256_t));
    }

    /**
    * @dev Generates a random encrypted 512-bit unsigned integer.
    */
    function randEbytes64() internal returns (ebytes64) {
      return ebytes64.wrap(Impl.rand(Common.ebytes64_t));
    }

    /**
    * @dev Generates a random encrypted 1024-bit unsigned integer.
    */
    function randEbytes128() internal returns (ebytes128) {
      return ebytes128.wrap(Impl.rand(Common.ebytes128_t));
    }

    /**
    * @dev Generates a random encrypted 2048-bit unsigned integer.
    */
    function randEbytes256() internal returns (ebytes256) {
      return ebytes256.wrap(Impl.rand(Common.ebytes256_t));
    }

    /**
    * @dev Convert an inputHandle with corresponding inputProof to an encrypted eaddress.
    */
    function asEaddress(einput inputHandle, bytes memory inputProof) internal returns (eaddress) {
      return eaddress.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, Common.euint160_t));
    }

    /**
    * @dev Convert a plaintext value to an encrypted address.
    */
    function asEaddress(address value) internal returns (eaddress) {
        return eaddress.wrap(Impl.trivialEncrypt(uint160(value), Common.euint160_t));
    }

    
    /**
    * @dev Convert the given inputHandle and inputProof to an encrypted ebytes64 value.
    */
    function asEbytes64(einput inputHandle, bytes memory inputProof) internal returns (ebytes64) {
      return ebytes64.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, Common.ebytes64_t));
    }

    /**
    * @dev Left-pad a bytes array with zeros such that it becomes of length 64.
    */
    function padToBytes64(bytes memory input) internal pure returns (bytes memory) {
      uint256 inputLength = input.length;

      if (inputLength > 64) {
          revert InputLengthAbove64Bytes(inputLength);
      }

      bytes memory result = new bytes(64);
      uint256 paddingLength = 64 - inputLength;

      for (uint256 i = 0; i < paddingLength; i++) {
          result[i] = 0;
      }
      for (uint256 i = 0; i < inputLength; i++) {
          result[paddingLength + i] = input[i];
      }
      return result;
    }

    /**
    * @dev Convert a plaintext value - must be a bytes array of size 64 - to an encrypted Bytes64.
    */
    function asEbytes64(bytes memory value) internal returns (ebytes64) {
        return ebytes64.wrap(Impl.trivialEncrypt(value, Common.ebytes64_t));
    }

    /**
    * @dev Convert the given inputHandle and inputProof to an encrypted ebytes128 value.
    */
    function asEbytes128(einput inputHandle, bytes memory inputProof) internal returns (ebytes128) {
      return ebytes128.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, Common.ebytes128_t));
    }

    /**
    * @dev Left-pad a bytes array with zeros such that it becomes of length 128.
    */
    function padToBytes128(bytes memory input) internal pure returns (bytes memory) {
      uint256 inputLength = input.length;

      if (inputLength > 128) {
          revert InputLengthAbove128Bytes(inputLength);    
      }

      bytes memory result = new bytes(128);
      uint256 paddingLength = 128 - inputLength;
      for (uint256 i = 0; i < paddingLength; i++) {
          result[i] = 0;
      }
      for (uint256 i = 0; i < inputLength; i++) {
          result[paddingLength + i] = input[i];
      }
      return result;
    }

    /**
    * @dev Convert a plaintext value - must be a bytes array of size 128 - to an encrypted Bytes128.
    */
    function asEbytes128(bytes memory value) internal returns (ebytes128) {
        return ebytes128.wrap(Impl.trivialEncrypt(value, Common.ebytes128_t));
    }
    
    /**
    * @dev Convert the given inputHandle and inputProof to an encrypted ebytes256 value.
    */
    function asEbytes256(einput inputHandle, bytes memory inputProof) internal returns (ebytes256) {
      return ebytes256.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, Common.ebytes256_t));
    }

    /**
    * @dev Left-pad a bytes array with zeros such that it becomes of length 256.
    */
    function padToBytes256(bytes memory input) internal pure returns (bytes memory) {
      uint256 inputLength = input.length;

      if (inputLength > 256) {
          revert InputLengthAbove256Bytes(inputLength);    
      }

      bytes memory result = new bytes(256);
      uint256 paddingLength = 256 - inputLength;
      for (uint256 i = 0; i < paddingLength; i++) {
          result[i] = 0;
      }
      for (uint256 i = 0; i < inputLength; i++) {
          result[paddingLength + i] = input[i];
      }
      return result;
    }

    /**
    * @dev Convert a plaintext value - must be a bytes array of size 256 - to an encrypted Bytes256.
    */
    function asEbytes256(bytes memory value) internal returns (ebytes256) {
        return ebytes256.wrap(Impl.trivialEncrypt(value, Common.ebytes256_t));
    }

    /**
    * @dev Returns true if the encrypted address is initialized and false otherwise.
    */
    function isInitialized(eaddress v) internal pure returns (bool) {
        return eaddress.unwrap(v) != 0;
    }

    /**
    * @dev Returns true if the encrypted value is initialized and false otherwise.
    */
    function isInitialized(ebytes64 v) internal pure returns (bool) {
        return ebytes64.unwrap(v) != 0;
    }

    /**
    * @dev Returns true if the encrypted value is initialized and false otherwise.
    */
    function isInitialized(ebytes128 v) internal pure returns (bool) {
        return ebytes128.unwrap(v) != 0;
    }
    
    /**
    * @dev Returns true if the encrypted value is initialized and false otherwise.
    */
    function isInitialized(ebytes256 v) internal pure returns (bool) {
        return ebytes256.unwrap(v) != 0;
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(ebool a, ebool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.eq(ebool.unwrap(a), ebool.unwrap(b), false));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(ebool a, ebool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.ne(ebool.unwrap(a), ebool.unwrap(b), false));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(ebool a, bool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        uint256 bProc = b?1:0;
        return ebool.wrap(Impl.eq(ebool.unwrap(a), bytes32(bProc), true));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(bool b, ebool a) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        uint256 bProc = b?1:0;
        return ebool.wrap(Impl.eq(ebool.unwrap(a), bytes32(bProc), true));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(ebool a, bool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        uint256 bProc = b?1:0;
        return ebool.wrap(Impl.ne(ebool.unwrap(a), bytes32(bProc), true));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(bool b, ebool a) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        uint256 bProc = b?1:0;
        return ebool.wrap(Impl.ne(ebool.unwrap(a), bytes32(bProc), true));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(eaddress a, eaddress b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), eaddress.unwrap(b), false));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(eaddress a, eaddress b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), eaddress.unwrap(b), false));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(eaddress a, address b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        bytes32 bProc = bytes32(uint256(uint160(b)));
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), bProc, true));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(address b, eaddress a) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        bytes32 bProc = bytes32(uint256(uint160(b)));
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), bProc, true));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(eaddress a, address b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        bytes32 bProc = bytes32(uint256(uint160(b)));
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), bProc, true));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(address b, eaddress a) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        bytes32 bProc = bytes32(uint256(uint160(b)));
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), bProc, true));
    }

    /**
    * @dev If 'control''s value is 'true', the result has the same value as 'a'.
    *      If 'control''s value is 'false', the result has the same value as 'b'.
    */
    function select(ebool control, ebool a, ebool b) internal returns (ebool) {
        return ebool.wrap(Impl.select(ebool.unwrap(control), ebool.unwrap(a), ebool.unwrap(b)));
    }

    /**
    * @dev If 'control''s value is 'true', the result has the same value as 'a'.
    *      If 'control''s value is 'false', the result has the same value as 'b'.
    */
    function select(ebool control, eaddress a, eaddress b) internal returns (eaddress) {
        return eaddress.wrap(Impl.select(ebool.unwrap(control), eaddress.unwrap(a), eaddress.unwrap(b)));
    }

    /**
    * @dev If 'control''s value is 'true', the result has the same value as 'a'.
    *      If 'control''s value is 'false', the result has the same value as 'b'.
    */
    function select(ebool control, ebytes64 a, ebytes64 b) internal returns (ebytes64) {
        return ebytes64.wrap(Impl.select(ebool.unwrap(control), ebytes64.unwrap(a), ebytes64.unwrap(b)));
    }

    /**
    * @dev If 'control''s value is 'true', the result has the same value as 'a'.
    *      If 'control''s value is 'false', the result has the same value as 'b'.
    */
    function select(ebool control, ebytes128 a, ebytes128 b) internal returns (ebytes128) {
        return ebytes128.wrap(Impl.select(ebool.unwrap(control), ebytes128.unwrap(a), ebytes128.unwrap(b)));
    }
        
    /**
    * @dev If 'control''s value is 'true', the result has the same value as 'a'.
    *      If 'control''s value is 'false', the result has the same value as 'b'.
    */
    function select(ebool control, ebytes256 a, ebytes256 b) internal returns (ebytes256) {
        return ebytes256.wrap(Impl.select(ebool.unwrap(control), ebytes256.unwrap(a), ebytes256.unwrap(b)));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(ebytes64 a, ebytes64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes64(padToBytes64(hex''));
        }
        if (!isInitialized(b)) {
            b = asEbytes64(padToBytes64(hex''));
        }
        return ebool.wrap(Impl.eq(ebytes64.unwrap(a), ebytes64.unwrap(b), false));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(ebytes64 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes64(padToBytes64(hex''));
        }
        return ebool.wrap(Impl.eq(ebytes64.unwrap(a), b, true));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(bytes memory a, ebytes64 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes64(padToBytes64(hex''));
        }
        return ebool.wrap(Impl.eq(ebytes64.unwrap(b), a, true));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(ebytes64 a, ebytes64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes64(padToBytes64(hex''));
        }
        if (!isInitialized(b)) {
            b = asEbytes64(padToBytes64(hex''));
        }
        return ebool.wrap(Impl.ne(ebytes64.unwrap(a), ebytes64.unwrap(b), false));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(ebytes64 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes64(padToBytes64(hex''));
        }
        return ebool.wrap(Impl.ne(ebytes64.unwrap(a), b, true));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(bytes memory a, ebytes64 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes64(padToBytes64(hex''));
        }
        return ebool.wrap(Impl.ne(ebytes64.unwrap(b), a, true));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(ebytes128 a, ebytes128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes128(padToBytes128(hex''));
        }
        if (!isInitialized(b)) {
            b = asEbytes128(padToBytes128(hex''));
        }
        return ebool.wrap(Impl.eq(ebytes128.unwrap(a), ebytes128.unwrap(b), false));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(ebytes128 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes128(padToBytes128(hex''));
        }
        return ebool.wrap(Impl.eq(ebytes128.unwrap(a), b, true));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(bytes memory a, ebytes128 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes128(padToBytes128(hex''));
        }
        return ebool.wrap(Impl.eq(ebytes128.unwrap(b), a, true));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(ebytes128 a, ebytes128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes128(padToBytes128(hex''));
        }
        if (!isInitialized(b)) {
            b = asEbytes128(padToBytes128(hex''));
        }
        return ebool.wrap(Impl.ne(ebytes128.unwrap(a), ebytes128.unwrap(b), false));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(ebytes128 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes128(padToBytes128(hex''));
        }
        return ebool.wrap(Impl.ne(ebytes128.unwrap(a), b, true));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(bytes memory a, ebytes128 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes128(padToBytes128(hex''));
        }
        return ebool.wrap(Impl.ne(ebytes128.unwrap(b), a, true));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(ebytes256 a, ebytes256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes256(padToBytes256(hex''));
        }
        if (!isInitialized(b)) {
            b = asEbytes256(padToBytes256(hex''));
        }
        return ebool.wrap(Impl.eq(ebytes256.unwrap(a), ebytes256.unwrap(b), false));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(ebytes256 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes256(padToBytes256(hex''));
        }
        return ebool.wrap(Impl.eq(ebytes256.unwrap(a), b, true));
    }

    /** 
     * @dev Evaluates eq(a, b) and returns the result.
     */
    function eq(bytes memory a, ebytes256 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes256(padToBytes256(hex''));
        }
        return ebool.wrap(Impl.eq(ebytes256.unwrap(b), a, true));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(ebytes256 a, ebytes256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes256(padToBytes256(hex''));
        }
        if (!isInitialized(b)) {
            b = asEbytes256(padToBytes256(hex''));
        }
        return ebool.wrap(Impl.ne(ebytes256.unwrap(a), ebytes256.unwrap(b), false));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(ebytes256 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes256(padToBytes256(hex''));
        }
        return ebool.wrap(Impl.ne(ebytes256.unwrap(a), b, true));
    }

    /** 
     * @dev Evaluates ne(a, b) and returns the result.
     */
    function ne(bytes memory a, ebytes256 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes256(padToBytes256(hex''));
        }
        return ebool.wrap(Impl.ne(ebytes256.unwrap(b), a, true));
    }
`;
  return result;
}

function implCustomMethods(): string {
  return `
    /**
    * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
    *         If 'control's value is 'false', the result has the same value as 'ifFalse'.
    */
    function select(bytes32 control, bytes32 ifTrue, bytes32 ifFalse) internal returns (bytes32 result) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
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
        uint8 toType
    ) internal returns (bytes32 result) {
      FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).verifyCiphertext(inputHandle, msg.sender, inputProof, bytes1(toType));
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
        uint8 toType
    ) internal returns (bytes32 result) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).cast(ciphertext, bytes1(toType));
    }

    /**
     * @notice          Does trivial encryption.
     * @param value     Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(
        uint256 value,
        uint8 toType
    ) internal returns (bytes32 result) {
      FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).trivialEncrypt(value, bytes1(toType));
    }

    /**
     * @notice          Does trivial encryption.
     * @param value     Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(
      bytes memory value,
      uint8 toType
    ) internal returns (bytes32 result) {
      FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).trivialEncrypt(value, bytes1(toType));
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
      FHEVMConfigStruct storage $ = getFHEVMConfig();
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
      FHEVMConfigStruct storage $ = getFHEVMConfig();
      result = ITFHEExecutor($.TFHEExecutorAddress).fheNe(lhs, rhs, scalarByte);
  }

    function rand(uint8 randType) internal returns(bytes32 result) {
      FHEVMConfigStruct storage $ = getFHEVMConfig();
      result = ITFHEExecutor($.TFHEExecutorAddress).fheRand(bytes1(randType));
    }

    function randBounded(uint256 upperBound, uint8 randType) internal returns(bytes32 result) {
      FHEVMConfigStruct storage $ = getFHEVMConfig();
      result = ITFHEExecutor($.TFHEExecutorAddress).fheRandBounded(upperBound, bytes1(randType));
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
      FHEVMConfigStruct storage $ = getFHEVMConfig();
      IACL($.ACLAddress).allowTransient(handle, account);
    }


    /**
     * @notice              Allows the use of handle for the address account.
     * @dev                 The caller must be allowed to use handle for allow() to succeed. If not, allow() reverts.
     * @param handle        Handle.
     * @param account       Address of the account.
     */
    function allow(bytes32 handle, address account) internal {
      FHEVMConfigStruct storage $ = getFHEVMConfig();
      IACL($.ACLAddress).allow(handle, account);
    }

    /**
     * @dev This function removes the transient allowances in the ACL, which could be useful for integration
     *      with Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor.
     */
    function cleanTransientStorageACL() internal {
      FHEVMConfigStruct storage $ = getFHEVMConfig();
      IACL($.ACLAddress).cleanTransientStorage();
    }


    /**
     * @dev This function removes the transient proofs in the InputVerifier, which could be useful for integration 
     *      with Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor.
     */
    function cleanTransientStorageInputVerifier() internal {
      FHEVMConfigStruct storage $ = getFHEVMConfig();
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
      FHEVMConfigStruct storage $ = getFHEVMConfig();
      return IACL($.ACLAddress).isAllowed(handle, account);
    }
    `;
}
