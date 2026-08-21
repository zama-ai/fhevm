// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {ZamaMultiChainConfig, ZamaConfig, CoprocessorConfig} from "../config/ZamaConfig.sol";
import {CoprocessorConfig, Impl} from "../lib/Impl.sol";

contract TestFHEVMContract is ZamaMultiChainConfig {
    function getCoprocessorConfig() public pure returns (CoprocessorConfig memory) {
        return Impl.getCoprocessorConfig();
    }
}

contract TestContract {
    function getCoprocessorConfig() public view returns (CoprocessorConfig memory) {
        CoprocessorConfig memory cfg = ZamaConfig.getCoprocessorConfig();
        return cfg;
    }
    function getEthereumCoprocessorConfig() public view returns (CoprocessorConfig memory) {
        CoprocessorConfig memory cfg = ZamaConfig.getEthereumCoprocessorConfig();
        return cfg;
    }
    function getPolygonCoprocessorConfig() public view returns (CoprocessorConfig memory) {
        CoprocessorConfig memory cfg = ZamaConfig.getPolygonCoprocessorConfig();
        return cfg;
    }
    function getConfidentialProtocolId() public view returns (uint256) {
        return ZamaConfig.getConfidentialProtocolId();
    }
}

contract MultiChainConfigTest is Test {
    function setUp() public {
        vm.warp(1_000_000);
    }

    function _assertConfigEq(CoprocessorConfig memory a, CoprocessorConfig memory b) internal pure {
        assertTrue(a.ACLAddress == b.ACLAddress);
        assertTrue(a.CoprocessorAddress == b.CoprocessorAddress);
        assertTrue(a.KMSVerifierAddress == b.KMSVerifierAddress);
    }

    function test_ZamaMultiChainConfigEthereumMainnet() public {
        vm.chainId(1);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 1
        CoprocessorConfig memory mainnetCfg = ZamaConfig.getCoprocessorConfig();

        assertTrue(cfg.ACLAddress == 0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6);
        assertTrue(cfg.CoprocessorAddress == 0xD82385dADa1ae3E969447f20A3164F6213100e75);
        assertTrue(cfg.KMSVerifierAddress == 0x77627828a55156b04Ac0DC0eb30467f1a552BB03);

        _assertConfigEq(cfg, mainnetCfg);
        assertTrue(testFhevmContract.confidentialProtocolId() == 1);
    }

    function test_ZamaMultiChainConfigSepolia() public {
        vm.chainId(11155111);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 11155111
        CoprocessorConfig memory sepoliaCfg = ZamaConfig.getCoprocessorConfig();

        assertTrue(cfg.ACLAddress == 0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D);
        assertTrue(cfg.CoprocessorAddress == 0x92C920834Ec8941d2C77D188936E1f7A6f49c127);
        assertTrue(cfg.KMSVerifierAddress == 0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A);

        _assertConfigEq(cfg, sepoliaCfg);
        assertTrue(testFhevmContract.confidentialProtocolId() == 10001);
    }

    function test_ZamaMultiChainConfigPolygonMainnet() public {
        vm.chainId(137);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 137
        CoprocessorConfig memory polygonCfg = ZamaConfig.getCoprocessorConfig();

        assertTrue(cfg.ACLAddress == 0x6737F17e31cf26a1b62fb0362acC5a16CB156F49);
        assertTrue(cfg.CoprocessorAddress == 0xAB0075E77fe06083f52bdf10e2ccDB3712483057);
        assertTrue(cfg.KMSVerifierAddress == 0x14e609595474874Dd6b6128376E336EfADfdBE37);

        _assertConfigEq(cfg, polygonCfg);
        assertTrue(testFhevmContract.confidentialProtocolId() == 1);
    }

    function test_ZamaMultiChainConfigPolygonAmoy() public {
        vm.chainId(80002);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 80002
        CoprocessorConfig memory amoyCfg = ZamaConfig.getCoprocessorConfig();

        assertTrue(cfg.ACLAddress == 0xD99Cb9Fc3c42c87f2A4A12e8Fd60318d6bDdf985);
        assertTrue(cfg.CoprocessorAddress == 0x89420269f61e4db00545cd99da0aEcA7fF0912f9);
        assertTrue(cfg.KMSVerifierAddress == 0xCD1D89E311bce4C8DEa9a0857a0c9A4E153D4041);

        _assertConfigEq(cfg, amoyCfg);
        assertTrue(testFhevmContract.confidentialProtocolId() == 10001);
    }

    function test_ZamaMultiChainConfigLocalChainId() public {
        vm.chainId(31337);

        TestFHEVMContract testFhevmContract = new TestFHEVMContract();
        CoprocessorConfig memory cfg = testFhevmContract.getCoprocessorConfig();
        // chainid == 31337
        CoprocessorConfig memory localCfg = ZamaConfig.getCoprocessorConfig();

        assertTrue(cfg.ACLAddress == 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D);
        assertTrue(cfg.CoprocessorAddress == 0xe3a9105a3a932253A70F126eb1E3b589C643dD24);
        assertTrue(cfg.KMSVerifierAddress == 0x901F8942346f7AB3a01F6D7613119Bca447Bb030);

        _assertConfigEq(cfg, localCfg);
        assertTrue(testFhevmContract.confidentialProtocolId() == type(uint256).max);
    }

    function test_ZamaMultiChainConfigMatchesEthereumFamilyGetter() public {
        TestContract testContract = new TestContract();

        uint256[3] memory chainIds = [uint256(1), 11155111, 31337];
        for (uint256 i = 0; i < chainIds.length; i++) {
            vm.chainId(chainIds[i]);
            _assertConfigEq(testContract.getCoprocessorConfig(), testContract.getEthereumCoprocessorConfig());
        }
    }

    function test_ZamaMultiChainConfigMatchesPolygonFamilyGetter() public {
        TestContract testContract = new TestContract();

        uint256[3] memory chainIds = [uint256(137), 80002, 31337];
        for (uint256 i = 0; i < chainIds.length; i++) {
            vm.chainId(chainIds[i]);
            _assertConfigEq(testContract.getCoprocessorConfig(), testContract.getPolygonCoprocessorConfig());
        }
    }

    function test_ZamaMultiChainConfigUnknownChainId() public {
        vm.chainId(123);

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        new TestFHEVMContract();
    }

    function test_ZamaConfigGetCoprocessorConfigUnknownChainId() public {
        vm.chainId(123);

        TestContract testContract = new TestContract();

        vm.expectRevert(abi.encodeWithSelector(ZamaConfig.ZamaProtocolUnsupported.selector));
        testContract.getCoprocessorConfig();
    }
}
