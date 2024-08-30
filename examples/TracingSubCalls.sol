// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;
import "../lib/TFHE.sol";

contract TracingSubCalls {
    function subCalls() external {
        try new SubContractCreate(400) {} catch {} // 0K
        try new SubContractCreateFail(500) {} catch {}
        SubContract subc = new SubContract();
        try subc.succeed() {} catch {} // OK   601
        try subc.fail() {} catch {}
        try subc.succeedFail() {} catch {} // OK only for first   603
        try subc.failSucceed() {} catch {}
        try subc.oogFail{gas: 100000}() {} catch {} // this should fail out-of-gas
        try subc.succeed2() {} catch {} // OK   608
        try subc.invalidFail() {} catch {}
        try subc.succeedStop() {} catch {} // OK   610
        try subc.succeedSelfDestruct() {} catch {} // OK   611
        try new SubContractCreate{salt: keccak256("aaa")}(700) {} catch {} // 0K   700
        try new SubContractCreateFail{salt: keccak256("aaa")}(800) {} catch {}
    }
}

contract SubContractCreate {
    constructor(uint256 input) {
        TFHE.asEuint64(input);
    }
}

contract SubContractCreateFail {
    constructor(uint256 input) {
        TFHE.asEuint64(input);
        require(false);
    }
}

contract SubContract {
    function succeed() external {
        TFHE.asEuint64(601);
    }

    function fail() external {
        TFHE.asEuint64(602);
        require(false);
    }

    function fail2(uint input) external {
        TFHE.asEuint64(input);
        require(false);
    }

    function succeedFail() external {
        TFHE.asEuint64(603);
        try this.fail2(604) {} catch {}
    }

    function failSucceed() external {
        this.fail2(605);
        TFHE.asEuint64(606);
    }

    function oogFail() external {
        TFHE.asEuint64(607);
        while (true) {}
    }

    function succeed2() external {
        TFHE.asEuint64(608);
    }

    function invalidFail() external {
        TFHE.asEuint64(609);
        assembly {
            invalid()
        }
    }

    function succeedStop() external {
        TFHE.asEuint64(610);
        assembly {
            stop()
        }
    }

    function succeedSelfDestruct() external {
        TFHE.asEuint64(611);
        selfdestruct(payable(address(1)));
    }
}
