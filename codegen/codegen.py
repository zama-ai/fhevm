### Generation script for boilerplate FHEUInt types

f = open("FHEOps.sol", "w")
f.write("""\
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

type FHEUInt8 is uint256;
type FHEUInt16 is uint256;
type FHEUInt32 is uint256;
type FHEUInt64 is uint256;
type FHEUInt128 is uint256;
type FHEUInt256 is uint256;

library Common {
// Values used to communicate types at runtime to the cast() precompile.
    uint8 internal constant typeUInt8 = 0;
    uint8 internal constant typeUInt16 = 1;
    uint8 internal constant typeUInt32 = 2;
    uint8 internal constant typeUInt64 = 3;
    uint8 internal constant typeUInt128 = 4;
    uint8 internal constant typeUInt256 = 5;
}

library FHEOps {""")

to_print =  """
    function {f}(FHEUInt{i} a, FHEUInt{j} b) internal view returns (FHEUInt{k}) {{
        return FHEUInt{k}.wrap(Impl.{f}(FHEUInt{i}.unwrap(a), FHEUInt{j}.unwrap(b)));
    }}
"""

for i in (2**p for p in range(3, 9)):
    for j in (2**p for p in range(3, 9)):
        f.write(to_print.format(i=i, j=j, k=i if i>j else j, f="add"))
        f.write(to_print.format(i=i, j=j, k=i if i>j else j, f="sub"))
        f.write(to_print.format(i=i, j=j, k=8, f="lte"))

to_print="""
    function toFHEUint{i}(FHEUInt{j} v) internal view returns (FHEUInt{i}) {{
        return FHEUInt{i}.wrap(Impl.cast(FHEUInt{j}.unwrap(v), Common.typeUInt{j}));
    }}
"""

for i in (2**p for p in range(3, 9)):
    for j in (2**q for q in range(3, 9)):
        if (i != j):
            f.write(to_print.format(i=i, j=j))

f.write("}")

f.write("""
library Impl {
    uint256 constant MaxCiphertextBytesLen = 32 + 65544;

    function add(uint256 a, uint256 b) internal view returns (uint256 result) {
        if (a == 0) {
            return b;
        } else if (b == 0) {
            return a;
        }
        bytes32[2] memory input;
        input[0] = bytes32(a);
        input[1] = bytes32(b);
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the add precompile.
        uint256 precompile = Precompiles.Add;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    function sub(uint256 a, uint256 b) internal view returns (uint256 result) {
        if (a == 0) {
            return b;
        } else if (b == 0) {
            return a;
        }
        bytes32[2] memory input;
        input[0] = bytes32(a);
        input[1] = bytes32(b);
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the add precompile.
        uint256 precompile = Precompiles.Subtract;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }

    // Evaluate `lhs <= rhs` on the given ciphertexts and, if successful, return the resulting ciphertext.
    // If successful, the resulting ciphertext is automatically verified.
    function lte(uint256 lhs, uint256 rhs) internal view returns (uint256 result) {
        bytes32[2] memory input;
        input[0] = bytes32(lhs);
        input[1] = bytes32(rhs);
        uint256 inputLen = 64;

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the lte precompile.
        uint256 precompile = Precompiles.LessThanOrEqual;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, output, outputLen)) {
                revert(0, 0)
            }
        }

        result = uint256(output[0]);
    }
    

//    function safeAdd(uint256 a, uint256 b) internal view returns (uint256) {
//        TODO: Call addSafe() precompile.
//        return 0;
//    }

    function cast(uint256 ciphertext, uint8 toType) internal view returns(uint256) {
        uint256 inputLen = 33;

        bytes memory input = new bytes(inputLen);

        assembly {
            mstore(add(input, 32), ciphertext)
        }

        // Pass in the desired return type
        input[inputLen - 1] = bytes1(toType);

        bytes32[1] memory output;
        uint256 outputLen = 32;

        // Call the cast precompile.
        uint256 precompile = Precompiles.Cast;
        assembly {
            if iszero(
                staticcall(
                    gas(),
                    precompile,
                    add(input, 32), // jump over the 32-bit `size` field of the `bytes` data structure to read actual bytes
                    inputLen,
                    output,
                    outputLen
                )
            ) {
                revert(0, 0)
            }
        }
        return 0;
    }

    function reencrypt(uint256 ciphertext) internal view returns (bytes memory reencrypted) {
        bytes32[1] memory input;
        input[0] = bytes32(ciphertext);
        uint256 inputLen = 32;

        reencrypted = new bytes(MaxCiphertextBytesLen);

        // Call the reencrypt precompile.
        uint256 precompile = Precompiles.Reencrypt;
        assembly {
            if iszero(
                staticcall(
                    gas(),
                    precompile,
                    input,
                    inputLen,
                    reencrypted,
                    MaxCiphertextBytesLen
                )
            ) { 
                revert(0, 0)
            }
        }
    }

    function verify(
        bytes memory _ciphertextWithProof,
        uint8 _toType
    ) internal view returns (uint256) {
        // TODO depending the TFHE-rs implementation of the type system.
        return 0;
    }

    function delegate(uint256 ciphertext) internal view {
        bytes32[1] memory input;
        input[0] = bytes32(ciphertext);
        uint256 inputLen = 32;

        // Call the delegate precompile
        uint256 precompile = Precompiles.Delegate;
        assembly {
            if iszero(staticcall(gas(), precompile, input, inputLen, 0, 0)) {
                revert(0, 0)
            }
        }
    }

    function requireCt(uint256 ciphertext) internal view {
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
""")

f.write("""
library Precompiles {
    uint256 public constant Add = 65;
    uint256 public constant Verify = 66;
    uint256 public constant Reencrypt = 67;
    uint256 public constant Delegate = 68;
    uint256 public constant Require = 69;
    uint256 public constant LessThanOrEqual = 70;
    uint256 public constant Subtract = 71;
    uint256 public constant Cast = 72;
}
""")

f.write("""
library Ciphertext {""")

to_print="""
    function verify{i}(bytes memory ciphertextWithProof) internal view returns (FHEUInt{i}) {{
        return FHEUInt{i}.wrap(Impl.verify(ciphertextWithProof, Common.typeUInt{i}));
    }}

    function reencrypt(FHEUInt{i} ciphertext) internal view returns (bytes memory reencrypted) {{
        return Impl.reencrypt(FHEUInt{i}.unwrap(ciphertext));
    }}

    function delegate(FHEUInt{i} ciphertext) internal view {{
        Impl.delegate(FHEUInt{i}.unwrap(ciphertext));
    }}
"""

for i in (2**p for p in range(3, 9)):
    f.write(to_print.format(i=i))

f.write("""
    function requireCt(FHEUInt8 ciphertext) internal view {{
        Impl.requireCt(FHEUInt8.unwrap(ciphertext));
    }}
""")

f.write("}")

f.close()