import { Operator, OperatorArguments, Precompile } from './common';

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
  return `
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
    }
    `;
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

    function req(uint256 ciphertext) internal view {
        bytes32[1] memory input;
        input[0] = bytes32(ciphertext);
        uint256 inputLen = 32;

        // Call the require precompile.
        uint256 precompile = Precompiles.Require;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, 0, 0)) {
                revert(0, 0)
            }
        }
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
    `;
}
