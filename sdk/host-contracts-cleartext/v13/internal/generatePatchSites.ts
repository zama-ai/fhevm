// Computes, and can refresh, internal/placeholders/patch-sites.json — the committed baseline of how many
// bytecode sites each placeholder is patched at, per contract.

import { writeFileSync } from 'node:fs';
import {
  PATCH_SITES_PATH,
  TARGET_CONTRACTS,
  patchSiteCounts,
  templatePathFor,
  type AddressReference,
} from './generateTemplates.ts';
import { readJson } from './utils.ts';

////////////////////////////////////////////////////////////////////////////////

type ParsedTemplate = { readonly addressReferences: Record<string, AddressReference> };

////////////////////////////////////////////////////////////////////////////////

function _collectPatchSites(): Record<string, Record<string, number>> {
  const sites: Record<string, Record<string, number>> = {};
  for (const target of TARGET_CONTRACTS) {
    const template = readJson<ParsedTemplate>(templatePathFor(target.contractName));
    sites[target.contractName] = patchSiteCounts(template.addressReferences);
  }
  return sites;
}

////////////////////////////////////////////////////////////////////////////////

/** Deterministic key order, so a refresh only ever diffs on numbers that actually changed. */
function _sortedDeep(sites: Record<string, Record<string, number>>): Record<string, Record<string, number>> {
  const sorted: Record<string, Record<string, number>> = {};
  for (const contractName of Object.keys(sites).sort()) {
    const counts = sites[contractName];
    if (counts === undefined) {
      continue;
    }
    const innerSorted: Record<string, number> = {};
    for (const name of Object.keys(counts).sort()) {
      innerSorted[name] = counts[name] ?? 0;
    }
    sorted[contractName] = innerSorted;
  }
  return sorted;
}

////////////////////////////////////////////////////////////////////////////////

/** Overwrites the committed baseline with the live counts, and hands them back for reporting. */
export function writePatchSites(): Record<string, Record<string, number>> {
  const sites = _collectPatchSites();
  writeFileSync(PATCH_SITES_PATH, `${JSON.stringify(_sortedDeep(sites), null, 2)}\n`, 'utf8');
  return sites;
}
