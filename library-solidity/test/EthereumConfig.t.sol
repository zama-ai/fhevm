// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {EthereumConfig, ZamaConfig, CoprocessorConfig} from "../config/ZamaConfig.sol";
import {CoprocessorConfig, Impl} from "../lib/Impl.sol";

contract TestFHEVMContract is EthereumConfig {
    function getCoprocessorConfig() public pure returns (CoprocessorConfig memory) {
        return Impl.getCoprocessorConfig();
    }
}

contract EthereumConfigTest is Test {
    function setUp() public {
        vm.warp(1_000_000);
    }

    function test_ZamaConfigEthereum() public {
        vm.chainId(1);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        CoprocessorConfig memory ethCfg = ZamaConfig.getEthereumConfig();

        assertTrue(cfg.ACLAddress == ethCfg.ACLAddress);
        assertTrue(cfg.CoprocessorAddress == ethCfg.CoprocessorAddress);
        assertTrue(cfg.KMSVerifierAddress == ethCfg.KMSVerifierAddress);

        assertTrue(cfg.ACLAddress == address(0));
        assertTrue(cfg.CoprocessorAddress == address(0));
        assertTrue(cfg.KMSVerifierAddress == address(0));
    }

    function test_ZamaProtocolIdEthereum() public {
        vm.chainId(1);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        assertTrue(testFhevmContract.protocolId() == 1);
    }

    function test_ZamaConfigSepolia() public {
        vm.chainId(11155111);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        CoprocessorConfig memory sepoliaCfg = ZamaConfig.getSepoliaConfig();

        assertTrue(cfg.ACLAddress == 0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D);
        assertTrue(cfg.CoprocessorAddress == 0x92C920834Ec8941d2C77D188936E1f7A6f49c127);
        assertTrue(cfg.KMSVerifierAddress == 0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A);

        assertTrue(cfg.ACLAddress == sepoliaCfg.ACLAddress);
        assertTrue(cfg.CoprocessorAddress == sepoliaCfg.CoprocessorAddress);
        assertTrue(cfg.KMSVerifierAddress == sepoliaCfg.KMSVerifierAddress);
    }

    function test_ZamaProtocolIdSepolia() public {
        vm.chainId(11155111);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        assertTrue(testFhevmContract.protocolId() == 10001);
    }

    function test_ZamaConfigUnknownChainId() public {
        vm.chainId(123);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();

        assertTrue(cfg.ACLAddress == address(0));
        assertTrue(cfg.CoprocessorAddress == address(0));
        assertTrue(cfg.KMSVerifierAddress == address(0));
    }

    function test_ZamaProtocolIdUnknownChainId() public {
        vm.chainId(123);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        assertTrue(testFhevmContract.protocolId() == 0);
    }
}
