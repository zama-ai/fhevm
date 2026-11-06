// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CleartextHCULimit} from "./CleartextHCULimit.sol";
import {VmSafe} from "forge-std/Vm.sol";

contract CleartextForgeHCULimit is CleartextHCULimit {
    /// @dev Calls cheatcodes, so this contract reverts outside forge and must never ship to a chain.
    ///      `DeployLocalStack.s.sol` broadcasts to a node and keeps `CleartextHCULimit` for that reason.
    VmSafe private constant vmSafe = VmSafe(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @notice Marks the Forge-only variant, mirroring forge-std's `IS_TEST`. A constant rather than a
    ///         storage variable: these run behind proxies, where an initialized state variable would
    ///         only ever be set in the implementation's own storage and read back as `false`.
    bool public constant IS_FORGE = true;

    /// @dev Transient: whether this transaction has already metered an operation. The base shares this
    ///      namespace, storing a reading per handle AT the handle's own value and the running transaction
    ///      total at slot 0, which a namespace hash stays clear of.
    uint256 private constant _METERED_THIS_TX_SLOT =
        uint256(keccak256("fhevm.cleartext.forge.hculimit.metered-this-tx"));

    /// @notice Total HCU metered for this transaction, or for the last one that metered anything.
    uint256 public lastTransactionHCU;

    /// @notice Deepest sequential handle chain metered for it, the reading the depth cap is applied to.
    uint256 public maxHandleHCU;

    /// @notice Raised if an operation ever meters a handle at zero HCU.
    /// @dev Not defensive: `_hcuOf` reads a non-zero reading AS the statement "this stack minted it", so a
    ///      zero-cost operation would mint a handle this contract then denies having seen. Upstream's
    ///      tables are positive constants throughout, with `UnsupportedOperation` for anything unlisted,
    ///      so there is no such path today -- and if one appears, it fails here rather than silently
    ///      turning an operator bug into a handle that looks older than the fork.
    error ZeroHCUForHandle(bytes32 handle);

    /**
     * @dev THE HCU EVERY HANDLE THIS STACK MINTED COST, and by the same token the record of WHICH handles
     *      it minted: the reading cannot be zero, so a zero answer means "never seen here".
     *
     *      WHY THE QUESTION IS ASKED. The cleartext store answers zero for a handle it never saw, and on a
     *      fork that is the normal case -- the chain is full of handles minted before the cleartext layer
     *      existed. A reader answering for those has to tell them from the other kind: a handle THIS stack
     *      minted and the cleartext layer failed to record, which is an operator upstream added that the
     *      mirror does not override. Without this, that bug reads back as whatever the unknown-handle
     *      policy says -- a plausible number where there should be an alarm. Every computed handle is
     *      metered (all ten minting sites in `FHEVMExecutor` are paired with an HCU call), so being here
     *      is the same statement as "this stack produced it". `verifyInput` is the one exception and
     *      correctly so: its handle comes from the caller, not from a computation.
     *
     *      PERSISTENT, THOUGH THE BASE'S OWN READING IS TRANSIENT, and measured rather than assumed
     *      (`ForgeTransientStorageSemantics.t.sol`). Transient survives across calls and across fork
     *      switches, but `revertToState` WIPES it instead of restoring it -- so after a revert it would
     *      disagree with the `CleartextDB` it is compared against, and an unmirrored handle would silently
     *      downgrade to "minted before the fork". Ordinary storage reverts in lockstep with the store,
     *      which is the invariant the comparison needs. It is also the only form the SDK can read, since
     *      forge has no `vm.load` for transient storage.
     *
     *      THE VALUE, not a flag, because it costs the same slot and answers more: the base keeps its own
     *      reading transiently and drops it at the end of the transaction, so this is the only place a
     *      test can ask what a particular handle cost after the fact.
     */
    mapping(bytes32 handle => uint256 hcu) private _hcuOf;

    /// @notice What `handle` cost when this stack minted it, or zero if this stack did not mint it.
    function hcuOf(bytes32 handle) external view returns (uint256) {
        return _hcuOf[handle];
    }

    /// @notice Whether this stack minted `handle`, as opposed to inheriting it from the forked chain.
    function wasMintedHere(bytes32 handle) external view returns (bool) {
        return _hcuOf[handle] != 0;
    }

    /**
     * @notice Zeroes both readings, so the next FHE call is measured on its own.
     *
     * @dev WHY THIS HAS TO EXIST. `_clearOnNewTransaction` clears once per TRANSACTION, which is right on
     *      a chain and useless under forge: a whole test function is one transaction
     *      (`ForgeTransientStorageSemantics.t.sol` measures it), so the guard fires on the first metered
     *      operation of the test and never again. Every later call then accumulates into the same total --
     *      one call reads correctly, two do not, which is the worst way for a reading to be wrong.
     *
     * @dev It clears the transient flag too, not only the numbers: leave it set and the next operation
     *      skips `_clearOnNewTransaction` and adds itself to whatever this reset left behind.
     *
     * @dev THE MINTED-HANDLE RECORD IS DELIBERATELY UNTOUCHED. `_hcuOf` answers "did this stack ever mint
     *      this handle", a different question with a different lifetime; zeroing it between calls would
     *      make handles minted a moment ago look as though they came from the forked chain.
     *
     * @dev IT ALSO REOPENS THE PER-TRANSACTION CAP, because the base accumulates the running total in the
     *      same transient slot the reading mirrors, and a reset that left it standing would immediately
     *      restore the old total on the next operation. That is not a side effect to apologise for: on a
     *      chain each call IS its own transaction with its own cap, and a forge test measuring calls one
     *      at a time is closer to that than one long transaction. A test deliberately probing
     *      `HCUTransactionLimitExceeded` across several calls must not reset between them.
     */
    function resetHCUReadings() external onlyACLOwner {
        vmSafe.pauseGasMetering();
        {
            uint256 slot = _METERED_THIS_TX_SLOT;
            assembly {
                // The base's running transaction total. Slot 0 is reserved for it -- `_setHCUForHandle`
                // keys handles by their own value and refuses the zero handle to keep out of the way.
                tstore(0, 0)
                // And our own "already cleared this transaction" flag, or the next operation skips its
                // clearing step and adds itself to what this reset just zeroed.
                tstore(slot, 0)
            }
            lastTransactionHCU = 0;
            maxHandleHCU = 0;
        }
        vmSafe.resumeGasMetering();
    }

    function _setHCUForTransaction(uint256 transactionHCU) internal override {
        super._setHCUForTransaction(transactionHCU);
        vmSafe.pauseGasMetering();
        {
            _clearOnNewTransaction();
            lastTransactionHCU = transactionHCU;
        }
        vmSafe.resumeGasMetering();
    }

    function _setHCUForHandle(bytes32 handle, uint256 handleHCU) internal override {
        super._setHCUForHandle(handle, handleHCU);
        vmSafe.pauseGasMetering();
        {
            _clearOnNewTransaction();
            if (handleHCU > maxHandleHCU) {
                maxHandleHCU = handleHCU;
            }
            // Inside the paused window with the rest: bookkeeping a test did not ask for must not land
            // on the gas it measures.
            if (handleHCU == 0) revert ZeroHCUForHandle(handle);
            _hcuOf[handle] = handleHCU;
        }
        vmSafe.resumeGasMetering();
    }

    /// @dev Clears the readings once per transaction, on the first metered operation.
    function _clearOnNewTransaction() private {
        uint256 slot = _METERED_THIS_TX_SLOT;
        bool metered;
        assembly {
            metered := tload(slot)
        }
        if (metered) {
            return;
        }
        assembly {
            tstore(slot, 1)
        }
        lastTransactionHCU = 0;
        maxHandleHCU = 0;
    }
}
