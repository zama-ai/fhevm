### Generation script for boilerplate euint type system

f = open("Common.sol", "w")
f.write("""\
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

type euint8 is uint256;
type euint16 is uint256;
type euint32 is uint256;

library Common {
    // Values used to communicate types to the runtime.
    uint8 internal constant euint8_t = 0;
    uint8 internal constant euint16_t = 1;
    uint8 internal constant euint32_t = 2;
}
"""
)
f.close()

f = open("Precompiles.sol", "w")
f.write("""\
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

library Precompiles {
    uint256 public constant Add = 65;
    uint256 public constant Verify = 66;
    uint256 public constant Reencrypt = 67;
    uint256 public constant FhePubKey = 68;
    uint256 public constant Require = 69;
    uint256 public constant LessThanOrEqual = 70;
    uint256 public constant Subtract = 71;
    uint256 public constant Multiply = 72;
    uint256 public constant LessThan = 73;
    uint256 public constant OptimisticRequire = 75;
    uint256 public constant Cast = 76;
    uint256 public constant TrivialEncrypt = 77;
    uint256 public constant BitwiseAnd = 78;
    uint256 public constant BitwiseOr = 79;
    uint256 public constant BitwiseXor = 80;
    uint256 public constant Equal = 81;
    uint256 public constant GreaterThanOrEqual = 82;
    uint256 public constant GreaterThan = 83;
    uint256 public constant ShiftLeft = 84;
    uint256 public constant ShiftRight = 85;
    uint256 public constant NotEqual = 86;
    uint256 public constant Min = 87;
    uint256 public constant Max = 88;
    uint256 public constant Negate = 89;
    uint256 public constant Not = 90;
}
"""
)
f.close()

f = open("Impl.sol", "w")
f.write("""\
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "./Common.sol";
import "./Precompiles.sol";

library Impl {
    // 32 bytes for the `byte` type header + 48 bytes for the NaCl anonymous
    // box overhead + 4 bytes for the plaintext value.
    uint256 constant reencryptedSize = 32 + 48 + 4;

    // 32 bytes for the `byte` header + 16553 bytes of key data.
    uint256 constant fhePubKeySize = 32 + 16553;

    function add(uint256 a, uint256 b, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        
        bytes memory input = bytes.concat(bytes32(a), bytes32(b), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the add precompile.
        uint256 precompile = Precompiles.Add;
        assembly {
            // jump over the 32-bit `size` field of the `bytes` data structure of the `input` to read actual bytes
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function sub(uint256 a, uint256 b, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        
        bytes memory input = bytes.concat(bytes32(a), bytes32(b), scalarByte);        
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the sub precompile.
        uint256 precompile = Precompiles.Subtract;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function mul(uint256 a, uint256 b, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        
        bytes memory input = bytes.concat(bytes32(a), bytes32(b), scalarByte);        
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the mul precompile.
        uint256 precompile = Precompiles.Multiply;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function and(uint256 a, uint256 b) internal view returns (uint256 result) {
        // scalars not currently supported for bitwise operators
        bytes memory input = bytes.concat(bytes32(a), bytes32(b), bytes1(0x00));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the AND precompile.
        uint256 precompile = Precompiles.BitwiseAnd;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function or(uint256 a, uint256 b) internal view returns (uint256 result) {
        // scalars not currently supported for bitwise operators
        bytes memory input = bytes.concat(bytes32(a), bytes32(b), bytes1(0x00));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the OR precompile.
        uint256 precompile = Precompiles.BitwiseOr;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function xor(uint256 a, uint256 b) internal view returns (uint256 result) {
        // scalars not currently supported for bitwise operators
        bytes memory input = bytes.concat(bytes32(a), bytes32(b), bytes1(0x00));
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the XOR precompile.
        uint256 precompile = Precompiles.BitwiseXor;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    // lhs << rhs
    function shl(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the left shift precompile.
        uint256 precompile = Precompiles.ShiftLeft;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    // lhs >> rhs
    function shr(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the right shift precompile.
        uint256 precompile = Precompiles.ShiftRight;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function eq(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the eq precompile.
        uint256 precompile = Precompiles.Equal;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function ne(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the not equal precompile.
        uint256 precompile = Precompiles.NotEqual;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function ge(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the ge precompile.
        uint256 precompile = Precompiles.GreaterThanOrEqual;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function gt(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the gt precompile.
        uint256 precompile = Precompiles.GreaterThan;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function le(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the le precompile.
        uint256 precompile = Precompiles.LessThanOrEqual;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function lt(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the lt precompile.
        uint256 precompile = Precompiles.LessThan;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function min(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the min precompile.
        uint256 precompile = Precompiles.Min;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function max(uint256 lhs, uint256 rhs, bool scalar) internal view returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        bytes memory input = bytes.concat(bytes32(lhs), bytes32(rhs), scalarByte);
        uint256 inputLen = input.length;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the max precompile.
        uint256 precompile = Precompiles.Max;
        assembly {
            if iszero(staticcall(gas(), precompile, add(input, 32), inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function neg(uint256 ct) internal view returns (uint256 result) {
        bytes32[1] memory input;
        input[0] = bytes32(ct);
        uint256 inputLen = 32;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the negation precompile.
        uint256 precompile = Precompiles.Negate;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function not(uint256 ct) internal view returns (uint256 result) {
        bytes32[1] memory input;
        input[0] = bytes32(ct);
        uint256 inputLen = 32;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the not precompile.
        uint256 precompile = Precompiles.Not;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    // If `control`'s value is 1, the result has the same value as `ifTrue`.
    // If `control`'s value is 0, the result has the same value as `ifFalse`.
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
            // jump over the 32-bit `size` field of the `bytes` data structure of the `input` to read actual bytes
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
            // jump over the 32-bit `size` field of the `bytes` data structure of the `input` to read actual bytes
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
            // jump over the 32-bit `size` field of the `bytes` data structure of the `input` to read actual bytes
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
}
"""
)
f.close()

