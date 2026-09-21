import { randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { readFile, readdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import type { Signature } from '@solana/kit';
export type RecoveryReceipt = { signature: Signature; lastValidBlockHeight: string };
import type { HostDeployContext } from './send';

// Persist before submission: a lost RPC acknowledgement must not lose the transaction receipt.
export async function journalRecovery(context: HostDeployContext, directory: string) {
  const namespace = process.env.SOLANA_PREVIEW_NAMESPACE;
  const kubectl = (args: string[], input?: string): string => {
    try {
      // Node's child stdin is a socket on Linux; kubectl opens /dev/stdin as a file.
      // A shell pipe keeps patch contents off argv and gives kubectl a readable pipe.
      return input === undefined
        ? execFileSync('kubectl', args, { stdio: ['pipe', 'pipe', 'ignore'], encoding: 'utf8' })
        : execFileSync('sh', ['-c', 'cat | kubectl "$@"', 'kubectl', ...args], {
            input, stdio: ['pipe', 'pipe', 'ignore'], encoding: 'utf8',
          });
    } catch {
      throw new Error('Cannot persist recovery accounting; no further transactions submitted');
    }
  };
  if (namespace) {
    if (!/^fhevm-ci-[a-z0-9-]+$/.test(namespace)) throw new Error('Invalid preview namespace');
    const remote = JSON.parse(kubectl(['get', 'configmap', 'solana-recovery-journal', '-n', namespace, '-o', 'json']));
    for (const [name, value] of Object.entries(remote.data ?? {})) {
      if (/^(receipts-|inventory-)[a-z0-9-]+\.json$/.test(name))
        await writeFile(path.join(directory, name), String(value), { mode: 0o600 });
    }
  }
  const name = `receipts-${randomUUID()}.json`;
  const receipts: RecoveryReceipt[] = [];
  const persist = async (file: string, value: string) => {
    await writeFile(path.join(directory, file), value, { mode: 0o600, flush: true });
    if (namespace)
      kubectl(
        ['patch', 'configmap', 'solana-recovery-journal', '-n', namespace, '--type=merge', '--patch-file=/dev/stdin'],
        JSON.stringify({ data: { [file]: value } }),
      );
  };
  context.beforeSubmit = async (signature, lastValidBlockHeight) => {
    receipts.push({ signature, lastValidBlockHeight: lastValidBlockHeight.toString() });
    await persist(name, JSON.stringify(receipts));
  };
  return {
    persist,
    receipts: async (): Promise<RecoveryReceipt[]> => {
      const signatures = new Map<Signature, RecoveryReceipt>();
      for (const file of await readdir(directory)) {
        if (/^receipts-[a-z0-9-]+\.json$/.test(file))
          for (const receipt of JSON.parse(await readFile(path.join(directory, file), 'utf8')))
            signatures.set(receipt.signature, receipt);
      }
      return [...signatures.values()];
    },
  };
}
