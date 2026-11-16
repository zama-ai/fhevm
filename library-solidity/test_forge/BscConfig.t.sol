// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {ZamaBscConfig, ZamaConfig, CoprocessorConfig} from "../config/ZamaConfig.sol";
import {CoprocessorConfig, Impl} from "../lib/Impl.sol";

contract TestFHEVMContract is ZamaBscConfig {
    function getCoprocessorConfig() public pure returns (CoprocessorConfig memory) {
        return Impl.getCoprocessorConfig();
    }
}

contract TestNoFHEVMContract {
    function getBscCoprocessorConfig() public view returns (CoprocessorConfig memory) {
        CoprocessorConfig memory cfg = ZamaConfig.getBscCoprocessorConfig();
        return cfg;
    }
    function getConfidentialProtocolId() public view returns (uint256) {
        return ZamaConfig.getConfidentialProtocolId();
    }
}

contract BscConfigTest is Test {
    function setUp() public {
        vm.warp(1_000_000);
    }

    function test_ZamaConfigBscMainnet() public {
        // BSC mainnet (chainid == 56) is not supported yet
        vm.chainId(56);

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        new TestFHEVMContract();

        TestNoFHEVMContract testNoFhevmContract = new TestNoFHEVMContract();

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        testNoFhevmContract.getBscCoprocessorConfig();
    }

    function test_ZamaProtocolIdBscMainnet() public {
        // BSC mainnet (chainid == 56) is not supported yet
        vm.chainId(56);

        TestNoFHEVMContract testNoFhevmContract = new TestNoFHEVMContract();
        assertTrue(testNoFhevmContract.getConfidentialProtocolId() == 0);
    }

    function test_ZamaConfigBscTestnet() public {
        vm.chainId(97);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 97
        CoprocessorConfig memory bscTestnetCfg = ZamaConfig.getBscCoprocessorConfig();

        assertTrue(cfg.ACLAddress == 0x9Adac873Ae2f737AF011f0778305Cff0B220ac36);
        assertTrue(cfg.CoprocessorAddress == 0x24f6bBCd97E2FE90E43cb9C32B89A9828d8Ece08);
        assertTrue(cfg.KMSVerifierAddress == 0x1F7CAD0c05fDED5F988683AdfDB4F242b310cdcB);

        assertTrue(cfg.ACLAddress == bscTestnetCfg.ACLAddress);
        assertTrue(cfg.CoprocessorAddress == bscTestnetCfg.CoprocessorAddress);
        assertTrue(cfg.KMSVerifierAddress == bscTestnetCfg.KMSVerifierAddress);
    }

    function test_ZamaProtocolIdBscTestnet() public {
        vm.chainId(97);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        assertTrue(testFhevmContract.confidentialProtocolId() == 10001);
        assertTrue(testFhevmContract.confidentialProtocolId() == ZamaConfig.getConfidentialProtocolId());
    }

    function test_ZamaConfigLocalChainId() public {
        vm.chainId(31337);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 31337
        CoprocessorConfig memory localCfg = ZamaConfig.getBscCoprocessorConfig();

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

    function test_ZamaConfigGetBscCoprocessorConfigUnknownChainId() public {
        vm.chainId(123);

        TestNoFHEVMContract testNoFhevmContract = new TestNoFHEVMContract();

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        testNoFhevmContract.getBscCoprocessorConfig();
    }

    function test_ZamaConfigGetBscCoprocessorConfigEthereumMainnetSeparation() public {
        vm.chainId(1);

        TestNoFHEVMContract testNoFhevmContract = new TestNoFHEVMContract();

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        testNoFhevmContract.getBscCoprocessorConfig();
    }

    function test_ZamaConfigGetBscCoprocessorConfigSepoliaSeparation() public {
        vm.chainId(11155111);

        TestNoFHEVMContract testNoFhevmContract = new TestNoFHEVMContract();

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        testNoFhevmContract.getBscCoprocessorConfig();
    }

    function test_ZamaConfigGetBscCoprocessorConfigPolygonMainnetSeparation() public {
        vm.chainId(137);

        TestNoFHEVMContract testNoFhevmContract = new TestNoFHEVMContract();

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        testNoFhevmContract.getBscCoprocessorConfig();
    }

    function test_ZamaConfigGetBscCoprocessorConfigPolygonAmoySeparation() public {
        vm.chainId(80002);

        TestNoFHEVMContract testNoFhevmContract = new TestNoFHEVMContract();

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        testNoFhevmContract.getBscCoprocessorConfig();
    }

    function test_ZamaConfigGetConfidentialProtocolIdUnknownChainId() public {
        vm.chainId(123);

        TestNoFHEVMContract testNoFhevmContract = new TestNoFHEVMContract();
        assertTrue(testNoFhevmContract.getConfidentialProtocolId() == 0);
    }
}