f = open("TFHE.sol", "w")
f.write("""\
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

import "./Common.sol";
import "./Impl.sol";

library TFHE {
euint8 constant NIL8 = euint8.wrap(0);
euint16 constant NIL16 = euint16.wrap(0);
euint32 constant NIL32 = euint32.wrap(0);""")
        
to_print_is_initialized =  """
    // Return true if the enrypted integer is initialized and false otherwise.
    function isInitialized(euint{i} v) internal pure returns (bool) {{
        return euint{i}.unwrap(v) != 0;
    }}
"""

f.write(to_print_is_initialized.format(i=8))
f.write(to_print_is_initialized.format(i=16))
f.write(to_print_is_initialized.format(i=32))

to_print_no_cast =  """
    // Evaluate {f}(a, b) and return the result.
    function {f}(euint{i} a, euint{j} b) internal view returns (euint{k}) {{
        if (!isInitialized(a)) {{
            a = asEuint{i}(0);
        }}
        if (!isInitialized(b)) {{
            b = asEuint{j}(0);
        }}
        return euint{k}.wrap(Impl.{f}(euint{i}.unwrap(a), euint{j}.unwrap(b), false));
    }}
"""

to_print_cast_a =  """
    // Evaluate {f}(a, b) and return the result.
    function {f}(euint{i} a, euint{j} b) internal view returns (euint{k}) {{
        if (!isInitialized(a)) {{
            a = asEuint{i}(0);
        }}
        if (!isInitialized(b)) {{
            b = asEuint{j}(0);
        }}
        return euint{k}.wrap(Impl.{f}(euint{j}.unwrap(asEuint{j}(a)), euint{j}.unwrap(b), false));
    }}
"""

to_print_cast_b =  """
    // Evaluate {f}(a, b) and return the result.
    function {f}(euint{i} a, euint{j} b) internal view returns (euint{k}) {{
        if (!isInitialized(a)) {{
            a = asEuint{i}(0);
        }}
        if (!isInitialized(b)) {{
            b = asEuint{j}(0);
        }}
        return euint{k}.wrap(Impl.{f}(euint{i}.unwrap(a), euint{i}.unwrap(asEuint{i}(b)), false));
    }}
"""

to_print_no_scalar_no_cast = """
    // Evaluate {f}(a, b) and return the result.
    function {f}(euint{i} a, euint{j} b) internal view returns (euint{k}) {{
        if (!isInitialized(a)) {{
            a = asEuint{i}(0);
        }}
        if (!isInitialized(b)) {{
            b = asEuint{j}(0);
        }}
        return euint{k}.wrap(Impl.{f}(euint{i}.unwrap(a), euint{j}.unwrap(b)));
    }}
"""

to_print_no_scalar_cast_a = """
    // Evaluate {f}(a, b) and return the result.
    function {f}(euint{i} a, euint{j} b) internal view returns (euint{k}) {{
        if (!isInitialized(a)) {{
            a = asEuint{i}(0);
        }}
        if (!isInitialized(b)) {{
            b = asEuint{j}(0);
        }}
        return euint{k}.wrap(Impl.{f}(euint{j}.unwrap(asEuint{j}(a)), euint{j}.unwrap(b)));
    }}
"""

