// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// DRAFT — see ../README.md. Not wired into the build, not compiled, not tested.
//
// Minimal local views, so this draft compiles against forge-std alone and nothing here can influence
// an address by pulling a real contract into the build. The production version should import
// pkg/forge/src/_internal/interfaces/{IACL,IPauserSet,IACLOwner}.sol, which already exist and are
// The same shape.
//
// Everything below is READ-BACK or a step A–E call. No initcode is built from these, so a signature
// mismatch here fails loudly at call time rather than silently moving an address — unlike the two
// initializer signatures in FhevmCreate2Base, which are inside the addresses.

/// @dev ACL's ownership surface. Ownable2Step: `transferOwnership` only OFFERS; ownership moves at
///      the acceptance. That distinction is load-bearing for the step sequence's A-before-C rule.
interface IOwnable2Step {
    function owner() external view returns (address);
    function pendingOwner() external view returns (address);
    function transferOwnership(address newOwner) external;
    function acceptOwnership() external;
}

interface IPauserSet {
    function addPauser(address account) external;
    function isPauser(address account) external view returns (bool);
}

/**
 * `getVersion()`, which almost every host contract has and `CleartextDB` does not.
 *
 * The one reading that says WHICH generation's code a proxy is executing, and therefore the only direct
 * evidence an upgrade did what it claims. Compared against the generated `LocalHostVersions` — never a
 * hand-written string, which would just be a second place to get it wrong.
 */
interface IVersioned {
    function getVersion() external view returns (string memory);
}

/**
 * The baked-in addresses each host contract carries, as getters.
 *
 * These return what the CONTRACT WAS COMPILED WITH, not what the manifest says — which is exactly
 * why they are worth reading. Everything else `verify` checks is a fact about the chain (is there
 * code here, who owns what); these are the only checks that can catch bytecode compiled against a
 * different address set: a stale build, a remapping that silently did not take, or placeholder
 * markers that survived (see pass 3's marker scan, which catches the same class at build time).
 *
 * Only callable AFTER step D. Before that the proxies point at the empty implementations, which do
 * not have these functions, and the call would revert rather than report.
 */
interface IWiredACL {
    function getFHEVMExecutorAddress() external view returns (address);
    function getPauserSetAddress() external view returns (address);
}

interface IWiredFHEVMExecutor {
    function getACLAddress() external view returns (address);
    function getHCULimitAddress() external view returns (address);
    function getInputVerifierAddress() external view returns (address);
    /// @dev Cleartext build only: the executor deployed here is CleartextFHEVMExecutor.
    function getCleartextArithmeticAddress() external view returns (address);
}

interface IWiredHCULimit {
    function getFHEVMExecutorAddress() external view returns (address);
}

interface IWiredCleartextArithmetic {
    function getCleartextDBAddress() external view returns (address);
}

interface IWiredCleartextDB {
    function getACLAddress() external view returns (address);
}

/**
 * The signer sets, as registered on chain.
 *
 * Not baked into bytecode like the addresses above — these arrive as initializer arguments at step D
 * and live in storage, so they are the one part of the stack that a correct deployment can still get
 * wrong without anything else noticing. The bootstrap config: a stack seeded with the wrong signers deploys fine,
 * verifies against itself, and fails only when the js-sdk relayer shows up and cannot find its own
 * key in the set the chain reports.
 */
interface IWiredInputVerifier {
    function getCoprocessorSigners() external view returns (address[] memory);
    function getThreshold() external view returns (uint256);
}

/// @dev The KMS signer set, which in this generation lives on the verifier itself rather than on a
///      separate ProtocolConfig contract. Same reasoning as IWiredInputVerifier above.
interface IWiredKMSVerifier {
    function getKmsSigners() external view returns (address[] memory);
    function getThreshold() external view returns (uint256);
}

interface IACLOwner {
    /// @dev Mirrors ACLOwner.Op exactly: point `proxy` at `implementation` and call `initData` on it.
    struct Op {
        address proxy;
        address implementation;
        bytes initData;
    }

    function ACL_ADDRESS() external view returns (address);
    function owner() external view returns (address);
    function pendingOwner() external view returns (address);

    function acceptACLOwnership() external;
    function upgrade(Op[] calldata ops) external;
    function execute(address target, bytes calldata data) external returns (bytes memory);
    function transferOwnership(address newOwner) external;
}
