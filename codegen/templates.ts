import { assert } from 'console';

import { Operator, OperatorArguments, Precompile, ReturnType } from './common';
import { ArgumentType, OverloadSignature } from './testgen';

export function commonSolHeader(): string {
  return `
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

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
  const scalarArg = isScalar && isEncrypted ? ', bool scalar' : '';
  const scalarByte = isScalar ? '0x01' : '0x00';
  const scalarSection =
    isScalar && isEncrypted
      ? `bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);`
      : `bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), bytes1(${scalarByte}));`;
  return (
    `
    function ${op.name}(uint256 lhs, uint256 rhs${scalarArg}) internal view returns (uint256 result) {
        ${scalarSection}
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;
        // Call the ${op.name} precompile.
        uint256 precompile = Precompiles.${op.precompileName};
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }` + '\n'
  );
}

export function implSol(operators: Operator[]): string {
  const res: string[] = [];

  res.push(`
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "./Common.sol";
import "./Precompiles.sol";

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

  res.push(implCustomMethods());

  res.push('}\n');

  return res.join('');
}

export function tfheSol(operators: Operator[], supportedBits: number[]): [string, OverloadSignature[]] {
  const signatures: OverloadSignature[] = [];
  const res: string[] = [];

  res.push(`// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "./Common.sol";
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
  supportedBits.forEach((bits) => res.push(tfheCustomUnaryOperators(bits)));

  res.push(tfheCustomMethods());

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
  var implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(${leftExpr}), euint${outputBits}.unwrap(${rightExpr})${scalarFlag})`;
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
    function ${operator.name}(euint${lhsBits} a, euint${rhsBits} b) internal view returns (${returnType}) {
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
    function ${operator.name}(euint${lhsBits} a, uint${rhsBits} b) internal view returns (${returnType}) {
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
    function ${operator.name}(uint${lhsBits} a, euint${rhsBits} b) internal view returns (${returnType}) {
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
    function cmux(ebool control, euint${inputBits} a, euint${inputBits} b) internal view returns (euint${inputBits}) {
        return euint${inputBits}.wrap(Impl.cmux(ebool.unwrap(control), euint${inputBits}.unwrap(a), euint${inputBits}.unwrap(b)));
    }`;
  }

  return `
    // If 'control's value is 'true', the result has the same value as 'a'.
    // If 'control's value is 'false', the result has the same value as 'b'.
    function cmux(ebool control, euint${inputBits} a, euint${inputBits} b) internal view returns (euint${inputBits}) {
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
    function asEuint${outputBits}(euint${inputBits} value) internal view returns (euint${outputBits}) {
        return euint${outputBits}.wrap(Impl.cast(euint${inputBits}.unwrap(value), Common.euint${outputBits}_t));
    }
    `;
}

function tfheAsEboolUnaryCast(bits: number): string {
  const res: string[] = [];
  res.push(`
    // Cast an encrypted integer from euint${bits} to ebool.
    function asEbool(euint${bits} value) internal view returns (ebool) {
        return ne(value, 0);
    }
    `);

  if (bits == 8) {
    res.push(`
    // Convert a serialized 'ciphertext' to an encrypted boolean.
    function asEbool(bytes memory ciphertext) internal view returns (ebool) {
        return asEbool(asEuint8(ciphertext));
    }

    // Convert a plaintext boolean to an encrypted boolean.
    function asEbool(bool value) internal view returns (ebool) {
        if (value) {
            return asEbool(asEuint8(1));
        } else {
            return asEbool(asEuint8(0));
        }
    }`);
  }

  return res.join('');
}

function tfheCustomUnaryOperators(bits: number): string {
  return `
    // Convert a serialized 'ciphertext' to an encrypted euint${bits} integer.
    function asEuint${bits}(bytes memory ciphertext) internal view returns (euint${bits}) {
        return euint${bits}.wrap(Impl.verify(ciphertext, Common.euint${bits}_t));
    }

    // Convert a plaintext value to an encrypted euint${bits} integer.
    function asEuint${bits}(uint256 value) internal view returns (euint${bits}) {
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

    // Return the negation of 'value'.
    function neg(euint${bits} value) internal view returns (euint${bits}) {
        return euint${bits}.wrap(Impl.neg(euint${bits}.unwrap(value)));
    }

    // Return '!value'.
    function not(euint${bits} value) internal view returns (euint${bits}) {
        return euint${bits}.wrap(Impl.not(euint${bits}.unwrap(value)));
    }
    `;
}

export function precompiles(precompiles: Precompile[]): string {
  const res: string[] = [];

  res.push(`// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

library Precompiles {
`);

  precompiles.forEach((op) => {
    res.push(`    uint256 public constant ${op.name} = ${op.code};
`);
  });

  res.push('}\n');

  return res.join('');
}

function unaryOperatorImpl(op: Operator): string {
  return `
    function ${op.name}(uint256 ct) internal view returns (uint256 result) {
        bytes32[1] memory input;
        input[0] = bytes32(ct);
        uint256 inputLen = 32;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the ${op.name} precompile.
        uint256 precompile = Precompiles.${op.precompileName};
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }
    `;
}

function tfheCustomMethods(): string {
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

    // Converts an 'ebool' to an 'euint8'.
    function asEuint8(ebool b) internal pure returns (euint8) {
        return euint8.wrap(ebool.unwrap(b));
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

function implCustomMethods(): string {
  return `
    // If 'control's value is 'true', the result has the same value as 'ifTrue'.
    // If 'control's value is 'false', the result has the same value as 'ifFalse'.
    function cmux(uint256 control, uint256 ifTrue, uint256 ifFalse) internal view returns (uint256 result) {
        // result = (ifTrue - ifFalse) * control + ifFalse
        bytes memory input = bytes.concat(bytes32(ifTrue), bytes32(ifFalse), bytes1(0x00));
        uint256 inputLen = input.length;

        bytes32[1] memory subOutput;
        uint256 outputLen = 32;

        // Call the sub precompile.
        uint256 precompile = Precompiles.Subtract;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, subOutput, outputLen)) {
                revert(0, 0)
            }
        }

        // Call the mul precompile.
        input = bytes.concat(bytes32(control), bytes32(subOutput[0]), bytes1(0x00));
        inputLen = input.length;
        precompile = Precompiles.Multiply;
        bytes32[1] memory mulOutput;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, mulOutput, outputLen)) {
                revert(0, 0)
            }
        }

        // Call the add precompile.
        input = bytes.concat(bytes32(mulOutput[0]), bytes32(ifFalse), bytes1(0x00));
        inputLen = input.length;
        precompile = Precompiles.Add;
        bytes32[1] memory addOutput;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, addOutput, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(addOutput[0]);
    }

    function optReq(uint256 ciphertext) internal view {
        bytes32[1] memory input;
        input[0] = bytes32(ciphertext);
        uint256 inputLen = 32;

        // Call the optimistic require precompile.
        uint256 precompile = Precompiles.OptimisticRequire;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, 0, 0)) {
                revert(0, 0)
            }
        }
    }

    function reencrypt(uint256 ciphertext, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        bytes32[2] memory input;
        input[0] = bytes32(ciphertext);
        input[1] = publicKey;
        uint256 inputLen = 64;

        reencrypted = new bytes(reencryptedSize);

        // Call the reencrypt precompile.
        uint256 precompile = Precompiles.Reencrypt;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, reencrypted, reencryptedSize)) {
                revert(0, 0)
            }
        }
    }

    function fhePubKey() internal view returns (bytes memory key) {
        // Set a byte value of 1 to signal the call comes from the library.
        bytes1[1] memory input;
        input[0] = 0x01;
        uint256 inputLen = 1;

        key = new bytes(fhePubKeySize);

        // Call the fhePubKey precompile.
        uint256 precompile = Precompiles.FhePubKey;
        assembly {
            if iszero(
                staticcall(
                    gas(),
                    precompile,
                    input,
                    inputLen,
                    key,
                    fhePubKeySize
                )
            ) {
                revert(0, 0)
            }
        }
    }

    function verify(
        bytes memory _ciphertextBytes,
        uint8 _toType
    ) internal view returns (uint256 result) {
        bytes memory input = bytes.concat(_ciphertextBytes, bytes1(_toType));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the verify precompile.
        uint256 precompile = Precompiles.Verify;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }
        result = uint256(output[0]);
    }

    function cast(
        uint256 ciphertext,
        uint8 toType
    ) internal view returns (uint256 result) {
        bytes memory input = bytes.concat(bytes32(ciphertext), bytes1(toType));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the cast precompile.
        uint256 precompile = Precompiles.Cast;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(
                staticcall(
                    gas(),
                    precompile,
                    add(input, 32),
                    inputLen,
                    output,
                    outputLen
                )
            ) {
                revert(0, 0)
            }
        }
        result = uint256(output[0]);
    }

    function trivialEncrypt(
        uint256 value,
        uint8 toType
    ) internal view returns (uint256 result) {
        bytes memory input = bytes.concat(bytes32(value), bytes1(toType));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the trivialEncrypt precompile.
        uint256 precompile = Precompiles.TrivialEncrypt;
        assembly {
            // jump over the 32-bit 'size' field of the 'bytes' data structure of the 'input' to read actual bytes
            if iszero(
                staticcall(
                    gas(),
                    precompile,
                    add(input, 32),
                    inputLen,
                    output,
                    outputLen
                )
            ) {
                revert(0, 0)
            }
        }
        result = uint256(output[0]);
    }

    function decrypt(uint256 ciphertext) internal view returns(uint256 result) {
        bytes32[1] memory input;
        input[0] = bytes32(ciphertext);
        uint256 inputLen = 32;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the decrypt precompile.
        uint256 precompile = Precompiles.Decrypt;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }
        // The output is a 32-byte buffer of a 256-bit big-endian unsigned integer.
        result = uint256(output[0]);
    }

    function rand(uint8 randType) internal view returns(uint256 result) {
      bytes32[1] memory input;
      input[0] = bytes1(randType);
      uint256 inputLen = 1;

      bytes32[1] memory output;
      uint256 outputLen = 32;

      // Call the rand precompile.
      uint256 precompile = Precompiles.Rand;
      assembly {
          if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
              revert(0, 0)
          }
      }
      result = uint256(output[0]);
    }
    `;
}
