import { readValidations } from '@openzeppelin/hardhat-upgrades/dist/utils/validations';
import {
  StorageLayout,
  getStorageLayout,
  getStorageUpgradeReport,
  getVersion,
  withValidationDefaults,
} from '@openzeppelin/upgrades-core';
import { ContractFactory } from 'ethers';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

type StorageException = {
  /** The contract version that makes the change; see test/upgrades/storageExceptions.ts. */
  addedIn: string;
  namespace: string;
  field: string;
  /** The field's type before and after, as OpenZeppelin labels them. Accepted in both directions. */
  types: [string, string];
};

// Storage changes that OpenZeppelin's upgrade check refuses although the stored bits stay valid. It reads no
// `@custom:oz-retyped-from` on a member of namespaced storage, so each change is listed here for exactly its field
// and its two types, and every other field is still compared.
export const STORAGE_EXCEPTIONS: StorageException[] = [
  {
    // A persisted permission keeps its slot and also holds the handle's carried depth: `allowed` is the former bool.
    addedIn: 'ACL v0.7.0',
    namespace: 'erc7201:fhevm.storage.ACL',
    field: 'persistedAllowedPairs',
    types: [
      'mapping(bytes32 => mapping(address => bool))',
      'mapping(bytes32 => mapping(address => struct ACL.Permission))',
    ],
  },
];

async function layoutOf(hre: HardhatRuntimeEnvironment, factory: ContractFactory): Promise<StorageLayout> {
  return getStorageLayout(await readValidations(hre), getVersion(factory.bytecode));
}

/**
 * Returns the new layout with every listed exception's field set back to its current type, or undefined when no
 * exception applies to this upgrade.
 */
function revertExceptions(current: StorageLayout, updated: StorageLayout): StorageLayout | undefined {
  let applied = false;
  const namespaces = { ...updated.namespaces };
  for (const exception of STORAGE_EXCEPTIONS) {
    const currentEntry = current.namespaces?.[exception.namespace]?.find((e) => e.label === exception.field);
    const updatedEntries = namespaces[exception.namespace];
    const updatedEntry = updatedEntries?.find((e) => e.label === exception.field);
    if (!currentEntry || !updatedEntry) continue;
    const change = [current.types[currentEntry.type]?.label, updated.types[updatedEntry.type]?.label];
    const [before, after] = exception.types;
    if ((change[0] === before && change[1] === after) || (change[0] === after && change[1] === before)) {
      namespaces[exception.namespace] = updatedEntries.map((e) =>
        e === updatedEntry ? { ...e, type: currentEntry.type } : e,
      );
      applied = true;
    }
  }
  return applied ? { ...updated, namespaces, types: { ...updated.types, ...current.types } } : undefined;
}

/**
 * Returns true when this upgrade makes a listed storage change, after checking with OpenZeppelin's comparator that
 * nothing else in the layout changed; the caller then skips OpenZeppelin's own storage check for this upgrade only.
 * Throws OpenZeppelin's report when anything else is incompatible.
 */
export async function acceptsStorageException(
  hre: HardhatRuntimeEnvironment,
  currentFactory: ContractFactory,
  newFactory: ContractFactory,
): Promise<boolean> {
  const current = await layoutOf(hre, currentFactory);
  const patched = revertExceptions(current, await layoutOf(hre, newFactory));
  if (patched === undefined) {
    return false;
  }
  const report = getStorageUpgradeReport(current, patched, withValidationDefaults({}));
  if (!report.ok) {
    throw new Error(`New storage layout is incompatible:\n${report.explain()}`);
  }
  console.log('Accepted a listed storage change (STORAGE_EXCEPTIONS); every other field was checked.');
  return true;
}
