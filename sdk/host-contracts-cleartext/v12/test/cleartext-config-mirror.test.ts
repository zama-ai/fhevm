// RULES.md rule 23: `sdk/cleartext-config.json` is THE source of truth for the cleartext stack's shared
// values; `internal/cleartext-config.ts` and `create2-deploy/script/FhevmCleartextConfig.sol` are faces of
// it — same names, same order, equal values.
//
// The rule's whole safety argument is that the faces are CHECKED rather than trusted, so this file is that
// argument. Without it the rule is a request.
//
// Four things are checked, and the last two could not exist before the JSON did:
//
//   1. the TypeScript face agrees with the JSON, name for name and value for value;
//   2. the Solidity face agrees with the JSON likewise;
//   3. every value that carries a `formula` is RECOMPUTED by evaluating that formula;
//   4. every entry carries a `note`.
//
// (3) matters more than it looks. Three of these values are keccak-derived, and a hex string is
// unverifiable by reading: a mistyped digit looks exactly like a correct one, survives review, and then
// every copy of it agrees with every other. Evaluating is the only way a wrong one is ever noticed.
//
// Both faces are read as TEXT rather than imported. Importing the TypeScript would give real values but
// leave the Solidity to parse anyway, and — more to the point — an import can only see the names it was
// written to look for, so a NAME that drifted would be invisible.

import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import test from 'node:test';
import { HDNodeWallet, getAddress, getCreateAddress, keccak256, toUtf8Bytes } from 'ethers';
import { FHEVM_CONFIG_REMAPPING_PREFIX, PACKAGE_ROOT_ABS_PATH, ZAMA_LOCAL_CONFIG } from '../internal/constants.ts';

/**
 * The JSON sits ABOVE every generation, at the `sdk/` root — it is shared by v12 and v13, so it cannot
 * live inside either. A generation extracted on its own therefore cannot run this test, and that is the
 * correct outcome rather than a gap to paper over: without the source of truth there is nothing to check
 * the faces against, and a test that quietly passed in that case would be worse than one that fails.
 */
const JSON_PATH = join(PACKAGE_ROOT_ABS_PATH, '..', '..', 'cleartext-config.json');
const TS_PATH = join(PACKAGE_ROOT_ABS_PATH, 'internal', 'cleartext-config.ts');
const SOL_PATH = join(PACKAGE_ROOT_ABS_PATH, 'create2-deploy', 'script', 'FhevmCleartextConfig.sol');
const LOCAL_HOST_ADDRESSES_PATH = join(
  PACKAGE_ROOT_ABS_PATH,
  'pkg',
  'forge',
  'src',
  '_internal',
  'LocalHostAddresses.sol',
);

/**
 * Which generation's `localhost` table this package is.
 *
 * Taken from the config remapping prefix — `fhevm-config-0.13.0/` — rather than from a constant added for
 * this test. That prefix already moves with the protocol minor and is already load-bearing, so it cannot
 * be stale while the package builds; a second declaration of "which generation am I" could be.
 *
 * Reading the SIBLING generation's table would be the stronger check, but this test deliberately does not:
 * a generation must be checkable from its own checkout, and `../v12` may not be there.
 */
function generationKey(): string {
  const m = /^fhevm-config-(\d+\.\d+\.\d+)\/$/.exec(FHEVM_CONFIG_REMAPPING_PREFIX);
  assert.ok(m, `cannot read a generation out of ${FHEVM_CONFIG_REMAPPING_PREFIX}`);
  return m[1] ?? '';
}

type Entry = {
  value?: string;
  alias?: string;
  ts: 'bigint' | 'number' | 'string';
  solidity: string;
  formula?: string;
  note?: string;
};

function readSourceOfTruth(): Map<string, Entry> {
  const parsed = JSON.parse(readFileSync(JSON_PATH, 'utf8')) as { constants: Record<string, Entry> };
  // Object key order is declaration order and the faces rely on it, so a Map (which preserves insertion
  // order) rather than a plain lookup.
  return new Map(Object.entries(parsed.constants));
}

