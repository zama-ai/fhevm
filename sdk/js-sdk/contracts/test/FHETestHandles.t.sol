// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {console} from "forge-std-1.11.0/Script.sol";
import {Test} from "forge-std-1.11.0/Test.sol";
import {FHETest} from "../src/FHETest.sol";
import {CoprocessorConfig} from "@fhevm/solidity/lib/Impl.sol";
import {FHE, ebool, euint8, euint16, euint32, euint64, euint128, euint256} from "@fhevm/solidity/lib/FHE.sol";

// forge test --match-contract FHETestHandles --rpc-url http://localhost:8545 -vvv

string constant FHEVM_MNEMONIC = "test test test test test test test future home engine virtual motion";

interface IACLDecryption {
    function isAllowedForDecryption(bytes32 handle) external view returns (bool);
}

contract FHETestHandles is Test {
    FHETest fheTest;
    address deployer;
    uint256 deployerPrivateKey;
    IACLDecryption acl;

    function setUp() public {
        uint32 index = uint32(vm.envOr("MNEMONIC_INDEX", uint256(0)));
        // forge-lint: disable-next-line(unsafe-cheatcode)
        deployerPrivateKey = vm.deriveKey(FHEVM_MNEMONIC, index);
        deployer = vm.addr(deployerPrivateKey);
        address deployed = vm.computeCreateAddress(deployer, 0);

        if (deployed.code.length == 0) {
            console.log("======================================================================================");
            console.log("= FHETest is not yet deployed at:", deployed);
            console.log("= To deploy, run:");
            console.log("= forge script script/DeployFHETest.s.sol --rpc-url http://localhost:8545 --broadcast");
            console.log("======================================================================================");
            return;
        }

        fheTest = FHETest(deployed);
        CoprocessorConfig memory cfg = fheTest.getCoprocessorConfig();
        acl = IACLDecryption(cfg.ACLAddress);
    }

    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    modifier asDeployer() {
        vm.prank(deployer);
        _;
    }

    function _logHandle(string memory label, bytes32 handle) internal pure {
        console.log(string.concat("  ", label, ":"), vm.toString(handle));
    }

    // -------------------------------------------------------------------------
    // Tests: handles are initialized (non-zero) for the deployer
    // -------------------------------------------------------------------------

    function test_ebool_isInitialized() public asDeployer {
        ebool v = fheTest.getEbool();
        _logHandle("ebool", ebool.unwrap(v));
        assertTrue(FHE.isInitialized(v), "ebool not initialized for deployer");
    }

    function test_euint8_isInitialized() public asDeployer {
        euint8 v = fheTest.getEuint8();
        _logHandle("euint8", euint8.unwrap(v));
        assertTrue(FHE.isInitialized(v), "euint8 not initialized for deployer");
    }

    function test_euint16_isInitialized() public asDeployer {
        euint16 v = fheTest.getEuint16();
        _logHandle("euint16", euint16.unwrap(v));
        assertTrue(FHE.isInitialized(v), "euint16 not initialized for deployer");
    }

    function test_euint32_isInitialized() public asDeployer {
        euint32 v = fheTest.getEuint32();
        _logHandle("euint32", euint32.unwrap(v));
        assertTrue(FHE.isInitialized(v), "euint32 not initialized for deployer");
    }

    function test_euint64_isInitialized() public asDeployer {
        euint64 v = fheTest.getEuint64();
        _logHandle("euint64", euint64.unwrap(v));
        assertTrue(FHE.isInitialized(v), "euint64 not initialized for deployer");
    }

    function test_euint128_isInitialized() public asDeployer {
        euint128 v = fheTest.getEuint128();
        _logHandle("euint128", euint128.unwrap(v));
        assertTrue(FHE.isInitialized(v), "euint128 not initialized for deployer");
    }

    function test_euint256_isInitialized() public asDeployer {
        euint256 v = fheTest.getEuint256();
        _logHandle("euint256", euint256.unwrap(v));
        assertTrue(FHE.isInitialized(v), "euint256 not initialized for deployer");
    }

    // -------------------------------------------------------------------------
    // Tests: handles are allowed for decryption (publicly decryptable)
    // -------------------------------------------------------------------------

    function test_ebool_isAllowedForDecryption() public asDeployer {
        bytes32 handle = ebool.unwrap(fheTest.getEbool());
        assertTrue(acl.isAllowedForDecryption(handle), "ebool not allowed for decryption");
    }

    function test_euint8_isAllowedForDecryption() public asDeployer {
        bytes32 handle = euint8.unwrap(fheTest.getEuint8());
        assertTrue(acl.isAllowedForDecryption(handle), "euint8 not allowed for decryption");
    }

    function test_euint16_isAllowedForDecryption() public asDeployer {
        bytes32 handle = euint16.unwrap(fheTest.getEuint16());
        assertTrue(acl.isAllowedForDecryption(handle), "euint16 not allowed for decryption");
    }

    function test_euint32_isAllowedForDecryption() public asDeployer {
        bytes32 handle = euint32.unwrap(fheTest.getEuint32());
        assertTrue(acl.isAllowedForDecryption(handle), "euint32 not allowed for decryption");
    }

    function test_euint64_isAllowedForDecryption() public asDeployer {
        bytes32 handle = euint64.unwrap(fheTest.getEuint64());
        assertTrue(acl.isAllowedForDecryption(handle), "euint64 not allowed for decryption");
    }

    function test_euint128_isAllowedForDecryption() public asDeployer {
        bytes32 handle = euint128.unwrap(fheTest.getEuint128());
        assertTrue(acl.isAllowedForDecryption(handle), "euint128 not allowed for decryption");
    }

    function test_euint256_isAllowedForDecryption() public asDeployer {
        bytes32 handle = euint256.unwrap(fheTest.getEuint256());
        assertTrue(acl.isAllowedForDecryption(handle), "euint256 not allowed for decryption");
    }
}
