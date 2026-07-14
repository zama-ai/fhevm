// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

interface IExecutor {
    function verifyInput(bytes32 inputHandle, address userAddress, bytes calldata inputProof, uint8 inputType)
        external
        returns (bytes32);
    function trivialEncrypt(uint256 pt, uint8 toType) external returns (bytes32);
    function fheAdd(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32);
}

interface IACL {
    function allow(bytes32 handle, address account) external;
    function allowForDecryption(bytes32[] memory handlesList) external;
}

/**
 * Stands in for a user contract, doing by hand exactly what `FHE.sol` would emit:
 *
 *   euint64 x = FHE.fromExternal(inputHandle, inputProof);   -> executor.verifyInput
 *   euint64 y = FHE.add(x, FHE.asEuint64(addend));           -> executor.trivialEncrypt + fheAdd
 *   FHE.allowThis(y); FHE.allow(y, user);                    -> acl.allow x2
 *   FHE.makePubliclyDecryptable(y);                          -> acl.allowForDecryption
 *
 * All of it must happen in ONE transaction: `verifyInput` grants only a TRANSIENT ACL allowance, which is
 * cleared at the end of the call, so the `allow` calls that persist it cannot be split into a later tx.
 * That is the whole reason this contract exists rather than driving the executor from the test directly.
 *
 * The two addresses are the ones `ZamaConfig._getLocalConfig()` pins — the same ones the plugin places the
 * stack at. If the plugin's patching were wrong, these calls would hit empty accounts.
 */
contract Harness {
    address constant ACL = 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D;
    address constant EXEC = 0xe3a9105a3a932253A70F126eb1E3b589C643dD24;

    uint8 constant EUINT64 = 5; // FheType.Uint64
    bytes1 constant NON_SCALAR = 0x00; // both operands are handles

    bytes32 public result;

    function ingestAndAdd(bytes32 inputHandle, bytes calldata inputProof, address user, uint256 addend)
        external
        returns (bytes32)
    {
        bytes32 x = IExecutor(EXEC).verifyInput(inputHandle, user, inputProof, EUINT64);
        bytes32 c = IExecutor(EXEC).trivialEncrypt(addend, EUINT64);
        bytes32 y = IExecutor(EXEC).fheAdd(x, c, NON_SCALAR);

        IACL(ACL).allow(y, address(this));
        IACL(ACL).allow(y, user);

        bytes32[] memory list = new bytes32[](1);
        list[0] = y;
        IACL(ACL).allowForDecryption(list);

        result = y;
        return y;
    }
}
