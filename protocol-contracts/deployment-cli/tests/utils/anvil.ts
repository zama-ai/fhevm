import { spawn } from "node:child_process";
import { setTimeout as delay } from "node:timers/promises";
import type { Chain } from "viem";
import { createTestClient, http, publicActions } from "viem";
import { TEST_CONFIG } from "../test-config.js";

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

/**
 * Anvil process management for forked chain testing
 */

export async function waitForRpcReady(
    client: ExtendedTestClient,
    attempts = 100,
    intervalMs = 200,
): Promise<void> {
    for (let i = 0; i < attempts; i += 1) {
        try {
            // eslint-disable-next-line no-await-in-loop
            await client.getBlockNumber();
            return;
        } catch (_) {
            if (i === attempts - 1) throw _;
            // eslint-disable-next-line no-await-in-loop
            await delay(intervalMs);
        }
    }
}

export async function startAnvilFork(
    params: StartAnvilParams,
): Promise<{ proc: AnvilProcess; client: ExtendedTestClient }> {
    const {
        forkUrl,
        chainId,
        port,
        host = "127.0.0.1",
        codeSizeLimit = 31000,
    } = params;
    const proc = spawn(
        "anvil",
        [
            "--fork-url",
            forkUrl,
            "--chain-id",
            String(chainId),
            "--port",
            String(port),
            "--host",
            host,
            "--code-size-limit",
            String(codeSizeLimit),
        ],
        { stdio: "ignore" },
    );

    // Create a custom chain definition for the forked network
    const customChain: Chain = {
        id: chainId,
        name:
            chainId === TEST_CONFIG.SEPOLIA_CHAIN_ID
                ? "Sepolia Fork"
                : "Gateway Fork",
        nativeCurrency: { name: "Ether", symbol: "ETH", decimals: 18 },
        rpcUrls: {
            default: { http: [`http://${host}:${port}`] },
            public: { http: [`http://${host}:${port}`] },
        },
    };

    const client = createTestClient({
        chain: customChain,
        mode: "anvil",
        transport: http(`http://${host}:${port}`),
    }).extend(publicActions);

    await waitForRpcReady(client);
    return { proc, client };
}

export async function stopAnvil(proc: AnvilProcess): Promise<void> {
    if (!proc) return;
    proc.kill();
    await new Promise((resolve) => {
        proc?.once("exit", resolve);
        setTimeout(resolve, 250).unref();
    });
}
