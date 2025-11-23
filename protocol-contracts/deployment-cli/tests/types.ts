import type { spawn } from "node:child_process";

/**
 * Shared type definitions for test suite
 */

// Anvil process types
export type AnvilProcess = ReturnType<typeof spawn> | undefined;

export interface StartAnvilParams {
    forkUrl: string;
    chainId: number;
    port: number;
    host?: string;
    codeSizeLimit?: number;
}

// Client types
export type ExtendedTestClient = ReturnType<
    typeof import("viem").createTestClient
> &
    ReturnType<typeof import("viem").publicActions>;
