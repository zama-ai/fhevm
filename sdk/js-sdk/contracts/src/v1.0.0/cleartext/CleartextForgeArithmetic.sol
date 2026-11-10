// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {CleartextArithmetic} from "./CleartextArithmetic.sol";
import {FheType, LibFheType} from "./shared/LibFheType.sol";
import {LibFhevmHandle} from "./shared/LibFhevmHandle.sol";
import {ICleartextDB} from "./shared/interfaces/ICleartextDB.sol";
import {hcuLimitAdd} from "../host-contracts/addresses/FHEVMHostAddresses.sol";
import {VmSafe} from "forge-std/Vm.sol";

/**
 * @title CleartextForgeArithmetic
 *
 * @notice The forge-only arithmetic: forge's randomness, and an answer for handles the store never saw.
 *
 * @dev WHY UNKNOWN HANDLES ARE THIS CONTRACT'S PROBLEM. On a stack that minted everything it is asked
 *      about, every operand is in the store and a miss is a bug. On a FORK it is the normal case: the
 *      chain is full of handles created before the cleartext layer existed, and their cleartexts are
 *      nowhere. Something has to decide what those read as, and it cannot be the SDK -- when a dApp calls
 *      `fheAdd(oldHandle, x)` the operand is read HERE, inside the stack, with no test code in the frame.
 *
 * @dev THE ORDER IS THE DESIGN, and each step exists because the one below it is not enough:
 *        1. A VALUE IN THE STORE WINS, always. A handle seeded by a test, or minted by this stack, is a
 *           fact -- a policy that could override it would make a seeded fuzz input unreliable.
 *        2. A PER-TYPE DEFAULT, because "every unknown value is 1" is rarely what a test means. An
 *           `eaddress` wants an address and an `ebool` wants a bit, and one global number gives neither.
 *        3. DERIVED FROM THE HANDLE, `keccak256(handle)` truncated to the type. Deterministic, so a run
 *           repeats; unrelated to the handle's layout, so it does not collide the way reading the handle's
 *           own bytes would -- those carry the chain id, type and version, identical across handles.
 *        4. OTHERWISE IT REVERTS, naming the handle and its type. A test that meant to seed and forgot
 *           gets told so, rather than a silent zero that decrypts to a plausible-looking answer.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
/// @notice The one thing this contract needs from the HCU limit: whether a handle came from this stack.
/// @dev Declared here because the generated `ICleartextHCULimit` is built from the DEPLOYED contract, and
///      the plain variant keeps no such record. The forge variants are installed as a set -- the local
///      deploy and the fork upgrade both take all four or none -- so this call is direct rather than
///      probed: a stack assembled from a mismatched pair should fail loudly, not lose the check quietly.
interface ICleartextForgeHCULimitMinted {
    function wasMintedHere(bytes32 handle) external view returns (bool);
}

contract CleartextForgeArithmetic is CleartextArithmetic {
    VmSafe private constant vmSafe = VmSafe(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @notice Marks the Forge-only variant, mirroring forge-std's `IS_TEST`. A constant rather than a
    ///         storage variable: these run behind proxies, where an initialized state variable would
    ///         only ever be set in the implementation's own storage and read back as `false`.
    bool public constant IS_FORGE = true;

    /// @notice What this stack answers for a handle its store never saw, per type.
    enum UnknownHandlePolicy {
        Revert,
        Fixed,
        FromHandle
    }

    /**
     * @notice Returned when a handle THIS STACK MINTED has no cleartext recorded for it.
     * @dev A different failure from an unknown handle, and the distinction is the point. An unknown handle
     *      is ordinary on a fork -- the chain is full of them, and the per-type policy exists to answer for
     *      them. This one cannot be ordinary: the stack computed the value and the cleartext layer did not
     *      write it down, which means an operator the mirror does not override. Answering it from the
     *      policy would turn a missing implementation into a plausible number.
     * @param handle The handle this stack minted and did not record.
     * @param fheType Its type, to say which operator family to look at.
     */
    error CleartextErrorUnrecordedResult(bytes32 handle, FheType fheType);

    /// @notice Returned when a handle is absent and nothing said what to do about it.
    /// @param handle The handle nothing knows a cleartext for.
    /// @param fheType Its type, which is the one a default would be set against.
    error CleartextErrorUnknownHandle(bytes32 handle, FheType fheType);

    /// @notice Emitted whenever a type's policy changes, so a trace shows what a test asked for.
    event UnknownHandlePolicySet(FheType indexed fheType, UnknownHandlePolicy policy, uint256 value);

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.CleartextForgeArithmetic")) - 1)) &
    ///      ~bytes32(uint256(0xff))
    bytes32 private constant CLEARTEXT_FORGE_ARITHMETIC_STORAGE_LOCATION =
        0xce0eae05c84cc8c8ab9d4d448f354b4a5699f4916150910949002e50675c4500;

    /// @custom:storage-location erc7201:fhevm.storage.CleartextForgeArithmetic
    struct CleartextForgeArithmeticStorage {
        mapping(FheType fheType => UnknownHandlePolicy policy) policy;
        mapping(FheType fheType => uint256 value) fixedValue;
    }

    function _getStorage() private pure returns (CleartextForgeArithmeticStorage storage $) {
        assembly {
            $.slot := CLEARTEXT_FORGE_ARITHMETIC_STORAGE_LOCATION
        }
    }

    // -----------------------------------------------------------------------
    // The policy, per type
    // -----------------------------------------------------------------------

    /**
     * @notice Answer `value` for every unknown handle of `fheType`.
     * @dev REFUSED, NOT CLAMPED, by the same helper every other plaintext entering the store goes
     *      through: a `uint8` default of 300 reverts here rather than reading back as 44 later. The
     *      caller named the type in this very call, so an out-of-range value is a mistake at the call
     *      site, and that is where it is worth reporting -- `trivialEncrypt(2, Bool)` already works this
     *      way. The DERIVED value is a different matter and is cut to fit: it is a hash, not something
     *      anyone chose.
     */
    function setUnknownHandleDefault(FheType fheType, uint256 value) external onlyACLOwner {
        CleartextForgeArithmeticStorage storage $ = _getStorage();
        $.policy[fheType] = UnknownHandlePolicy.Fixed;
        $.fixedValue[fheType] = _normalizePlaintextToType(value, fheType);
        emit UnknownHandlePolicySet(fheType, UnknownHandlePolicy.Fixed, value);
    }

    /// @notice Derive every unknown handle of `fheType` from the handle itself.
    function setUnknownHandleFromHandle(FheType fheType) external onlyACLOwner {
        _getStorage().policy[fheType] = UnknownHandlePolicy.FromHandle;
        emit UnknownHandlePolicySet(fheType, UnknownHandlePolicy.FromHandle, 0);
    }

    /// @notice Back to refusing: an unknown handle of `fheType` reverts again.
    function clearUnknownHandlePolicy(FheType fheType) external onlyACLOwner {
        _getStorage().policy[fheType] = UnknownHandlePolicy.Revert;
        emit UnknownHandlePolicySet(fheType, UnknownHandlePolicy.Revert, 0);
    }

    /// @notice The policy in force for `fheType`, and the value it would answer if it is `Fixed`.
    function unknownHandlePolicy(FheType fheType) external view returns (UnknownHandlePolicy, uint256) {
        CleartextForgeArithmeticStorage storage $ = _getStorage();
        return ($.policy[fheType], $.fixedValue[fheType]);
    }

    /// @notice What this stack WOULD answer for `handle` were it unknown, without consulting the store.
    /// @dev Exposed so a test can assert the derivation rather than reverse-engineer it.
    function derivedValueFor(bytes32 handle) public pure returns (uint256) {
        FheType fheType = LibFhevmHandle.typeOf(handle);
        return _fitToType(uint256(keccak256(abi.encodePacked(handle))), fheType);
    }

    // -----------------------------------------------------------------------
    // The seam
    // -----------------------------------------------------------------------

    /// @dev The base reads every operand through this; here it answers for what the store does not hold.
    function _operand(ICleartextDB db, bytes32 handle) internal view override returns (uint256) {
        if (db.has(handle)) return db.get(handle);

        FheType fheType = LibFhevmHandle.typeOf(handle);

        // BEFORE ANY POLICY. The HCU limit meters every handle the executor computes, so a reading there
        // is the same statement as "this stack produced it" -- and a handle this stack produced should be
        // in the store. It is not, so the cleartext layer missed an operator, and no policy may cover it.
        if (ICleartextForgeHCULimitMinted(hcuLimitAdd).wasMintedHere(handle)) {
            revert CleartextErrorUnrecordedResult(handle, fheType);
        }

        CleartextForgeArithmeticStorage storage $ = _getStorage();
        UnknownHandlePolicy policy = $.policy[fheType];

        if (policy == UnknownHandlePolicy.Fixed) return $.fixedValue[fheType];
        if (policy == UnknownHandlePolicy.FromHandle) return derivedValueFor(handle);
        revert CleartextErrorUnknownHandle(handle, fheType);
    }

    /// @dev `keccak256` gives 256 bits of entropy per handle; the type says how many of them are legal.
    ///      A bool is one bit rather than a truncation, because every other value would be out of range.
    function _fitToType(uint256 word, FheType fheType) private pure returns (uint256) {
        if (fheType == FheType.Bool) return word & 1;
        uint256 bits = LibFheType.bitWidthForType(fheType);
        return (bits >= 256) ? word : word & ((uint256(1) << bits) - 1);
    }

    function randomUint256(
        FheType,
        /* randType */
        bytes16 /* seed */
    )
        internal
        view
        override
        returns (uint256)
    {
        return vmSafe.randomUint();
    }

    function randomBoundedUint256(
        uint256 upperBound,
        bytes16 /* seed */
    )
        internal
        view
        override
        returns (uint256)
    {
        return vmSafe.randomUint() % upperBound;
    }
}
