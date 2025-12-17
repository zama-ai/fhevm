pragma solidity 0.8.27;

import {FHE, euint64, externalEuint64} from "@fhevm/solidity/lib/FHE.sol";
import {RegulatedERC7984Upgradeable} from "../token/RegulatedERC7984Upgradeable.sol";


contract BurnableRegulatedERC7984Upgradeable is RegulatedERC7984Upgradeable {
    function burn(
        externalEuint64 amount,
        bytes calldata inputProof
    ) public virtual onlyRole(WRAPPER_ROLE) returns (euint64) {
        return burn(FHE.fromExternal(amount, inputProof), msg.sender);
    }
}
