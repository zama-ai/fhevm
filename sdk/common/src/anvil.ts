import { spawn, type ChildProcess } from 'node:child_process';
import { createPublicClient, http } from 'viem';
import { foundry } from 'viem/chains';
import { ANVIL_PORT } from './constants.ts';

export { ANVIL_PORT, ANVIL_RPC_URL, MNEMONIC } from './constants.ts';
export { isPortOpen } from './net.ts';

export type AnvilNode = {
  process: ChildProcess;
  rpcUrl: string;
};

function sleep(ms: number): Promise<void> {
  return new Promise((resolveDelay) => {
    setTimeout(resolveDelay, ms);
  });
}

function anvilRpcUrl(port = ANVIL_PORT): string {
  return `http://127.0.0.1:${port}`;
}

export function startAnvil(parameters?: {
  readonly port?: number | undefined;
  readonly mnemonic?: string | undefined;
  readonly derivationPath?: string | undefined;
  readonly printBanner?: boolean | undefined;
}): AnvilNode {
  const port = parameters?.port ?? ANVIL_PORT;
  const mnemonic = parameters?.mnemonic;
  const derivationPath = parameters?.derivationPath ?? "m/44'/60'/0'/0/";

  // Raise the EIP-170 (24576 B) contract-size cap: the cleartext test implementations (e.g.
  // CleartextFHEVMExecutor ~25 KB) inline cleartext arithmetic and exceed the mainnet limit.
  const args = ['--host', '127.0.0.1', '--port', String(port), '--code-size-limit', '60000'];

  if (mnemonic !== undefined) {
    args.push('--mnemonic');
    args.push(mnemonic);
    args.push('--derivation-path');
    args.push(derivationPath);
  }

  const rpcUrl = anvilRpcUrl(port);
  const anvil = spawn('anvil', args, {
    stdio: 'ignore',
  });

  if (parameters?.printBanner === true) {
    const separator = '================================================================================';
    console.log(`
🟨${separator}
⛓️  ANVIL STARTING
🔌 RPC URL: ${rpcUrl}
🟨${separator}
`);
  }

  return { process: anvil, rpcUrl };
}

export async function waitForAnvil(rpcUrl: string): Promise<void> {
  const publicClient = createPublicClient({ chain: foundry, transport: http(rpcUrl) });
  const deadline = Date.now() + 10_000;
  let lastError: unknown;

  while (Date.now() < deadline) {
    try {
      await publicClient.getChainId();
      return;
    } catch (error) {
      lastError = error;
      await sleep(100);
    }
  }

  throw new Error(`Timed out waiting for anvil at ${rpcUrl}`, { cause: lastError });
}

export async function stopAnvil(anvil: ChildProcess): Promise<void> {
  if (anvil.exitCode !== null || anvil.signalCode !== null || anvil.pid === undefined) {
    return;
  }

  const exited = new Promise<void>((resolveExit) => {
    anvil.once('exit', () => {
      resolveExit();
    });
  });

  anvil.kill('SIGTERM');
  const stoppedGracefully = await Promise.race([exited.then(() => true), sleep(2_000).then(() => false)]);
  if (stoppedGracefully) return;

  anvil.kill('SIGKILL');
  await exited;
}