to_print_no_scalar_cast_b = """
// Evaluate {f}(a, b) and return the result.
function {f}(euint{i} a, euint{j} b) internal view returns (euint{k}) {{
        if (!isInitialized(a)) {{
            a = asEuint{i}(0);
        }}
        if (!isInitialized(b)) {{
            b = asEuint{j}(0);
        }}
        return euint{k}.wrap(Impl.{f}(euint{i}.unwrap(a), euint{i}.unwrap(asEuint{i}(b))));
    }}
"""

to_print_scalar = """
    // Evaluate {f}(a, b) and return the result.
    function {f}(euint{i} a, uint{i} b) internal view returns (euint{i}) {{
        if (!isInitialized(a)) {{
            a = asEuint{i}(0);
        }}
        return euint{i}.wrap(Impl.{f}(euint{i}.unwrap(a), uint256(b), true));
    }}

    // Evaluate {f}(a, b) and return the result.
    function {f}(uint{i} a, euint{i} b) internal view returns (euint{i}) {{
        if (!isInitialized(b)) {{
            b = asEuint{i}(0);
        }}
        return euint{i}.wrap(Impl.{g}(euint{i}.unwrap(b), uint256(a), true));
    }}
"""

for i in (2**p for p in range(3, 6)):
    for j in (2**p for p in range(3, 6)):
        if i == j:
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="add", g="add"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="sub", g="sub"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="mul", g="mul"))
            f.write(to_print_no_scalar_no_cast.format(i=i, j=j, k=i, f="and"))
            f.write(to_print_no_scalar_no_cast.format(i=i, j=j, k=i, f="or"))
            f.write(to_print_no_scalar_no_cast.format(i=i, j=j, k=i, f="xor"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="shl", g="shl"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="shr", g="shr"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="eq", g="eq"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="ne", g="ne"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="ge", g="le"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="gt", g="lt"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="le", g="ge"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="lt", g="gt"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="min", g="min"))
            f.write(to_print_no_cast.format(i=i, j=j, k=i, f="max", g="max"))
        elif i < j:
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="add", g="add"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="sub", g="sub"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="mul", g="mul"))
            f.write(to_print_no_scalar_cast_a.format(i=i, j=j, k=j, f="and"))
            f.write(to_print_no_scalar_cast_a.format(i=i, j=j, k=j, f="or"))
            f.write(to_print_no_scalar_cast_a.format(i=i, j=j, k=j, f="xor"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="shl", g="shl"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="shr", g="shr"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="eq", g="eq"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="ne", g="ne"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="ge", g="le"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="gt", g="lt"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="le", g="ge"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="lt", g="gt"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="min", g="min"))
            f.write(to_print_cast_a.format(i=i, j=j, k=j, f="max", g="max"))
        else:
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="add", g="add"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="sub", g="sub"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="mul", g="mul"))
            f.write(to_print_no_scalar_cast_b.format(i=i, j=j, k=i, f="and"))
            f.write(to_print_no_scalar_cast_b.format(i=i, j=j, k=i, f="or"))
            f.write(to_print_no_scalar_cast_b.format(i=i, j=j, k=i, f="xor"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="shl", g="shl"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="shr", g="shr"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="eq", g="eq"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="ne", g="ne"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="ge", g="le"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="gt", g="lt"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="le", g="ge"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="lt", g="gt"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="min", g="min"))
            f.write(to_print_cast_b.format(i=i, j=j, k=i, f="max", g="max"))
    f.write(to_print_scalar.format(i=i, f="add", g="add"))
    f.write(to_print_scalar.format(i=i, f="sub", g="sub"))
    f.write(to_print_scalar.format(i=i, f="mul", g="mul"))
    f.write(to_print_scalar.format(i=i, f="shl", g="shl"))
    f.write(to_print_scalar.format(i=i, f="shr", g="shr"))
    f.write(to_print_scalar.format(i=i, f="eq", g="eq"))
    f.write(to_print_scalar.format(i=i, f="ne", g="ne"))
    f.write(to_print_scalar.format(i=i, f="ge", g="le"))
    f.write(to_print_scalar.format(i=i, f="gt", g="lt"))
    f.write(to_print_scalar.format(i=i, f="le", g="ge"))
    f.write(to_print_scalar.format(i=i, f="lt", g="gt"))
    f.write(to_print_scalar.format(i=i, f="min", g="min"))
    f.write(to_print_scalar.format(i=i, f="max", g="max"))


to_print =  """
    // If `control`'s value is 1, the result has the same value as `a`.
    // If `control`'s value is 0, the result has the same value as `b`.
    function cmux(euint{i} control, euint{i} a, euint{i} b) internal view returns (euint{i}) {{
        return euint{i}.wrap(Impl.cmux(euint{i}.unwrap(control), euint{i}.unwrap(a), euint{i}.unwrap(b)));
    }}
"""
for i in (2**p for p in range(3, 6)):
    for j in (2**p for p in range(3, 6)):
        if i == j: # TODO: Decide whether we want to have mixed-inputs for CMUX
            f.write(to_print.format(i=i))

