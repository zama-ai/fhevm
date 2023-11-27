import { assert } from 'console';

import { CodegenContext, Operator, OperatorArguments, ReturnType } from './common';
import { ArgumentType, OverloadSignature } from './testgen';

export function commonSolLib(): string {
  return `
type ebool is uint256;
type euint8 is uint256;
type euint16 is uint256;
type euint32 is uint256;

library Common {
    // Values used to communicate types to the runtime.
    uint8 internal constant ebool_t = 0;
    uint8 internal constant euint8_t = 0;
    uint8 internal constant euint16_t = 1;
    uint8 internal constant euint32_t = 2;
}
`;
}

function binaryOperatorImpl(op: Operator, isScalar: boolean, isEncrypted: boolean): string {
  const fname = operatorFheLibFunction(op);
  const scalarArg = isScalar && isEncrypted ? ', bool scalar' : '';
  const scalarByte = isScalar ? '0x01' : '0x00';
  const scalarSection =
    isScalar && isEncrypted
      ? `bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }`
      : `bytes1 scalarByte = ${scalarByte};`;
  return (
    `
    function ${op.name}(uint256 lhs, uint256 rhs${scalarArg}) internal pure returns (uint256 result) {
        ${scalarSection}
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).${fname}(lhs, rhs, scalarByte);
    }` + '\n'
  );
}

export function implSol(ctx: CodegenContext, operators: Operator[]): string {
  const res: string[] = [];

  const fheLibInterface = generateImplFhevmLibInterface(operators);

  res.push(`
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.19;

${fheLibInterface}

address constant EXT_TFHE_LIBRARY = address(${ctx.libFheAddress});

library Impl {
    // 32 bytes for the 'byte' type header + 48 bytes for the NaCl anonymous
    // box overhead + 4 bytes for the plaintext value.
    uint256 constant reencryptedSize = 32 + 48 + 4;

    // 32 bytes for the 'byte' header + 16553 bytes of key data.
    uint256 constant fhePubKeySize = 32 + 16553;

`);

  operators.forEach((op) => {
    switch (op.arguments) {
      case OperatorArguments.Binary:
        res.push(binaryOperatorImpl(op, op.hasScalar, op.hasEncrypted));
        break;
      case OperatorArguments.Unary:
        res.push(unaryOperatorImpl(op));
        break;
    }
  });

  res.push(implCustomMethods(ctx));

  res.push('}\n');

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
  operators.forEach((op) => {
    let functionName = operatorFheLibFunction(op);
    const tail = 'external pure returns (uint256 result);';
    let functionArguments: string;
    switch (op.arguments) {
      case OperatorArguments.Binary:
        functionArguments = '(uint256 lhs, uint256 rhs, bytes1 scalarByte)';
        res.push(`  function ${functionName}${functionArguments} ${tail}`);
        break;
      case OperatorArguments.Unary:
        functionArguments = '(uint256 ct)';
        res.push(`  function ${functionName}${functionArguments} ${tail}`);
        break;
    }
  });

  res.push(fheLibCustomInterfaceFunctions());

  res.push('}');

  return res.join('\n');
}

function fheLibCustomInterfaceFunctions(): string {
  return `
    function reencrypt(uint256 ct, uint256 publicKey) external view returns (bytes memory);
    function fhePubKey(bytes1 fromLib) external view returns (bytes memory result);
    function verifyCiphertext(bytes memory input) external pure returns (uint256 result);
    function cast(uint256 ct, bytes1 toType) external pure returns (uint256 result);
    function trivialEncrypt(uint256 ct, bytes1 toType) external pure returns (uint256 result);
    function decrypt(uint256 ct) external view returns (uint256 result);
    function fheRand(bytes1 inp) external view returns (uint256 result);
  `;
}