/**
 * How to compare a value, taken from its Solidity type rather than from a separate tag.
 *
 * There is no `kind` field to get out of step with the declared types, and none is needed — the Solidity
 * type already says everything: `address` compares case-insensitively, `uint*` numerically, `string`
 * exactly. Nothing in the file is fractional, every numeric value being a non-negative integer, so
 * "numeric" needs no further distinction.
 */
function comparisonOf(solidityType: string): 'address' | 'uint' | 'string' {
  if (solidityType === 'address') return 'address';
  if (/^u?int\d*$/.test(solidityType)) return 'uint';
  return 'string';
}

/**
 * A value reduced to the one form both languages can be compared in.
 *
 * Each normalisation step is a difference in how the two languages SPELL a value, never a difference in
 * the value: `100733346448153n` against `100733346448153`, `'0x6189…'` against a bare `0x6189…`, single
 * against double quotes. Trailing characters are NOT normalised — a mnemonic path ending `/` on one side
 * and not the other is exactly the failure rule 23 describes.
 */
function canonical(solidityType: string, literal: string): string {
  const v = literal.trim();
  switch (comparisonOf(solidityType)) {
    case 'uint': {
      const digits = /^(\d[\d_]*)n?$/.exec(v);
      assert.ok(digits, `not an integer literal: ${v}`);
      return BigInt((digits[1] ?? '').replaceAll('_', '')).toString();
    }
    case 'address':
      // Case-insensitive: Solidity REQUIRES the EIP-55 checksum on a hex literal containing letters
      // (error 9429) and TypeScript does not care. Digits and length still have to match.
      return unquote(v).toLowerCase();
    case 'string':
      return unquote(v);
  }
}

