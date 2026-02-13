export interface TestSuite {
  label: string;
  grep: string;
  parallel?: boolean;
}

export const TEST_SUITES: Record<string, TestSuite> = {
  "input-proof": {
    label: "INPUT PROOF (uint64)",
    grep: "test user input uint64",
  },
  "input-proof-compute-decrypt": {
    label: "INPUT PROOF + COMPUTE + DECRYPT (uint64)",
    grep: "test add 42 to uint64 input and decrypt",
  },
  "user-decryption": {
    label: "USER DECRYPTION",
    grep: "test user decrypt",
  },
  "delegated-user-decryption": {
    label: "DELEGATED USER DECRYPTION",
    grep: "test delegated user decrypt",
  },
  "public-decryption": {
    label: "PUBLIC DECRYPTION",
    grep: "test async decrypt (uint.*|ebytes.* trivial|ebytes64 non-trivial|ebytes256 non-trivial with snapshot|addresses|several addresses)",
  },
  "erc20": {
    label: "ERC20",
    grep: "should transfer tokens between two users.",
  },
  "public-decrypt-http-ebool": {
    label: "PUBLIC DECRYPTION OVER HTTP FOR EBOOL",
    grep: "test HTTPPublicDecrypt ebool",
  },
  "public-decrypt-http-mixed": {
    label: "PUBLIC DECRYPTION OVER HTTP FOR MIXED",
    grep: "test HTTPPublicDecrypt mixed",
  },
  "operators": {
    label: "OPERATORS",
    grep: "test operator|FHEVM manual operations",
    parallel: true,
  },
  "random": {
    label: "RANDOM OPERATORS",
    grep: "generate and decrypt|generating rand in reverting sub-call|upper bound and decrypt",
  },
  "random-subset": {
    label: "RANDOM OPERATORS (SUBSET)",
    grep: "64 bits generate and decrypt|generating rand in reverting sub-call|64 bits generate with upper bound and decrypt",
  },
  "paused-host-contracts": {
    label: "PAUSED HOST CONTRACTS",
    grep: "test paused host.*",
  },
  "paused-gateway-contracts": {
    label: "PAUSED GATEWAY CONTRACTS",
    grep: "test paused gateway.*",
  },
};

export const SUITE_NAMES = Object.keys(TEST_SUITES);
