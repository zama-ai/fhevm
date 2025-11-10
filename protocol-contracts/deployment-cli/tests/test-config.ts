import { parseEther } from "viem";

/**
 * Centralized test configuration constants
 */
export const TEST_CONFIG = {
    // L1 (Sepolia) fork
    ANVIL_L1_HOST: "127.0.0.1",
    ANVIL_L1_PORT: 8545,
    ANVIL_L1_URL: "http://127.0.0.1:8545",
    SEPOLIA_CHAIN_ID: 11155111,

    // Gateway fork
    ANVIL_GATEWAY_HOST: "127.0.0.1",
    ANVIL_GATEWAY_PORT: 8546,
    ANVIL_GATEWAY_URL: "http://127.0.0.1:8546",
    GATEWAY_CHAIN_ID: 10901,

    // Other
    ADMIN_EXECUTOR_BALANCE: parseEther("10").toString(),
    DEPLOYMENT_STATE_FILE: "zama-protocol-testnet-v0-9.addresses.json",
} as const;
