// The cleartext stack's configuration — the values the harness and the payload must agree on.
//
// NOT the source of truth. `sdk/cleartext-config.json` is: it sits at the repository's
// sdk root because the generations share it, it records the keccak FORMULA behind each derived value rather
// than only the hex, and it is what `test/cleartext-config-mirror.test.ts` checks this file against — name
// for name, in declaration order, value for value, and bigint-vs-number literal shape included.
// `create2-deploy/script/FhevmCleartextConfig.sol` is the same JSON's Solidity face.
//
// This file exists TWICE, byte-for-byte identical:
//
//   internal/cleartext-config.ts   the source of truth. Edit here, and only here.
//   pkg/ts/cleartext-config.ts     a generated copy, written by `npm run generate:cleartext-config`
//                                  (a step of `make generate`).
//
// A copy rather than an import, because neither side can reach the other: `internal/tsconfig.json` sets
// `rootDir: "."` so internal/ cannot import the payload, and the payload must not import the harness at
// all — internal/ is never published, so a payload importing it would ship a dangling
// specifier. Copying is the only mechanism that leaves both sides self-contained.
//
// Byte-for-byte is what makes the duplication safe: `npm run check:cleartext-config` compares the two and
// `test/templates.test.ts` fails on any difference, so the copy cannot silently drift from the original.
// Keep this module free of imports and of anything Node-specific — the copy is compiled into the
// published package, which must stay browser-safe.

// uint48(uint256(keccak256("fhevm.cheat.chainId cleartext gateway")))
export const CLEARTEXT_GATEWAY_CHAIN_ID = 100733346448153n;

export const CLEARTEXT_RELAYER_URL = 'https://relayer.cleartext.foo';

// Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext input verification"))))`.
export const CLEARTEXT_INPUT_VERIFICATION_ADDRESS = '0x6189F6c0c3E40B4a3c72ec86262295D78d845297';

// Calculated as `address(uint160(uint256(keccak256("fhevm.cheat.address cleartext decryption"))))`.
export const CLEARTEXT_DECRYPTION_ADDRESS = '0xEaaA2FC6BC259dF015Aa7Dc8e59e0B67df622721';

export const CLEARTEXT_COPROCESSOR_COUNT = 4;
export const CLEARTEXT_COPROCESSOR_THRESHOLD = 4;
export const CLEARTEXT_KMS_NODE_COUNT = 4;

export const CLEARTEXT_HCU_CAP_PER_BLOCK = 281474976710655n;
export const CLEARTEXT_MAX_HCU_DEPTH_PER_TX = 5000000n;
export const CLEARTEXT_MAX_HCU_PER_TX = 20000000n;

export const FHEVM_MNEMONIC = 'test test test test test test test future home engine virtual motion';

export const CLEARTEXT_COPROCESSORS_MNEMONIC = FHEVM_MNEMONIC;
export const CLEARTEXT_COPROCESSORS_MNEMONIC_PATH = "m/44'/60'/0'/2/";
export const CLEARTEXT_COPROCESSORS_MNEMONIC_INDEX = 0;

export const CLEARTEXT_KMS_NODES_MNEMONIC = FHEVM_MNEMONIC;
export const CLEARTEXT_KMS_NODES_MNEMONIC_PATH = "m/44'/60'/0'/3/";
export const CLEARTEXT_KMS_NODES_MNEMONIC_INDEX = 0;

export const CLEARTEXT_KMS_NODES_TX_SENDER_MNEMONIC = FHEVM_MNEMONIC;
export const CLEARTEXT_KMS_NODES_TX_SENDER_MNEMONIC_PATH = "m/44'/60'/0'/4/";
export const CLEARTEXT_KMS_NODES_TX_SENDER_MNEMONIC_INDEX = 0;

export const CLEARTEXT_KMS_NODE_IP_ADDRESS_PREFIX = '127.0.0.';
export const CLEARTEXT_KMS_NODE_STORAGE_URL_PREFIX = 's3://kms-bucket-';
