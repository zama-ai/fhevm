// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import {externalEuint64, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {IERC7984} from "@openzeppelin/confidential-contracts/interfaces/IERC7984.sol";

/// @dev Interface for ERC7984ERC20Wrapper contract.
interface IERC7984ERC20Wrapper is IERC7984 {
    /// @dev Wraps `amount` of the underlying token into a confidential token and sends it to `to`.
    function wrap(address to, uint256 amount) external;

    /**
     * @dev Unwraps tokens from `from` and sends the underlying tokens to `to`. The caller must be `from`
     * or be an approved operator for `from`.
     *
     * NOTE: The caller *must* already be approved by ACL for the given `amount`.
     */
    function unwrap(address from, address to, externalEuint64 encryptedAmount, bytes calldata inputProof) external;

    /// @dev Fills an unwrap request for a given cipher-text `burntAmount` with the `cleartextAmount` and `decryptionProof`.
    function finalizeUnwrap(euint64 burntAmount, uint64 burntAmountCleartext, bytes calldata decryptionProof) external;

    /// @dev Returns the address of the underlying ERC-20 token that is being wrapped.
    function underlying() external view returns (address);
}
