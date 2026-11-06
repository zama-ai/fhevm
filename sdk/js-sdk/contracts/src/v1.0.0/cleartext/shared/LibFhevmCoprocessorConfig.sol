// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// EXTRACT from upstream `library-solidity/lib/Impl.sol` (published as @fhevm/solidity's lib/Impl.sol). DO NOT EDIT
// — everything below is upstream's text, so that it can be re-checked against upstream rather than maintained
// here. The extraction is two slices, each anchored on an exact line:
//
//   struct  from "/**\n * @title   CoprocessorConfig" through the struct's closing brace
//   library from the "/// keccak256(abi.encode(...)" comment through the end of `setCoprocessor`
//
// ONE DELIBERATE DIFFERENCE: `COPROCESSOR_CONFIG_LOCATION` is `public` here and `private` upstream (in every
// release so far). `LibForgeFhevmConfig` reads the slot from outside this library, which a private constant
// forbids; the VALUE is upstream's, unchanged.
//
// Checked against the fhevm repository at the tag this generation vendors, v0.15.0-3: the struct, the slot
// value and both functions are byte-identical (and so they were at v0.14.1). Upstream's file also holds
// operator constants between the slot and the functions; they are unrelated to the config and deliberately not
// carried here. @fhevm/solidity itself is still published on the 0.13 line only (0.13.4), with the same text.
//
// ## Why a copy, and why byte-exact
//
// `FHE` is an internal library: its code runs in the CALLER's context, and every call reads the three
// addresses out of `COPROCESSOR_CONFIG_LOCATION` in the caller's own storage. A forge test that wants to
// call `FHE.*` itself therefore has to write that slot, and a dApp compiled against a different network's
// config has to be retargeted at it. One digit wrong in the slot and the reads land somewhere harmless
// looking and entirely wrong, so the constant and the struct layout are not restated here — they are
// lifted, unchanged, from the library that defines them.
//
// This package deliberately does not depend on `@fhevm/solidity`, which is the other half of the reason:
// a consumer's tests must work whatever version of that library they compile against, and a copy pinned
// to a known version is checkable in a way a peer dependency is not.
//
// ## The struct is a distinct type
//
// `CoprocessorConfig` below has the same name and the same layout as the upstream one, but it is a
// DIFFERENT Solidity type. A file that imports both this and `@fhevm/solidity/lib/Impl.sol` must alias
// one of them. Layout is what matters — the storage written is identical either way.

/**
 * @title   CoprocessorConfig
 * @notice  This struct contains all addresses of core contracts, which are needed in a typical dApp.
 */
struct CoprocessorConfig {
    address ACLAddress;
    address CoprocessorAddress;
    address KMSVerifierAddress;
}

/**
 * @title   LibFhevmCoprocessorConfig
 * @notice  Reads and writes the coprocessor config in the CALLING contract's storage, exactly as the
 *          fhevm Solidity library's `Impl` does — same slot, same layout, same semantics.
 * @dev     `setCoprocessor` is what `SepoliaConfig` and `ZamaConfig` call from their constructors. Use it
 *          to make a contract FHE-capable that is not one of those, a forge test contract most of all.
 */
library LibFhevmCoprocessorConfig {
    /// keccak256(abi.encode(uint256(keccak256("confidential.storage.config")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 public constant COPROCESSOR_CONFIG_LOCATION =
        0x9e7b61f58c47dc699ac88507c4f5bb9f121c03808c5676a8078fe583e4649700;

    /**
     * @dev Returns the Coprocessor config.
     */
    function getCoprocessorConfig() internal pure returns (CoprocessorConfig storage $) {
        assembly {
            $.slot := COPROCESSOR_CONFIG_LOCATION
        }
    }

    /**
     * @notice                  Sets the coprocessor addresses.
     * @param coprocessorConfig Coprocessor config struct that contains contract addresses.
     */
    function setCoprocessor(CoprocessorConfig memory coprocessorConfig) internal {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        $.ACLAddress = coprocessorConfig.ACLAddress;
        $.CoprocessorAddress = coprocessorConfig.CoprocessorAddress;
        $.KMSVerifierAddress = coprocessorConfig.KMSVerifierAddress;
    }
}
