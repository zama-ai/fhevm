// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {ZamaPolygonConfig, ZamaConfig, CoprocessorConfig} from "../config/ZamaConfig.sol";
import {CoprocessorConfig, Impl} from "../lib/Impl.sol";

contract TestFHEVMContract is ZamaPolygonConfig {
    function getCoprocessorConfig() public pure returns (CoprocessorConfig memory) {
        return Impl.getCoprocessorConfig();
    }
}

contract TestContract {
    function getPolygonCoprocessorConfig() public view returns (CoprocessorConfig memory) {
        CoprocessorConfig memory cfg = ZamaConfig.getPolygonCoprocessorConfig();
        return cfg;
    }
    function getConfidentialProtocolId() public view returns (uint256) {
        return ZamaConfig.getConfidentialProtocolId();
    }
}

contract PolygonConfigTest is Test {
    function setUp() public {
        vm.warp(1_000_000);
    }

    function test_ZamaConfigPolygonMainnet() public {
        vm.chainId(137);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 137
        CoprocessorConfig memory polygonCfg = ZamaConfig.getPolygonCoprocessorConfig();

        assertTrue(cfg.ACLAddress == 0x6737F17e31cf26a1b62fb0362acC5a16CB156F49);
        assertTrue(cfg.CoprocessorAddress == 0xAB0075E77fe06083f52bdf10e2ccDB3712483057);
        assertTrue(cfg.KMSVerifierAddress == 0x14e609595474874Dd6b6128376E336EfADfdBE37);

        assertTrue(cfg.ACLAddress == polygonCfg.ACLAddress);
        assertTrue(cfg.CoprocessorAddress == polygonCfg.CoprocessorAddress);
        assertTrue(cfg.KMSVerifierAddress == polygonCfg.KMSVerifierAddress);
    }

    function test_ZamaProtocolIdPolygonMainnet() public {
        vm.chainId(137);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        assertTrue(testFhevmContract.confidentialProtocolId() == 1);
        assertTrue(testFhevmContract.confidentialProtocolId() == ZamaConfig.getConfidentialProtocolId());
    }

    function test_ZamaConfigPolygonAmoy() public {
        vm.chainId(80002);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 80002
        CoprocessorConfig memory amoyCfg = ZamaConfig.getPolygonCoprocessorConfig();

        assertTrue(cfg.ACLAddress == 0xD99Cb9Fc3c42c87f2A4A12e8Fd60318d6bDdf985);
        assertTrue(cfg.CoprocessorAddress == 0x89420269f61e4db00545cd99da0aEcA7fF0912f9);
        assertTrue(cfg.KMSVerifierAddress == 0xCD1D89E311bce4C8DEa9a0857a0c9A4E153D4041);

        assertTrue(cfg.ACLAddress == amoyCfg.ACLAddress);
        assertTrue(cfg.CoprocessorAddress == amoyCfg.CoprocessorAddress);
        assertTrue(cfg.KMSVerifierAddress == amoyCfg.KMSVerifierAddress);
    }

    function test_ZamaProtocolIdPolygonAmoy() public {
        vm.chainId(80002);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        assertTrue(testFhevmContract.confidentialProtocolId() == 10001);
        assertTrue(testFhevmContract.confidentialProtocolId() == ZamaConfig.getConfidentialProtocolId());
    }

    function test_ZamaConfigLocalChainId() public {
        vm.chainId(31337);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 31337
        CoprocessorConfig memory localCfg = ZamaConfig.getPolygonCoprocessorConfig();

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

    function test_ZamaConfigGetPolygonCoprocessorConfigUnknownChainId() public {
        vm.chainId(123);

        TestContract testContract = new TestContract();

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        testContract.getPolygonCoprocessorConfig();
    }

    function test_ZamaConfigGetConfidentialProtocolIdUnknownChainId() public {
        vm.chainId(123);

        TestContract testContract = new TestContract();
        assertTrue(testContract.getConfidentialProtocolId() == 0);
    }
}