export function tfheSol(
  ctx: CodegenContext,
  operators: Operator[],
  supportedBits: number[],
): [string, OverloadSignature[]] {
  const signatures: OverloadSignature[] = [];
  const res: string[] = [];

  res.push(`// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity 0.8.19;

${commonSolLib()}

import "./Impl.sol";

library TFHE {
`);

  supportedBits.forEach((b) => {
    res.push(`     euint${b} constant NIL${b} = euint${b}.wrap(0);
`);
  });
  supportedBits.forEach((b) => {
    res.push(`
    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(euint${b} v) internal pure returns (bool) {
        return euint${b}.unwrap(v) != 0;
    }
`);
  });

  supportedBits.forEach((lhsBits) => {
    supportedBits.forEach((rhsBits) => {
      operators.forEach((operator) => res.push(tfheEncryptedOperator(lhsBits, rhsBits, operator, signatures)));
    });
    operators.forEach((operator) => res.push(tfheScalarOperator(lhsBits, lhsBits, operator, signatures)));
  });

  // TODO: Decide whether we want to have mixed-inputs for CMUX
  supportedBits.forEach((bits) => res.push(tfheCmux(bits)));
  supportedBits.forEach((outputBits) => {
    supportedBits.forEach((inputBits) => {
      res.push(tfheAsEboolCustomCast(inputBits, outputBits));
    });
    res.push(tfheAsEboolUnaryCast(outputBits));
  });
  supportedBits.forEach((bits) => res.push(tfheUnaryOperators(bits, operators, signatures)));
  supportedBits.forEach((bits) => res.push(tfheCustomUnaryOperators(bits, signatures)));

  res.push(tfheCustomMethods(ctx));

  res.push('}\n');

  supportedBits.forEach((bits) => {
    operators.forEach((op) => {
      res.push(tfheSolidityOperator(bits, op, signatures));
    });
  });

  return [res.join(''), signatures];
}

