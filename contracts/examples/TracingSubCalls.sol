// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;
import "../lib/TFHE.sol";

/// @notice Main contract for testing various subcalls and their behaviors
contract TracingSubCalls {
    /// @notice Executes a series of subcalls to test different scenarios
    /// @dev This function attempts various contract creations and function calls,
    ///      catching any errors to ensure the main execution continues
    function subCalls() external {
        try new SubContractCreate(400) {} catch {} // OK
        try new SubContractCreateFail(500) {} catch {}
        SubContract subc = new SubContract();
        try subc.succeed() {} catch {} // OK   601
        try subc.fail() {} catch {}
        try subc.succeedFail() {} catch {} // OK only for first   603
        try subc.failSucceed() {} catch {}
        try subc.oogFail{gas: 100000}() {} catch {} // This should fail out-of-gas
        try subc.succeed2() {} catch {} // OK   608
        try subc.invalidFail() {} catch {}
        try subc.succeedStop() {} catch {} // OK   610
        try subc.succeedSelfDestruct() {} catch {} // OK   611
        try new SubContractCreate{salt: keccak256("aaa")}(700) {} catch {} // OK   700
        try new SubContractCreateFail{salt: keccak256("aaa")}(800) {} catch {}
    }
}

/// @notice Contract that creates a new instance with an encrypted input
contract SubContractCreate {
    /// @dev Constructor that encrypts the input
    /// @param input The value to be encrypted
    constructor(uint64 input) {
        TFHE.asEuint64(input);
    }
}

/// @notice Contract that attempts to create a new instance but always fails
contract SubContractCreateFail {
    /// @dev Constructor that encrypts the input and then fails
    /// @param input The value to be encrypted before failing
    constructor(uint64 input) {
        TFHE.asEuint64(input);
        require(false, "This constructor always fails");
    }
}

/// @notice Contract with various test functions for success and failure scenarios
contract SubContract {
    /// @notice Function that always succeeds
    /// @dev Encrypts a specific value (601)
    function succeed() external {
        TFHE.asEuint64(601);
    }

    /// @notice Function that always fails
    /// @dev Encrypts a value (602) before failing
    function fail() external {
        TFHE.asEuint64(602);
        require(false, "This function always fails");
    }

    /// @notice Internal function that fails with a custom input
    /// @dev Encrypts the input before failing
    /// @param input The value to be encrypted before failing
    function fail2(uint64 input) external {
        TFHE.asEuint64(input);
        require(false, "This function always fails with custom input");
    }

    /// @notice Function that succeeds and then calls a failing function
    /// @dev Encrypts a value (603) and then attempts to call fail2
    function succeedFail() external {
        TFHE.asEuint64(603);
        try this.fail2(604) {} catch {}
    }

    /// @notice Function that attempts to fail and then succeed
    /// @dev Calls fail2 and then attempts to encrypt a value (606)
    function failSucceed() external {
        this.fail2(605);
        TFHE.asEuint64(606);
    }

    /// @notice Function that runs out of gas
    /// @dev Encrypts a value (607) and then enters an infinite loop
    function oogFail() external {
        TFHE.asEuint64(607);
        while (true) {}
    }

    /// @notice Another function that always succeeds
    /// @dev Encrypts a specific value (608)
    function succeed2() external {
        TFHE.asEuint64(608);
    }

    /// @notice Function that fails with an invalid operation
    /// @dev Encrypts a value (609) and then executes an invalid operation
    function invalidFail() external {
        TFHE.asEuint64(609);
        assembly {
            invalid()
        }
    }

    /// @notice Function that succeeds and then stops execution
    /// @dev Encrypts a value (610) and then stops the execution
    function succeedStop() external {
        TFHE.asEuint64(610);
        assembly {
            stop()
        }
    }

    /// @notice Function that succeeds and then self-destructs the contract
    /// @dev Encrypts a value (611) and then self-destructs the contract
    function succeedSelfDestruct() external {
        TFHE.asEuint64(611);
        selfdestruct(payable(address(1)));
    }
}
