import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';

import type { SolanaDeployProgram } from './constants';

/** Address-only output consumed by the existing contracts chart. */
export const writeSolanaAddressArtifact = async (
  addressesDir: string,
  ids: Partial<Record<SolanaDeployProgram, string>>,
): Promise<string> => {
  await mkdir(addressesDir, { recursive: true });
  const file = path.join(addressesDir, '.env.solana');
  await writeFile(
    file,
    Object.entries(ids)
      .map(([name, id]) => `${name.toUpperCase()}_ADDRESS=${id}\n`)
      .join(''),
  );
  return file;
};