to_print="""
    // Cast an encrypted integer from euint{j} to euint{i}.
    function asEuint{i}(euint{j} value) internal view returns (euint{i}) {{
        return euint{i}.wrap(Impl.cast(euint{j}.unwrap(value), Common.euint{i}_t));
    }}
"""

for i in (2**p for p in range(3, 6)):
    for j in (2**p for p in range(3, 6)):
        if i != j:
            f.write(to_print.format(i=i, j=j))

to_print="""
    // Convert a serialized `ciphertext` to an encrypted euint{i} integer.
    function asEuint{i}(bytes memory ciphertext) internal view returns (euint{i}) {{
        return euint{i}.wrap(Impl.verify(ciphertext, Common.euint{i}_t));
    }}

    // Convert a plaintext value to an encrypted euint{i} integer.
    function asEuint{i}(uint256 value) internal view returns (euint{i}) {{
        return euint{i}.wrap(Impl.trivialEncrypt(value, Common.euint{i}_t));
    }}

    // Reencrypt the given `value` under the given `publicKey`.
    // Return a serialized euint{i} ciphertext.
    function reencrypt(euint{i} value, bytes32 publicKey) internal view returns (bytes memory reencrypted) {{
        return Impl.reencrypt(euint{i}.unwrap(value), publicKey);
    }}

    // Reencrypt the given `value` under the given `publicKey`.
    // If `value` is not initialized, the returned value will contain the `defaultValue` constant.
    // Return a serialized euint{i} ciphertext.
    function reencrypt(euint{i} value, bytes32 publicKey, uint{i} defaultValue) internal view returns (bytes memory reencrypted) {{
        if (euint{i}.unwrap(value) != 0) {{
            return Impl.reencrypt(euint{i}.unwrap(value), publicKey);
        }} else {{
            return Impl.reencrypt(euint{i}.unwrap(asEuint{i}(defaultValue)), publicKey);
        }}
    }}

    // Require that the encrypted `value` is not equal to 0.
    // Involves decrypting `value`.
    function req(euint{i} value) internal view {{
        Impl.req(euint{i}.unwrap(value));
    }}

    // Return the negation of `value`.
    function neg(euint{i} value) internal view returns (euint{i}) {{
        return euint{i}.wrap(Impl.neg(euint{i}.unwrap(value)));
    }}

    // Return `!value`.
    function not(euint{i} value) internal view returns (euint{i}) {{
        return euint{i}.wrap(Impl.not(euint{i}.unwrap(value)));
    }}
"""

to_print_cast_or="""
    // Optimistically require that `value` is not equal to 0.
    //
    // This function does not evaluate `value` at the time of the call.
    // Instead, it accumulates all optimistic requires and evaluates a single combined
    // require at the end of the transaction. A side effect of this mechanism
    // is that a method call with a failed optimistic require will always incur the full
    // gas cost, as if all optimistic requires were true. Yet, the transaction will be
    // reverted at the end if any of the optimisic requires were false.
    //
    // The benefit of optimistic requires is that they are faster than non-optimistic ones,
    // because there is a single call to the decryption oracle per transaction, irrespective
    // of how many optimistic requires were used.
    function optReq(euint{i} value) internal view {{
        Impl.optReq(euint8.unwrap(asEuint8(value)));
    }}
"""

to_print_no_cast_or="""
    // Optimistically require that `value` is not equal to 0.
    //
    // This function does not evaluate `value` at the time of the call.
    // Instead, it accumulates all optimistic requires and evaluates a single combined
    // require at the end of the transaction. A side effect of this mechanism
    // is that a method call with a failed optimistic require will always incur the full
    // gas cost, as if all optimistic requires were true. Yet, the transaction will be
    // reverted at the end if any of the optimisic requires were false.
    //
    // The benefit of optimistic requires is that they are faster than non-optimistic ones,
    // because there is a single call to the decryption oracle per transaction, irrespective
    // of how many optimistic requires were used.
    function optReq(euint{i} value) internal view {{
        Impl.optReq(euint{i}.unwrap(value));
    }}
"""

for i in (2**p for p in range(3, 6)):
    f.write(to_print.format(i=i))
    if i != 8:
        f.write(to_print_cast_or.format(i=i))
    else:
        f.write(to_print_no_cast_or.format(i=i))


f.write("\n")
f.write("""\
    // Return the network public FHE key.
    function fhePubKey() internal view returns (bytes memory) {
        return Impl.fhePubKey();
    }
""")

f.write("}\n")
f.close()