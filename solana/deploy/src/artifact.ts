import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';

import type { SolanaDeployProgram } from './constants';

/** Address-only output consumed by the existing contracts chart. */
export const writeSolanaAddressArtifact = async (
  addressesDir: string,
  ids: Partial<Record<SolanaDeployProgram, string>>,
): Promise<string> => {
  await mkdir(addressesDir, { recursive: true });
  const file = path.join(addressesDir, '.env.solana');
  // `host` and `demos` may share one addresses directory, so keep the other command's entries.
  const existing = await readFile(file, 'utf8').catch(() => '');
  const entries = new Map(
    existing
      .split('\n')
      .filter((line) => line.includes('='))
      .map((line) => line.split('=', 2) as [string, string]),
  );
  for (const [name, id] of Object.entries(ids)) entries.set(`${name.toUpperCase()}_ADDRESS`, id);
  await writeFile(file, [...entries].map(([name, id]) => `${name}=${id}\n`).join(''));
  return file;
};
