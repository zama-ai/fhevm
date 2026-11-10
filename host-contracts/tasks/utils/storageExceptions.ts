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
   * The field's type before and after, as OpenZeppelin labels them. Only this direction is accepted: a rollback
   * points the proxy back at the deployed implementation with upgradeToAndCall, without these tasks.
   */
  types: [string, string];
  /** The struct the new type stores, as `label:type` per member; all in one slot, the first at offset 0. */
  valueStruct: { label: string; members: string[] };
};

// Storage changes that OpenZeppelin's upgrade check refuses although the stored bits stay valid. It reads no
// `@custom:oz-retyped-from` on a member of namespaced storage, so each change is listed here for exactly its field,
// its two types and the new struct's members, and every other field is still compared.
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
    valueStruct: {
      label: 'struct ACL.Permission',
      members: ['allowed:t_bool', 'depth:t_uint48', 'storedBlock:t_uint48'],
    },
  },
];

function assertValueStruct(layout: StorageLayout, exception: StorageException): void {
  const struct = Object.values(layout.types).find((t) => t.label === exception.valueStruct.label);
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
 * Returns the listed exception this upgrade makes, or undefined when it makes none. When it makes one, checks with
 * OpenZeppelin's comparator that nothing else in the layout changed, and throws its report otherwise.
 */
export function storageExceptionFor(current: StorageLayout, updated: StorageLayout): StorageException | undefined {
  let applied: StorageException | undefined;
  const namespaces = { ...updated.namespaces };
  for (const exception of STORAGE_EXCEPTIONS) {
    const currentEntry = current.namespaces?.[exception.namespace]?.find((e) => e.label === exception.field);
    const updatedEntries = namespaces[exception.namespace];
    const updatedEntry = updatedEntries?.find((e) => e.label === exception.field);
    if (!currentEntry || !updatedEntry) continue;
    const [before, after] = exception.types;
    if (current.types[currentEntry.type]?.label === before && updated.types[updatedEntry.type]?.label === after) {
      assertValueStruct(updated, exception);
      namespaces[exception.namespace] = updatedEntries.map((e) =>
        e === updatedEntry ? { ...e, type: currentEntry.type } : e,
      );
      applied = exception;
    }
  }
  if (applied === undefined) return undefined;
  // The new layout's definitions win: the current ones only fill in the types of the fields set back above.
  const patched = { ...updated, namespaces, types: { ...current.types, ...updated.types } };
  const report = getStorageUpgradeReport(current, patched, withValidationDefaults({}));
  if (!report.ok) {
    throw new Error(`New storage layout is incompatible:\n${report.explain()}`);
  }
  return applied;
}

function minorOf(version: string): [number, number] {
  const match = version.match(/v(\d+)\.(\d+)\./);
  if (!match) throw new Error(`Cannot read a version from "${version}"`);
  return [Number(match[1]), Number(match[2])];
}

async function layoutOf(hre: HardhatRuntimeEnvironment, factory: ContractFactory): Promise<StorageLayout> {
  return getStorageLayout(await readValidations(hre), getVersion(factory.bytecode));
}

/**
 * Returns true when upgrading the proxy from the current to the new implementation makes a listed storage change and
 * nothing else; the caller then skips OpenZeppelin's own storage check for this upgrade only. Throws when anything
 * else in the layout is incompatible, or when the proxy already runs the version that made the change.
 */
export async function acceptsStorageException(
  hre: HardhatRuntimeEnvironment,
  proxyAddress: string,
  currentFactory: ContractFactory,
  newFactory: ContractFactory,
): Promise<boolean> {
  const exception = storageExceptionFor(await layoutOf(hre, currentFactory), await layoutOf(hre, newFactory));
  if (exception === undefined) {
    return false;
  }
  const proxy = new Contract(proxyAddress, ['function getVersion() view returns (string)'], hre.ethers.provider);
  const deployed: string = await proxy.getVersion();
  const [major, minor] = minorOf(deployed);
  const [addedMajor, addedMinor] = minorOf(exception.addedIn);
  if (major > addedMajor || (major === addedMajor && minor >= addedMinor)) {
    throw new Error(
      `The proxy already runs ${deployed}, which has the ${exception.field} change: pass its sources as the current ` +
        'implementation',
    );
  }
  console.log(`Accepted the listed storage change on ${exception.field}; every other field was checked.`);
  return true;
}
