// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// Shared constants for FHEVM host scripts.
// Add further constants here so they can be imported by any script or library.

string constant FHEVM_HOST_CONTRACTS_MNEMONIC =
    "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer";
string constant ANVIL_MNEMONIC = "test test test test test test test test test test test junk";

string constant FHEVM_MNEMONIC = "test test test test test test test future home engine virtual motion";
string constant COPROCESSOR_SIGNERS_MNEMONIC_PREFIX = "m/44'/60'/0'/2/";
string constant KMS_SIGNERS_MNEMONIC_PREFIX = "m/44'/60'/0'/3/";
string constant PAUSERS_MNEMONIC_PREFIX = "m/44'/60'/0'/4/";

// BIP-32 path prefix (trailing slash included) and account index used to derive
// the deployer from FHEVM_HOST_CONTRACTS_MNEMONIC. Full path = prefix + index.
string constant FHEVM_HOST_CONTRACTS_MNEMONIC_DERIVATION_PREFIX = "m/44'/60'/0'/0/";
uint32 constant FHEVM_HOST_CONTRACTS_MNEMONIC_DERIVATION_INDEX = 5;

string constant DEPLOYER_PRIVATE_KEY_ENV = "DEPLOYER_PRIVATE_KEY";

// Dedicated key used to deploy `EmptyUUPSProxyACL` / `EmptyUUPSProxy` impls so
// the main deployer's nonce timeline stays aligned with the committed proxy
// addresses. On a real chain this address must be pre-funded with gas.
string constant EMPTY_UUPS_DEPLOYER_PRIVATE_KEY_ENV = "EMPTY_UUPS_DEPLOYER_PRIVATE_KEY";
uint32 constant FHEVM_HOST_CONTRACTS_MNEMONIC_EMPTY_UUPS_DEPLOYER_INDEX = 4;

// Indexed address-list env vars. Each pair: `NUM_<ROLE>` for the count, and
// `<ROLE>_ADDRESS_i` for the i-th address. If the count var is unset, the
// resolver iterates until the first missing index. If NEITHER the count nor
// any indexed var is set, the resolver falls back to deriving `DEFAULT_*_COUNT`
// addresses from FHEVM_HOST_CONTRACTS_MNEMONIC starting at
// `DEFAULT_*_START_INDEX` on the standard derivation prefix.

string constant NUM_PAUSERS_ENV = "NUM_PAUSERS";
string constant PAUSER_ADDRESS_ENV_PREFIX = "PAUSER_ADDRESS_";
uint32 constant DEFAULT_PAUSER_START_INDEX = 2; // accounts[2..3]
uint32 constant DEFAULT_PAUSER_COUNT = 2;

string constant NUM_KMS_NODES_ENV = "NUM_KMS_NODES";
string constant KMS_SIGNER_ADDRESS_ENV_PREFIX = "KMS_SIGNER_ADDRESS_";
uint32 constant DEFAULT_KMS_SIGNER_START_INDEX = 7; // accounts[7..10]
uint32 constant DEFAULT_KMS_SIGNER_COUNT = 4;

string constant NUM_COPROCESSORS_ENV = "NUM_COPROCESSORS";
string constant COPROCESSOR_SIGNER_ADDRESS_ENV_PREFIX = "COPROCESSOR_SIGNER_ADDRESS_";
uint32 constant DEFAULT_COPROCESSOR_SIGNER_START_INDEX = 11; // accounts[11..14]
uint32 constant DEFAULT_COPROCESSOR_SIGNER_COUNT = 4;

// Path to the auto-generated FHEVMHostAddresses.sol, relative to the
// foundry project root (sdk/js-sdk/contracts/).
string constant FHEVM_HOST_ADDRESSES_FILE = "src/host-contracts/addresses/FHEVMHostAddresses.sol";

// Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext input verification"))))`.
address constant INPUT_VERIFICATION_ADDRESS = 0x6189F6c0c3E40B4a3c72ec86262295D78d845297;
// Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext decryption"))))`.
address constant DECRYPTION_ADDRESS = 0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721;

string constant CHAIN_ID_GATEWAY_ENV = "CHAIN_ID_GATEWAY";
uint64 constant DEFAULT_CHAIN_ID_GATEWAY = 654321;

// Multisig thresholds for KMS (public decryption) and coprocessor input
// verification. In practice the threshold is set to `floor(n/2) + 1` with
// `n` the number of signers.
string constant KMS_THRESHOLD_ENV = "PUBLIC_DECRYPTION_THRESHOLD";
uint256 constant DEFAULT_KMS_THRESHOLD = 1;

string constant COPROCESSOR_THRESHOLD_ENV = "COPROCESSOR_THRESHOLD";
uint256 constant DEFAULT_COPROCESSOR_THRESHOLD = 1;

string constant FHE_TEST_USER_PRIVATE_KEY_ENV = "FHE_TEST_USER_PRIVATE_KEY";

// Cheat-slot address for the FHEVM registry.
// Calculated as `address(uint160(uint256(keccak256("fhevm cheat code"))))`.
address constant FHEVM_CHEATS_ADDRESS = 0xC71923396eE5fFc886cb769aC7841b8d8d94DD50;

// type(uint48).max, // hcuCapPerBlock
// 5_000_000, // maxHCUDepthPerTx
// 20_000_000 // maxHCUPerTx
