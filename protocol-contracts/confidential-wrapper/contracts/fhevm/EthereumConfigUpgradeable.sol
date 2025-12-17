// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {FHE} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

abstract contract EthereumConfigUpgradeable is Initializable {
    function __EthereumConfig_init() internal onlyInitializing {
        __EthereumConfig_init_unchained();
    }

    function __EthereumConfig_init_unchained() internal onlyInitializing {
        FHE.setCoprocessor(ZamaConfig.getEthereumCoprocessorConfig());
    }

    function confidentialProtocolId() public view returns (uint256) {
        return ZamaConfig.getConfidentialProtocolId();
    }
}
