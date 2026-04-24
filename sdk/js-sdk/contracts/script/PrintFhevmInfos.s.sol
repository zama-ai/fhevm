// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "forge-std/Script.sol";
import "forge-std/console2.sol";
import {ACL} from "@fhevm/host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "@fhevm/host-contracts/contracts/FHEVMExecutor.sol";
import {HCULimit} from "@fhevm/host-contracts/contracts/HCULimit.sol";
import {InputVerifier} from "@fhevm/host-contracts/contracts/InputVerifier.sol";
import {KMSVerifier} from "@fhevm/host-contracts/contracts/KMSVerifier.sol";
import {PauserSet} from "@fhevm/host-contracts/contracts/immutable/PauserSet.sol";
import {FHETest} from "../src/FHETest.sol";
import "./libraries/FhevmCheatsBase.sol";

contract PrintFhevmInfos is Script, FhevmCheatsBase {
    uint256 private constant KMS_CONTEXT_COUNTER_BASE = uint256(0x07) << 248;

    function run() external {
        string memory json = string.concat(
            "{",
            _basicJson(),
            ",",
            _hostContractsJson(),
            ",",
            _versionsJson(),
            ",",
            _signersJson(),
            "}"
        );

        console2.log("JSON_RESULT_START");
        console2.log(json);
        console2.log("JSON_RESULT_END");
    }

    function _basicJson() internal view returns (string memory) {
        return string.concat(
            '"chainId":',
            vm.toString(block.chainid),
            ',"blockNumber":',
            vm.toString(block.number),
            ',"acl":"',
            vm.toString(fhevmCheats.acl()),
            '","fhevmExecutor":"',
            vm.toString(fhevmCheats.fhevmExecutor()),
            '","kmsVerifier":"',
            vm.toString(fhevmCheats.kmsVerifier()),
            '","inputVerifier":"',
            vm.toString(fhevmCheats.inputVerifier()),
            '"'
        );
    }

    function _hostContractsJson() internal view returns (string memory) {
        return string.concat(
            '"hcuLimit":"',
            vm.toString(fhevmCheats.hcuLimit()),
            '","pauserSet":"',
            vm.toString(fhevmCheats.pauserSet()),
            '","fheTest":"',
            vm.toString(fhevmCheats.fheTest()),
            '"'
        );
    }

    function _versionsJson() internal view returns (string memory) {
        return string.concat(
            '"aclVersion":"',
            ACL(fhevmCheats.acl()).getVersion(),
            '","fhevmExecutorVersion":"',
            FHEVMExecutor(fhevmCheats.fhevmExecutor()).getVersion(),
            '","kmsVerifierVersion":"',
            KMSVerifier(fhevmCheats.kmsVerifier()).getVersion(),
            '","inputVerifierVersion":"',
            InputVerifier(fhevmCheats.inputVerifier()).getVersion(),
            '","hcuLimitVersion":"',
            HCULimit(fhevmCheats.hcuLimit()).getVersion(),
            '","pauserSetVersion":"',
            PauserSet(fhevmCheats.pauserSet()).getVersion(),
            '","fheTestVersion":"',
            FHETest(fhevmCheats.fheTest()).CONTRACT_NAME(),
            '"'
        );
    }

    function _signersJson() internal view returns (string memory) {
        uint256 kmsCurrentContextId = KMSVerifier(fhevmCheats.kmsVerifier()).getCurrentKmsContextId();
        return string.concat(
            '"coprocessorThreshold":',
            vm.toString(InputVerifier(fhevmCheats.inputVerifier()).getThreshold()),
            ',"kmsThreshold":',
            vm.toString(KMSVerifier(fhevmCheats.kmsVerifier()).getThreshold()),
            ',"kmsCurrentContextId":',
            vm.toString(kmsCurrentContextId),
            ',"kmsCurrentContextIndex":',
            vm.toString(kmsCurrentContextId - KMS_CONTEXT_COUNTER_BASE),
            ',"coprocessorSigners":',
            _coprocessorSignersJson(),
            ',"kmsSigners":',
            _kmsSignersJson()
        );
    }

    function _coprocessorSignersJson() internal view returns (string memory) {
        address[] memory signers = InputVerifier(fhevmCheats.inputVerifier()).getCoprocessorSigners();
        return _addressArrayToJson(signers);
    }

    function _kmsSignersJson() internal view returns (string memory) {
        address[] memory signers = KMSVerifier(fhevmCheats.kmsVerifier()).getKmsSigners();
        return _addressArrayToJson(signers);
    }

    function _addressArrayToJson(address[] memory addrs) internal view returns (string memory) {
        bytes memory json = "[";

        for (uint256 i = 0; i < addrs.length; i++) {
            if (i > 0) {
                json = abi.encodePacked(json, ",");
            }
            json = abi.encodePacked(json, '"', vm.toString(addrs[i]), '"');
        }

        return string(abi.encodePacked(json, "]"));
    }
}