function unquote(v: string): string {
  const m = /^(['"])([\s\S]*)\1$/.exec(v);
  return m ? (m[2] ?? '') : v;
}

/**
 * A `formula` evaluated — the Solidity expression itself, not a tag describing it.
 *
 * Parsing the expression rather than switching on some `formulaKind` is the point: a tag would be a second
 * statement about the same value, so a formula could be edited while its tag went on claiming the old
 * shape and this check would keep passing. Here there is nothing to disagree with. The grammar is tiny by
 * design — `keccak256("literal")` innermost, integer and `address` casts peeled outward, plus
 * `type(uintN).max` — and a formula outside it is a hard failure rather than a skip, so a new kind of
 * derivation cannot slip in unchecked.
 *
 * Casts MASK rather than range-check, which is what Solidity's explicit narrowing conversions do:
 * `uint48(uint256(keccak256(...)))` keeps the low 48 bits.
 */
function evaluate(expr: string): bigint {
  const e = expr.trim();

  const max = /^type\(uint(\d+)\)\.max$/.exec(e);
  if (max) return (1n << BigInt(max[1] ?? '0')) - 1n;

  const call = /^([A-Za-z][A-Za-z0-9_]*)\(([\s\S]*)\)$/.exec(e);
  assert.ok(call, `unparseable formula: ${e}`);
  const fn = call[1] ?? '';
  const arg = (call[2] ?? '').trim();

  if (fn === 'keccak256') {
    const lit = /^"([\s\S]*)"$/.exec(arg);
    assert.ok(lit, `keccak256 of something other than a string literal: ${arg}`);
    return BigInt(keccak256(toUtf8Bytes(lit[1] ?? '')));
  }

  const inner = evaluate(arg);
  if (fn === 'address') return inner & ((1n << 160n) - 1n);
  const uint = /^u?int(\d+)$/.exec(fn);
  assert.ok(uint, `unsupported cast in a formula: ${fn}`);
  return inner & ((1n << BigInt(uint[1] ?? '0')) - 1n);
}

/** What a formula produces, in the same canonical form the recorded value normalises to. */
function evaluateAs(solidityType: string, formula: string): string {
  const n = evaluate(formula);
  if (comparisonOf(solidityType) === 'address') {
    return getAddress(`0x${n.toString(16).padStart(40, '0')}`).toLowerCase();
  }
  return n.toString();
}

/** `export const NAME = VALUE;` from the TypeScript face, in declaration order. */
function readTsFace(): Map<string, string> {
  const src = readFileSync(TS_PATH, 'utf8');
  const out = new Map<string, string>();
  for (const [, name, value] of src.matchAll(/^export const ([A-Za-z_][A-Za-z0-9_]*) = ([^\n]+?);?$/gm)) {
    if (name === undefined || value === undefined) continue;
    out.set(name, value.trim());
  }
  return out;
}

/**
 * `<type> internal constant NAME = VALUE;` from the Solidity face, in declaration order.
 *
 * A value may wrap onto following lines — prettier does that to the mnemonic — so this reads to the
 * terminating semicolon rather than to end of line. A regex suffices where a parser would not be worth it:
 * the file is a flat library of constants by construction, and rule 23 keeps it that way.
 */
function readSolFace(): Map<string, { type: string; value: string }> {
  const src = readFileSync(SOL_PATH, 'utf8');
  const out = new Map<string, { type: string; value: string }>();
  for (const m of src.matchAll(
    /\b(string|address|bytes32|uint\d*|int\d*|bool)\s+internal\s+constant\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*([\s\S]*?);/g,
  )) {
    const [, type, name, value] = m;
    if (type === undefined || name === undefined || value === undefined) continue;
    out.set(name, { type, value: value.replaceAll(/\s+/g, ' ').trim() });
  }
  return out;
}

void test('rule 23: the source of truth parses and is not vacuous', () => {
  const truth = readSourceOfTruth();
  // A parser that silently matched nothing would make every assertion below pass trivially. The floor is
  // deliberately loose but non-zero.
  assert.ok(truth.size >= 15, `parsed only ${truth.size} constants from ${JSON_PATH}`);
  for (const [name, e] of truth) {
    assert.ok(
      (e.value === undefined) !== (e.alias === undefined),
      `${name}: exactly one of "value" or "alias" must be present`,
    );
    if (e.alias !== undefined) {
      assert.ok(truth.has(e.alias), `${name}: aliases ${e.alias}, which is not declared`);
    }
  }
});

void test('rule 23: every entry says why', () => {
  // A value nobody explained is a value nobody can safely change: the next person cannot tell a deliberate
  // choice from an arbitrary one, so they either leave a wrong value alone or edit a load-bearing one.
  // Required, therefore, rather than encouraged.
  const missing = [...readSourceOfTruth()].filter(([, e]) => (e.note ?? '').trim().length === 0).map(([name]) => name);
  assert.deepEqual(missing, [], 'these entries have no "note" explaining the value');
});

void test('rule 23: the declared TypeScript type is the minimal exact one', () => {
  // `ts` is a FLOOR that a face may widen, so it has to be the narrowest type that still holds the value
  // exactly. A uint that fits a double may be declared `number` and emitted as either; one that does not
  // must be declared `bigint`, because narrowing it to a `number` would silently drop digits.
  for (const [name, e] of readSourceOfTruth()) {
    if (e.value === undefined || comparisonOf(e.solidity) !== 'uint') continue;
    const fits = BigInt(e.value) <= BigInt(Number.MAX_SAFE_INTEGER);
    assert.equal(
      e.ts,
      fits ? 'number' : 'bigint',
      `${name}: value ${e.value} ${fits ? 'fits' : 'does not fit'} a JS number exactly, so "ts" should be ` +
        (fits ? '"number"' : '"bigint"'),
    );
  }
});

void test('rule 23: every recorded formula reproduces its value', () => {
  const truth = readSourceOfTruth();
  const withFormula = [...truth].filter(([, e]) => e.formula !== undefined);
  // The formula-derived values are the only ones checkable at all; if this filter ever matched nothing the
  // test would be a no-op that reads like coverage.
  assert.ok(withFormula.length >= 3, `expected at least 3 formula-derived values, found ${withFormula.length}`);
  for (const [name, e] of withFormula) {
    assert.equal(
      evaluateAs(e.solidity, e.formula ?? ''),
      canonical(e.solidity, e.value ?? ''),
      `${name}: the recorded value is not what its formula produces.\n  formula: ${e.formula ?? ''}`,
    );
  }
});

void test('rule 23: addresses are EIP-55 checksummed', () => {
  // Not cosmetic: Solidity REJECTS a hex address literal containing letters unless it is checksummed
  // (error 9429), so an un-checksummed value in the JSON cannot be emitted into the Solidity face at all.
  for (const [name, e] of readSourceOfTruth()) {
    if (e.solidity !== 'address' || e.value === undefined) continue;
    assert.equal(e.value, getAddress(e.value), `${name}: not EIP-55 checksummed`);
  }
});

void test('rule 23: the TypeScript face matches the source of truth', () => {
  const truth = readSourceOfTruth();
  const face = readTsFace();

  assert.deepEqual(
    [...face.keys()],
    [...truth.keys()],
    `internal/cleartext-config.ts declares a different set of constants, or in a different order, than\n` +
      `sdk/cleartext-config.json. Names are verbatim and order is declaration order — an alias must follow\n` +
      `the constant it aliases.`,
  );

  const diffs: string[] = [];
  for (const [name, e] of truth) {
    const got = face.get(name);
    if (got === undefined) continue; // reported above, with a better message
    if (e.alias !== undefined) {
      if (got !== e.alias) diffs.push(`${name}\n    want alias of ${e.alias}\n    got   ${got}`);
      continue;
    }
    // A `bigint` floor may be widened to, never narrowed from: a value declared `bigint` must carry the
    // `n` suffix here, while one declared `number` may be spelled either way.
    if (e.ts === 'bigint' && !got.endsWith('n')) {
      diffs.push(`${name}: declared bigint (the value needs one), literal is ${got}`);
    }
    const want = canonical(e.solidity, e.value ?? '');
    const have = canonical(e.solidity, got);
    if (have !== want) diffs.push(`${name}\n    json: ${want}\n    ts:   ${have}`);
  }
  assert.deepEqual(diffs, [], 'value drift between sdk/cleartext-config.json and internal/cleartext-config.ts');
});

void test('rule 23: the Solidity face matches the source of truth', () => {
  const truth = readSourceOfTruth();
  const face = readSolFace();

  assert.deepEqual(
    [...face.keys()],
    [...truth.keys()],
    `create2-deploy/script/FhevmCleartextConfig.sol declares a different set of constants, or in a\n` +
      `different order, than sdk/cleartext-config.json.\n` +
      `The mirror is COMPLETE, not trimmed to what today's scripts use — a partial mirror is an invitation\n` +
      `to declare the missing half somewhere else. And a Solidity-only value (a role name, an artifact\n` +
      `path) does not belong in this file at all.`,
  );

  const diffs: string[] = [];
  for (const [name, e] of truth) {
    const got = face.get(name);
    if (got === undefined) continue; // reported above
    if (got.type !== e.solidity) diffs.push(`${name}: declared ${e.solidity} in JSON, ${got.type} in Solidity`);
    if (e.alias !== undefined) {
      if (got.value !== e.alias) diffs.push(`${name}\n    want alias of ${e.alias}\n    got   ${got.value}`);
      continue;
    }
    const want = canonical(e.solidity, e.value ?? '');
    const have = canonical(e.solidity, got.value);
    if (have !== want) diffs.push(`${name}\n    json: ${want}\n    sol:  ${have}`);
  }
  assert.deepEqual(
    diffs,
    [],
    `value drift between sdk/cleartext-config.json and FhevmCleartextConfig.sol.\n` +
      `A trailing "/" on a mnemonic path is compared exactly and is load-bearing: vm.deriveKey derives at\n` +
      `{path}{index}, so dropping it derives a real key at the wrong path.`,
  );
});

////////////////////////////////////////////////////////////////////////////////
// The localhost address set — rules 15 and 17
//
// A different kind of check from the ones above. Those compare a value against two copies of it; these
// RE-DERIVE the value from the recipe and compare the copies against the result. That is possible here and
// not there, and it is the stronger form: nothing in this section is trusted, not even the JSON.
//
// Why it matters more than a normal transcription: `ZamaConfig.sol` is a library dApps INHERIT, so three
// of these addresses are compiled into consumer bytecode and cannot be reconfigured afterwards. A local
// deploy that lands anywhere else leaves every such dApp calling addresses that hold no code — and the
// deploy itself looks fine, because it verifies against whatever it produced.
////////////////////////////////////////////////////////////////////////////////

type NonceEntry = { nonce: number; address: string; role: string };

/**
 * A generation's deploy layout, in the two categories the code already distinguishes.
 *
 *   primary    the protocol stack — the two empty-proxy implementations plus everything in the
 *              `FhevmAddresses` type, i.e. exactly what `HOST_NONCE_OFFSET` covers in pkg/ts/addresses.ts
 *   secondary  what is positioned AFTER it, against `HOST_NONCE_COUNT`: the two cleartext-only contracts
 *              and `PauserSet`
 *
 * Splitting them is what makes the v12/v13 divergence legible rather than a coincidence of numbering: the
 * primary block is what grows between generations, and the secondary block is the same three roles in the
 * same order every time, merely starting later. The checks below assert both halves of that.
 */
type NonceTable = {
  PROXY_COUNT: string;
  ADDRESSED_NONCE_COUNT: string;
  nonces: { primary: NonceEntry[]; secondary: NonceEntry[] };
};

type Localhost = {
  chainId: { value: string };
  MNEMONIC: { value: string };
  DEPLOYER_ADDRESS_INDEX: { value: string };
  DEPLOYER_ADDRESS: { value: string };
  DEPLOYER_START_NONCE: { value: string };
  zamaConfigLocal: Record<string, string>;
  generations: Record<string, NonceTable>;
};

function readLocalhost(): Localhost {
  return (JSON.parse(readFileSync(JSON_PATH, 'utf8')) as { localhost: Localhost }).localhost;
}

/** Both categories in deploy order — primary first, which is the only order the nonces allow. */
function allNonces(t: NonceTable): NonceEntry[] {
  return [...t.nonces.primary, ...t.nonces.secondary];
}

/** This generation's table, or a failure naming what the JSON does hold. */
function thisGeneration(l: Localhost): NonceTable {
  const key = generationKey();
  const table = l.generations[key];
  assert.ok(
    table,
    `sdk/cleartext-config.json has no localhost.generations["${key}"]. It holds: ` +
      `${Object.keys(l.generations).join(', ')}. A new generation must add its own nonce table — the ` +
      `addresses are positional, so borrowing another generation's would name the wrong contracts.`,
  );
  return table;
}

/** `<type> constant NAME = VALUE;` from the generated LocalHostAddresses.sol. */
function readGeneratedLocalHost(): Map<string, string> {
  const src = readFileSync(LOCAL_HOST_ADDRESSES_PATH, 'utf8');
  const out = new Map<string, string>();
  for (const m of src.matchAll(
    /\b(?:address|uint\d*|int\d*|bool)\s+constant\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*([^;]*);/g,
  )) {
    const [, name, value] = m;
    if (name === undefined || value === undefined) continue;
    out.set(name, value.trim());
  }
  return out;
}

void test('rules 15/17: the deployer is the account the mnemonic and index name', () => {
  const l = readLocalhost();
  const path = `m/44'/60'/0'/0/${l.DEPLOYER_ADDRESS_INDEX.value}`;
  const derived = HDNodeWallet.fromPhrase(l.MNEMONIC.value, undefined, path).address;
  assert.equal(
    derived,
    getAddress(l.DEPLOYER_ADDRESS.value),
    `the recorded DEPLOYER_ADDRESS is not what MNEMONIC produces at ${path}`,
  );
});

void test('rules 15/17: every recorded address is CREATE(deployer, its nonce)', () => {
  const l = readLocalhost();
  const from = getAddress(l.DEPLOYER_ADDRESS.value);
  const start = BigInt(l.DEPLOYER_START_NONCE.value);

  for (const [gen, table] of Object.entries(l.generations)) {
    // Every generation's table is re-derived, not just this package's. The derivation needs nothing from
    // the generation itself — only the deployer and a nonce — so there is no reason to check less.
    const entries = allNonces(table);
    assert.equal(
      entries.length,
      Number(table.ADDRESSED_NONCE_COUNT),
      `generation ${gen}: ADDRESSED_NONCE_COUNT says ${table.ADDRESSED_NONCE_COUNT} but the two categories ` +
        `hold ${entries.length} entries between them. The count is "one past the last pinned nonce", so it ` +
        `must equal the total exactly — a short table would leave a pinned address unchecked.`,
    );
    entries.forEach((e, i) => {
      assert.equal(e.nonce, i, `generation ${gen}: nonce table is out of order at index ${i}`);
      assert.equal(
        getAddress(e.address),
        getCreateAddress({ from, nonce: start + BigInt(e.nonce) }),
        `generation ${gen}, nonce ${e.nonce} (${e.role}): not CREATE(deployer, nonce)`,
      );
    });
  }
});

/**
 * `ZamaConfig`'s `CoprocessorConfig` field names, mapped to the role names the nonce table uses.
 *
 * `CoprocessorAddress` **is** the FHEVMExecutor address — the two names describe one contract, and reading
 * it as some other component is the easiest way to get rule 17 wrong. `internal/checkZamaLocalConfig.ts`
 * carries the same mapping against its own key names, and says the same thing.
 */
const ZAMA_FIELD_TO_ROLE: Readonly<Record<string, string>> = {
  ACLAddress: 'ACL_ADDRESS',
  CoprocessorAddress: 'FHEVM_EXECUTOR_ADDRESS',
  KMSVerifierAddress: 'KMS_VERIFIER_ADDRESS',
};

/** The same three fields, mapped to `ZAMA_LOCAL_CONFIG`'s keys. */
const ZAMA_HARNESS_KEY: Readonly<Record<string, string>> = {
  ACLAddress: 'aclAddress',
  CoprocessorAddress: 'fhevmExecutorAddress',
  KMSVerifierAddress: 'kmsVerifierAddress',
};

void test('rules 15/17: the ZamaConfig subset agrees with the nonce table and with the harness', () => {
  const l = readLocalhost();
  const byRole = new Map(allNonces(thisGeneration(l)).map((e) => [e.role, e.address]));

  // Every field, not just the ones that happen to be present: a missing key here would otherwise read as
  // "nothing to compare" and pass.
  assert.deepEqual(
    Object.keys(l.zamaConfigLocal)
      .filter((k) => !k.startsWith('_'))
      .sort(),
    Object.keys(ZAMA_FIELD_TO_ROLE).sort(),
    "localhost.zamaConfigLocal does not hold exactly ZamaConfig's three CoprocessorConfig fields",
  );

  for (const [field, role] of Object.entries(ZAMA_FIELD_TO_ROLE)) {
    const recorded = l.zamaConfigLocal[field];
    assert.ok(recorded !== undefined, `localhost.zamaConfigLocal has no ${field}`);

    const fromTable = byRole.get(role);
    assert.ok(fromTable !== undefined, `this generation's nonce table has no ${role}`);
    assert.equal(
      getAddress(recorded),
      getAddress(fromTable),
      `zamaConfigLocal.${field} does not match ${role} in the nonce table it is a subset of`,
    );

    // And the same value as the harness's own transcription of ZamaConfig.sol. That closes the loop:
    // `npm run check:zama-config` compares ZAMA_LOCAL_CONFIG against the upstream file by parsing
    // `_getLocalConfig()`, so agreeing with it means agreeing with the library dApps actually inherit —
    // which is the only reason these three addresses are not ours to choose.
    const harness = ZAMA_LOCAL_CONFIG[ZAMA_HARNESS_KEY[field] as keyof typeof ZAMA_LOCAL_CONFIG];
    assert.equal(
      getAddress(recorded),
      getAddress(harness),
      `zamaConfigLocal.${field} disagrees with internal/constants.ts ZAMA_LOCAL_CONFIG`,
    );
  }
});

void test('rules 15/17: the generated LocalHostAddresses.sol matches the source of truth', () => {
  const l = readLocalhost();
  const table = thisGeneration(l);
  const gen = readGeneratedLocalHost();

  // Non-vacuity first: a regex that matched nothing would make every comparison below trivially true, and
  // this file is generated, so it is exactly the kind of input that can change shape underneath a parser.
  assert.ok(gen.size >= 10, `parsed only ${gen.size} constants from ${LOCAL_HOST_ADDRESSES_PATH}`);

  const diffs: string[] = [];
  const cmp = (name: string, want: string, isAddress: boolean) => {
    const got = gen.get(name);
    if (got === undefined) {
      diffs.push(`${name}: absent from the generated file`);
      return;
    }
    const a = isAddress ? getAddress(got) : BigInt(got).toString();
    const b = isAddress ? getAddress(want) : BigInt(want).toString();
    if (a !== b) diffs.push(`${name}\n    json:      ${b}\n    generated: ${a}`);
  };

  cmp('DEPLOYER_ADDRESS', l.DEPLOYER_ADDRESS.value, true);
  cmp('DEPLOYER_ADDRESS_INDEX', l.DEPLOYER_ADDRESS_INDEX.value, false);
  cmp('DEPLOYER_START_NONCE', l.DEPLOYER_START_NONCE.value, false);
  cmp('PROXY_COUNT', table.PROXY_COUNT, false);
  cmp('ADDRESSED_NONCE_COUNT', table.ADDRESSED_NONCE_COUNT, false);

  // Only the named roles. The two empty-proxy implementations occupy nonces but are not constants there —
  // nothing bakes their addresses in, which is why they have no name in the generated file either.
  for (const e of allNonces(table)) {
    if (!e.role.endsWith('_ADDRESS')) continue;
    cmp(e.role, e.address, true);
  }

  assert.deepEqual(
    diffs,
    [],
    `sdk/cleartext-config.json and the generated LocalHostAddresses.sol disagree.\n` +
      `The generated file is rebuilt by \`npm run generate:local-host-bytecode\`; if it is the JSON that is\n` +
      `wrong, note that every address is re-derived by the test above, so a value can only be wrong here by\n` +
      `being attached to the wrong ROLE — check the nonce, not the hex.`,
  );
});

void test('rules 15/17: secondary starts exactly where primary ends', () => {
  // The categories are POSITIONAL, not labels: `secondary` means "positioned after the primary block", so
  // its first nonce must be the primary block's length. Anything else would mean a gap or an overlap, and
  // in a CREATE(deployer, nonce) layout both are silent — every address stays a valid address, just of a
  // different contract than the role says.
  for (const [gen, table] of Object.entries(readLocalhost().generations)) {
    const first = table.nonces.secondary[0];
    assert.ok(first, `generation ${gen}: no secondary entries`);
    assert.equal(
      first.nonce,
      table.nonces.primary.length,
      `generation ${gen}: primary holds ${table.nonces.primary.length} nonces, so secondary must start at ` +
        `that index. This boundary is HOST_NONCE_COUNT in pkg/ts/addresses.ts.`,
    );
  }
});

void test('rules 15/17: the categories hold what their definitions say', () => {
  for (const [gen, table] of Object.entries(readLocalhost().generations)) {
    // The two empty-proxy implementations are the only entries with no baked-in address, and they belong to
    // the primary block by construction — they are what its proxies are constructed over.
    const unnamed = allNonces(table).filter((e) => !e.role.endsWith('_ADDRESS'));
    assert.deepEqual(
      unnamed.map((e) => e.role),
      ['EmptyUUPSProxyACL', 'EmptyUUPSProxy'],
      `generation ${gen}: expected exactly the two empty-proxy implementations to be unnamed`,
    );
    assert.deepEqual(
      table.nonces.secondary.filter((e) => !e.role.endsWith('_ADDRESS')),
      [],
      `generation ${gen}: every secondary entry is a named contract`,
    );
  }
});

void test('rules 15/17: every generation has the same secondary roles in the same order', () => {
  // The invariant the split exists to make visible: only the PRIMARY block changes shape between
  // generations. The secondary block is the same three contracts every time and merely starts later, so a
  // generation whose secondary list differs has either gained a cleartext contract — which would need
  // saying out loud — or mis-assigned a role across the boundary.
  const roleLists = Object.entries(readLocalhost().generations).map(
    ([gen, t]) => [gen, t.nonces.secondary.map((e) => e.role)] as const,
  );
  const [first, ...rest] = roleLists;
  assert.ok(first, 'no generations declared');
  for (const [gen, roles] of rest) {
    assert.deepEqual(
      roles,
      first[1],
      `generation ${gen}'s secondary roles differ from ${first[0]}'s. If a generation genuinely gained or ` +
        `lost one, say so here rather than relaxing the check.`,
    );
  }
});

////////////////////////////////////////////////////////////////////////////////
// The vitest suite's bootstrap fixture — a third face, checked like the other two
//
// `test/ts/utils/expectedBootstrap.ts` holds the HCU triple that a default deploy must produce. It cannot
// import the constants: `test/ts/tsconfig.json` sets `rootDir: "."` so nothing there may reach the source
// tree, which is what stops a test in that suite from exercising our source when its purpose is to
// exercise the PUBLISHED package. Honouring that boundary costs one hand-written copy, and this is what
// makes the copy safe — the same trade rule 23 makes for the other two faces.
////////////////////////////////////////////////////////////////////////////////

const EXPECTED_BOOTSTRAP_PATH = join(PACKAGE_ROOT_ABS_PATH, 'test', 'ts', 'utils', 'expectedBootstrap.ts');

void test('rule 23: the vitest bootstrap fixture matches the source of truth', () => {
  const truth = readSourceOfTruth();
  const src = readFileSync(EXPECTED_BOOTSTRAP_PATH, 'utf8');

  // Found by the trailing `// CLEARTEXT_…` comment naming the constant, which is why that comment is
  // required rather than decorative. Only lines inside the returned object are considered, so the
  // explanatory header — which quotes the old duplicated literal on purpose — cannot satisfy the check.
  const body = src.slice(src.indexOf('return {'));
  const found = new Map<string, string>();
  for (const [, literal, name] of body.matchAll(/:\s*(\d[\d_]*n?),\s*\/\/\s*([A-Z][A-Z0-9_]*)/g)) {
    if (literal === undefined || name === undefined) continue;
    found.set(name, literal);
  }

  const want = ['CLEARTEXT_HCU_CAP_PER_BLOCK', 'CLEARTEXT_MAX_HCU_DEPTH_PER_TX', 'CLEARTEXT_MAX_HCU_PER_TX'];
  assert.deepEqual(
    [...found.keys()].sort(),
    [...want].sort(),
    `expectedBootstrap.ts must annotate each value with the constant it copies — that annotation is how\n` +
      `this check locates it. Found: ${found.size === 0 ? '(none)' : [...found.keys()].join(', ')}`,
  );

  const diffs: string[] = [];
  for (const [name, literal] of found) {
    const entry = truth.get(name);
    assert.ok(entry, `${name} is not declared in the source of truth`);
    const have = canonical(entry.solidity, literal);
    const wanted = canonical(entry.solidity, entry.value ?? '');
    if (have !== wanted) diffs.push(`${name}\n    json:    ${wanted}\n    fixture: ${have}`);
    // These reach `deploy()` as bigints, so the fixture must spell them that way whatever the JSON's
    // minimal type says — a `number` here would change the call, not just the literal.
    if (!literal.endsWith('n')) diffs.push(`${name}: must be a bigint literal, found ${literal}`);
  }
  assert.deepEqual(diffs, [], 'test/ts/utils/expectedBootstrap.ts disagrees with sdk/cleartext-config.json');
});
