// Private recovery keys survive a crashed test process. Preview runs mirror them into the
// namespace before funding; the local copy remains available after namespace destruction.
import fs from 'node:fs/promises';
import path from 'node:path';
import { spawn } from 'node:child_process';
import { generateSolanaKeypair, loadKeypairSigner, type GeneratedKeypair } from './provision';

export const recoveryDirectory = (): string => {
  const directory = process.env.SOLANA_RECOVERY_DIR;
  if (!directory || !path.isAbsolute(directory)) throw new Error('devnet requires an absolute SOLANA_RECOVERY_DIR');
  return directory;
};

export const saveRecoveryKey = async (name: string, bytes: Uint8Array): Promise<string> => {
  if (!/^[a-zA-Z0-9-]+$/.test(name)) throw new Error('invalid recovery key name');
  const directory = recoveryDirectory();
  await fs.mkdir(directory, { recursive: true, mode: 0o700 });
  const file = path.join(directory, `${name}.json`);
  await fs.writeFile(file, JSON.stringify([...bytes]), { mode: 0o600, flag: 'wx' });
  await mirrorRecoveryKeys();
  return file;
};

export const mirrorRecoveryKeys = async (): Promise<void> => {
  const namespace = process.env.SOLANA_PREVIEW_NAMESPACE;
  if (!namespace) throw new Error("SOLANA_PREVIEW_NAMESPACE is required before devnet funding");
  if (!/^fhevm-ci-[a-z0-9-]+$/.test(namespace)) throw new Error('invalid preview namespace');
  const directory = recoveryDirectory();
  const data: Record<string, string> = {};
  for (const name of await fs.readdir(directory)) {
    if (/^(run-|demo-|browser-).+\.json$/.test(name)) {
      const wallet = await loadKeypairSigner(path.join(directory, name));
      data[`run-${wallet.address}.json`] = (await fs.readFile(path.join(directory, name))).toString('base64');
    } else if (name.startsWith('inventory-') && name.endsWith('.json')) {
      data[name] = (await fs.readFile(path.join(directory, name))).toString('base64');
    }
  }
  // stdin only: neither secret data nor kubectl diagnostics can reach process output.
  await new Promise<void>((resolve, reject) => {
    const child = spawn('kubectl', ['patch', 'secret', 'solana-recovery', '-n', namespace, '--type=merge', '--patch-file=/dev/stdin'], { stdio: ['pipe', 'ignore', 'ignore'] });
    child.on('error', () => reject(new Error('cannot mirror recovery keys')));
    child.on('close', code => code === 0 ? resolve() : reject(new Error('cannot mirror recovery keys; refusing to fund')));
    child.stdin.on('error', () => {});
    child.stdin.end(JSON.stringify({ data }));
  });
};

export const ensureDemoRecoveryKey = async (role: string): Promise<void> => {
  const file = path.join(recoveryDirectory(), `demo-${role}.json`);
  try { await loadKeypairSigner(file); return; }
  catch (error) { if ((error as NodeJS.ErrnoException).code !== 'ENOENT') throw error; }
  const wallet = await generateSolanaKeypair();
  await saveRecoveryKey(`demo-${role}`, wallet.bytes);
};

export const recordRunWallet = async (wallet: GeneratedKeypair): Promise<void> => {
  await saveRecoveryKey(`run-${wallet.signer.address}`, wallet.bytes);
};
