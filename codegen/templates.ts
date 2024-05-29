import { assert } from 'console';

import { CodegenContext, Operator, OperatorArguments, ReturnType } from './common';
import { ArgumentType, OverloadSignature } from './testgen';
import { getUint } from './utils';

export function commonSolLib(): string {
  return `
type ebool is uint256;
type euint4 is uint256;
type euint8 is uint256;
type euint16 is uint256;
type euint32 is uint256;
type euint64 is uint256;
type eaddress is uint256;

library Common {
    // Values used to communicate types to the runtime.
    uint8 internal constant ebool_t = 0;
    uint8 internal constant euint4_t = 1;
    uint8 internal constant euint8_t = 2;
    uint8 internal constant euint16_t = 3;
    uint8 internal constant euint32_t = 4;
    uint8 internal constant euint64_t = 5;
    uint8 internal constant euint128_t = 6;
    uint8 internal constant euint160_t = 7;
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

pragma solidity ^0.8.20;

import "./TFHE.sol";

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
    function fheIfThenElse(uint256 control, uint256 ifTrue, uint256 ifFalse) external pure returns (uint256 result);
    function fheArrayEq(uint256[] memory lhs, uint256[] memory rhs) external pure returns (uint256 result);
    function fheRand(bytes1 randType) external view returns (uint256 result);
    function fheRandBounded(uint256 upperBound, bytes1 randType) external view returns (uint256 result);
  `;
}

