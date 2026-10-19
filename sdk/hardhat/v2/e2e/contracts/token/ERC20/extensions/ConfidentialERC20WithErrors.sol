// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, ebool, euint8, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {ConfidentialERC20} from "../ConfidentialERC20.sol";
import {EncryptedErrors} from "../../../utils/EncryptedErrors.sol";

/**
 * @title   ConfidentialERC20WithErrors.
 * @notice  This contract implements an encrypted ERC20-like token with confidential balances using
 *          Zama's FHE (Fully Homomorphic Encryption) library.
 * @dev     It supports standard ERC20 functions such as transferring tokens, minting,
 *          and setting allowances, but uses encrypted data types.
 *          The total supply is not encrypted.
 *          It also supports error handling for encrypted errors.
 */
abstract contract ConfidentialERC20WithErrors is ConfidentialERC20, EncryptedErrors {
    /**
     * @notice Error codes allow tracking (in the storage) whether a transfer worked.
     * @dev    NO_ERROR: the transfer worked as expected.
     *         UNSUFFICIENT_BALANCE: the transfer failed because the
     *         from balances were strictly inferior to the amount to transfer.
     *         UNSUFFICIENT_APPROVAL: the transfer failed because the sender allowance
     *         was strictly lower than the amount to transfer.
     */
    enum ErrorCodes {
        NO_ERROR,
        UNSUFFICIENT_BALANCE,
        UNSUFFICIENT_APPROVAL
    }

    /**
     * @param name_     Name of the token.
     * @param symbol_   Symbol.
     */
    constructor(string memory name_, string memory symbol_)
        ConfidentialERC20(name_, symbol_)
        EncryptedErrors(uint8(type(ErrorCodes).max))
    {}

    /**
     * @notice See {IConfidentialERC20-transfer}.
     */
    function transfer(address to, euint64 amount) public virtual override returns (bool) {
        _isSenderAllowedForAmount(amount);
        /// @dev Check whether the owner has enough tokens.
        ebool canTransfer = FHE.le(amount, _balances[msg.sender]);
        euint8 errorCode = _errorDefineIfNot(canTransfer, uint8(ErrorCodes.UNSUFFICIENT_BALANCE));
        _errorSave(errorCode);
        FHE.allow(errorCode, msg.sender);
        FHE.allow(errorCode, to);
        _transfer(msg.sender, to, amount, canTransfer);
        return true;
    }

    /**
     * @notice See {IConfidentialERC20-transferFrom}.
     */
    function transferFrom(address from, address to, euint64 amount) public virtual override returns (bool) {
        _isSenderAllowedForAmount(amount);
        address spender = msg.sender;
        ebool isTransferable = _updateAllowance(from, spender, amount);
        _transfer(from, to, amount, isTransferable);
        return true;
    }

    /**
     * @notice            Return the error for a transfer id.
     * @param transferId  Transfer id. It can be read from the `Transfer` event.
     * @return errorCode  Encrypted error code.
     */
    function getErrorCodeForTransferId(uint256 transferId) public view virtual returns (euint8 errorCode) {
        errorCode = _errorGetCodeEmitted(transferId);
    }

    function _transfer(address from, address to, euint64 amount, ebool isTransferable) internal override {
        _transferNoEvent(from, to, amount, isTransferable);
        /// @dev It was incremented in _saveError.
        emit Transfer(from, to, _errorGetCounter() - 1);
    }

    function _updateAllowance(address owner, address spender, euint64 amount)
        internal
        virtual
        override
        returns (ebool isTransferable)
    {
        euint64 currentAllowance = _allowance(owner, spender);
        /// @dev It checks whether the allowance suffices.
        ebool allowedTransfer = FHE.le(amount, currentAllowance);
        euint8 errorCode = _errorDefineIfNot(allowedTransfer, uint8(ErrorCodes.UNSUFFICIENT_APPROVAL));
        /// @dev It checks that the owner has enough tokens.
        ebool canTransfer = FHE.le(amount, _balances[owner]);
        ebool isNotTransferableButIsApproved = FHE.and(FHE.not(canTransfer), allowedTransfer);
        errorCode = _errorChangeIf(
            isNotTransferableButIsApproved,
            /// @dev Should indeed check that spender is approved to not leak information.
            ///      on balance of `from` to unauthorized spender via calling reencryptTransferError afterwards.
            uint8(ErrorCodes.UNSUFFICIENT_BALANCE),
            errorCode
        );
        _errorSave(errorCode);
        FHE.allow(errorCode, owner);
        FHE.allow(errorCode, spender);
        isTransferable = FHE.and(canTransfer, allowedTransfer);
        _approve(owner, spender, FHE.select(isTransferable, FHE.sub(currentAllowance, amount), currentAllowance));
    }
}
