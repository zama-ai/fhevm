// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title  FhevmCleartextConfig
 * @notice The Solidity face of `sdk/cleartext-config.json`.
 *
 * That JSON, at the repository's `sdk/` root, is where every value the harness, the payload and the js-sdk
 * cleartext relayer must agree on is DECIDED. It sits above the generations because they share it. This
 * library is its Solidity face, and it is the only one: no script may declare a constant that also exists
 * there. `internal/cleartext-config.ts` is the TypeScript face of the same JSON.
 *
 * ## Same names, deliberately
 *
 * Every constant keeps its TypeScript name verbatim — `CLEARTEXT_KMS_NODE_COUNT`, not `KMS_NODE_COUNT`.
 * The prefix looks like noise inside a file that is entirely about the cleartext stack, and dropping it
 * is exactly the temptation the rule exists to refuse. The names are the only thing that makes a drift
 * FINDABLE: `grep CLEARTEXT_KMS_NODE_COUNT` has to reach both sides, and a renamed mirror is a copy that
 * nobody will think to check. Naming is the check.
 *
 * ## Values are byte-for-byte, including what looks like a typo
 *
 * The mnemonic paths end in `/`, matching the TypeScript. That trailing slash is load-bearing here in a
 * way it is not there: forge's `vm.deriveKey(mnemonic, path, index)` derives at `{path}{index}` by plain
 * concatenation, so a path written `m/44'/60'/0'/2` silently derives `m/44'/60'/0'/20` for index 0 — a
 * valid path, a real key, an entirely wrong signer set. A hand-copied constant in `FhevmVerify.s.sol` had
 * exactly that shape before this file existed. Copy values, do not tidy them.
 *
 * ## Why a checked-in face rather than a generated one
 *
 * Solidity cannot read JSON at compile time, so this file has to exist either way. Generating it at build
 * time would put a build step between an operator and a `forge script` they may need to run against a
 * testnet from a checkout that has not been built — so it is checked in, and made safe by being CHECKED:
 * `test/cleartext-config-mirror.test.ts` requires the same names in the same order with equal values, and
 * recomputes every value the JSON records a formula for.
 *
 * @dev A library rather than an abstract contract, so a script that needs one value can use it without
 *      inheriting, and so nothing here can be overridden by a subclass. `internal constant` costs no
 *      bytecode, which is why the mirror is COMPLETE rather than trimmed to what today's scripts use — a
 *      partial mirror is an invitation to declare the missing half somewhere else.
 */
library FhevmCleartextConfig {
    // uint48(uint256(keccak256("fhevm.cheat.chainId cleartext gateway")))
    uint256 internal constant CLEARTEXT_GATEWAY_CHAIN_ID = 100733346448153;

    string internal constant CLEARTEXT_RELAYER_URL = "https://relayer.cleartext.foo";

    // Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext input verification"))))`.
    address internal constant CLEARTEXT_INPUT_VERIFICATION_ADDRESS = 0x6189F6c0c3E40B4a3c72ec86262295D78d845297;

    // Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext decryption"))))`.
    address internal constant CLEARTEXT_DECRYPTION_ADDRESS = 0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721;

    uint256 internal constant CLEARTEXT_COPROCESSOR_COUNT = 4;
    uint256 internal constant CLEARTEXT_COPROCESSOR_THRESHOLD = 4;
    uint256 internal constant CLEARTEXT_KMS_NODE_COUNT = 4;

    uint256 internal constant CLEARTEXT_HCU_CAP_PER_BLOCK = 281474976710655;
    uint256 internal constant CLEARTEXT_MAX_HCU_DEPTH_PER_TX = 5000000;
    uint256 internal constant CLEARTEXT_MAX_HCU_PER_TX = 20000000;

    /**
     * @dev Published on purpose, and NOT the deploy mnemonic — a different string with a different job
     *      (`internal/constants.ts` carries the same warning about its own).
     *
     *      This one is what makes a cleartext stack SDK-compatible: the js-sdk cleartext relayer derives
     *      its keys from it at the paths below and looks a signer up by the address they produce. Seed a
     *      different set and every other check still passes — the stack deploys, verifies against itself,
     *      and fails only when the relayer arrives. It is also why the stack is testnet-only: on mainnet
     *      these are keys everyone has.
     */
    string internal constant FHEVM_MNEMONIC = "test test test test test test test future home engine virtual motion";

    string internal constant CLEARTEXT_COPROCESSORS_MNEMONIC = FHEVM_MNEMONIC;
    string internal constant CLEARTEXT_COPROCESSORS_MNEMONIC_PATH = "m/44'/60'/0'/2/";
    uint32 internal constant CLEARTEXT_COPROCESSORS_MNEMONIC_INDEX = 0;

    string internal constant CLEARTEXT_KMS_NODES_MNEMONIC = FHEVM_MNEMONIC;
    string internal constant CLEARTEXT_KMS_NODES_MNEMONIC_PATH = "m/44'/60'/0'/3/";
    uint32 internal constant CLEARTEXT_KMS_NODES_MNEMONIC_INDEX = 0;

    string internal constant CLEARTEXT_KMS_NODES_TX_SENDER_MNEMONIC = FHEVM_MNEMONIC;
    string internal constant CLEARTEXT_KMS_NODES_TX_SENDER_MNEMONIC_PATH = "m/44'/60'/0'/4/";
    uint32 internal constant CLEARTEXT_KMS_NODES_TX_SENDER_MNEMONIC_INDEX = 0;

    string internal constant CLEARTEXT_KMS_NODE_IP_ADDRESS_PREFIX = "127.0.0.";
    string internal constant CLEARTEXT_KMS_NODE_STORAGE_URL_PREFIX = "s3://kms-bucket-";
}
