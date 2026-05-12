// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {ACL} from "../host-contracts/contracts/ACL.sol";

/// @notice (handle, contractAddress) pair used by user-decryption APIs so each
///         requested handle can be bound to the specific contract the user
///         authorized for it, rather than applying one contract to every handle.
struct HandleContractPair {
    bytes32 handle;
    address contractAddress;
}

/**
 * @title CleartextACL
 * @notice Debug-only ACL that records the cleartext uint256 value associated
 *         with each handle alongside normal permissioning. For local testing
 *         and mock execution — MUST NOT be deployed to production networks.
 */
contract CleartextACL is ACL {
    /// @custom:storage-location erc7201:fhevm.storage.CleartextACL
    struct CleartextACLStorage {
        mapping(bytes32 handle => bool saved) handleSaved;
        mapping(bytes32 handle => uint256 clearValue) handleValues;
    }

    struct CleartextACLTransientEntry {
        bool saved;
        uint256 value;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.CleartextACL")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant CleartextACLStorageLocation =
        0x824fadfb469545d67b450a05af804c0e678c33c2ad27dc109bc3b09f9ee12400;

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.transient.CleartextACL")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant CleartextACLTransientLocation =
        0xcf73bb34faecb01bcebf428344a14fd3c1bb1822c0d124c3f10f22e281ea5600;

    error CleartextACLNotFHEVMExecutor(address caller);
    error CleartextACLTransientMismatch(bytes32 handle, uint256 existing, uint256 attempted);
    error CleartextACLTransientNotSet(bytes32 handle);
    error CleartextACLCleartextNotSaved(bytes32 handle);
    error CleartextACLInputAlreadyPersisted(bytes32 inputHandle);
    error HandleNotDelegatedForUserDecryption(
        bytes32 handle, address contractAddress, address delegator, address delegate
    );

    error UserAddressEqualsContractAddress();
    error InvalidUserDecryptSignature();
    error HandleNotAllowedForPublicDecryption(bytes32 handle);
    error UserNotAuthorizedForDecrypt(bytes32 handle, address userAddress);
    error ContractNotAuthorizedForDecrypt(bytes32 handle, address contractAddress);

    function _getCleartextACLStorage() private pure returns (CleartextACLStorage storage $) {
        assembly { $.slot := CleartextACLStorageLocation }
    }

    // ---- Transient helpers ----

    /// @dev Base transient slot for `handle`.
    ///      Layout: `slot` holds the cleartext value, `slot + 1` holds the "was set" flag.
    ///      The flag slot is required to disambiguate "never written" from "written with value 0".
    function _tslot(bytes32 handle) private pure returns (bytes32 slot) {
        slot = keccak256(abi.encode(handle, CleartextACLTransientLocation));
    }

    /// @notice Records the cleartext `pt` for `handle` in transient storage.
    /// @dev Writes (value, flag) = (`pt`, 1) at (`_tslot(handle)`, `_tslot(handle) + 1`).
    ///      Idempotent for the same `pt`; reverts with `CleartextACLTransientMismatch`
    ///      if the flag is already set and the stored value differs (handles are
    ///      derived deterministically, so a divergent write is a caller bug).
    function _setTransientCleartext(bytes32 handle, uint256 pt) private {
        bytes32 slot = _tslot(handle);
        uint256 existing;
        uint256 wasSet;
        assembly {
            existing := tload(slot)
            wasSet := tload(add(slot, 1))
        }
        if (wasSet != 0 && existing != pt) {
            revert CleartextACLTransientMismatch(handle, existing, pt);
        }
        assembly {
            tstore(slot, pt)
            tstore(add(slot, 1), 1)
        }
    }

    function _isTransientCleartextSet(bytes32 handle) private view returns (bool) {
        bytes32 slot = _tslot(handle);
        uint256 wasSet;
        assembly {
            wasSet := tload(add(slot, 1))
        }
        return wasSet != 0;
    }

    function _getTransientCleartextUnchecked(bytes32 handle) private view returns (uint256 pt) {
        bytes32 slot = _tslot(handle);
        assembly { pt := tload(slot) }
    }

    /// @notice Reads the transient cleartext for `handle`.
    /// @dev Checks the flag at `_tslot(handle) + 1` first to disambiguate a
    ///      legitimate 0 from an unwritten slot, then loads the value.
    ///      Reverts with `CleartextACLTransientNotSet(handle)` if not set in this tx.
    function _getTransientCleartext(bytes32 handle) private view returns (uint256 pt) {
        bytes32 slot = _tslot(handle);
        uint256 wasSet;
        assembly { wasSet := tload(add(slot, 1)) }
        if (wasSet == 0) {
            revert CleartextACLTransientNotSet(handle);
        }
        assembly { pt := tload(slot) }
    }

    // ---- Debug entry points ----

    /// @notice Debug variant of allowTransient: also records the cleartext value.
    /// @dev Only callable by CleartextFHEVMExecutor.
    function allowTransientWithCleartext(bytes32 handle, address account, uint256 clearText) external virtual {
        if (msg.sender != getFHEVMExecutorAddress()) {
            revert CleartextACLNotFHEVMExecutor(msg.sender);
        }
        _setTransientCleartext(handle, clearText);
        super.allowTransient(handle, account);
    }

    /// @notice Returns `(isAllowed(handle, account), plaintext(handle))`.
    /// @dev Cleartext resolution is identical to `plaintext` (transient first,
    ///      then persistent, else revert `CleartextACLCleartextNotSaved`). The
    ///      `allowed` bool is computed independently from the base ACL.
    function isAllowedWithCleartext(bytes32 handle, address account) external view returns (bool allowed, uint256 pt) {
        pt = plaintext(handle);
        allowed = isAllowed(handle, account);
    }

    // ---- Overrides: propagate transient cleartext into persistent storage ----

    function allow(bytes32 handle, address account) public virtual override {
        CleartextACLStorage storage $ = _getCleartextACLStorage();
        if (!$.handleSaved[handle]) {
            _persistCleartextFromTransient(handle);
        }
        super.allow(handle, account);
    }

    /// @notice Debug override of `ACL.allowTransient` that asserts a cleartext
    ///         exists for `handle` after delegating to the base implementation.
    /// @dev Every handle flowing through this debug ACL must have a cleartext
    ///      recorded either transiently (via `allowTransientWithCleartext` in this tx)
    ///      or persistently (promoted by a previous tx). Reverts with
    ///      `CleartextACLTransientNotSet(handle)` if neither is present — that would
    ///      break later `plaintext` / `_persistCleartextFromTransient` reads.
    function allowTransient(bytes32 handle, address account) public virtual override {
        super.allowTransient(handle, account);

        if (_isTransientCleartextSet(handle)) {
            return;
        }

        CleartextACLStorage storage $ = _getCleartextACLStorage();
        if (!$.handleSaved[handle]) {
            revert CleartextACLTransientNotSet(handle);
        }
    }

    function allowForDecryption(bytes32[] memory handlesList) public virtual override {
        CleartextACLStorage storage $ = _getCleartextACLStorage();
        uint256 n = handlesList.length;
        for (uint256 i = 0; i < n; ++i) {
            if (!$.handleSaved[handlesList[i]]) {
                _persistCleartextFromTransient(handlesList[i]);
            }
        }
        super.allowForDecryption(handlesList);
    }

    /// @notice Promotes the transient cleartext for `handle` into persistent storage.
    /// @dev Caller MUST have recorded `handle` via `allowTransientWithCleartext` earlier
    ///      in the same tx; otherwise `_getTransientCleartext` reverts with
    ///      `CleartextACLTransientNotSet`. Post-condition: `handleSaved[handle] = true`
    ///      and `handleValues[handle]` holds the cleartext.
    function _persistCleartextFromTransient(bytes32 handle) private {
        CleartextACLStorage storage $ = _getCleartextACLStorage();
        $.handleValues[handle] = _getTransientCleartext(handle);
        $.handleSaved[handle] = true;
    }

    /// @notice Returns the cleartext recorded for `handle`, preferring transient storage.
    /// @dev Lookup order: transient (written this tx by `_setTransientCleartext`), then
    ///      persistent `handleValues[handle]` (if `handleSaved[handle]`), else revert
    ///      with `CleartextACLCleartextNotSaved`. Transient-first so a value read in the
    ///      same tx that produced it doesn't need to wait for persistent promotion.
    function plaintext(bytes32 handle) public view returns (uint256 pt) {
        if (_isTransientCleartextSet(handle)) {
            return _getTransientCleartextUnchecked(handle);
        }

        CleartextACLStorage storage $ = _getCleartextACLStorage();
        if (!$.handleSaved[handle]) {
            revert CleartextACLCleartextNotSaved(handle);
        }
        pt = $.handleValues[handle];
    }

    function plaintextForPublicDecryption(bytes32[] calldata handles)
        public
        view
        returns (uint256[] memory cleartexts)
    {
        cleartexts = new uint256[](handles.length);
        for (uint256 i = 0; i < handles.length; i++) {
            if (!isAllowedForDecryption(handles[i])) {
                revert HandleNotAllowedForPublicDecryption(handles[i]);
            }
            cleartexts[i] = plaintext(handles[i]);
        }
    }

    function plaintextForUserDecryption(HandleContractPair[] calldata pairs, address userAddress)
        public
        view
        returns (uint256[] memory cleartexts)
    {
        uint256 n = pairs.length;
        cleartexts = new uint256[](n);
        for (uint256 i = 0; i < n; i++) {
            bytes32 h = pairs[i].handle;
            address c = pairs[i].contractAddress;

            if (userAddress == c) {
                revert UserAddressEqualsContractAddress();
            }
            if (!persistAllowed(h, userAddress)) {
                revert UserNotAuthorizedForDecrypt(h, userAddress);
            }
            if (!persistAllowed(h, c)) {
                revert ContractNotAuthorizedForDecrypt(h, c);
            }
            cleartexts[i] = plaintext(h);
        }
    }

    function plaintextForDelegatedUserDecryption(
        HandleContractPair[] calldata pairs,
        address delegator,
        address delegate
    ) public view returns (uint256[] memory cleartexts) {
        uint256 n = pairs.length;
        cleartexts = new uint256[](n);
        for (uint256 i = 0; i < n; i++) {
            bytes32 h = pairs[i].handle;
            address c = pairs[i].contractAddress;

            if (!isHandleDelegatedForUserDecryption(delegator, delegate, c, h)) {
                revert HandleNotDelegatedForUserDecryption(h, c, delegator, delegate);
            }
            cleartexts[i] = plaintext(h);
        }
    }

    /// @notice Writes the cleartext of a user-supplied encrypted input straight to storage.
    /// @dev Called by `CleartextFHEVMExecutor.persistNewInput`. Input handles skip the
    ///      transient tier because the cleartext comes from the proof, not an on-chain op.
    ///      Only callable by the FHEVMExecutor (`CleartextACLNotFHEVMExecutor` otherwise),
    ///      and rejects duplicate persists with `CleartextACLInputAlreadyPersisted` —
    ///      an input should be registered at most once.
    function registerInputCleartext(bytes32 inputHandle, uint256 clearText) external virtual {
        if (msg.sender != getFHEVMExecutorAddress()) {
            revert CleartextACLNotFHEVMExecutor(msg.sender);
        }
        CleartextACLStorage storage $ = _getCleartextACLStorage();
        if ($.handleSaved[inputHandle]) {
            revert CleartextACLInputAlreadyPersisted(inputHandle);
        }
        $.handleValues[inputHandle] = clearText;
        $.handleSaved[inputHandle] = true;
    }

    function requireNotPaused() public view virtual {
        _requireNotPaused();
    }

    //     function publicDecrypt(bytes32[] memory handles, address kmsVerifierAdd)
    //         external
    //         view
    //         virtual
    //         returns (uint256[] memory cleartexts, bytes32 digest)
    //     {
    //         //TODO
    //         bytes memory extraData;
    //         cleartexts = new uint256[](handles.length);
    //         for (uint256 i = 0; i < handles.length; i++) {
    //             if (!isAllowedForDecryption(handles[i])) {
    //                 revert HandleNotAllowedForPublicDecryption(handles[i]);
    //             }
    //             cleartexts[i] = plaintext(handles[i]);
    //         }

    //         bytes memory abiEncodedCleartexts = abi.encode(cleartexts);
    //         (, string memory name, string memory version, uint256 chainId, address verifyingContract,,) =
    //             KMSVerifier(kmsVerifierAdd).eip712Domain();

    //         bytes32 domainSeparator = keccak256(
    //             abi.encode(
    //                 EIP712_DOMAIN_TYPEHASH,
    //                 keccak256(bytes("PublicDecryptVerification")),
    //                 keccak256(bytes(version)),
    //                 chainId,
    //                 verifyingContract
    //             )
    //         );

    //         // see KMSVerifier function _hashDecryptionResult(PublicDecryptVerification memory decRes) internal view virtual returns (bytes32) {
    //         bytes32 structHash = keccak256(
    //             abi.encode(
    //                 KMSVerifier(kmsVerifierAdd).DECRYPTION_RESULT_TYPEHASH,
    //                 keccak256(abi.encodePacked(handles)),
    //                 keccak256(abiEncodedCleartexts),
    //                 keccak256(abi.encodePacked(extraData))
    //             )
    //         );

    //         digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
    //     }

    //     function userDecrypt(
    //         bytes32 handle,
    //         address userAddress,
    //         address contractAddress,
    //         bytes memory userSignature,
    //         bytes memory publicKey,
    //         uint256 startTimestamp,
    //         uint256 durationDays,
    //         address kmsVerifierAdd
    //     ) internal returns (uint256) {
    //         //TODO
    //         bytes memory extraData;
    //         if (userAddress == contractAddress) {
    //             revert UserAddressEqualsContractAddress();
    //         }

    //         if (!persistAllowed(handle, userAddress)) {
    //             revert UserNotAuthorizedForDecrypt(handle, userAddress);
    //         }

    //         if (!persistAllowed(handle, contractAddress)) {
    //             revert ContractNotAuthorizedForDecrypt(handle, contractAddress);
    //         }

    //         address[] memory contractAddresses = new address[](1);
    //         contractAddresses[0] = contractAddress;

    // /*
    //     readonly domain: Readonly<{
    //         name: "Decryption";
    //         version: "1";
    //         chainId: Uint64BigInt;
    //         verifyingContract: ChecksummedAddress;
    //     }>;
    //     readonly types: KmsUserDecryptEip712Types;
    //     readonly primaryType: "UserDecryptRequestVerification";
    //     readonly message: KmsUserDecryptEip712Message;
    // */

    //         (, string memory name, string memory version, uint256 chainId, address verifyingContract,,) =
    //             KMSVerifier(kmsVerifierAdd).eip712Domain();

    //         bytes32 domainSeparator = keccak256(
    //             abi.encode(
    //                 EIP712_DOMAIN_TYPEHASH, keccak256(bytes(name)), keccak256(bytes(version)), chainId, verifyingContract
    //             )
    //         );

    //         bytes32 structHash = keccak256(
    //             abi.encode(
    //                 USER_DECRYPT_REQUEST_TYPEHASH,
    //                 keccak256(publicKey),
    //                 keccak256(abi.encodePacked(contractAddresses)),
    //                 startTimestamp,
    //                 durationDays,
    //                 keccak256(extraData)
    //             )
    //         );

    //         bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));

    //         // bytes32 digest = UserDecryptHelper.computeUserDecryptDigest(
    //         //     abi.encodePacked(userAddress), contractAddresses, startTimestamp, durationDays, extraData, domainSeparator
    //         // );

    //         (uint8 v, bytes32 r, bytes32 s) = _decodeSignature(userSignature);
    //         address recoveredSigner = ecrecover(digest, v, r, s);
    //         if (recoveredSigner == address(0) || recoveredSigner != userAddress) {
    //             revert InvalidUserDecryptSignature();
    //         }

    //         return _plaintexts[handle];
    //     }

    //         function _decodeSignature(bytes memory signature) internal pure returns (uint8 v, bytes32 r, bytes32 s) {
    //         if (signature.length != 65) {
    //             revert InvalidUserDecryptSignature();
    //         }

    //         assembly {
    //             r := mload(add(signature, 0x20))
    //             s := mload(add(signature, 0x40))
    //             v := byte(0, mload(add(signature, 0x60)))
    //         }

    //         if (v < 27) {
    //             v += 27;
    //         }
    //         if (v != 27 && v != 28) {
    //             revert InvalidUserDecryptSignature();
    //         }
    //     }
}
