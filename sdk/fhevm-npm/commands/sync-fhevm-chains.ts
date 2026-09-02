import { writeFileSync } from 'node:fs';
import { relative } from 'node:path';

import {
  CHAINS_CONFIG_FILE,
  chainsConfigPath,
  githubRegistryReader,
  pinnedCommit,
  renderChainsConfig,
} from '../base/fhevm-chains.ts';

// The writer half of the pair (check-fhevm-chains-origin is the read-only half). The pin resolves, in
// order: --commit, --latest (registry HEAD), else the pin already recorded in the committed file.
export async function syncFhevmChains(options: {
  readonly workspaceRoot: string;
  readonly commit?: string;
  readonly latest: boolean;
}): Promise<void> {
  const reader = githubRegistryReader();
  const commit = resolveCommit(options, () => reader.resolveHead());
  const path = chainsConfigPath(options.workspaceRoot);
  writeFileSync(path, await renderChainsConfig(options.workspaceRoot, reader, commit));
  console.log(`✅ Generated ${relative(process.cwd(), path) || '.'} @ ${commit.slice(0, 12)}`);
}

function resolveCommit(
  options: { readonly workspaceRoot: string; readonly commit?: string; readonly latest: boolean },
  resolveHead: () => string,
): string {
  if (options.commit !== undefined && options.latest) {
    throw new Error('sync-fhevm-chains: pass --commit or --latest, not both');
  }
  if (options.commit !== undefined) {
    if (!/^[0-9a-f]{40}$/.test(options.commit)) {
      throw new Error(`sync-fhevm-chains: --commit expects a full 40-hex sha, got '${options.commit}'`);
    }
    return options.commit;
  }
  if (options.latest) return resolveHead();
  const pinned = pinnedCommit(options.workspaceRoot);
  if (pinned === undefined) {
    throw new Error(`${CHAINS_CONFIG_FILE} does not exist yet — run sync-fhevm-chains --latest to create it`);
  }
  return pinned;
}
