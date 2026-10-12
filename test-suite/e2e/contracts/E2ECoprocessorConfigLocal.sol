// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CoprocessorConfig, FHE} from "@fhevm/solidity/lib/FHE.sol";

library DefaultCoprocessorConfig {
    function getConfig() internal view returns (CoprocessorConfig memory config) {
        if (block.chainid == 31337) {
            config = _getCleartextConfig();
        } else {
            config = _getConfig();
        }
    }

    function _getConfig() internal pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c,
                CoprocessorAddress: 0xcCAe95fF1d11656358E782570dF0418F59fA40e1,
                KMSVerifierAddress: 0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11
            });
    }

    function _getCleartextConfig() internal pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D,
                CoprocessorAddress: 0xe3a9105a3a932253A70F126eb1E3b589C643dD24,
                KMSVerifierAddress: 0x901F8942346f7AB3a01F6D7613119Bca447Bb030
            });
    }
}

contract E2ECoprocessorConfig {
    constructor() {
        FHE.setCoprocessor(DefaultCoprocessorConfig.getConfig());
    }
}
