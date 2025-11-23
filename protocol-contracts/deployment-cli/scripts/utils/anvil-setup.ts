import type { AnvilProcess, ExtendedTestClient } from "../../tests/types.js";
import {
    startAnvilFork,
    stopAnvil,
    waitForRpcReady,
} from "../../tests/utils/anvil.js";

export interface AnvilForks {
    l1: { proc: AnvilProcess; client: ExtendedTestClient };
    l2: { proc: AnvilProcess; client: ExtendedTestClient };
}

export interface StartAnvilForksParams {
    l1RpcUrl: string;
    l2RpcUrl: string;
    l1ChainId: number;
    l2ChainId: number;
    l1Port: number;
    l2Port: number;
}

/**
 * Start anvil forks for both L1 and L2 networks
 */
export async function startAnvilForks(
    params: StartAnvilForksParams,
): Promise<AnvilForks> {
    const { l1RpcUrl, l2RpcUrl, l1ChainId, l2ChainId, l1Port, l2Port } = params;

    const l1 = await startAnvilFork({
        forkUrl: l1RpcUrl,
        chainId: l1ChainId,
        port: l1Port,
    });
    await waitForRpcReady(l1.client);

    let l2: { proc: AnvilProcess; client: ExtendedTestClient };
    try {
        l2 = await startAnvilFork({
            forkUrl: l2RpcUrl,
            chainId: l2ChainId,
            port: l2Port,
        });
        await waitForRpcReady(l2.client);
    } catch (e) {
        await stopAnvil(l1.proc);
        throw e;
    }

    return { l1, l2 };
}

/**
 * Stop both anvil fork processes
 */
export async function stopAnvilForks(forks: AnvilForks): Promise<void> {
    await Promise.all([stopAnvil(forks.l1.proc), stopAnvil(forks.l2.proc)]);
}
