// Migration helper: computes which contracts need an upgrade op when moving from the previous generation
// to this one. Read-only — it writes nothing and is not part of any build.
//
// Used from the migration guide (README step 7). It answers, per contract, the two-sided question that
// step describes: did the bytecode change, and did the reinitializer version bump?
//
// Three signals, all read from committed JSON so no compilation is needed:
//
//   1. bytecode — each generation's template, with every placeholder patched to one COMMON address set
//      so the two are comparable (the generations use different marker values), then compared by hash.
//   2. is it a proxy target — does abi/<C>.json expose `initializeFromEmptyProxy`? That, not the
//      reinitializer, is what marks a contract as living behind a proxy: `CleartextDB`, `KMSGeneration`
//      and `ProtocolConfig` are proxy targets with no reinitializer at all. A contract without it
//      (ACLOwner, PauserSet, the proxies themselves) is reported as `not a proxy target`.
//   3. reinitializer — the `reinitializeV<n>` name. Solidity ABIs are flattened, so an inherited
//      reinitializer shows up too: CleartextKMSVerifier reports KMSVerifier's `reinitializeV3` without
//      this having to follow `is` clauses.
//
// The verdicts mirror README step 7: a bytecode change without a version bump means an
// upgrade of that proxy would carry no replay guard; a bump without a bytecode change is a wasted version.

import { createHash } from 'node:crypto';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { join, resolve } from 'node:path';

////////////////////////////////////////////////////////////////////////////////

type AddressReference = {
  readonly placeholder: string;
  readonly bytecodeOffsets: readonly number[];
  readonly deployedBytecodeOffsets: readonly number[];
};

type Template = {
  readonly contractName: string;
  readonly deployedBytecode: string;
  readonly addressReferences: Record<string, AddressReference>;
};

type AbiEntry = { readonly type?: string; readonly name?: string };

export type Verdict =
  | 'materialize'
  | 'reinitialize'
  | 'no op'
  | 'not a proxy target'
  | 'removed upstream'
  | 'CHANGED, NOT BUMPED'
  | 'BUMPED, UNCHANGED';

export type UpgradeOp = {
  readonly contractName: string;
  readonly bytecodeChanged: boolean | undefined;
  readonly previousReinitializer: string | undefined;
  readonly currentReinitializer: string | undefined;
  /** For a `materialize` verdict: the `initialize*` entry points available to call. */
  readonly materializers: readonly string[];
  readonly verdict: Verdict;
};

type Initializers = {
  /** Present iff the contract is meant to sit behind a proxy. */
  readonly isProxyTarget: boolean;
  readonly reinitializer: string | undefined;
  /** Every `initialize*` entry point the contract offers. Which one a materialization should call is a
   * decision, not a lookup — `updateV12ToV13` calls `initializeFromMigration` on ProtocolConfig (it seeds
   * The migrated KMS context) but `initializeFromEmptyProxy` on KMSGeneration. So list them, don't pick. */
  readonly materializers: readonly string[];
};

const MATERIALIZERS = ['initializeFromEmptyProxy', 'initializeFromMigration'] as const;

////////////////////////////////////////////////////////////////////////////////

/** A generation's committed artifacts, tolerating both the pre-split and pkg/ layouts. */
function _locate(packageRoot: string): { readonly templates: string; readonly abi: string } {
  for (const prefix of ['pkg', '.']) {
    const templates = resolve(packageRoot, prefix, 'templates');
    const abi = resolve(packageRoot, prefix, 'abi');
    if (existsSync(templates) && existsSync(abi)) {
      return { templates, abi };
    }
  }
  throw new Error(`${packageRoot} has no templates/ + abi/ (looked in ./ and ./pkg)`);
}

////////////////////////////////////////////////////////////////////////////////

