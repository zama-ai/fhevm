import { existsSync, readFileSync } from 'node:fs';

import type { CommandReport } from '../base/diagnostics.ts';
import {
  CHAINS_CONFIG_FILE,
  type RegistryReader,
  chainsConfigPath,
  githubRegistryReader,
  pinnedCommit,
  renderChainsConfig,
} from '../base/fhevm-chains.ts';

// Read-only currency check: does the committed fhevm-chains.config.json still say what the registry's
// CURRENT main head says? Deliberately head-based, not pin-based — a registry release that changed an
// fhevm address must turn this red until `sync-fhevm-chains --latest` catches up. The comparison embeds
// the committed pin into the fresh render, so unrelated registry commits (staking, tokens) stay green.
export async function checkFhevmChainsOrigin(options: {
  readonly workspaceRoot: string;
  readonly reader?: RegistryReader;
}): Promise<CommandReport> {
  const path = chainsConfigPath(options.workspaceRoot);
  const violation = (message: string): CommandReport => ({
    command: 'check-fhevm-chains-origin',
    checkedPackageKeys: [`./${CHAINS_CONFIG_FILE}`],
    checkedItemLabel: 'chains config file(s)',
    violations: [{ rule: 'fhevm-chains-origin', packageKey: `./${CHAINS_CONFIG_FILE}`, message }],
  });

  if (!existsSync(path)) {
    return violation('missing — run `fhevm-npm sync-fhevm-chains --latest` to create it');
  }
  const pinned = pinnedCommit(options.workspaceRoot);
  if (pinned === undefined) throw new Error('unreachable: the file exists');

  const reader = options.reader ?? githubRegistryReader();
  const head = reader.resolveHead();
  const rendered = await renderChainsConfig(options.workspaceRoot, reader, head, pinned);
  if (readFileSync(path, 'utf8') !== rendered) {
    return violation(
      `differs from what the registry's main head (${head.slice(0, 12)}) renders — the addresses have ` +
        `changed since the recorded sync (${pinned.slice(0, 12)}), or the file was edited by hand: ` +
        'run `fhevm-npm sync-fhevm-chains --latest`',
    );
  }
  return {
    command: 'check-fhevm-chains-origin',
    checkedPackageKeys: [`./${CHAINS_CONFIG_FILE}`],
    checkedItemLabel: 'chains config file(s)',
    violations: [],
  };
}
