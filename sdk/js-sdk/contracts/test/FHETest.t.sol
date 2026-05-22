// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {console} from "forge-std-1.11.0/Script.sol";
import {Test} from "forge-std-1.11.0/Test.sol";
import {FHETest} from "../src/FHETest.sol";
import {LocalConfig} from "../src/LocalConfig.sol";
import {CoprocessorConfig} from "@fhevm/solidity/lib/Impl.sol";

string constant FHEVM_MNEMONIC = "test test test test test test test future home engine virtual motion";

interface IACL {
    // forge-lint: disable-next-line(mixed-case-function)
    function getFHEVMExecutorAddress() external view returns (address);
}

contract FHETestCoprocessorConfig is Test {
    FHETest fheTest;

    function setUp() public {
        // Compute FHETest address from mnemonic (index 0, nonce 0)
        uint32 index = uint32(vm.envOr("MNEMONIC_INDEX", uint256(0)));
        // forge-lint: disable-next-line(unsafe-cheatcode)
        uint256 deployerPrivateKey = vm.deriveKey(FHEVM_MNEMONIC, index);
        address deployer = vm.addr(deployerPrivateKey);
        address deployed = vm.computeCreateAddress(deployer, 0);

        if (deployed.code.length == 0) {
            console.log("======================================================================================");
            console.log("= FHETest is not yet deployed at:", deployed);
            console.log("= To deploy FHETest.sol call:");
            console.log("= forge script script/DeployFHETest.s.sol --rpc-url http://localhost:8545 --broadcast");
            console.log("======================================================================================");
            return;
        }

        fheTest = FHETest(deployed);
    }

    function test_coprocessorConfig_ACL() public view {
        CoprocessorConfig memory expected = LocalConfig.getLocalConfig();
        CoprocessorConfig memory actual = fheTest.getCoprocessorConfig();
        assertEq(actual.ACLAddress, expected.ACLAddress, "ACL address mismatch");
    }

    function test_coprocessorConfig_Coprocessor() public view {
        CoprocessorConfig memory expected = LocalConfig.getLocalConfig();
        CoprocessorConfig memory actual = fheTest.getCoprocessorConfig();
        assertEq(actual.CoprocessorAddress, expected.CoprocessorAddress, "Coprocessor address mismatch");
    }

    function test_coprocessorConfig_KMSVerifier() public view {
        CoprocessorConfig memory expected = LocalConfig.getLocalConfig();
        CoprocessorConfig memory actual = fheTest.getCoprocessorConfig();
        assertEq(actual.KMSVerifierAddress, expected.KMSVerifierAddress, "KMSVerifier address mismatch");
    }

    function test_getFHEVMExecutorAddress() public view {
        CoprocessorConfig memory cfg = fheTest.getCoprocessorConfig();

        IACL acl = IACL(cfg.ACLAddress);
        assertEq(acl.getFHEVMExecutorAddress(), cfg.CoprocessorAddress, "FHEVMExecutor address mismatch");
    }
}