function _readTemplates(dir: string): Map<string, Template> {
  const templates = new Map<string, Template>();
  for (const entry of readdirSync(dir)) {
    if (!entry.endsWith('.json')) {
      continue;
    }
    const template = JSON.parse(readFileSync(join(dir, entry), 'utf8')) as Template;
    templates.set(template.contractName, template);
  }
  return templates;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Rewrite every placeholder to a value derived from its NAME, so the same logical address gets the same
 * bytes in both generations. Without this the comparison is meaningless: the generations use different
 * marker values, so every contract would look changed.
 */
function _normalizedDeployedBytecode(template: Template): string {
  let hex = template.deployedBytecode.replace(/^0x/, '').toLowerCase();
  for (const [name, reference] of Object.entries(template.addressReferences)) {
    const placeholder = reference.placeholder.replace(/^0x/, '').toLowerCase();
    // Must be injective over names. Deriving it from `name.length` was not: FHEVM_EXECUTOR_ADDRESS,
    // INPUT_VERIFIER_ADDRESS and KMS_GENERATION_ADDRESS are all 22 characters, and
    // KMS_VERIFIER_ADDRESS/CLEARTEXT_DB_ADDRESS are both 20 — five of the ten names collided. A
    // contract that swaps one address reference for another at the same offset then normalizes to
    // identical bytes and is reported "no op" when its semantics changed.
    const canonical = createHash('sha256').update(name).digest('hex').slice(0, 40);
    for (const offset of reference.deployedBytecodeOffsets) {
      const at = offset * 2;
      if (hex.slice(at, at + 40) !== placeholder) {
        throw new Error(`${template.contractName}: offset ${String(offset)} does not hold ${name}`);
      }
      hex = `${hex.slice(0, at)}${canonical}${hex.slice(at + 40)}`;
    }
  }
  return hex;
}

////////////////////////////////////////////////////////////////////////////////

function _readInitializers(abiDir: string, contractName: string): Initializers {
  const path = join(abiDir, `${contractName}.json`);
  if (!existsSync(path)) {
    return { isProxyTarget: false, reinitializer: undefined, materializers: [] };
  }
  const abi = JSON.parse(readFileSync(path, 'utf8')) as readonly AbiEntry[];
  const names = new Set(abi.filter((entry) => entry.type === 'function').map((entry) => entry.name));
  return {
    isProxyTarget: names.has('initializeFromEmptyProxy'),
    reinitializer: [...names].find((name) => name?.startsWith('reinitializeV') === true),
    materializers: MATERIALIZERS.filter((name) => names.has(name)),
  };
}

////////////////////////////////////////////////////////////////////////////////

export function listUpgradeOps(previousPackageRoot: string, currentPackageRoot: string): UpgradeOp[] {
  const previous = _locate(previousPackageRoot);
  const current = _locate(currentPackageRoot);
  const previousTemplates = _readTemplates(previous.templates);
  const currentTemplates = _readTemplates(current.templates);

  const ops: UpgradeOp[] = [];
  for (const contractName of [...new Set([...previousTemplates.keys(), ...currentTemplates.keys()])].sort()) {
    const before = previousTemplates.get(contractName);
    const after = currentTemplates.get(contractName);
    const wasProxyTarget = _readInitializers(previous.abi, contractName);
    const isProxyTarget = _readInitializers(current.abi, contractName);
    const previousReinitializer = wasProxyTarget.reinitializer;
    const currentReinitializer = isProxyTarget.reinitializer;
    const common = { contractName, previousReinitializer, currentReinitializer };

    if (after === undefined) {
      ops.push({ ...common, bytecodeChanged: undefined, materializers: [], verdict: 'removed upstream' });
      continue;
    }
    if (!isProxyTarget.isProxyTarget) {
      // ACLOwner, PauserSet, the proxies themselves — nothing to upgrade, they are deployed not upgraded
      ops.push({ ...common, bytecodeChanged: undefined, materializers: [], verdict: 'not a proxy target' });
      continue;
    }
    if (before === undefined || !wasProxyTarget.isProxyTarget) {
      // new in this generation: the proxy has to be created and its implementation initialized
      ops.push({
        ...common,
        bytecodeChanged: undefined,
        materializers: isProxyTarget.materializers,
        verdict: 'materialize',
      });
      continue;
    }

    const bytecodeChanged = _normalizedDeployedBytecode(before) !== _normalizedDeployedBytecode(after);
    const bumped = previousReinitializer !== currentReinitializer;

    let verdict: Verdict;
    if (bytecodeChanged && bumped) {
      verdict = 'reinitialize';
    } else if (!bytecodeChanged && !bumped) {
      verdict = 'no op';
    } else if (bytecodeChanged) {
      verdict = 'CHANGED, NOT BUMPED';
    } else {
      verdict = 'BUMPED, UNCHANGED';
    }

    ops.push({ ...common, bytecodeChanged, materializers: [], verdict });
  }
  return ops;
}
