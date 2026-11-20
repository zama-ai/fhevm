import { readValidations } from '@openzeppelin/hardhat-upgrades/dist/utils/validations';
import {
  StorageLayout,
  getStorageLayout,
  getStorageUpgradeReport,
  getVersion,
  withValidationDefaults,
} from '@openzeppelin/upgrades-core';
import { Contract, ContractFactory } from 'ethers';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

export type StorageException = {
  /** The contract version that makes the change; see test/upgrades/storageExceptions.ts. */
  addedIn: string;
  namespace: string;
  field: string;
  /**
   * The field's type before and after, as OpenZeppelin labels them without the contract prefix of struct names
   * (`struct Permission`, not `struct ACL.Permission`). Only this direction is accepted: a rollback
   * points the proxy back at the deployed implementation with upgradeToAndCall, without these tasks.
   */
  types: [string, string];
  /** The struct the new type stores, as `label:type` per member; all in one slot, the first at offset 0. */
  valueStruct: { label: string; members: string[] };
};

// A storage change that OpenZeppelin's upgrade check refuses although the stored bits stay valid. It reads no
// `@custom:oz-retyped-from` on a member of namespaced storage, so the change is described here for exactly its
// field, its two types and the new struct's members, and every other field is still compared.
export const STORAGE_EXCEPTION: StorageException = {
  // A persisted permission keeps its slot and also holds the handle's carried depth: `allowed` is the former bool.
  addedIn: 'ACL v0.7.0',
  namespace: 'erc7201:fhevm.storage.ACL',
  field: 'persistedAllowedPairs',
  types: ['mapping(bytes32 => mapping(address => bool))', 'mapping(bytes32 => mapping(address => struct Permission))'],
  valueStruct: {
    label: 'struct Permission',
    members: ['allowed:t_bool', 'depth:t_uint48', 'storedBlock:t_uint48'],
  },
};

function labelOf(layout: StorageLayout, typeId: string): string | undefined {
  return layout.types[typeId]?.label.replace(/struct \w+\./g, 'struct ');
}

function assertValueStruct(layout: StorageLayout, exception: StorageException): void {
  const id = Object.keys(layout.types).find((typeId) => labelOf(layout, typeId) === exception.valueStruct.label);
  const struct = id === undefined ? undefined : layout.types[id];
  const members = struct?.members as { label: string; type: string; offset?: number; slot?: string }[] | undefined;
  const found = members?.map((m) => `${m.label}:${m.type}`);
  const oneSlot = members?.every((m) => m.slot === '0') && members[0]?.offset === 0;
  if (JSON.stringify(found) !== JSON.stringify(exception.valueStruct.members) || !oneSlot) {
    throw new Error(
      `${exception.valueStruct.label} must be ${exception.valueStruct.members.join(', ')} in one slot, ` +
        `the first at offset 0, for the storage exception on ${exception.field}; found ${JSON.stringify(members)}`,
    );
  }
}

/**
 * Returns true when this upgrade makes the STORAGE_EXCEPTION change, and false when it does not. When it does, checks
 * with OpenZeppelin's comparator that nothing else in the layout changed, and throws its report otherwise.
 */
export function makesStorageException(current: StorageLayout, updated: StorageLayout): boolean {
  const { namespace, field, types } = STORAGE_EXCEPTION;
  const currentEntry = current.namespaces?.[namespace]?.find((e) => e.label === field);
  const updatedEntries = updated.namespaces?.[namespace] ?? [];
  const updatedEntry = updatedEntries.find((e) => e.label === field);
  if (!currentEntry || !updatedEntry) return false;
  if (labelOf(current, currentEntry.type) !== types[0] || labelOf(updated, updatedEntry.type) !== types[1]) {
    return false;
  }
  assertValueStruct(updated, STORAGE_EXCEPTION);
  const namespaces = {
    ...updated.namespaces,
    [namespace]: updatedEntries.map((e) => (e === updatedEntry ? { ...e, type: currentEntry.type } : e)),
  };
  // The new layout's definitions win: the current ones only fill in the type of the field set back above.
  const patched = { ...updated, namespaces, types: { ...current.types, ...updated.types } };
  const report = getStorageUpgradeReport(current, patched, withValidationDefaults({}));
  if (!report.ok) {
    throw new Error(`New storage layout is incompatible:\n${report.explain()}`);
  }
  return true;
}

/** Returns the major and minor numbers of a version such as `ACL v0.7.0`. */
export function majorMinorOf(version: string): [number, number] {
  const match = version.match(/v(\d+)\.(\d+)\./);
  if (!match) throw new Error(`Cannot read a version from "${version}"`);
  return [Number(match[1]), Number(match[2])];
}

export async function layoutOf(hre: HardhatRuntimeEnvironment, factory: ContractFactory): Promise<StorageLayout> {
  return getStorageLayout(await readValidations(hre), getVersion(factory.bytecode));
}

/**
 * Returns true when upgrading the proxy from the current to the new implementation makes the STORAGE_EXCEPTION change
 * and nothing else; the caller then skips OpenZeppelin's own storage check for this upgrade only. Throws when anything
 * else in the layout is incompatible, or when the proxy already runs the version that made the change.
 */
export async function acceptsStorageException(
  hre: HardhatRuntimeEnvironment,
  proxyAddress: string,
  currentFactory: ContractFactory,
  newFactory: ContractFactory,
): Promise<boolean> {
  if (!makesStorageException(await layoutOf(hre, currentFactory), await layoutOf(hre, newFactory))) {
    return false;
  }
  const { addedIn, field } = STORAGE_EXCEPTION;
  const proxy = new Contract(proxyAddress, ['function getVersion() view returns (string)'], hre.ethers.provider);
  const deployed: string = await proxy.getVersion();
  const [major, minor] = majorMinorOf(deployed);
  const [addedMajor, addedMinor] = majorMinorOf(addedIn);
  if (major > addedMajor || (major === addedMajor && minor >= addedMinor)) {
    throw new Error(
      `The proxy already runs ${deployed}, which has the ${field} change: pass its sources as the current ` +
        'implementation',
    );
  }
  console.log(`Accepted the listed storage change on ${field}; every other field was checked.`);
  return true;
}
