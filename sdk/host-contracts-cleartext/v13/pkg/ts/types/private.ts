import type { ContractTemplate } from '../artifacts/types.js';

// Private types
export type HexString = `0x${string}`;

export type ContractArtifact = {
  abi: readonly unknown[];
  bytecode: string;
  deployedBytecode: string;
};

export type TemplateBytecodeField = 'bytecode' | 'deployedBytecode';

export type AddressReplacement = {
  readonly referenceName: string;
  readonly replacement: string;
};

/**
 * Result of comparing on-chain code at an address against an expected deployed bytecode.
 * - `not-deployed`: nothing is deployed at the address (empty code).
 * - `match`: the on-chain code equals the expected deployed bytecode.
 * - `mismatch`: code is present but differs (both values returned, `0x`-prefixed, for diffing).
 */
export type DeployedBytecodeCheck =
  | { readonly status: 'not-deployed' }
  | { readonly status: 'match' }
  | { readonly status: 'mismatch'; readonly actualDeployedBytecode: string; readonly expectedDeployedBytecode: string };

/**
 * How to initialize one proxy within an upgrade.
 * - `initFn`: `initializeFromEmptyProxy` for a first materialization (empty → real), or
 *   `reinitializeVX` for a live upgrade (real → real). `initializeFromEmptyProxy` is guarded on-chain
 *   by `onlyFromEmptyProxy` and reverts if the proxy is already materialized.
 * - `initArgs`: the arguments to `initFn` (empty for `reinitializeVX` and for ACL/FHEVMExecutor
 *   bootstrap; use the `*InitArgs` helpers to build them type-safely for the others).
 */
export type ContractUpgradeSpec = {
  readonly initFn: string;
  readonly initArgs: readonly unknown[];
};

/**
 * Phase 1 output for a single proxy: the freshly deployed implementation plus the calldata to point
 * the proxy at it. No transaction is sent — consumed by `deploy` / `updateV12ToV13` (via `ACLOwner`).
 */
export type DeployedImplementation = {
  readonly contractName: string;
  readonly proxyAddress: string;
  readonly implementationAddress: string;
  readonly initData: HexString; // initFn(initArgs), the inner upgradeToAndCall payload
  readonly upgradeCalldata: HexString; // upgradeToAndCall(implementationAddress, initData)
};

/** A single proxy to (re)point at a freshly deployed implementation, with its init spec. */
export type UpgradeTarget = {
  readonly contractName: string;
  readonly proxyAddress: string;
  readonly template: ContractTemplate;
  readonly abi: readonly unknown[];
  readonly spec: ContractUpgradeSpec;
};