function tfheSolidityOperator(bits: number, operator: Operator, os: OverloadSignature[]): string {
  const res: string[] = [];

  if (operator.hasEncrypted && operator.binarySolidityOperator) {
    const fname = `tfheBinaryOperator${capitalizeFirstLetter(operator.name)}${bits}`;
    res.push(`
      using {${fname} as ${operator.binarySolidityOperator}} for euint${bits} global;
    `);

    res.push(`
      function ${fname}(euint${bits} lhs, euint${bits} rhs) pure returns (euint${bits}) {
        return TFHE.${operator.name}(lhs, rhs);
      }
    `);

    os.push({
      binaryOperator: operator.binarySolidityOperator,
      arguments: [
        { type: ArgumentType.EUint, bits },
        { type: ArgumentType.EUint, bits },
      ],

      name: `bin_op_${operator.name}`,
      returnType: { type: ArgumentType.EUint, bits },
    });
  }

  if (operator.hasEncrypted && operator.unarySolidityOperator) {
    const fname = `tfheUnaryOperator${capitalizeFirstLetter(operator.name)}${bits}`;
    res.push(`
      using {${fname} as ${operator.unarySolidityOperator}} for euint${bits} global;
    `);

    res.push(`
      function ${fname}(euint${bits} input) pure returns (euint${bits}) {
        return TFHE.${operator.name}(input);
      }
    `);

    os.push({
      unaryOperator: operator.unarySolidityOperator,
      arguments: [{ type: ArgumentType.EUint, bits }],

      name: `unary_op_${operator.name}`,
      returnType: { type: ArgumentType.EUint, bits },
    });
  }

  return res.join('');
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
  const boolCastNeeded = outputBits > 8 && operator.returnType == ReturnType.Ebool;
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
  if (boolCastNeeded) {
    implExpression = `Impl.cast(${implExpression}, Common.ebool_t)`;
  }
  signatures.push({
    name: operator.name,
    arguments: [
      { type: ArgumentType.EUint, bits: lhsBits },
      { type: ArgumentType.EUint, bits: rhsBits },
    ],
    returnType: { type: returnTypeOverload, bits: outputBits },
  });
  res.push(`
    // Evaluate ${operator.name}(a, b) and return the result.
    function ${operator.name}(euint${lhsBits} a, euint${rhsBits} b) internal pure returns (${returnType}) {
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
  const boolCastNeeded = outputBits > 8 && operator.returnType == ReturnType.Ebool;
  const returnType =
    operator.returnType == ReturnType.Uint
      ? `euint${outputBits}`
      : operator.returnType == ReturnType.Ebool
      ? `ebool`
      : assert(false, 'Unknown return type');
  const returnTypeOverload = operator.returnType == ReturnType.Uint ? ArgumentType.EUint : ArgumentType.Ebool;
  var scalarFlag = operator.hasEncrypted && operator.hasScalar ? ', true' : '';
  const leftOpName = operator.leftScalarInvertOp ?? operator.name;
  var implExpressionA = `Impl.${operator.name}(euint${outputBits}.unwrap(a), uint256(b)${scalarFlag})`;
  var implExpressionB = `Impl.${leftOpName}(euint${outputBits}.unwrap(b), uint256(a)${scalarFlag})`;
  var maybeEncryptLeft = '';
  if (operator.leftScalarEncrypt) {
    // workaround until tfhe-rs left scalar support:
    // do the trivial encryption and preserve order of operations
    scalarFlag = ', false';
    maybeEncryptLeft = `euint${outputBits} aEnc = asEuint${outputBits}(a);`;
    implExpressionB = `Impl.${leftOpName}(euint${outputBits}.unwrap(aEnc), euint${outputBits}.unwrap(b)${scalarFlag})`;
  }
  if (boolCastNeeded) {
    implExpressionA = `Impl.cast(${implExpressionA}, Common.ebool_t)`;
    implExpressionB = `Impl.cast(${implExpressionB}, Common.ebool_t)`;
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
    // Evaluate ${operator.name}(a, b) and return the result.
    function ${operator.name}(euint${lhsBits} a, uint${rhsBits} b) internal pure returns (${returnType}) {
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

    // Evaluate ${operator.name}(a, b) and return the result.
    function ${operator.name}(uint${lhsBits} a, euint${rhsBits} b) internal pure returns (${returnType}) {
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

function tfheCmux(inputBits: number): string {
  if (inputBits == 8) {
    return `
    // If 'control''s value is 'true', the result has the same value as 'a'.
    // If 'control''s value is 'false', the result has the same value as 'b'.
    function cmux(ebool control, euint${inputBits} a, euint${inputBits} b) internal pure returns (euint${inputBits}) {
        return euint${inputBits}.wrap(Impl.cmux(ebool.unwrap(control), euint${inputBits}.unwrap(a), euint${inputBits}.unwrap(b)));
    }`;
  }

  return `
    // If 'control's value is 'true', the result has the same value as 'a'.
    // If 'control's value is 'false', the result has the same value as 'b'.
    function cmux(ebool control, euint${inputBits} a, euint${inputBits} b) internal pure returns (euint${inputBits}) {
        euint${inputBits} ctrl = asEuint${inputBits}(asEuint8(control));
        return euint${inputBits}.wrap(Impl.cmux(euint${inputBits}.unwrap(ctrl), euint${inputBits}.unwrap(a), euint${inputBits}.unwrap(b)));
    }`;
}

function tfheAsEboolCustomCast(inputBits: number, outputBits: number): string {
  if (inputBits == outputBits) {
    return '';
  }

  return `
    // Cast an encrypted integer from euint${inputBits} to euint${outputBits}.
    function asEuint${outputBits}(euint${inputBits} value) internal pure returns (euint${outputBits}) {
        return euint${outputBits}.wrap(Impl.cast(euint${inputBits}.unwrap(value), Common.euint${outputBits}_t));
    }
    `;
}

function tfheAsEboolUnaryCast(bits: number): string {
  const res: string[] = [];
  res.push(`
    // Cast an encrypted integer from euint${bits} to ebool.
    function asEbool(euint${bits} value) internal pure returns (ebool) {
        return ne(value, 0);
    }
    `);

  if (bits == 8) {
    res.push(`
    // Convert a serialized 'ciphertext' to an encrypted boolean.
    function asEbool(bytes memory ciphertext) internal pure returns (ebool) {
        return asEbool(asEuint8(ciphertext));
    }

    // Convert a plaintext boolean to an encrypted boolean.
    function asEbool(bool value) internal pure returns (ebool) {
        if (value) {
            return asEbool(asEuint8(1));
        } else {
            return asEbool(asEuint8(0));
        }
    }

    // Converts an 'ebool' to an 'euint8'.
    function asEuint8(ebool b) internal pure returns (euint8) {
        return euint8.wrap(ebool.unwrap(b));
    }

    // Evaluate and(a, b) and return the result.
    function and(ebool a, ebool b) internal pure returns (ebool) {
        return asEbool(and(asEuint8(a), asEuint8(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(ebool a, ebool b) internal pure returns (ebool) {
        return asEbool(or(asEuint8(a), asEuint8(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(ebool a, ebool b) internal pure returns (ebool) {
        return asEbool(xor(asEuint8(a), asEuint8(b)));
    }

    function not(ebool a) internal pure returns (ebool) {
        return asEbool(and(not(asEuint8(a)), asEuint8(1)));
    }
    
    // If 'control''s value is 'true', the result has the same value as 'a'.
    // If 'control''s value is 'false', the result has the same value as 'b'.
    function cmux(ebool cond, ebool a, ebool b) internal pure returns (ebool) {
        return asEbool(cmux(cond, asEuint8(a), asEuint8(b)));
    }
    `);
  } else {
    res.push(`
    // Converts an 'ebool' to an 'euint${bits}'.
    function asEuint${bits}(ebool b) internal pure returns (euint${bits}) {
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
        function ${op.name}(euint${bits} value) internal pure returns (euint${bits}) {
            return euint${bits}.wrap(Impl.${op.name}(euint${bits}.unwrap(value)));
        }
      `);
    }
  });

  return res.join('\n');
}

function tfheCustomUnaryOperators(bits: number, signatures: OverloadSignature[]): string {
  return `
    // Convert a serialized 'ciphertext' to an encrypted euint${bits} integer.
    function asEuint${bits}(bytes memory ciphertext) internal pure returns (euint${bits}) {
        return euint${bits}.wrap(Impl.verify(ciphertext, Common.euint${bits}_t));
    }

    // Convert a plaintext value to an encrypted euint${bits} integer.
    function asEuint${bits}(uint256 value) internal pure returns (euint${bits}) {
        return euint${bits}.wrap(Impl.trivialEncrypt(value, Common.euint${bits}_t));
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint${bits} ciphertext.
    function reencrypt(euint${bits} value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint${bits}.unwrap(value), publicKey);
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // If 'value' is not initialized, the returned value will contain the 'defaultValue' constant.
    // Return a serialized euint${bits} ciphertext.
    function reencrypt(euint${bits} value, bytes32 publicKey, uint${bits} defaultValue) internal view returns (bytes memory reencrypted) {
        if (euint${bits}.unwrap(value) != 0) {
            return Impl.reencrypt(euint${bits}.unwrap(value), publicKey);
        } else {
            return Impl.reencrypt(euint${bits}.unwrap(asEuint${bits}(defaultValue)), publicKey);
        }
    }

    // Decrypts the encrypted 'value'.
    function decrypt(euint${bits} value) internal view returns (uint${bits}) {
        return uint${bits}(Impl.decrypt(euint${bits}.unwrap(value)));
    }
    `;
}

function unaryOperatorImpl(op: Operator): string {
  let fname = operatorFheLibFunction(op);
  return `
    function ${op.name}(uint256 ct) internal pure returns (uint256 result) {
      result = FhevmLib(address(EXT_TFHE_LIBRARY)).${fname}(ct);
    }
  `;
}

function tfheCustomMethods(ctx: CodegenContext): string {
  return `
    // Optimistically require that 'b' is true.
    //
    // This function does not evaluate 'b' at the time of the call.
    // Instead, it accumulates all optimistic requires and evaluates a single combined
    // require at the end of the transaction. A side effect of this mechanism
    // is that a method call with a failed optimistic require will always incur the full
    // gas cost, as if all optimistic requires were true. Yet, the transaction will be
    // reverted at the end if any of the optimisic requires were false.
    //
    // Exceptions to above rule are reencryptions and decryptions via
    // TFHE.reencrypt() and TFHE.decrypt(), respectively. If either of them
    // are encountered and if optimistic requires have been used before in the
    // txn, the optimisic requires will be immediately evaluated. Rationale is
    // that we want to avoid decrypting or reencrypting a value if the txn is about
    // to fail and be reverted anyway at the end. Checking immediately and reverting on the spot
    // would avoid unnecessary decryptions.
    //
    // The benefit of optimistic requires is that they are faster than non-optimistic ones,
    // because there is a single call to the decryption oracle per transaction, irrespective
    // of how many optimistic requires were used.
    function optReq(ebool b) internal view {
        Impl.optReq(ebool.unwrap(b));
    }

    // Decrypts the encrypted 'value'.
    function decrypt(ebool value) internal view returns (bool) {
        return (Impl.decrypt(ebool.unwrap(value)) != 0);
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint8 value.
    function reencrypt(ebool value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(ebool.unwrap(value), publicKey);
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint8 value.
    // If 'value' is not initialized, the returned value will contain the 'defaultValue' constant.
    function reencrypt(ebool value, bytes32 publicKey, bool defaultValue) internal view returns (bytes memory reencrypted) {
        if (ebool.unwrap(value) != 0) {
            return Impl.reencrypt(ebool.unwrap(value), publicKey);
        } else {
            return Impl.reencrypt(ebool.unwrap(asEbool(defaultValue)), publicKey);
        }
    }

    // Returns the network public FHE key.
    function fhePubKey() internal view returns (bytes memory) {
        return Impl.fhePubKey();
    }

    // Generates a random encrypted 8-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint8() internal view returns (euint8) {
      return euint8.wrap(Impl.rand(Common.euint8_t));
    }

    // Generates a random encrypted 16-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint16() internal view returns (euint16) {
      return euint16.wrap(Impl.rand(Common.euint16_t));
    }

    // Generates a random encrypted 32-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint32() internal view returns (euint32) {
      return euint32.wrap(Impl.rand(Common.euint32_t));
    }
`;
}

function implCustomMethods(ctx: CodegenContext): string {
  return `
    // If 'control's value is 'true', the result has the same value as 'ifTrue'.
    // If 'control's value is 'false', the result has the same value as 'ifFalse'.
    function cmux(uint256 control, uint256 ifTrue, uint256 ifFalse) internal pure returns (uint256 result) {
        // result = (ifTrue - ifFalse) * control + ifFalse
        uint256 subOutput = FhevmLib(address(EXT_TFHE_LIBRARY)).fheSub(ifTrue, ifFalse, bytes1(0x00));
        uint256 mulOutput = FhevmLib(address(EXT_TFHE_LIBRARY)).fheMul(control, subOutput, bytes1(0x00));
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheAdd(mulOutput, ifFalse, bytes1(0x00));
    }

    // We do assembly here because ordinary call will emit extcodesize check which is zero for our precompiles
    // and revert the transaction because we don't return any data for this precompile method
    function optReq(uint256 ciphertext) internal view {
        bytes memory input = abi.encodeWithSignature("optimisticRequire(uint256)", ciphertext);
        uint256 inputLen = input.length;

        // Call the optimistic require method in precompile.
        address precompile = EXT_TFHE_LIBRARY;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, 0, 0)) {
                revert(0, 0)
            }
        }
    }

    function reencrypt(uint256 ciphertext, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return FhevmLib(address(EXT_TFHE_LIBRARY)).reencrypt(ciphertext, uint256(publicKey));
    }

    function fhePubKey() internal view returns (bytes memory key) {
        // Set a byte value of 1 to signal the call comes from the library.
        key = FhevmLib(address(EXT_TFHE_LIBRARY)).fhePubKey(bytes1(0x01));
    }

    function verify(
        bytes memory _ciphertextBytes,
        uint8 _toType
    ) internal pure returns (uint256 result) {
        bytes memory input = bytes.concat(_ciphertextBytes, bytes1(_toType));
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).verifyCiphertext(input);
    }

    function cast(
        uint256 ciphertext,
        uint8 toType
    ) internal pure returns (uint256 result) {
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).cast(ciphertext, bytes1(toType));
    }

    function trivialEncrypt(
        uint256 value,
        uint8 toType
    ) internal pure returns (uint256 result) {
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).trivialEncrypt(value, bytes1(toType));
    }

    function decrypt(uint256 ciphertext) internal view returns(uint256 result) {
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).decrypt(ciphertext);
    }

    function rand(uint8 randType) internal view returns(uint256 result) {
      result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheRand(bytes1(randType));
    }
    `;
}
