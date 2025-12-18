// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {ZamaEthereumConfig, ZamaConfig, CoprocessorConfig} from "../config/ZamaConfig.sol";
import {CoprocessorConfig, Impl} from "../lib/Impl.sol";

contract TestFHEVMContract is ZamaEthereumConfig {
    function getCoprocessorConfig() public pure returns (CoprocessorConfig memory) {
        return Impl.getCoprocessorConfig();
    }
}

contract TestContract {
    function getEthereumCoprocessorConfig() public view returns (CoprocessorConfig memory) {
        CoprocessorConfig memory cfg = ZamaConfig.getEthereumCoprocessorConfig();
        return cfg;
    }
    function getConfidentialProtocolId() public view returns (uint256) {
        return ZamaConfig.getConfidentialProtocolId();
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
        // chainid == 1
        CoprocessorConfig memory ethCfg = ZamaConfig.getEthereumCoprocessorConfig();

        assertTrue(cfg.ACLAddress == ethCfg.ACLAddress);
        assertTrue(cfg.CoprocessorAddress == ethCfg.CoprocessorAddress);
        assertTrue(cfg.KMSVerifierAddress == ethCfg.KMSVerifierAddress);

        assertTrue(cfg.ACLAddress == 0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6);
        assertTrue(cfg.CoprocessorAddress == 0xD82385dADa1ae3E969447f20A3164F6213100e75);
        assertTrue(cfg.KMSVerifierAddress == 0x77627828a55156b04Ac0DC0eb30467f1a552BB03);
    }

    function test_ZamaProtocolIdEthereum() public {
        vm.chainId(1);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        assertTrue(testFhevmContract.confidentialProtocolId() == 1);
        assertTrue(testFhevmContract.confidentialProtocolId() == ZamaConfig.getConfidentialProtocolId());
    }

    function test_ZamaConfigSepolia() public {
        vm.chainId(11155111);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 11155111
        CoprocessorConfig memory sepoliaCfg = ZamaConfig.getEthereumCoprocessorConfig();

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
        assertTrue(testFhevmContract.confidentialProtocolId() == 10001);
        assertTrue(testFhevmContract.confidentialProtocolId() == ZamaConfig.getConfidentialProtocolId());
    }

    function test_ZamaConfigLocalChainId() public {
        vm.chainId(31337);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 31337
        CoprocessorConfig memory localCfg = ZamaConfig.getEthereumCoprocessorConfig();

        assertTrue(cfg.ACLAddress == 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D);
        assertTrue(cfg.CoprocessorAddress == 0xe3a9105a3a932253A70F126eb1E3b589C643dD24);
        assertTrue(cfg.KMSVerifierAddress == 0x901F8942346f7AB3a01F6D7613119Bca447Bb030);

        assertTrue(cfg.ACLAddress == localCfg.ACLAddress);
        assertTrue(cfg.CoprocessorAddress == localCfg.CoprocessorAddress);
        assertTrue(cfg.KMSVerifierAddress == localCfg.KMSVerifierAddress);
    }

    function test_ZamaProtocolIdLocalChainId() public {
        vm.chainId(31337);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        assertTrue(testFhevmContract.confidentialProtocolId() == type(uint256).max);
        assertTrue(testFhevmContract.confidentialProtocolId() == ZamaConfig.getConfidentialProtocolId());
    }

    function test_ZamaConfigUnknownChainId() public {
        vm.chainId(123);

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        new TestFHEVMContract();
    }

    function test_ZamaConfigGetEthereumCoprocessorConfigUnknownChainId() public {
        vm.chainId(123);

        TestContract testContract = new TestContract();

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        testContract.getEthereumCoprocessorConfig();
    }

    function test_ZamaConfigGetConfidentialProtocolIdUnknownChainId() public {
        vm.chainId(123);

        TestContract testContract = new TestContract();
        assertTrue(testContract.getConfidentialProtocolId() == 0);
    }
}
