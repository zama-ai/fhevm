// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.25;

import "./ACL.sol";
import "./ACLAddress.sol";
import "./FhevmLib.sol";

address constant EXT_TFHE_LIBRARY = address(0x000000000000000000000000000000000000005d);

contract TFHEExecutor {
    ACL private constant acl = ACL(address(aclAdd));

    function fheAdd(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheAdd(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheSub(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheSub(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheMul(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheMul(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheDiv(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheDiv(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheRem(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheRem(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheBitAnd(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));
        require(acl.isAllowed(rhs, msg.sender));
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheBitAnd(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheBitOr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));
        require(acl.isAllowed(rhs, msg.sender));
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheBitOr(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheBitXor(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));
        require(acl.isAllowed(rhs, msg.sender));
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheBitXor(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheShl(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheShl(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheShr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheShr(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheRotl(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheRotl(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheRotr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheRotr(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheEq(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheEq(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheNe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheNe(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheGe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheGe(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheGt(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheGt(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheLe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheLe(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheLt(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheLt(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheMin(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheMin(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheMax(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result) {
        require(acl.isAllowed(lhs, msg.sender));

        if (scalarByte == 0x00) {
            require(acl.isAllowed(rhs, msg.sender));
        }

        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheMax(lhs, rhs, scalarByte);
        acl.allowTransient(result, msg.sender);
    }

    function fheNeg(uint256 ct) external returns (uint256 result) {
        require(acl.isAllowed(ct, msg.sender));
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheNeg(ct);
        acl.allowTransient(result, msg.sender);
    }

    function fheNot(uint256 ct) external returns (uint256 result) {
        require(acl.isAllowed(ct, msg.sender));
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheNot(ct);
        acl.allowTransient(result, msg.sender);
    }

    function verifyCiphertext(
        bytes32 inputHandle,
        address callerAddress,
        bytes memory inputProof,
        bytes1 inputType
    ) external returns (uint256 result) {
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).verifyCiphertext(
            inputHandle,
            callerAddress,
            inputProof,
            inputType
        );
        acl.allowTransient(result, msg.sender);
    }

    function cast(uint256 ct, bytes1 toType) external returns (uint256 result) {
        require(acl.isAllowed(ct, msg.sender));
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).cast(ct, toType);
        acl.allowTransient(result, msg.sender);
    }

    function trivialEncrypt(uint256 plaintext, bytes1 toType) external returns (uint256 result) {
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).trivialEncrypt(plaintext, toType);
        acl.allowTransient(result, msg.sender);
    }

    function fheIfThenElse(uint256 control, uint256 ifTrue, uint256 ifFalse) external returns (uint256 result) {
        require(acl.isAllowed(control, msg.sender));
        require(acl.isAllowed(ifTrue, msg.sender));
        require(acl.isAllowed(ifFalse, msg.sender));
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheIfThenElse(control, ifTrue, ifFalse);
        acl.allowTransient(result, msg.sender);
    }

    function fheRand(bytes1 randType) external returns (uint256 result) {
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheRand(randType, 0);
        acl.allowTransient(result, msg.sender);
    }

    function fheRandBounded(uint256 upperBound, bytes1 randType) external returns (uint256 result) {
        result = FhevmLib(address(EXT_TFHE_LIBRARY)).fheRandBounded(upperBound, randType, 0);
        acl.allowTransient(result, msg.sender);
    }

    function cleanTransientStorage() external {}
}