export function tfheSol(
  ctx: CodegenContext,
  operators: Operator[],
  supportedBits: number[],
  mocked: boolean,
): [string, OverloadSignature[]] {
  const signatures: OverloadSignature[] = [];
  const res: string[] = [];

  res.push(`// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

${commonSolLib()}

import "./Impl.sol";

library TFHE {
`);

  supportedBits.forEach((b) => {
    res.push(`     euint${b} constant NIL${b} = euint${b}.wrap(0);
`);
  });
  if (mocked) {
    res.push(`
    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(ebool /*v*/) internal pure returns (bool) {
        return true;
    }
  `);
    supportedBits.forEach((b) => {
      res.push(`
      // Return true if the enrypted integer is initialized and false otherwise.
      function isInitialized(euint${b} /*v*/) internal pure returns (bool) {
          return true;
      }
    `);
    });
  } else {
    res.push(`
    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(ebool v) internal pure returns (bool) {
        return ebool.unwrap(v) != 0;
    }
  `);
    supportedBits.forEach((b) => {
      res.push(`
      // Return true if the enrypted integer is initialized and false otherwise.
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
  supportedBits.forEach((bits) => res.push(tfheEq(bits)));
  supportedBits.forEach((outputBits) => {
    supportedBits.forEach((inputBits) => {
      res.push(tfheAsEboolCustomCast(inputBits, outputBits));
    });
    res.push(tfheAsEboolUnaryCast(outputBits));
  });
  supportedBits.forEach((bits) => res.push(tfheUnaryOperators(bits, operators, signatures)));
  supportedBits.forEach((bits) => res.push(tfheCustomUnaryOperators(bits, signatures, mocked)));

  res.push(tfheCustomMethods(ctx, mocked));

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

    // os.push({
    //   binaryOperator: operator.binarySolidityOperator,
    //   arguments: [
    //     { type: ArgumentType.EUint, bits },
    //     { type: ArgumentType.EUint, bits },
    //   ],

    //   name: `bin_op_${operator.name}`,
    //   returnType: { type: ArgumentType.EUint, bits },
    // });
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

    // os.push({
    //   unaryOperator: operator.unarySolidityOperator,
    //   arguments: [{ type: ArgumentType.EUint, bits }],

    //   name: `unary_op_${operator.name}`,
    //   returnType: { type: ArgumentType.EUint, bits },
    // });
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
    function ${operator.name}(euint${lhsBits} a, ${getUint(rhsBits)} b) internal pure returns (${returnType}) {
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
    function ${operator.name}(${getUint(lhsBits)} a, euint${rhsBits} b) internal pure returns (${returnType}) {
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
  }

  // Code and test for shift(euint{inputBits},uint8}
  scalarFlag = ', true';
  implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(a), uint256(b)${scalarFlag})`;
  if (mocked) {
    if (rotate) {
      implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(a), uint256(b) % ${lhsBits}, ${lhsBits}${scalarFlag})`;
    } else {
      implExpression = `Impl.${operator.name}(euint${outputBits}.unwrap(a), uint256(b) % ${lhsBits}${scalarFlag})`;
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
    // Evaluate ${operator.name}(a, b) and return the result.
    function ${operator.name}(euint${lhsBits} a, ${getUint(rhsBits)} b) internal pure returns (${returnType}) {
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
    // If 'control''s value is 'true', the result has the same value as 'a'.
    // If 'control''s value is 'false', the result has the same value as 'b'.
    function cmux(ebool control, euint${inputBits} a, euint${inputBits} b) internal pure returns (euint${inputBits}) {
        return euint${inputBits}.wrap(Impl.select(ebool.unwrap(control), euint${inputBits}.unwrap(a), euint${inputBits}.unwrap(b)));
    }
    
    function select(ebool control, euint${inputBits} a, euint${inputBits} b) internal pure returns (euint${inputBits}) {
        return euint${inputBits}.wrap(Impl.select(ebool.unwrap(control), euint${inputBits}.unwrap(a), euint${inputBits}.unwrap(b)));
    }`;
}

function tfheEq(inputBits: number): string {
  return `
    function eq(euint${inputBits}[] memory a, euint${inputBits}[] memory b) internal pure returns (ebool) {
        require(a.length == b.length, "Both arrays are not of the same size.");
        uint256[] memory lhs = new uint256[](a.length);
        uint256[] memory rhs = new uint256[](b.length);
        for (uint i = 0; i < a.length; i++) {
          lhs[i] = euint${inputBits}.unwrap(a[i]);
        }
        for (uint i = 0; i < b.length; i++) {
          rhs[i] = euint${inputBits}.unwrap(b[i]);
        }
        return ebool.wrap(Impl.eq(lhs, rhs));
    }
  `;
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
    // Convert a serialized 'ciphertext' to an encrypted euint8 integer.
    function asEbool(bytes memory ciphertext) internal pure returns (ebool) {
        return ebool.wrap(Impl.verify(ciphertext, Common.ebool_t));
    }

    // Convert a plaintext value to an encrypted euint8 integer.
    function asEbool(uint256 value) internal pure returns (ebool) {
        return ebool.wrap(Impl.trivialEncrypt(value, Common.ebool_t));
    }

    // Convert a plaintext boolean to an encrypted boolean.
    function asEbool(bool value) internal pure returns (ebool) {
        if (value) {
            return asEbool(1);
        } else {
            return asEbool(0);
        }
    }

    // Converts an 'ebool' to an 'euint8'.
    function asEuint8(ebool value) internal pure returns (euint8) {
      return euint8.wrap(Impl.cast(ebool.unwrap(value), Common.euint8_t));
    }

    // Evaluate and(a, b) and return the result.
    function and(ebool a, ebool b) internal pure returns (ebool) {
        return ebool.wrap(Impl.and(ebool.unwrap(a), ebool.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(ebool a, ebool b) internal pure returns (ebool) {
        return ebool.wrap(Impl.or(ebool.unwrap(a), ebool.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(ebool a, ebool b) internal pure returns (ebool) {
        return ebool.wrap(Impl.xor(ebool.unwrap(a), ebool.unwrap(b)));
    }

    function not(ebool a) internal pure returns (ebool) {
        return ebool.wrap(Impl.not(ebool.unwrap(a)));
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

function tfheCustomUnaryOperators(bits: number, signatures: OverloadSignature[], mocked: boolean): string {
  let result = `
    // Convert a serialized 'ciphertext' to an encrypted euint${bits} integer.
    function asEuint${bits}(bytes memory ciphertext) internal pure returns (euint${bits}) {
        return euint${bits}.wrap(Impl.verify(ciphertext, Common.euint${bits}_t));
    }

    // Convert a plaintext value to an encrypted euint${bits} integer.
    function asEuint${bits}(uint256 value) internal pure returns (euint${bits}) {
        return euint${bits}.wrap(Impl.trivialEncrypt(value, Common.euint${bits}_t));
    }

    `;
  if (mocked) {
    result += `
    // Decrypts the encrypted 'value'.
    function decrypt(euint${bits} value) internal view returns (${getUint(bits)}) {
        return ${getUint(bits)}(Impl.decrypt(euint${bits}.unwrap(value)) % 2**${bits});
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint${bits} ciphertext.
    function reencrypt(euint${bits} value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint${bits}.unwrap(value) % 2**${bits}, publicKey);
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // If 'value' is not initialized, the returned value will contain the 'defaultValue' constant.
    // Return a serialized euint${bits} ciphertext.
    function reencrypt(euint${bits} value, bytes32 publicKey, ${getUint(
      bits,
    )} defaultValue) internal view returns (bytes memory reencrypted) {
        if (euint${bits}.unwrap(value) != 0) {
            return Impl.reencrypt(euint${bits}.unwrap(value) % 2**${bits}, publicKey);
        } else {
            return Impl.reencrypt(euint${bits}.unwrap(asEuint${bits}(defaultValue)) % 2**${bits}, publicKey);
        }
    }
    `;
  } else {
    result += `
    // Decrypts the encrypted 'value'.
    function decrypt(euint${bits} value) internal view returns (${getUint(bits)}) {
        return ${getUint(bits)}(Impl.decrypt(euint${bits}.unwrap(value)));
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // Return a serialized euint${bits} ciphertext.
    function reencrypt(euint${bits} value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
        return Impl.reencrypt(euint${bits}.unwrap(value), publicKey);
    }

    // Reencrypt the given 'value' under the given 'publicKey'.
    // If 'value' is not initialized, the returned value will contain the 'defaultValue' constant.
    // Return a serialized euint${bits} ciphertext.
    function reencrypt(euint${bits} value, bytes32 publicKey, ${getUint(
      bits,
    )} defaultValue) internal view returns (bytes memory reencrypted) {
        if (euint${bits}.unwrap(value) != 0) {
            return Impl.reencrypt(euint${bits}.unwrap(value), publicKey);
        } else {
            return Impl.reencrypt(euint${bits}.unwrap(asEuint${bits}(defaultValue)), publicKey);
        }
    }
    `;
  }
  return result;
}

function unaryOperatorImpl(op: Operator): string {
  let fname = operatorFheLibFunction(op);
  return `
    function ${op.name}(uint256 ct) internal pure returns (uint256 result) {
      result = FhevmLib(address(EXT_TFHE_LIBRARY)).${fname}(ct);
    }
  `;
}

function tfheCustomMethods(ctx: CodegenContext, mocked: boolean): string {
  let result = `
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

    // Generates a random encrypted 8-bit unsigned integer in the [0, upperBound) range.
    // The upperBound must be a power of 2.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint8(uint8 upperBound) internal view returns (euint8) {
      return euint8.wrap(Impl.randBounded(upperBound, Common.euint8_t));
    }

    // Generates a random encrypted 16-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint16() internal view returns (euint16) {
      return euint16.wrap(Impl.rand(Common.euint16_t));
    }

    // Generates a random encrypted 16-bit unsigned integer in the [0, upperBound) range.
    // The upperBound must be a power of 2.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint16(uint16 upperBound) internal view returns (euint16) {
      return euint16.wrap(Impl.randBounded(upperBound, Common.euint16_t));
    }

    // Generates a random encrypted 32-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint32() internal view returns (euint32) {
      return euint32.wrap(Impl.rand(Common.euint32_t));
    }

    // Generates a random encrypted 64-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint64() internal view returns (euint64) {
      return euint64.wrap(Impl.rand(Common.euint64_t));
    }

    // Generates a random encrypted 32-bit unsigned integer in the [0, upperBound) range.
    // The upperBound must be a power of 2.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint32(uint32 upperBound) internal view returns (euint32) {
      return euint32.wrap(Impl.randBounded(upperBound, Common.euint32_t));
    }

    function randEuint64(uint64 upperBound) internal view returns (euint64) {
      return euint64.wrap(Impl.randBounded(upperBound, Common.euint64_t));
    }
    // Decrypts the encrypted 'value'.
    function decrypt(eaddress value) internal view returns (address) {
        return address(uint160(Impl.decrypt(eaddress.unwrap(value))));
    }

    // Reencrypt  the encrypted 'value'.
    function reencrypt(eaddress value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {
      return Impl.reencrypt(eaddress.unwrap(value), publicKey);
  }

    // From bytes to eaddress
    function asEaddress(bytes memory ciphertext) internal pure returns (eaddress) {
      return eaddress.wrap(Impl.verify(ciphertext, Common.euint160_t));

    }

    // Convert a plaintext value to an encrypted asEaddress.
    function asEaddress(address value) internal pure returns (eaddress) {
        return eaddress.wrap(Impl.trivialEncrypt(uint160(value), Common.euint160_t));
    }

    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(eaddress v) internal pure returns (bool) {
        return eaddress.unwrap(v) != 0;
    }

    // Evaluate eq(a, b) and return the result.
    function eq(eaddress a, eaddress b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), eaddress.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(eaddress a, eaddress b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), eaddress.unwrap(b), false));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(eaddress a, address b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), bProc, true));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(address b, eaddress a) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), bProc, true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(eaddress a, address b) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), bProc, true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(address b, eaddress a) internal pure returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), bProc, true));
    }
    
    function select(ebool control, eaddress a, eaddress b) internal pure returns (eaddress) {
        return eaddress.wrap(Impl.select(ebool.unwrap(control), eaddress.unwrap(a), eaddress.unwrap(b)));
    }
`;
  if (mocked) {
    result += `
    // Decrypts the encrypted 'value'.
    function decrypt(ebool value) internal view returns (bool) {
        return (Impl.decrypt(ebool.unwrap(value)) % 2 == 1);
    }
    `;
  } else {
    result += `
    // Decrypts the encrypted 'value'.
    function decrypt(ebool value) internal view returns (bool) {
        return (Impl.decrypt(ebool.unwrap(value)) != 0);
    }
    `;
  }
  return result;
}

function implCustomMethods(ctx: CodegenContext): string {
  return `
    // If 'control's value is 'true', the result has the same value as 'ifTrue'.
    // If 'control's value is 'false', the result has the same value as 'ifFalse'.
    function select(uint256 control, uint256 ifTrue, uint256 ifFalse) internal pure returns (uint256 result) {
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheIfThenElse(control, ifTrue, ifFalse);
    }

    function eq(uint256[] memory lhs, uint256[] memory rhs) internal pure returns (uint256 result) {
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheArrayEq(lhs, rhs);
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

    function randBounded(uint256 upperBound, uint8 randType) internal view returns(uint256 result) {
      result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheRandBounded(upperBound, bytes1(randType));
    }
    `;
}

export function implSolMock(ctx: CodegenContext, operators: Operator[]): string {
  const res: string[] = [];

  res.push(`
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.20;

import "./TFHE.sol";

library Impl {
  function add(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
    unchecked {
        result = lhs + rhs;
    }
  }

  function sub(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      unchecked {
          result = lhs - rhs;
      }
  }

  function mul(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      unchecked {
          result = lhs * rhs;
      }
  }

  function div(uint256 lhs, uint256 rhs) internal pure returns (uint256 result) {
      result = lhs / rhs; // unchecked does not change behaviour even when dividing by 0
  }

  function rem(uint256 lhs, uint256 rhs) internal pure returns (uint256 result) {
      result = lhs % rhs;
  }

  function and(uint256 lhs, uint256 rhs) internal pure returns (uint256 result) {
      result = lhs & rhs;
  }

  function or(uint256 lhs, uint256 rhs) internal pure returns (uint256 result) {
      result = lhs | rhs;
  }

  function xor(uint256 lhs, uint256 rhs) internal pure returns (uint256 result) {
      result = lhs ^ rhs;
  }

  function shl(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      result = lhs << rhs;
  }

  function shr(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      result = lhs >> rhs;
  }

  function rotl(uint256 lhs, uint256 rhs, uint256 bits, bool /*scalar*/) internal pure returns (uint256 result) {
      uint count = rhs;
      result = (lhs << count) | (lhs >> (bits - count));
  }

  function rotr(uint256 lhs, uint256 rhs, uint256 bits, bool /*scalar*/) internal pure returns (uint256 result) {
      uint count = rhs;
      result = (lhs >> count) | (lhs << (bits - count));
  }

  function eq(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      result = (lhs == rhs) ? 1 : 0;
  }

  function eq(uint256[] memory lhs, uint256[] memory rhs) internal pure returns (uint256 result) {
      require(lhs.length == rhs.length, "Both arrays are not of the same size.");
      result = 1;
      for (uint i = 0; i < lhs.length; i++) {
        if (lhs[i] != rhs[i]) return;
      }
  }

  function ne(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      result = (lhs != rhs) ? 1 : 0;
  }

  function ge(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      result = (lhs >= rhs) ? 1 : 0;
  }

  function gt(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      result = (lhs > rhs) ? 1 : 0;
  }

  function le(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      result = (lhs <= rhs) ? 1 : 0;
  }

  function lt(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      result = (lhs < rhs) ? 1 : 0;
  }

  function min(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      result = (lhs < rhs) ? lhs : rhs;
  }

  function max(uint256 lhs, uint256 rhs, bool /*scalar*/) internal pure returns (uint256 result) {
      result = (lhs > rhs) ? lhs : rhs;
  }

  function neg(uint256 ct) internal pure returns (uint256 result) {
      uint256 y;
      assembly {
          y := not(ct)
      }
      unchecked {
          return y + 1;
      }
  }

  function not(uint256 ct) internal pure returns (uint256 result) {
      uint256 y;
      assembly {
          y := not(ct)
      }
      return y;
  }

  function cmux(uint256 control, uint256 ifTrue, uint256 ifFalse) internal pure returns (uint256 result) {
      result = (control == 1) ? ifTrue : ifFalse;
  }

  function select(uint256 control, uint256 ifTrue, uint256 ifFalse) internal pure returns (uint256 result) {
      result = (control == 1) ? ifTrue : ifFalse;
  }

  function optReq(uint256 ciphertext) internal view {
      this; // silence state mutability warning
      require(ciphertext == 1, "transaction execution reverted");
  }

  function reencrypt(uint256 ciphertext, bytes32 /*publicKey*/) internal view returns (bytes memory reencrypted) {
      this; // silence state mutability warning
      reencrypted = new bytes(32);
        assembly {
            mstore(add(reencrypted, 32), ciphertext)
        }
        return reencrypted;
  }

  function fhePubKey() internal view returns (bytes memory key) {
      this; // silence state mutability warning
      key = hex"0123456789ABCDEF";
  }

  function verify(bytes memory _ciphertextBytes, uint8 /*_toType*/) internal pure returns (uint256 result) {
      uint256 x;
      assembly {
          switch gt(mload(_ciphertextBytes), 31)
          case 1 {
              x := mload(add(_ciphertextBytes, add(32, sub(mload(_ciphertextBytes), 32))))
          }
          default {
              x := mload(add(_ciphertextBytes, 32))
          }
      }
      if (_ciphertextBytes.length < 32) {
          x = x >> ((32 - _ciphertextBytes.length) * 8);
      }
      return x;
  }

  function cast(uint256 ciphertext, uint8 toType) internal pure returns (uint256 result) {
    if (toType == 0) {
        result = uint256(uint8(ciphertext));
    }
    if (toType == 1) {
        result = uint256(uint8(ciphertext));
    }
    if (toType == 2) {
        result = uint256(uint8(ciphertext));
    }
    if (toType == 3) {
        result = uint256(uint16(ciphertext));
    }
    if (toType == 4) {
        result = uint256(uint32(ciphertext));
    }
    if (toType == 5) {
        result = uint256(uint64(ciphertext));
    }
  }

  function trivialEncrypt(uint256 value, uint8 /*toType*/) internal pure returns (uint256 result) {
      result = value;
  }

  function decrypt(uint256 ciphertext) internal view returns (uint256 result) {
      this; // silence state mutability warning
      result = ciphertext;
  }

  function rand(uint8 randType) internal view returns (uint256 result) {
      uint256 randomness = uint256(keccak256(abi.encodePacked(block.number, gasleft(), msg.sender))); // assuming no duplicated tx by same sender in a single block
      if (randType == Common.euint8_t) {
        result = uint8(randomness);
      } else if (randType == Common.euint16_t) {
        result = uint16(randomness);
      } else if (randType == Common.euint32_t) {
        result = uint32(randomness);
      } else if (randType == Common.euint64_t) {
        result = uint64(randomness);
      } else {
        revert("rand() mock invalid type");
      }
  }

  function randBounded(uint256 upperBound, uint8 randType) internal view returns (uint256 result) {
    // Here, we assume upperBound is a power of 2. Therefore, using modulo is secure.
    // If not a power of 2, we might have to do something else (though might not matter
    // much as this is a mock).
    result = rand(randType) % upperBound;
  }
`);

  res.push('}\n');

  return res.join('');
}
