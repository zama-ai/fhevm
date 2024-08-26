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
type ebytes256 is uint256;
type einput is bytes32;

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
    uint8 internal constant euint256_t = 8;
    uint8 internal constant ebytes64_t = 9;
    uint8 internal constant ebytes128_t = 10;
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
    function ${op.name}(uint256 lhs, uint256 rhs${scalarArg}) internal returns (uint256 result) {
        ${scalarSection}
        result = ITFHEExecutor(tfheExecutorAdd).${fname}(lhs, rhs, scalarByte);
    }` + '\n'
  );
}

export function implSol(ctx: CodegenContext, operators: Operator[]): string {
  const res: string[] = [];

  const coprocessorInterface = generateImplCoprocessorInterface(operators);
  const aclInterface = generateACLInterface();

  res.push(`
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "./TFHE.sol";
import "./TFHEExecutorAddress.sol";
import "./ACLAddress.sol";

${coprocessorInterface}

${aclInterface}

library Impl {
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

  res.push(implCustomMethods(ctx));

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

  res.push('interface ITFHEExecutor {');
  operators.forEach((op) => {
    let functionName = operatorFheLibFunction(op);
    const tail = 'external returns (uint256 result);';
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

  res.push(coprocessorInterfaceCustomFunctions());

  res.push('}');

  return res.join('');
}

function fheLibCustomInterfaceFunctions(): string {
  return `
    function verifyCiphertext(bytes32 inputHandle, address callerAddress, address contractAddress, bytes memory inputProof, bytes1 inputType) external pure returns (uint256 result);
  `;
}

function coprocessorInterfaceCustomFunctions(): string {
  return `
    function verifyCiphertext(bytes32 inputHandle, address callerAddress, bytes memory inputProof, bytes1 inputType) external returns (uint256 result);
    function cast(uint256 ct, bytes1 toType) external returns (uint256 result);
    function trivialEncrypt(uint256 ct, bytes1 toType) external returns (uint256 result);
    function fheIfThenElse(uint256 control, uint256 ifTrue, uint256 ifFalse) external returns (uint256 result);
    function fheRand(bytes1 randType) external returns (uint256 result);
    function fheRandBounded(uint256 upperBound, bytes1 randType) external returns (uint256 result);
  `;
}

function generateACLInterface(): string {
  return `
  interface IACL {
    function allowTransient(uint256 ciphertext, address account) external;
    function allow(uint256 handle, address account) external;
    function cleanTransientStorage() external;
    function isAllowed(uint256 handle, address account) external view returns(bool);
  }
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

pragma solidity ^0.8.24;

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
    // Return true if the enrypted bool is initialized and false otherwise.
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
    // Return true if the enrypted bool is initialized and false otherwise.
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
  supportedBits.forEach((outputBits) => {
    supportedBits.forEach((inputBits) => {
      res.push(tfheAsEboolCustomCast(inputBits, outputBits));
    });
    res.push(tfheAsEboolUnaryCast(outputBits));
  });
  supportedBits.forEach((bits) => res.push(tfheUnaryOperators(bits, operators, signatures)));
  supportedBits.forEach((bits) => res.push(tfheCustomUnaryOperators(bits, signatures, mocked)));

  res.push(tfheCustomMethods(ctx, mocked));

  res.push(tfheAclMethods(supportedBits));

  res.push('}\n');

  // supportedBits.forEach((bits) => {
  //   operators.forEach((op) => {
  //     res.push(tfheSolidityOperator(bits, op, signatures));
  //   });
  // });

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

    // Evaluate ${operator.name}(a, b) and return the result.
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
    // Evaluate ${operator.name}(a, b) and return the result.
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
    // If 'control''s value is 'true', the result has the same value as 'a'.
    // If 'control''s value is 'false', the result has the same value as 'b'.
    function select(ebool control, euint${inputBits} a, euint${inputBits} b) internal returns (euint${inputBits}) {
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
    function asEuint${outputBits}(euint${inputBits} value) internal returns (euint${outputBits}) {
        return euint${outputBits}.wrap(Impl.cast(euint${inputBits}.unwrap(value), Common.euint${outputBits}_t));
    }
    `;
}

function tfheAsEboolUnaryCast(bits: number): string {
  const res: string[] = [];
  res.push(`
    // Cast an encrypted integer from euint${bits} to ebool.
    function asEbool(euint${bits} value) internal returns (ebool) {
        return ne(value, 0);
    }
    `);

  if (bits == 8) {
    res.push(`
    // Convert an inputHandle with corresponding inputProof to an encrypted boolean.
    function asEbool(einput inputHandle, bytes memory inputProof) internal returns (ebool) {
        return ebool.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, Common.ebool_t));
    }

    // Convert a plaintext value to an encrypted boolean.
    function asEbool(uint256 value) internal returns (ebool) {
        return ebool.wrap(Impl.trivialEncrypt(value, Common.ebool_t));
    }

    // Convert a plaintext boolean to an encrypted boolean.
    function asEbool(bool value) internal returns (ebool) {
        if (value) {
            return asEbool(1);
        } else {
            return asEbool(0);
        }
    }

    // Converts an 'ebool' to an 'euint8'.
    function asEuint8(ebool value) internal returns (euint8) {
      return euint8.wrap(Impl.cast(ebool.unwrap(value), Common.euint8_t));
    }

    // Evaluate and(a, b) and return the result.
    function and(ebool a, ebool b) internal returns (ebool) {
        return ebool.wrap(Impl.and(ebool.unwrap(a), ebool.unwrap(b)));
    }

    // Evaluate or(a, b) and return the result.
    function or(ebool a, ebool b) internal returns (ebool) {
        return ebool.wrap(Impl.or(ebool.unwrap(a), ebool.unwrap(b)));
    }

    // Evaluate xor(a, b) and return the result.
    function xor(ebool a, ebool b) internal returns (ebool) {
        return ebool.wrap(Impl.xor(ebool.unwrap(a), ebool.unwrap(b)));
    }

    function not(ebool a) internal returns (ebool) {
        return ebool.wrap(Impl.not(ebool.unwrap(a)));
    }
    `);
  } else {
    res.push(`
    // Converts an 'ebool' to an 'euint${bits}'.
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
    // Convert an inputHandle with corresponding inputProof to an encrypted euint${bits} integer.
    function asEuint${bits}(einput inputHandle, bytes memory inputProof) internal returns (euint${bits}) {
        return euint${bits}.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, Common.euint${bits}_t));
    }

    // Convert a plaintext value to an encrypted euint${bits} integer.
    function asEuint${bits}(uint256 value) internal returns (euint${bits}) {
        return euint${bits}.wrap(Impl.trivialEncrypt(value, Common.euint${bits}_t));
    }

    `;
  return result;
}

function unaryOperatorImpl(op: Operator): string {
  let fname = operatorFheLibFunction(op);
  return `
    function ${op.name}(uint256 ct) internal returns (uint256 result) {
      result = ITFHEExecutor(tfheExecutorAdd).${fname}(ct);
    }
  `;
}

function tfheAclMethods(supportedBits: number[]): string {
  const res: string[] = [];

  res.push(
    `
    // cleans the transient storage of ACL containing all the allowedTransient accounts
    // to be used for integration with Account Abstraction or when bundling UserOps calling the FHEVMCoprocessor
    function cleanTransientStorage() internal {
      return Impl.cleanTransientStorage();
    }

    function isAllowed(ebool value, address account) internal view returns (bool) {
      return Impl.isAllowed(ebool.unwrap(value), account);
    }
  `,
  );

  supportedBits.forEach((bits) =>
    res.push(`function isAllowed(euint${bits} value, address account) internal view returns (bool) {
      return Impl.isAllowed(euint${bits}.unwrap(value), account);
    }`),
  );

  res.push(
    `
    function isAllowed(eaddress value, address account) internal view returns(bool) {
      return Impl.isAllowed(eaddress.unwrap(value), account);
    }

    function isAllowed(ebytes256 value, address account) internal view returns (bool) {
      return Impl.isAllowed(ebytes256.unwrap(value), account);
    }

    function isSenderAllowed(ebool value) internal view returns (bool) {
      return Impl.isAllowed(ebool.unwrap(value), msg.sender);
    }
    `,
  );

  supportedBits.forEach((bits) =>
    res.push(
      `
      function isSenderAllowed(euint${bits} value) internal view returns (bool) {
        return Impl.isAllowed(euint${bits}.unwrap(value), msg.sender);
      }
      `,
    ),
  );

  res.push(
    `
    function isSenderAllowed(eaddress value) internal view returns(bool) {
      return Impl.isAllowed(eaddress.unwrap(value), msg.sender);
    }

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
    `,
  );

  supportedBits.forEach((bits) =>
    res.push(
      `
    function allow(euint${bits} value, address account) internal {
      Impl.allow(euint${bits}.unwrap(value), account);
    }
    \n`,
    ),
  );

  res.push(
    `
    function allow(eaddress value, address account) internal {
      Impl.allow(eaddress.unwrap(value), account);
    }

    function allow(ebytes256 value, address account) internal {
      Impl.allow(ebytes256.unwrap(value), account);
    }
    `,
  );

  res.push(
    `
    function allowTransient(ebool value, address account) internal {
      Impl.allowTransient(ebool.unwrap(value), account);
    }
    `,
  );

  supportedBits.forEach((bits) =>
    res.push(
      `
    function allowTransient(euint${bits} value, address account) internal {
      Impl.allowTransient(euint${bits}.unwrap(value), account);
    }
    \n`,
    ),
  );

  res.push(
    `
    function allowTransient(eaddress value, address account) internal {
      Impl.allowTransient(eaddress.unwrap(value), account);
    }

    function allowTransient(ebytes256 value, address account) internal {
      Impl.allowTransient(ebytes256.unwrap(value), account);
    }
    `,
  );

  return res.join('');
}

function tfheCustomMethods(ctx: CodegenContext, mocked: boolean): string {
  let result = `
    // Generates a random encrypted 8-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint8() internal returns (euint8) {
      return euint8.wrap(Impl.rand(Common.euint8_t));
    }

    // Generates a random encrypted 8-bit unsigned integer in the [0, upperBound) range.
    // The upperBound must be a power of 2.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint8(uint8 upperBound) internal returns (euint8) {
      return euint8.wrap(Impl.randBounded(upperBound, Common.euint8_t));
    }

    // Generates a random encrypted 16-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint16() internal returns (euint16) {
      return euint16.wrap(Impl.rand(Common.euint16_t));
    }

    // Generates a random encrypted 16-bit unsigned integer in the [0, upperBound) range.
    // The upperBound must be a power of 2.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint16(uint16 upperBound) internal returns (euint16) {
      return euint16.wrap(Impl.randBounded(upperBound, Common.euint16_t));
    }

    // Generates a random encrypted 32-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint32() internal returns (euint32) {
      return euint32.wrap(Impl.rand(Common.euint32_t));
    }

    // Generates a random encrypted 32-bit unsigned integer in the [0, upperBound) range.
    // The upperBound must be a power of 2.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint32(uint32 upperBound) internal returns (euint32) {
      return euint32.wrap(Impl.randBounded(upperBound, Common.euint32_t));
    }

    // Generates a random encrypted 64-bit unsigned integer.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint64() internal returns (euint64) {
      return euint64.wrap(Impl.rand(Common.euint64_t));
    }

    // Generates a random encrypted 64-bit unsigned integer in the [0, upperBound) range.
    // The upperBound must be a power of 2.
    // Important: The random integer is generated in the plain! An FHE-based version is coming soon.
    function randEuint64(uint64 upperBound) internal returns (euint64) {
      return euint64.wrap(Impl.randBounded(upperBound, Common.euint64_t));
    }

    // Convert an inputHandle with corresponding inputProof to an encrypted eaddress.
    function asEaddress(einput inputHandle, bytes memory inputProof) internal returns (eaddress) {
      return eaddress.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, Common.euint160_t));
    }

    // Convert a plaintext value to an encrypted asEaddress.
    function asEaddress(address value) internal returns (eaddress) {
        return eaddress.wrap(Impl.trivialEncrypt(uint160(value), Common.euint160_t));
    }
    
    // Convert the given inputHandle and inputProof to an encrypted ebytes256 value.
    function asEbytes256(einput inputHandle, bytes memory inputProof) internal returns (ebytes256) {
      return ebytes256.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, Common.ebytes256_t));
    }

    // Return true if the enrypted address is initialized and false otherwise.
    function isInitialized(eaddress v) internal pure returns (bool) {
        return eaddress.unwrap(v) != 0;
    }
    
    // Return true if the enrypted value is initialized and false otherwise.
    function isInitialized(ebytes256 v) internal pure returns (bool) {
        return ebytes256.unwrap(v) != 0;
    }

    // Evaluate eq(a, b) and return the result.
    function eq(eaddress a, eaddress b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), eaddress.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(eaddress a, eaddress b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), eaddress.unwrap(b), false));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(eaddress a, address b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), bProc, true));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(address b, eaddress a) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), bProc, true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(eaddress a, address b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), bProc, true));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(address b, eaddress a) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        uint256 bProc = uint256(uint160(b));
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), bProc, true));
    }

    // If 'control''s value is 'true', the result has the same value as 'a'.
    // If 'control''s value is 'false', the result has the same value as 'b'.
    function select(ebool control, eaddress a, eaddress b) internal returns (eaddress) {
        return eaddress.wrap(Impl.select(ebool.unwrap(control), eaddress.unwrap(a), eaddress.unwrap(b)));
    }

    // Evaluate eq(a, b) and return the result.
    function eq(ebytes256 a, ebytes256 b) internal returns (ebool) {
        require(isInitialized(a), "a is uninitialized");
        require(isInitialized(b), "b is uninitialized");
        return ebool.wrap(Impl.eq(ebytes256.unwrap(a), ebytes256.unwrap(b), false));
    }

    // Evaluate ne(a, b) and return the result.
    function ne(ebytes256 a, ebytes256 b) internal returns (ebool) {
        require(isInitialized(a), "a is uninitialized");
        require(isInitialized(b), "b is uninitialized");
        return ebool.wrap(Impl.ne(ebytes256.unwrap(a), ebytes256.unwrap(b), false));
    }
`;
  return result;
}

function implCustomMethods(ctx: CodegenContext): string {
  return `
    // If 'control's value is 'true', the result has the same value as 'ifTrue'.
    // If 'control's value is 'false', the result has the same value as 'ifFalse'.
    function select(uint256 control, uint256 ifTrue, uint256 ifFalse) internal returns (uint256 result) {
        result = ITFHEExecutor(tfheExecutorAdd).fheIfThenElse(control, ifTrue, ifFalse);
    }

    function verify(
        bytes32 inputHandle,
        bytes memory inputProof,
        uint8 toType
    ) internal returns (uint256 result) {
        result = ITFHEExecutor(tfheExecutorAdd).verifyCiphertext(inputHandle, msg.sender, inputProof, bytes1(toType));
        IACL(aclAdd).allowTransient(result, msg.sender);
    }

    function cast(
        uint256 ciphertext,
        uint8 toType
    ) internal returns (uint256 result) {
        result = ITFHEExecutor(tfheExecutorAdd).cast(ciphertext, bytes1(toType));
    }

    function trivialEncrypt(
        uint256 value,
        uint8 toType
    ) internal returns (uint256 result) {
        result = ITFHEExecutor(tfheExecutorAdd).trivialEncrypt(value, bytes1(toType));
    }

    function rand(uint8 randType) internal returns(uint256 result) {
      result = ITFHEExecutor(tfheExecutorAdd).fheRand(bytes1(randType));
    }

    function randBounded(uint256 upperBound, uint8 randType) internal returns(uint256 result) {
      result = ITFHEExecutor(tfheExecutorAdd).fheRandBounded(upperBound, bytes1(randType));
    }

    function allowTransient(uint256 handle, address account) internal {
      IACL(aclAdd).allowTransient(handle, account);
    }

    function allow(uint256 handle, address account) internal {
      IACL(aclAdd).allow(handle, account);
    }

    function cleanTransientStorage() internal {
      IACL(aclAdd).cleanTransientStorage();
    }

    function isAllowed(uint256 handle, address account) internal view returns (bool) {
      return IACL(aclAdd).isAllowed(handle, account);
    }
    `;
}

export function implSolMock(ctx: CodegenContext, operators: Operator[]): string {
  const res: string[] = [];

  res.push(`
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

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
        if (lhs[i] != rhs[i]) return 0;
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

  function select(uint256 control, uint256 ifTrue, uint256 ifFalse) internal pure returns (uint256 result) {
      result = (control == 1) ? ifTrue : ifFalse;
  }

  function select(uint256 control, uint256 ifTrue, uint256 ifFalse) internal pure returns (uint256 result) {
      result = (control == 1) ? ifTrue : ifFalse;
  }

  function optReq(uint256 ciphertext) internal view {
      this; // silence state mutability warning
      require(ciphertext == 1, "transaction execution reverted");
  }

  function fhePubKey() internal view returns (bytes memory key) {
      this; // silence state mutability warning
      key = hex"0123456789ABCDEF";
  }

  function verify(einput inputHandle,
        bytes memory inputProof,
        uint8 toType) internal returns (uint256 result) {
      // TODO: fix implementation
      uint256 x;
      assembly {
          switch gt(mload(inputProof), 31)
          case 1 {
              x := mload(add(inputProof, add(32, sub(mload(inputProof), 32))))
          }
          default {
              x := mload(add(inputProof, 32))
          }
      }
      if (inputProof.length < 32) {
          x = x >> ((32 - inputProof.length) * 8);
      }
      return x;
  }

  function cast(uint256 ciphertext, uint8 toType) internal returns (uint256 result) {
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

  function trivialEncrypt(uint256 value, uint8 /*toType*/) internal returns (uint256 result) {
      result = value;
  }

  function rand(uint8 randType) internal returns (uint256 result) {
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

  function randBounded(uint256 upperBound, uint8 randType) internal returns (uint256 result) {
    // Here, we assume upperBound is a power of 2. Therefore, using modulo is secure.
    // If not a power of 2, we might have to do something else (though might not matter
    // much as this is a mock).
    result = rand(randType) % upperBound;
  }
`);

  res.push('}\n');

  return res.join('');
}
