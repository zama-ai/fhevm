// Upgrade a LIVE v12 cleartext stack to v13, through the canonical CREATE2 factory.
//
//
// The sibling of deploy-testnet.ts, and everything that is not a stage comes from common.ts: argument
// parsing, the config file, the out-dir identity check, the chain and factory preflight, signer
// resolution, the reorg/finality waits, the journal, the seal gate and broadcast().
//
// ## Two invariants, structural rather than checked-and-hoped
//
// OWNERSHIP NEVER CHANGES. The live stack's ACL is already owned by a standing ACLOwner, and that
// ACLOwner is already owned by the admin; the upgrade runs entirely THROUGH that existing root. So this
// coordinator has no offer/accept stages at all — not stages that no-op, but absent — and it therefore
// cannot move ownership even when invoked wrongly. `verify` asserts it anyway.
//
// THE PAUSERS STAY THE SAME. No pauser stage either. PauserSet is untouched: same contract, same
// membership. Also asserted rather than assumed, because PauserSet exposes no enumeration.
//
// ## What is NOT here
//
// The nonce path (scripts/deploy.sh) and the TypeScript `updateV12ToV13` in pkg/ts are both untouched and
// remain what they were. This is a third path, for a stack whose addresses have to be predictable before
// anything is sent.

import { copyFileSync, existsSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

import {
  appendJsonl,
  capture,
  captureOrFail,
  ensureDir,
  fail,
  freePort,
  hexToNumber,
  pad,
  readJson,
  readJsonl,
  removeIfPresent,
  requireTool,
  runLogged,
  say,
  sleep,
  spawnBackground,
  startTranscript,
  waitUntil,
  warn,
} from './utils.ts';
import {
  broadcast,
  buildContext,
  type Ctx,
  existingFlagFor,
  EXISTING_ROLES,
  finalizedBlock,
  type Flow,
  generatedConfigEnv,
  hasCodeAt,
  headBlock,
  type JournalEntry,
  loadConfigFile,
  type Manifest,
  manifestPath,
  PACKAGE_ROOT,
  parseCliArgs,
  preflight,
  readJournal,
  recordObservation,
  requireBuiltArtifacts,
  resolveOptions,
  ROLE_WIDTH,
  RULE_WIDTH,
  SCRIPT_DIR,
  scriptEnv,
  settledBlock,
  showJournal,
  stageReport,
  traceArgs,
  waitForBlock,
} from './common.ts';

////////////////////////////////////////////////////////////////////////////////
// Stages
////////////////////////////////////////////////////////////////////////////////

/**
 * Four real stages against the deploy's nine, plus `precheck`, the read-only gate `materialize` runs first.
 *
 * Absent by design, and their absence is what enforces the invariants: `pausers`, `offer-acl`, `accept-acl`,
 * `offer-admin`, `accept-admin`. An upgrade that cannot name those stages cannot run them.
 */
export type Stage =
  | 'compute'
  | 'creates'
  | 'precheck'
  | 'rehearse'
  | 'materialize'
  | 'verify'
  | 'status'
  | 'log'
  | 'report'
  | 'progress'
  | 'params'
  | 'all';

const ALL_STAGES: readonly Stage[] = [
  'compute',
  'creates',
  'precheck',
  'rehearse',
  'materialize',
  'verify',
  'status',
  'log',
  'report',
  'progress',
  'params',
  'all',
];

/** What `--stage all` runs, in order. `precheck` is not listed because `materialize` always runs it first. */
const RUN_ORDER: readonly Stage[] = ['compute', 'creates', 'rehearse', 'materialize', 'verify'];

/** The seven proxies `ACLOwner.upgrade` re-points, in op order — the coordinator's copy of the Solidity table. */
const UPGRADED_ROLES: readonly string[] = [
  'PROTOCOL_CONFIG_ADDRESS',
  'KMS_GENERATION_ADDRESS',
  'ACL_ADDRESS',
  'FHEVM_EXECUTOR_ADDRESS',
  'HCU_LIMIT_ADDRESS',
  'KMS_VERIFIER_ADDRESS',
  'CLEARTEXT_ARITHMETIC_ADDRESS',
];

/** The five of those that already exist: the live proxies whose implementation slot the upgrade moves. */
const LIVE_UPGRADED_ROLES: readonly string[] = UPGRADED_ROLES.filter(
  (role) => role !== 'PROTOCOL_CONFIG_ADDRESS' && role !== 'KMS_GENERATION_ADDRESS',
);

/** The two live proxies the upgrade must leave alone: byte-identical across the two generations. */
const UNTOUCHED_ROLES: readonly string[] = ['INPUT_VERIFIER_ADDRESS', 'CLEARTEXT_DB_ADDRESS'];

/** The ten sealed creates, in dependency order. */
const CREATE_ROLES: readonly string[] = [
  'IMPL_EMPTY_UUPS_PROXY',
  'PROTOCOL_CONFIG_ADDRESS',
  'KMS_GENERATION_ADDRESS',
  ...UPGRADED_ROLES.map((role) => `IMPL_${role}`),
];

const REPORT_STEPS: ReadonlyArray<{ readonly label: string; readonly title: string }> = [
  { label: 'creates', title: 'every CREATE2 through the factory' },
  { label: 'D', title: 'ACLOwner.upgrade — 2 materializations + 5 reinitializations' },
];

const DEFAULT_CONFIG_NAME = 'upgrade.config.json';

/** `log`, `report` and `progress` read only local files, so they work with no network at all. */
function needsChain(stage: string): boolean {
  return stage !== 'log' && stage !== 'report' && stage !== 'progress';
}

/**
 * Which stages need the deployer resolved from the KEYSTORE, at the cost of a password prompt.
 *
 * `compute` is on this side of the line for the same reason it is in the deploy: the new addresses are a
 * function of the deployer, so taking it from anywhere but the key itself would let a typo seal a set
 * nobody holds the key to.
 */
function needsDeployerKey(stage: string): boolean {
  return stage === 'compute' || stage === 'creates' || stage === 'all';
}

////////////////////////////////////////////////////////////////////////////////

const HELP = `
Upgrade a live v12 cleartext stack to v13, via the canonical CREATE2 factory.

Usage: node create2-deploy/upgrade-testnet.ts --rpc-url URL [--account NAME] --admin 0x...
                             --deployment-id ID --previous-manifest PATH [--handle 0x...]

  --rpc-url URL        node to upgrade on (required)
  --account NAME       forge keystore account to broadcast from. Required on every chain EXCEPT a
                       local anvil, where accounts 0 and 1 of anvil's public mnemonic are used
  --admin 0x...        the CURRENT ACLOwner owner. Not a value this sets — a value it VERIFIES, since
                       ownership must not change
  --deployment-id ID   reuse the deployment's own id. The salt mixes the version, so "0.13" here and
                       "0.12" for the original deploy already give disjoint addresses

THE LIVE STACK — nine addresses, best supplied through --previous-manifest or the config file:

  --acl 0x...                    --hcu-limit 0x...
  --fhevm-executor 0x...         --cleartext-arithmetic 0x...
  --kms-verifier 0x...           --cleartext-db 0x...
  --input-verifier 0x...         --pauser-set 0x...
  --acl-owner 0x...

  --previous-manifest PATH
                       manifest.json from the deploy that produced the live stack. Seeds all nine
                       roles above from its "address" block, and is cross-checked FIRST: its chainId
                       must be this chain and its deploymentId must be --deployment-id. Lowest
                       precedence — the config file's "existing" block and the flags above both still
                       win, so one address can be corrected without editing anything

  Every one is validated against the live chain before anything is computed, because the seven v13
  implementations BAKE these addresses into their creation code: a wrong entry produces a stack that
  materializes cleanly and fails only in use. Eight are cross-checked against the stack's own wiring;
  --kms-verifier cannot be (no contract exposes a getter returning it), so it gets a weaker check and
  says so.

  --handle 0x...       a cleartext handle already in the live CleartextDB, repeatable. compute records
                       its value; verify requires it unchanged. STRONGLY RECOMMENDED: without one,
                       verify can only prove the stack still works, not that existing data survived

  --migration PATH     the KMS migration seed. Omit to reconstruct it from the live KMSVerifier plus
                       the package defaults, which is refused if the live signer set disagrees.
                       Keys are ProtocolConfig.initializeFromMigration's parameter names, verbatim:
                       existingContextId, existingKmsNodes[], existingThresholds. "existing" is the
                       KMS CONTEXT being carried over from v12; inside each node only signerAddress
                       exists on v12 — txSenderAddress, ipAddress and storageUrl are new in v13 and
                       are what this file is for
  --previous-abi-dir PATH
                       v12 ABI directory used to snapshot every zero-argument getter. Defaults to
                       ../v12/pkg/abi; compute refuses to use v13 ABIs as a weaker fallback


  --confirmations N    reorg DEPTH floor for the between-stage waits
  --no-finality        wait only for --confirmations of depth, not for finality
  --out-dir PATH       where this upgrade's seal, generated config and journal go
  --dry-run            run the chosen stage WITHOUT --broadcast
  --min-block N        FHEVM_MIN_BLOCK for a single manual --stage run
  --no-confirm         do not ask about the seal before the first transaction
  --no-build           reuse the artifacts already in the out dir
  --no-git             this upgrade needs no git-committed seal
  --config PATH        JSON file holding the stable arguments (default: ${DEFAULT_CONFIG_NAME})
  --stage STAGE        one of, in order:
                         compute       2 builds + 2 passes, writes the manifest      (no tx)
                         creates       the CREATE2s through the factory
                         precheck      the gate before the point of no return          (no tx)
                         rehearse      precheck, then the exact upgrade calldata applied on a
                                       local anvil FORK of this chain, then verify against the
                                       fork. Nothing reaches this chain                (no tx)
                         materialize   precheck, then one atomic ACLOwner.upgrade
                         verify        versions, wiring, and the invariants           (no tx)
                       or a read-only view: status, log, report, progress, params
                       default: all
                       precheck and verify recompile into <out-dir>/build-check first, so the
                       seal is re-derived from the current checkout rather than from the build
                       that produced it; --no-build checks against the sealed build instead
  -v, --verbose        pass -vvvv to forge
  -h, --help           this

PROGRESS. Every step of every stage is announced as it starts and reported as it ends, with its duration
and the head block, and appended to <out-dir>/progress.jsonl; the full output of each invocation,
forge's included, is kept in <out-dir>/logs/<time>-<stage>.log. --stage progress renders the ledger
without a node. --stage all prints the plan first.

WAITS. Nothing that decides anything is read from a block that can still be reorged. compute snapshots
at the settled block (finalized, or --confirmations deep with --no-finality); rehearse and materialize
wait until every create is settled; verify waits until the materialize block is. All of it is derived
from the chain — never from the journal — so an interrupted run resumes by re-running the same command.
`;

////////////////////////////////////////////////////////////////////////////////
// The live stack
////////////////////////////////////////////////////////////////////////////////

/**
 * One `cast call` returning an address, or null if the call reverted or answered zero.
 *
 * `capture`, not `captureOrFail`: a revert is a legitimate ANSWER here rather than a failure. It is what
 * a wrong address looks like when it holds no code, or holds something without that getter — and the
 * caller is what decides whether that is fatal.
 */
/** `--block N` for a read that must be taken at the snapshot block; nothing for a read at the head. */
function atBlock(block?: number): string[] {
  return block === undefined ? [] : ['--block', String(block)];
}

function callAddress(ctx: Ctx, target: string, sig: string, block?: number): string | null {
  const r = capture('cast', ['call', target, sig, '--rpc-url', ctx.opt.rpcUrl, ...atBlock(block)]);
  if (!r.ok) return null;
  const out = r.stdout;
  // `cast call` answers with a 32-byte word; an address is its low 20 bytes.
  if (out === '' || /^0x0*$/.test(out)) return null;
  return `0x${out.slice(-40)}`;
}

/**
 * One `cast call` returning a string (`getVersion`, and the EIP-712 domain name).
 *
 * No `abi-decode` step: giving `cast call` a signature WITH the return type — `getVersion()(string)` —
 * makes it decode for us and answer `"KMSVerifier v0.2.0"`, quotes included. Decoding that again fails,
 * which reported a perfectly good KMSVerifier as uninitialized and would have blocked a correct upgrade.
 * Only the quotes need stripping.
 */
function callString(ctx: Ctx, target: string, sig: string): string | null {
  const r = capture('cast', ['call', target, sig, '--rpc-url', ctx.opt.rpcUrl]);
  return r.ok ? r.stdout.replace(/^"|"$/g, '') : null;
}

function callUint(ctx: Ctx, target: string, sig: string, block?: number): bigint | null {
  const r = capture('cast', ['call', target, sig, '--rpc-url', ctx.opt.rpcUrl, ...atBlock(block)]);
  if (!r.ok || r.stdout === '') return null;
  const value = /^(0x[0-9a-fA-F]+|[0-9]+)/.exec(r.stdout)?.[1];
  if (value === undefined) return null;
  try {
    return BigInt(value);
  } catch {
    return null;
  }
}

function callAddresses(ctx: Ctx, target: string, sig: string, block?: number): string[] | null {
  const r = capture('cast', ['call', target, sig, '--rpc-url', ctx.opt.rpcUrl, ...atBlock(block)]);
  if (!r.ok) return null;
  return r.stdout.match(/0x[0-9a-fA-F]{40}/g) ?? [];
}

function isAddress(value: unknown): value is string {
  return typeof value === 'string' && /^0x[0-9a-fA-F]{40}$/.test(value);
}

function sameAddressList(a: readonly string[], b: readonly string[]): boolean {
  return a.length === b.length && a.every((value, i) => value.toLowerCase() === b[i]?.toLowerCase());
}

type MigrationNode = {
  readonly txSenderAddress: string;
  readonly signerAddress: string;
  readonly ipAddress: string;
  readonly storageUrl: string;
};

type SealedMigration = {
  readonly existingContextId: string;
  readonly existingKmsNodes: readonly MigrationNode[];
  readonly existingThresholds: {
    readonly publicDecryption: string;
    readonly userDecryption: string;
    readonly kmsGen: string;
    readonly mpc: string;
  };
};

type AbiParameter = { readonly type?: string; readonly components?: readonly AbiParameter[] };
type AbiEntry = {
  readonly type?: string;
  readonly name?: string;
  readonly stateMutability?: string;
  readonly inputs?: readonly AbiParameter[];
  readonly outputs?: readonly AbiParameter[];
};

type PreUpgradeSnapshot = {
  readonly blockNumber: number;
  readonly handles: Readonly<Record<string, string>>;
  readonly readings: Readonly<Record<string, string>>;
  readonly admin: string;
  readonly kmsSigners: readonly string[];
  readonly kmsThreshold: number;
  readonly coprocessorSigners: readonly string[];
  readonly coprocessorThreshold: number;
  readonly migration: SealedMigration;
  readonly implementation: Readonly<Record<string, string>>;
};

const ERC1967_IMPLEMENTATION_SLOT = '0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc';

const SURVEY_TARGETS: ReadonlyArray<{ readonly label: string; readonly role: string; readonly abi: string }> = [
  { label: 'ACL', role: 'ACL_ADDRESS', abi: 'ACL.json' },
  { label: 'FHEVMExecutor', role: 'FHEVM_EXECUTOR_ADDRESS', abi: 'CleartextFHEVMExecutor.json' },
  { label: 'KMSVerifier', role: 'KMS_VERIFIER_ADDRESS', abi: 'CleartextKMSVerifier.json' },
  { label: 'InputVerifier', role: 'INPUT_VERIFIER_ADDRESS', abi: 'CleartextInputVerifier.json' },
  { label: 'HCULimit', role: 'HCU_LIMIT_ADDRESS', abi: 'HCULimit.json' },
  { label: 'CleartextArithmetic', role: 'CLEARTEXT_ARITHMETIC_ADDRESS', abi: 'CleartextArithmetic.json' },
  { label: 'CleartextDB', role: 'CLEARTEXT_DB_ADDRESS', abi: 'CleartextDB.json' },
  { label: 'PauserSet', role: 'PAUSER_SET_ADDRESS', abi: 'PauserSet.json' },
  { label: 'ACLOwner', role: 'ACL_OWNER', abi: 'ACLOwner.json' },
];

function abiType(parameter: AbiParameter): string {
  const type = parameter.type ?? '';
  if (!type.startsWith('tuple')) return type;
  const suffix = type.slice('tuple'.length);
  return `(${(parameter.components ?? []).map(abiType).join(',')})${suffix}`;
}

function previousAbiDir(ctx: Ctx): string {
  return ctx.opt.previousAbiDir ?? join(PACKAGE_ROOT, '..', 'v12', 'pkg', 'abi');
}

function surveyStack(ctx: Ctx, block?: number): Readonly<Record<string, string>> {
  const dir = previousAbiDir(ctx);
  const readings: Record<string, string> = {};
  for (const target of SURVEY_TARGETS) {
    const path = join(dir, target.abi);
    if (!existsSync(path)) fail(`Error: required v12 ABI is missing: ${path}`, '       Pass --previous-abi-dir PATH.');
    const abi = JSON.parse(readFileSync(path, 'utf8')) as AbiEntry[];
    const getters = abi.filter(
      (entry) =>
        entry.type === 'function' &&
        (entry.stateMutability === 'view' || entry.stateMutability === 'pure') &&
        (entry.inputs ?? []).length === 0 &&
        entry.name !== undefined,
    );
    for (const getter of getters) {
      const name = getter.name ?? '';
      const returns = (getter.outputs ?? []).map(abiType).join(',');
      const signature = returns === '' ? `${name}()` : `${name}()(${returns})`;
      const result = capture('cast', [
        'call',
        ctx.opt.existing[target.role] ?? '',
        signature,
        '--rpc-url',
        ctx.opt.rpcUrl,
        ...atBlock(block),
      ]);
      readings[`${target.label}.${name}`] = result.ok ? result.stdout : '<reverted>';
    }
  }
  if (Object.keys(readings).length < 50) {
    fail(
      `Error: the v12 ABI survey produced only ${String(Object.keys(readings).length)} readings; refusing a vacuous snapshot.`,
    );
  }
  return readings;
}

function solidityArray(source: string, functionName: string, quoted: boolean): string[] {
  const match = new RegExp(`function ${functionName}\\(\\)[\\s\\S]*?\\{([\\s\\S]*?)\\n    \\}`).exec(source);
  const body = match?.[1];
  if (body === undefined) fail(`Error: cannot read ${functionName}() from generated LocalHostBootstrap.sol.`);
  const values = [...body.matchAll(/out\[\d+\] = ([^;]+);/g)].map((entry) => (entry[1] ?? '').trim());
  if (values.length === 0) fail(`Error: ${functionName}() has no entries in generated LocalHostBootstrap.sol.`);
  return quoted ? values.map((value) => JSON.parse(value) as string) : values;
}

function defaultMigrationNodes(liveSigners: readonly string[]): MigrationNode[] {
  const source = readFileSync(join(PACKAGE_ROOT, 'pkg', 'forge', 'src', '_internal', 'LocalHostBootstrap.sol'), 'utf8');
  const signers = solidityArray(source, 'kmsSigners', false);
  const txSenders = solidityArray(source, 'kmsTxSenders', false);
  const ips = solidityArray(source, 'kmsIpAddresses', true);
  const urls = solidityArray(source, 'kmsStorageUrls', true);
  return liveSigners.map((signer) => {
    const index = signers.findIndex((candidate) => candidate.toLowerCase() === signer.toLowerCase());
    if (index < 0) {
      fail(
        `Error: live KMS signer ${signer} is not in LocalHostBootstrap's default signer pool.`,
        '       Supply --migration with the real node metadata; defaults would register the wrong nodes.',
      );
    }
    const txSenderAddress = txSenders[index];
    const ipAddress = ips[index];
    const storageUrl = urls[index];
    if (!isAddress(txSenderAddress) || ipAddress === undefined || storageUrl === undefined) {
      fail('Error: generated LocalHostBootstrap KMS metadata arrays are not index-aligned.');
    }
    return { txSenderAddress, signerAddress: signer, ipAddress, storageUrl };
  });
}

function parseBigInt(value: unknown, label: string): bigint {
  if (typeof value !== 'string' && typeof value !== 'number') fail(`Error: ${label} must be an integer string.`);
  if (typeof value === 'number' && !Number.isSafeInteger(value))
    fail(`Error: ${label} is not a safe JSON integer; quote it.`);
  try {
    const parsed = BigInt(value);
    if (parsed < 0n) fail(`Error: ${label} must not be negative.`);
    return parsed;
  } catch {
    fail(`Error: ${label} is not an integer.`);
  }
}

function resolveMigration(
  ctx: Ctx,
  liveContextId: bigint,
  liveSigners: readonly string[],
  liveThreshold: bigint,
): SealedMigration {
  let contextId = liveContextId;
  let nodes: readonly MigrationNode[];
  let thresholds = {
    publicDecryption: liveThreshold,
    userDecryption: liveThreshold,
    kmsGen: liveThreshold,
    mpc: liveThreshold,
  };

  if (ctx.opt.migrationPath !== null) {
    const raw = readJson<Record<string, unknown>>(ctx.opt.migrationPath);
    if (raw === null) fail(`Error: migration file not found: ${ctx.opt.migrationPath}`);
    contextId = parseBigInt(raw.existingContextId, 'migration.existingContextId');
    if (!Array.isArray(raw.existingKmsNodes)) fail('Error: migration.existingKmsNodes must be an array.');
    nodes = raw.existingKmsNodes.map((entry, index) => {
      if (typeof entry !== 'object' || entry === null)
        fail(`Error: migration.existingKmsNodes[${String(index)}] is not an object.`);
      const node = entry as Record<string, unknown>;
      if (!isAddress(node.txSenderAddress) || !isAddress(node.signerAddress)) {
        fail(`Error: migration.existingKmsNodes[${String(index)}] has an invalid address.`);
      }
      if (typeof node.ipAddress !== 'string' || typeof node.storageUrl !== 'string') {
        fail(`Error: migration.existingKmsNodes[${String(index)}] has invalid metadata.`);
      }
      return {
        txSenderAddress: node.txSenderAddress,
        signerAddress: node.signerAddress,
        ipAddress: node.ipAddress,
        storageUrl: node.storageUrl,
      };
    });
    const rawThresholds = raw.existingThresholds;
    if (typeof rawThresholds !== 'object' || rawThresholds === null) {
      fail('Error: migration.existingThresholds must be an object.');
    }
    const t = rawThresholds as Record<string, unknown>;
    thresholds = {
      publicDecryption: parseBigInt(t.publicDecryption, 'migration.existingThresholds.publicDecryption'),
      userDecryption: parseBigInt(t.userDecryption, 'migration.existingThresholds.userDecryption'),
      kmsGen: parseBigInt(t.kmsGen, 'migration.existingThresholds.kmsGen'),
      mpc: parseBigInt(t.mpc, 'migration.existingThresholds.mpc'),
    };
  } else {
    nodes = defaultMigrationNodes(liveSigners);
  }

  const migrationSigners = nodes.map((node) => node.signerAddress);
  if (!sameAddressList(migrationSigners, liveSigners)) {
    fail('Error: migration KMS signers do not exactly match the live v12 signer set (including order).');
  }
  if (contextId !== liveContextId) fail('Error: migration context id does not match the live v12 context id.');
  if (liveThreshold <= 0n || Object.values(thresholds).some((threshold) => threshold !== liveThreshold)) {
    fail('Error: all four migration thresholds must equal the live v12 KMS threshold.');
  }
  const txSenders = nodes.map((node) => node.txSenderAddress.toLowerCase());
  if (new Set(txSenders).size !== txSenders.length) fail('Error: migration KMS tx-sender addresses must be unique.');

  return {
    existingContextId: contextId.toString(),
    existingKmsNodes: nodes,
    existingThresholds: Object.fromEntries(
      Object.entries(thresholds).map(([key, value]) => [key, value.toString()]),
    ) as SealedMigration['existingThresholds'],
  };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Every supplied address, checked against the live chain before anything is computed.
 *
 * This is the most important thing this coordinator does, and the reason is worth stating where the code
 * is: the seven v13 implementations bake these addresses into their CREATION CODE. A wrong entry does not
 * fail here-and-now — it produces implementations at addresses derived from the wrong inputs, whose
 * compiled-in references point at nothing. The stack materializes cleanly and fails in use.
 *
 * The stack largely describes itself, so most of the work is asking it. Every expectation below is a
 * reading from a DIFFERENT contract than the one being checked, which is what makes it a check rather
 * than a tautology.
 */
function validateExisting(ctx: Ctx): void {
  const existing = ctx.opt.existing;

  const missing = EXISTING_ROLES.filter((role) => (existing[role] ?? '') === '');
  if (missing.length > 0) {
    const previous = ctx.opt.previousManifest;
    const fromManifest =
      previous === null
        ? []
        : ['', `       --previous-manifest supplied ${String(Object.keys(previous.address).length)} of them.`];
    fail(
      `Error: the live stack is not fully specified — ${String(missing.length)} of ${String(EXISTING_ROLES.length)} addresses missing:`,
      ...missing.map((role) => `         ${existingFlagFor(role)}  (${role})`),
      ...fromManifest,
      '',
      `       Put them in the "existing" block of ${DEFAULT_CONFIG_NAME}, or pass the flags. An upgrade`,
      '       derives nothing from the chain it does not first verify.',
    );
  }

  const at = (role: string): string => existing[role] ?? '';
  const acl = at('ACL_ADDRESS');
  const executor = at('FHEVM_EXECUTOR_ADDRESS');
  const arithmetic = at('CLEARTEXT_ARITHMETIC_ADDRESS');
  const aclOwner = at('ACL_OWNER');

  say('🔬  validating the live stack');

  // Where the address under test came from, so the transcript records the seal's provenance and not
  // just its content. `?` for a role no layer supplied — the missing gate above has already failed.
  const from = (role: string): string => `[${ctx.opt.existingSource[role] ?? '?'}]`;

  let bad = 0;
  const expect = (role: string, target: string, sig: string, source: string): void => {
    const got = callAddress(ctx, target, sig);
    const want = at(role);
    if (got === null) {
      say(`    ?    ${pad(role, ROLE_WIDTH)} ${source} did not answer — is that address a live v12 contract?`);
      bad += 1;
      return;
    }
    if (got.toLowerCase() !== want.toLowerCase()) {
      say(`    FAIL ${pad(role, ROLE_WIDTH)} ${source} says ${got}, you passed ${want} ${from(role)}`);
      bad += 1;
      return;
    }
    say(`    ok   ${pad(role, ROLE_WIDTH)} confirmed by ${source}  ${from(role)}`);
  };

  // Eight addresses, each corroborated by a contract OTHER than itself. ACL by three.
  expect('ACL_ADDRESS', executor, 'getACLAddress()(address)', 'FHEVMExecutor.getACLAddress');
  expect('ACL_ADDRESS', at('CLEARTEXT_DB_ADDRESS'), 'getACLAddress()(address)', 'CleartextDB.getACLAddress');
  expect('ACL_ADDRESS', aclOwner, 'ACL_ADDRESS()(address)', 'ACLOwner.ACL_ADDRESS');
  expect('FHEVM_EXECUTOR_ADDRESS', acl, 'getFHEVMExecutorAddress()(address)', 'ACL.getFHEVMExecutorAddress');
  expect(
    'FHEVM_EXECUTOR_ADDRESS',
    at('HCU_LIMIT_ADDRESS'),
    'getFHEVMExecutorAddress()(address)',
    'HCULimit.getFHEVMExecutorAddress',
  );
  expect(
    'INPUT_VERIFIER_ADDRESS',
    executor,
    'getInputVerifierAddress()(address)',
    'FHEVMExecutor.getInputVerifierAddress',
  );
  expect('HCU_LIMIT_ADDRESS', executor, 'getHCULimitAddress()(address)', 'FHEVMExecutor.getHCULimitAddress');
  expect(
    'CLEARTEXT_ARITHMETIC_ADDRESS',
    executor,
    'getCleartextArithmeticAddress()(address)',
    'FHEVMExecutor.getCleartextArithmeticAddress',
  );
  expect(
    'CLEARTEXT_DB_ADDRESS',
    arithmetic,
    'getCleartextDBAddress()(address)',
    'CleartextArithmetic.getCleartextDBAddress',
  );
  expect('PAUSER_SET_ADDRESS', acl, 'getPauserSetAddress()(address)', 'ACL.getPauserSetAddress');
  expect('ACL_OWNER', acl, 'owner()(address)', 'ACL.owner');

  // --admin is verified, never set. The upgrade must not move ownership, so a mismatch here means
  // the operator believes something false about who controls the stack.
  const liveAdmin = callAddress(ctx, aclOwner, 'owner()(address)');
  if (liveAdmin?.toLowerCase() !== ctx.opt.admin.toLowerCase()) {
    say(
      `    FAIL ${pad('ACLOwner.owner (--admin)', ROLE_WIDTH)} chain says ${liveAdmin ?? '(no answer)'}, you passed ${ctx.opt.admin}`,
    );
    bad += 1;
  } else {
    say(`    ok   ${pad('ACLOwner.owner (--admin)', ROLE_WIDTH)} matches --admin`);
  }

  // KMS_VERIFIER_ADDRESS: no contract in the v12 set exposes a getter returning it, so it cannot be
  // corroborated by wiring the way the others are. What follows establishes "this is an initialized v12
  // KMSVerifier" — NOT that it is this stack's. The post-upgrade signer-set comparison is what would
  // catch the residual case of pointing at another deployment's verifier on the same chain.
  const kms = at('KMS_VERIFIER_ADDRESS');
  const kmsCode = capture('cast', ['code', kms, '--rpc-url', ctx.opt.rpcUrl]);
  const kmsVersion = callString(ctx, kms, 'getVersion()(string)');
  const kmsSigners = callAddresses(ctx, kms, 'getKmsSigners()(address[])');
  const kmsThreshold = callUint(ctx, kms, 'getThreshold()(uint256)');
  const domain = capture('cast', [
    'call',
    kms,
    'eip712Domain()(bytes1,string,string,uint256,address,bytes32,uint256[])',
    '--rpc-url',
    ctx.opt.rpcUrl,
  ]);
  if (
    !kmsCode.ok ||
    kmsCode.stdout === '' ||
    kmsCode.stdout === '0x' ||
    kmsVersion !== 'KMSVerifier v0.2.0' ||
    kmsSigners === null ||
    kmsSigners.length === 0 ||
    kmsThreshold === null ||
    kmsThreshold <= 0n ||
    !domain.ok ||
    !domain.stdout.includes('Decryption')
  ) {
    say(`    FAIL ${pad('KMS_VERIFIER_ADDRESS', ROLE_WIDTH)} not an initialized v12 Decryption KMSVerifier`);
    bad += 1;
  } else {
    say(
      `    ok   ${pad('KMS_VERIFIER_ADDRESS', ROLE_WIDTH)} reports ${kmsVersion} — WEAK: not corroborated by wiring  ${from('KMS_VERIFIER_ADDRESS')}`,
    );
    warn('       KMS_VERIFIER_ADDRESS is the one address the live stack cannot confirm.');
  }

  if (bad > 0) {
    fail(
      '',
      `Error: ${String(bad)} of the supplied addresses do not match the live stack.`,
      '       Nothing has been computed. Fix them before re-running: the seven v13 implementations bake',
      '       these addresses into their creation code, so a wrong one produces a stack that deploys',
      '       cleanly and fails only in use.',
    );
  }
  say(`    ${String(EXISTING_ROLES.length)} addresses verified against the live stack`);
}

////////////////////////////////////////////////////////////////////////////////
// Stages
////////////////////////////////////////////////////////////////////////////////

/**
 * Give a missing source a coordinator-level error rather than Forge's less useful "script not found".
 */
function requireScript(name: string): string {
  const path = join(SCRIPT_DIR, name);
  if (!existsSync(join(PACKAGE_ROOT, path))) {
    fail(`Error: ${path} does not exist yet.`, '       Restore the upgrade script before running this stage.');
  }
  return path;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Has this upgrade already put anything on chain? Asked of the CHAIN, through the sealed manifest's own
 * addresses, so a run killed between sending and journaling still answers yes. The journal is consulted
 * too, but only as a second witness — the chain is the one that cannot be lost with a directory.
 */
function upgradeStarted(ctx: Ctx): boolean {
  if (readJsonl<JournalEntry>(ctx.journalPath).length > 0) return true;
  const manifest = readJson<Manifest>(manifestPath(ctx));
  if (manifest?.address === undefined) return false;
  const head = headBlock(ctx);
  return CREATE_ROLES.some((role) => hasCodeAt(ctx, manifest.address?.[role] ?? '', head));
}

async function stageCompute(ctx: Ctx): Promise<void> {
  say('🍟 compute (2 passes, 1 rebuild)');

  // Same reasoning as the deploy: recomputing after transactions have been sent would move the sealed
  // address set out from under a half-applied upgrade. An upgrade is worse than a deploy here, because
  // `materialize` is NOT idempotent — the reinitializers are reinitializer(n)-guarded, so a second run
  // reverts rather than no-oping. Also the snapshot block would move past the creates, and verify's event
  // window with it.
  if (upgradeStarted(ctx)) {
    fail(
      `Error: '${ctx.opt.deploymentId}' has already put contracts on chain (see ${ctx.journalPath} and --stage status),`,
      '       so its addresses are not safe to recompute. Resume with the next stage instead.',
      `       To discard this upgrade's record and start over: rm -rf ${ctx.outDir}`,
    );
  }

  ensureDir(ctx.outDir);
  removeIfPresent(
    ctx.buildOut,
    join(ctx.outDir, 'addresses.sol'),
    join(ctx.outDir, 'pass2.json'),
    manifestPath(ctx),
    preUpgradePath(ctx),
  );

  // Before anything is computed, and before any build. This is the cheap check that prevents the
  // expensive mistake, and the only point at which the old values are still available.
  await step(ctx, 'compute', 'validate the nine live addresses against the stack', () => {
    validateExisting(ctx);
  });
  await step(ctx, 'compute', 'snapshot the live stack at the settled block', () => {
    writePreUpgrade(ctx, capturePreUpgrade(ctx));
  });
  writeInitialAddresses(ctx);

  const script = requireScript('FhevmComputeUpgradeAddresses.s.sol');
  const build = async (env: NodeJS.ProcessEnv): Promise<void> => {
    if (ctx.opt.noBuild) {
      requireBuiltArtifacts(ctx);
      say('  (--no-build: using the artifacts already in the out dir)');
      return;
    }
    if ((await runLogged('forge', ['build', '--out', ctx.buildOut, '--skip', 'test'], env)) !== 0) {
      fail('Error: forge build failed.');
    }
  };
  const pass = async (n: number, env: NodeJS.ProcessEnv): Promise<void> => {
    // --rpc-url even though a pass sends nothing: without it forge runs against its own in-memory EVM
    // where block.chainid is 31337, and the manifest would be sealed for the wrong chain.
    const args = [
      'script',
      `${script}:FhevmComputeUpgradeAddresses`,
      '--out',
      ctx.buildOut,
      '--rpc-url',
      ctx.opt.rpcUrl,
      ...traceArgs(ctx),
    ];
    if ((await runLogged('forge', args, { ...env, FHEVM_PASS: String(n) })) !== 0) {
      fail(`Error: compute pass ${String(n)} failed.`);
    }
  };

  // Two passes, not the deploy's three: the ACL already exists, so nothing computed here feeds
  // anything the live stack has already fixed.
  const env = { ...scriptEnv(ctx, existingEnv(ctx)), ...generatedConfigEnv(ctx) };
  await step(ctx, 'compute', 'build, then pass 1: the empty implementation and the two new proxies', async () => {
    await build(env);
    await pass(1, env);
  });
  await step(ctx, 'compute', 'rebuild, then pass 2: the seven implementations, assert, seal', async () => {
    await build(env);
    await pass(2, env);
  });
  mergePreUpgradeIntoManifest(ctx);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The value of each `--handle`, read before the upgrade and required unchanged after it.
 *
 * Reported rather than silently skipped when empty, because the two situations give different guarantees
 * and conflating them in the output would overstate what `verify` proved.
 */
function recordHandleValues(ctx: Ctx, block?: number): Readonly<Record<string, string>> {
  const values: Record<string, string> = {};
  if (ctx.opt.handles.length === 0) {
    warn(
      '  no --handle given: verify will be able to prove the stack still WORKS after the upgrade, but',
      '  not that existing cleartext data survived it. Those are different claims.',
    );
    return values;
  }
  const db = ctx.opt.existing.CLEARTEXT_DB_ADDRESS ?? '';
  for (const handle of ctx.opt.handles) {
    if (!/^0x[0-9a-fA-F]{64}$/.test(handle)) fail(`Error: --handle is not bytes32: ${handle}`);
    const r = capture('cast', [
      'call',
      db,
      'get(bytes32)(uint256)',
      handle,
      '--rpc-url',
      ctx.opt.rpcUrl,
      ...atBlock(block),
    ]);
    if (!r.ok) {
      fail(
        `Error: --handle ${handle} does not resolve in the live CleartextDB at ${db}.`,
        '       A handle that cannot be read now cannot be shown to survive.',
      );
    }
    values[handle.toLowerCase()] = r.stdout;
    say(`    ${pad(handle, ROLE_WIDTH)} = ${r.stdout}`);
  }
  return values;
}

function safeNumber(value: bigint, label: string): number {
  if (value > BigInt(Number.MAX_SAFE_INTEGER)) fail(`Error: ${label} exceeds JavaScript's safe integer range.`);
  return Number(value);
}

/**
 * The pre-upgrade snapshot, every reading taken at ONE settled block.
 *
 * One block, so the readings are consistent with each other; a settled one, so none of them can be
 * reorged away. A snapshot read at the head is the seal's only witness to "before", and a witness whose
 * block is later orphaned has seen a chain that no longer exists.
 */
function capturePreUpgrade(ctx: Ctx): PreUpgradeSnapshot {
  const blockNumber = settledBlock(ctx);
  if (!Number.isSafeInteger(blockNumber) || blockNumber < 0) fail('Error: could not capture the pre-upgrade block.');
  say(`🔭  snapshotting the live stack at settled block ${String(blockNumber)}`);

  const kms = ctx.opt.existing.KMS_VERIFIER_ADDRESS ?? '';
  const inputVerifier = ctx.opt.existing.INPUT_VERIFIER_ADDRESS ?? '';
  const aclOwner = ctx.opt.existing.ACL_OWNER ?? '';
  const admin = callAddress(ctx, aclOwner, 'owner()(address)', blockNumber);
  const kmsSigners = callAddresses(ctx, kms, 'getKmsSigners()(address[])', blockNumber);
  const kmsThreshold = callUint(ctx, kms, 'getThreshold()(uint256)', blockNumber);
  const kmsContextId = callUint(ctx, kms, 'getCurrentKmsContextId()(uint256)', blockNumber);
  const coprocessorSigners = callAddresses(ctx, inputVerifier, 'getCoprocessorSigners()(address[])', blockNumber);
  const coprocessorThreshold = callUint(ctx, inputVerifier, 'getThreshold()(uint256)', blockNumber);
  if (
    admin === null ||
    kmsSigners === null ||
    kmsThreshold === null ||
    kmsContextId === null ||
    coprocessorSigners === null ||
    coprocessorThreshold === null
  ) {
    fail('Error: could not capture the pre-upgrade authority and signer snapshot.');
  }
  say(`    every v12 zero-argument getter, from ${previousAbiDir(ctx)}`);
  const readings = surveyStack(ctx, blockNumber);
  say(`    ${String(Object.keys(readings).length)} getter readings captured at block ${String(blockNumber)}`);

  // Every live proxy, including the two the upgrade must not touch: verify needs their "before" to say
  // "unchanged", and a proxy nobody meant to re-point is exactly the one nobody thought to snapshot.
  const implementation: Record<string, string> = {};
  for (const role of [...LIVE_UPGRADED_ROLES, ...UNTOUCHED_ROLES]) {
    const value = implementationSlot(ctx, ctx.opt.existing[role] ?? '', blockNumber);
    if (value === null) fail(`Error: could not snapshot the implementation slot of ${role}.`);
    implementation[role] = value;
  }

  return {
    blockNumber,
    handles: recordHandleValues(ctx, blockNumber),
    readings,
    admin,
    kmsSigners,
    kmsThreshold: safeNumber(kmsThreshold, 'KMS threshold'),
    coprocessorSigners,
    coprocessorThreshold: safeNumber(coprocessorThreshold, 'coprocessor threshold'),
    migration: resolveMigration(ctx, kmsContextId, kmsSigners, kmsThreshold),
    implementation,
  };
}

function preUpgradePath(ctx: Ctx): string {
  return join(ctx.outDir, 'pre-upgrade.json');
}

function writePreUpgrade(ctx: Ctx, snapshot: PreUpgradeSnapshot): void {
  writeFileSync(preUpgradePath(ctx), `${JSON.stringify(snapshot, null, 2)}\n`);
}

function writeInitialAddresses(ctx: Ctx): void {
  const address = (role: string): string => ctx.opt.existing[role] ?? '';
  const constants: ReadonlyArray<readonly [string, string]> = [
    ['ACL_ADDRESS', address('ACL_ADDRESS')],
    ['FHEVM_EXECUTOR_ADDRESS', address('FHEVM_EXECUTOR_ADDRESS')],
    ['KMS_VERIFIER_ADDRESS', address('KMS_VERIFIER_ADDRESS')],
    ['INPUT_VERIFIER_ADDRESS', address('INPUT_VERIFIER_ADDRESS')],
    ['HCU_LIMIT_ADDRESS', address('HCU_LIMIT_ADDRESS')],
    ['PROTOCOL_CONFIG_ADDRESS', '0x00000000000000000000000000000000dead1000'],
    ['KMS_GENERATION_ADDRESS', '0x00000000000000000000000000000000dead1001'],
    ['CLEARTEXT_ARITHMETIC_ADDRESS', address('CLEARTEXT_ARITHMETIC_ADDRESS')],
    ['CLEARTEXT_DB_ADDRESS', address('CLEARTEXT_DB_ADDRESS')],
    ['PAUSER_SET_ADDRESS', address('PAUSER_SET_ADDRESS')],
  ];
  const source = [
    '// SPDX-License-Identifier: BSD-3-Clause-Clear',
    'pragma solidity ^0.8.24;',
    '// Initial v12 addresses plus markers for the two v13 proxies. Generated; do not edit.',
    ...constants.map(
      ([name, value]) => `address constant ${name} = address(uint160(0x00${value.toLowerCase().slice(2)}));`,
    ),
    '',
  ].join('\n');
  writeFileSync(join(ctx.outDir, 'addresses.sol'), source);
}

function mergePreUpgradeIntoManifest(ctx: Ctx): void {
  const manifest = readJson<Record<string, unknown>>(manifestPath(ctx));
  const snapshot = readJson<PreUpgradeSnapshot>(preUpgradePath(ctx));
  if (manifest === null) fail(`Error: compute did not seal ${manifestPath(ctx)}.`);
  if (snapshot === null) fail(`Error: pre-upgrade snapshot missing at ${preUpgradePath(ctx)}.`);
  writeFileSync(manifestPath(ctx), `${JSON.stringify({ ...manifest, preUpgrade: snapshot }, null, 2)}\n`);
}

/** The live stack, passed to the forge scripts as FHEVM_EXISTING_<ROLE>. */
function existingEnv(ctx: Ctx): NodeJS.ProcessEnv {
  const env: NodeJS.ProcessEnv = {};
  for (const role of EXISTING_ROLES) {
    env[`FHEVM_EXISTING_${role}`] = ctx.opt.existing[role] ?? '';
  }
  return env;
}

function checkExistingManifestIdentity(ctx: Ctx): void {
  const manifest = readJson<Manifest>(manifestPath(ctx));
  if (manifest?.address === undefined) return;
  for (const role of EXISTING_ROLES) {
    const sealed = manifest.address[role];
    const supplied = ctx.opt.existing[role];
    if (sealed === undefined || supplied?.toLowerCase() !== sealed.toLowerCase()) {
      fail(
        `Error: ${role} differs from the live stack sealed at compute.`,
        `         sealed:   ${sealed ?? '(missing)'}`,
        `         supplied: ${supplied ?? '(missing)'}`,
        '       Use the original upgrade config; a different live stack needs its own out-dir.',
      );
    }
  }
}

function sealedMigrationEnv(ctx: Ctx): NodeJS.ProcessEnv {
  const manifest = readJson<{ readonly preUpgrade?: PreUpgradeSnapshot }>(manifestPath(ctx));
  const snapshot = manifest?.preUpgrade;
  if (snapshot === undefined) fail('Error: manifest has no sealed preUpgrade snapshot; run compute again.');
  const env: NodeJS.ProcessEnv = {
    FHEVM_MIGRATION_CONTEXT_ID: snapshot.migration.existingContextId,
    FHEVM_MIGRATION_NODE_COUNT: String(snapshot.migration.existingKmsNodes.length),
    FHEVM_MIGRATION_PUBLIC_DECRYPTION_THRESHOLD: snapshot.migration.existingThresholds.publicDecryption,
    FHEVM_MIGRATION_USER_DECRYPTION_THRESHOLD: snapshot.migration.existingThresholds.userDecryption,
    FHEVM_MIGRATION_KMS_GEN_THRESHOLD: snapshot.migration.existingThresholds.kmsGen,
    FHEVM_MIGRATION_MPC_THRESHOLD: snapshot.migration.existingThresholds.mpc,
  };
  snapshot.migration.existingKmsNodes.forEach((node, i) => {
    env[`FHEVM_MIGRATION_NODE_${String(i)}_TX_SENDER`] = node.txSenderAddress;
    env[`FHEVM_MIGRATION_NODE_${String(i)}_SIGNER`] = node.signerAddress;
    env[`FHEVM_MIGRATION_NODE_${String(i)}_IP`] = node.ipAddress;
    env[`FHEVM_MIGRATION_NODE_${String(i)}_STORAGE`] = node.storageUrl;
  });
  return env;
}

const MAY_CHANGE = new Set([
  'ACL.getVersion',
  'FHEVMExecutor.getVersion',
  'KMSVerifier.getVersion',
  'HCULimit.getVersion',
  'CleartextArithmetic.getVersion',
  'HCULimit.getBlockMeter',
]);

function verifySurveyAndHandles(ctx: Ctx): void {
  const manifest = readJson<{ readonly preUpgrade?: PreUpgradeSnapshot }>(manifestPath(ctx));
  const before = manifest?.preUpgrade;
  if (before === undefined) fail('Error: manifest has no pre-upgrade snapshot.');
  const after = surveyStack(ctx);

  const vanished = Object.keys(before.readings).filter((key) => !(key in after));
  if (vanished.length > 0)
    fail('Error: getters vanished after the upgrade:', ...vanished.map((key) => `       ${key}`));

  const changed: string[] = [];
  for (const [key, was] of Object.entries(before.readings)) {
    const now = after[key];
    if (!MAY_CHANGE.has(key) && now !== was) changed.push(`${key}: ${was} -> ${String(now)}`);
  }
  if (changed.length > 0)
    fail('Error: readable values changed outside the allow-list:', ...changed.map((v) => `       ${v}`));

  const unused = [...MAY_CHANGE].filter((key) => key in before.readings && before.readings[key] === after[key]);
  if (unused.length > 0)
    fail('Error: survey allow-list entries did not change:', ...unused.map((key) => `       ${key}`));
  say(`    ok   ${String(Object.keys(before.readings).length)} v12 getter readings survived`);

  const db = ctx.opt.existing.CLEARTEXT_DB_ADDRESS ?? '';
  for (const [handle, was] of Object.entries(before.handles)) {
    const result = capture('cast', ['call', db, 'get(bytes32)(uint256)', handle, '--rpc-url', ctx.opt.rpcUrl]);
    if (!result.ok || result.stdout !== was)
      fail(
        `Error: cleartext handle changed: ${handle}`,
        `         before: ${was}`,
        `         after:  ${result.stdout}`,
      );
  }
  if (Object.keys(before.handles).length > 0) {
    say(`    ok   ${String(Object.keys(before.handles).length)} pre-upgrade cleartext handle(s) survived`);
  } else {
    warn('no pre-upgrade handle was sealed; existing CleartextDB data survival was not proven.');
  }
}

type Log = {
  readonly address: string;
  readonly topics: readonly string[];
  readonly blockNumber: string;
  readonly transactionHash: string;
};

/** Blocks per `eth_getLogs` call. Public RPCs cap the range, and compute-to-verify can span days. */
const LOG_WINDOW = 2000;

/** Every `signature` event `address` emitted from `fromBlock` to the head, fetched in RPC-sized windows. */
function scanLogs(ctx: Ctx, address: string, signature: string, fromBlock: number): Log[] {
  const head = headBlock(ctx);
  const logs: Log[] = [];
  for (let from = fromBlock; from <= head; from += LOG_WINDOW) {
    const to = Math.min(from + LOG_WINDOW - 1, head);
    const r = capture('cast', [
      'logs',
      '--json',
      '--from-block',
      String(from),
      '--to-block',
      String(to),
      '--address',
      address,
      signature,
      '--rpc-url',
      ctx.opt.rpcUrl,
    ]);
    if (!r.ok) {
      fail(`Error: event scan failed for ${signature} on ${address} (blocks ${String(from)}-${String(to)}).`, r.stderr);
    }
    logs.push(...(JSON.parse(r.stdout === '' ? '[]' : r.stdout) as Log[]));
  }
  return logs;
}

/** The address an indexed `address` topic carries. */
function topicAddress(topic: string | undefined): string {
  return `0x${(topic ?? '').slice(-40)}`.toLowerCase();
}

function sealedManifest(ctx: Ctx): {
  readonly address: Readonly<Record<string, string>>;
  readonly preUpgrade: PreUpgradeSnapshot;
} {
  const manifest = readJson<Manifest & { readonly preUpgrade?: PreUpgradeSnapshot }>(manifestPath(ctx));
  if (manifest?.address === undefined || manifest.preUpgrade === undefined) {
    fail('Error: manifest has no sealed addresses and pre-upgrade snapshot; run compute first.');
  }
  return { address: manifest.address, preUpgrade: manifest.preUpgrade };
}

/**
 * The upgrade's own events: the path between the two endpoints was straight.
 *
 * Slot values prove where each proxy ENDED UP. They cannot see an intermediate implementation that ran
 * for a block and was then replaced by the sealed one — code with full storage access that the endpoint
 * checks would never know about. The logs can: exactly one `Upgraded` per re-pointed proxy, to the sealed
 * implementation, all seven in ONE transaction; none at all on the two proxies the op list must not
 * touch; and seven `HostUpgraded` on the ACLOwner naming the same (proxy, implementation) pairs in op
 * order.
 *
 * That one transaction is also the materialize receipt when the admin is a multisig and no local record
 * exists — so it is written to the journal as an observation, once.
 */
function verifyUpgradeEvents(ctx: Ctx): { readonly hash: string; readonly block: number | null } {
  const { address, preUpgrade } = sealedManifest(ctx);
  const fromBlock = preUpgrade.blockNumber + 1;
  const at = (role: string): string => (address[role] ?? '').toLowerCase();

  const hashes = new Set<string>();
  let block: number | null = null;
  for (const role of UPGRADED_ROLES) {
    // A proxy created by this upgrade emits `Upgraded` once at construction, to the empty implementation,
    // and once at materialize. A live proxy only at materialize. Both histories are pinned in full.
    const isNew = !LIVE_UPGRADED_ROLES.includes(role);
    const expected = isNew ? [at('IMPL_EMPTY_UUPS_PROXY'), at(`IMPL_${role}`)] : [at(`IMPL_${role}`)];
    const logs = scanLogs(ctx, at(role), 'Upgraded(address)', fromBlock);
    if (logs.length !== expected.length) {
      fail(
        `Error: ${role} emitted ${String(logs.length)} Upgraded event(s) since the seal; exactly ${String(expected.length)} expected.`,
      );
    }
    logs.forEach((log, i) => {
      const implementation = topicAddress(log.topics[1]);
      if (implementation !== expected[i]) {
        fail(
          `Error: ${role} Upgraded[${String(i)}] points at ${implementation}, not the sealed ${expected[i] ?? '?'}.`,
        );
      }
    });
    const last = logs[logs.length - 1];
    if (last === undefined) fail(`Error: ${role} has no Upgraded event.`);
    hashes.add(last.transactionHash);
    block = hexToNumber(last.blockNumber);
  }
  for (const role of UNTOUCHED_ROLES) {
    if (scanLogs(ctx, at(role), 'Upgraded(address)', fromBlock).length !== 0) {
      fail(`Error: ${role} emitted an Upgraded event; the upgrade must not touch it.`);
    }
  }
  const [hash] = [...hashes];
  if (hashes.size !== 1 || hash === undefined) {
    fail('Error: the seven Upgraded events span more than one transaction; the upgrade was not atomic.');
  }

  const host = scanLogs(ctx, at('ACL_OWNER'), 'HostUpgraded(address,address)', fromBlock);
  if (host.length !== UPGRADED_ROLES.length) {
    fail(
      `Error: ACLOwner emitted ${String(host.length)} HostUpgraded events; exactly ${String(UPGRADED_ROLES.length)} are expected.`,
    );
  }
  host.forEach((log, i) => {
    const role = UPGRADED_ROLES[i] ?? '';
    const matches =
      log.transactionHash === hash &&
      topicAddress(log.topics[1]) === at(role) &&
      topicAddress(log.topics[2]) === at(`IMPL_${role}`);
    if (!matches) fail(`Error: HostUpgraded[${String(i)}] does not match the sealed op for ${role}.`);
  });

  say(`    ok   one atomic ACLOwner.upgrade, ${String(UPGRADED_ROLES.length)} ops: ${hash} (block ${String(block)})`);
  recordExternalMaterialize(ctx, hash, block);
  return { hash, block };
}

/** A materialize this tooling did not send has no local receipt; the block it landed in is still worth a line. */
function recordExternalMaterialize(ctx: Ctx, hash: string, block: number | null): void {
  const known = readJournal(ctx).some((row) => row.hash === hash || row.note?.includes(hash) === true);
  if (known) return;
  recordObservation(ctx, 'D', `ACLOwner.upgrade sent externally: ${hash}`, block ?? 0);
}

function verifyNoAuthorityEvents(ctx: Ctx): void {
  const { address, preUpgrade } = sealedManifest(ctx);
  const fromBlock = preUpgrade.blockNumber + 1;
  const scans: ReadonlyArray<readonly [string, string, string]> = [
    ['ACL ownership', address.ACL_ADDRESS ?? '', 'OwnershipTransferStarted(address,address)'],
    ['ACL ownership', address.ACL_ADDRESS ?? '', 'OwnershipTransferred(address,address)'],
    ['ACLOwner ownership', address.ACL_OWNER ?? '', 'OwnershipTransferStarted(address,address)'],
    ['ACLOwner ownership', address.ACL_OWNER ?? '', 'OwnershipTransferred(address,address)'],
    ['PauserSet', address.PAUSER_SET_ADDRESS ?? '', 'AddPauser(address)'],
    ['PauserSet', address.PAUSER_SET_ADDRESS ?? '', 'RemovePauser(address)'],
    ['PauserSet', address.PAUSER_SET_ADDRESS ?? '', 'SwapPauser(address,address)'],
  ];
  for (const [label, target, signature] of scans) {
    if (scanLogs(ctx, target, signature, fromBlock).length > 0) {
      fail(`Error: forbidden ${label} event emitted during upgrade: ${signature}`);
    }
  }
  say('    ok   no ownership or pauser event was emitted during the upgrade');
}

/**
 * The verify's own record: at which block it passed, how deep that block was, and which transaction it
 * verified. `verify` is meant to be run again later at greater depth; this is how two runs are compared.
 */
function writeVerifyReport(
  ctx: Ctx,
  materialize: { readonly hash: string; readonly block: number | null },
  buildOut: string,
): void {
  const report = {
    deploymentId: ctx.opt.deploymentId,
    chainId: ctx.chainId,
    verifiedAt: new Date().toISOString(),
    verifiedAtBlock: headBlock(ctx),
    finalizedBlock: ctx.useFinality ? finalizedBlock(ctx) : null,
    materializeTx: materialize.hash,
    materializeBlock: materialize.block,
    buildOut,
  };
  const path = join(ctx.outDir, 'verify-report.json');
  writeFileSync(path, `${JSON.stringify(report, null, 2)}\n`);
  say(`    wrote ${path}`);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * A fresh `forge --out` for the read-only checks.
 *
 * Checking against the build that sealed the manifest compares the seal with itself. A recompile into an
 * empty directory makes the current checkout re-derive every init-code hash — the independent recompile
 * an auditor would do — for the price of a few seconds of solc.
 */
function checkBuildOut(ctx: Ctx): string {
  if (ctx.opt.noBuild) {
    say('  (--no-build: checking against the sealed build, not a fresh recompile)');
    return ctx.buildOut;
  }
  const dir = join(ctx.outDir, 'build-check');
  removeIfPresent(dir);
  return dir;
}

/** One read-only forge script: simulated against the head, nothing signed, nothing sent. */
async function runReadOnly(
  ctx: Ctx,
  file: string,
  contract: string,
  out: string,
  env: NodeJS.ProcessEnv,
): Promise<boolean> {
  const script = requireScript(file);
  const args = ['script', `${script}:${contract}`, '--out', out, '--rpc-url', ctx.opt.rpcUrl, ...traceArgs(ctx)];
  return (await runLogged('forge', args, env)) === 0;
}

////////////////////////////////////////////////////////////////////////////////

async function stageCreates(ctx: Ctx): Promise<void> {
  say('🥩 creates (one CREATE2 per create, each gated on getCode)');
  ctx.stageLabel = 'creates';
  requireScript('FhevmUpgradeCreates.s.sol');
  await broadcast(ctx, 'FhevmUpgradeCreates.s.sol:FhevmUpgradeCreates', undefined, undefined, existingEnv(ctx));
}

/**
 * The gate before the point of no return. Read-only; reports everything rather than stopping at the first
 * failure, so the operator sees the whole picture before the one transaction that cannot be retried.
 */
async function stagePrecheck(ctx: Ctx): Promise<void> {
  say('🚧  precheck — everything that must hold before the atomic upgrade');
  const env = { ...scriptEnv(ctx, existingEnv(ctx)), ...generatedConfigEnv(ctx), ...sealedMigrationEnv(ctx) };
  await step(ctx, 'precheck', 'pre-materialize check, from a fresh recompile', async () => {
    const ok = await runReadOnly(
      ctx,
      'FhevmPreMaterializeCheck.s.sol',
      'FhevmPreMaterializeCheck',
      checkBuildOut(ctx),
      env,
    );
    if (!ok) {
      fail('Error: the pre-materialize check failed. Nothing was sent; read the FAIL lines above before retrying.');
    }
  });
}

/**
 * The exact `ACLOwner.upgrade` payload, written by the materialize script itself in prepare-only mode, so
 * what a multisig signs and what a rehearsal applies is byte-identical to what a key here would send.
 */
async function prepareCalldata(ctx: Ctx): Promise<{ readonly calldata: string; readonly path: string }> {
  const env = {
    ...scriptEnv(ctx, existingEnv(ctx)),
    ...generatedConfigEnv(ctx),
    ...sealedMigrationEnv(ctx),
    FHEVM_MIN_BLOCK: String(ctx.opt.minBlockOverride ?? 0),
    FHEVM_PREPARE_ONLY: 'true',
  };
  const code = await runLogged(
    'forge',
    [
      'script',
      `${SCRIPT_DIR}/FhevmMaterializeUpgrade.s.sol:FhevmMaterializeUpgrade`,
      '--rpc-url',
      ctx.opt.rpcUrl,
      '--out',
      ctx.buildOut,
      '--sender',
      ctx.opt.admin,
      ...traceArgs(ctx),
    ],
    env,
  );
  if (code !== 0) fail('Error: could not prepare the ACLOwner.upgrade payload.');
  const path = join(ctx.outDir, 'materialize-calldata.txt');
  return { calldata: readFileSync(path, 'utf8').trim(), path };
}

/**
 * The upgrade, for real, on a chain that does not matter.
 *
 * Everything before this point asks the chain and the build questions. This asks the only one that
 * matters and that no static check can answer — does the exact payload, applied to the exact live state,
 * leave a stack that passes the exact verify — and asks it where a wrong answer costs nothing: an anvil
 * forked from the live node at its head, with the admin impersonated. The verify that runs against the
 * fork is the same code, byte for byte, that will run against the live chain afterwards.
 *
 * What the fork keeps separate: its own out dir under `rehearsal/`, so the rehearsal's journal line and
 * `verify-report.json` can never be mistaken for the real ones. The manifest and generated config are
 * copied in, not moved — the seal stays where it is.
 */
async function stageRehearse(ctx: Ctx): Promise<void> {
  say('🎭  rehearse — the upgrade on a fork of this chain');
  requireTool('anvil');
  if (!ctx.opt.dryRun) await waitForCreatesSettled(ctx);
  await stagePrecheck(ctx);
  if (isMaterialized(ctx)) {
    say('  materialize already complete — nothing left to rehearse; run verify.');
    return;
  }
  const { calldata } = await step(ctx, 'rehearse', 'prepare the exact ACLOwner.upgrade payload', () =>
    prepareCalldata(ctx),
  );

  const forkDir = join(ctx.outDir, 'rehearsal');
  removeIfPresent(forkDir);
  ensureDir(forkDir);
  copyFileSync(manifestPath(ctx), join(forkDir, 'manifest.json'));
  copyFileSync(join(ctx.outDir, 'addresses.sol'), join(forkDir, 'addresses.sol'));

  const port = await freePort();
  const forkUrl = `http://127.0.0.1:${String(port)}`;
  const forkedAt = headBlock(ctx);
  say(`  ▸ fork ${ctx.opt.rpcUrl} at block ${String(forkedAt)}`);
  const anvil = spawnBackground('anvil', [
    '--fork-url',
    ctx.opt.rpcUrl,
    '--fork-block-number',
    String(forkedAt),
    '--port',
    String(port),
    '--host',
    '127.0.0.1',
    '--auto-impersonate',
    '--silent',
  ]);
  try {
    await waitUntil(() => capture('cast', ['chain-id', '--rpc-url', forkUrl]).ok, 60_000, `anvil fork on ${forkUrl}`);
    // A fork mines only when sent to, so a depth wait there would never end: on the fork, settled is head.
    const fork: Ctx = {
      ...ctx,
      opt: { ...ctx.opt, rpcUrl: forkUrl, minBlockOverride: null, confirmations: 0 },
      outDir: forkDir,
      buildOut: join(forkDir, 'build'),
      broadcastDir: join(forkDir, 'broadcast'),
      journalPath: join(forkDir, 'journal.jsonl'),
      useFinality: false,
      nextMinBlock: 0,
      finalityTarget: 0,
    };
    say(`  ✔ forked ${ctx.opt.rpcUrl} at block ${String(forkedAt)} onto ${forkUrl}`);

    // A multisig admin may hold no ETH; on the fork it is given some, and its signature is not needed.
    await step(ctx, 'rehearse', 'apply the payload on the fork as the impersonated admin', () => {
      const admin = ctx.opt.admin;
      const target = ctx.opt.existing.ACL_OWNER ?? '';
      captureOrFail('cast', ['rpc', 'anvil_setBalance', admin, '0xDE0B6B3A7640000', '--rpc-url', forkUrl]);
      const sent = capture('cast', [
        'send',
        '--json',
        '--unlocked',
        '--from',
        admin,
        target,
        calldata,
        '--rpc-url',
        forkUrl,
      ]);
      if (!sent.ok) fail('Error: ACLOwner.upgrade REVERTED on the fork. The live chain is untouched.', sent.stderr);
      const receipt = JSON.parse(sent.stdout) as { readonly status?: string; readonly transactionHash?: string };
      if (receipt.status !== '0x1') {
        fail(`Error: ACLOwner.upgrade failed on the fork (status ${receipt.status ?? '?'}).`);
      }
      say(`    applied ACLOwner.upgrade on the fork: ${receipt.transactionHash ?? '?'}`);
    });

    await step(ctx, 'rehearse', 'verify against the fork', () => stageVerify(fork));
    say(
      '',
      `  REHEARSAL PASSED at block ${String(forkedAt)}: the exact payload leaves a stack that passes verify.`,
      `  Nothing was sent to ${ctx.opt.rpcUrl}. Records: ${forkDir}`,
    );
  } finally {
    if (anvil.exitCode === null && anvil.signalCode === null) anvil.kill('SIGTERM');
  }
}

async function stageMaterialize(ctx: Ctx): Promise<void> {
  say('🧩  materialize — one atomic ACLOwner.upgrade');
  ctx.stageLabel = 'D';
  requireScript('FhevmMaterializeUpgrade.s.sol');

  // The gate runs at the depth the transaction will be sent at, and always — a manual `--stage
  // materialize` gets it too. A dry run never waits, by its own rule.
  if (!ctx.opt.dryRun) await waitForCreatesSettled(ctx);
  await stagePrecheck(ctx);

  if (ctx.opt.adminSigner === null) {
    const { calldata, path: calldataPath } = await step(ctx, 'D', 'write the external ACLOwner.upgrade payload', () =>
      prepareCalldata(ctx),
    );
    say(
      '  external admin action required:',
      `    target   ${ctx.opt.existing.ACL_OWNER ?? ''}`,
      '    value    0',
      `    calldata ${calldata}`,
      `    keccak   ${captureOrFail('cast', ['keccak', calldata])}  <- must equal the digest precheck printed`,
      `    saved    ${calldataPath}`,
    );
    if (ctx.opt.stage === 'all') {
      fail('Error: --stage all stops until the contract/multisig admin executes the payload above.');
    }
    return;
  }
  const adminSigner = ctx.opt.adminSigner;
  await step(ctx, 'D', 'broadcast ACLOwner.upgrade as the admin', () =>
    broadcast(ctx, 'FhevmMaterializeUpgrade.s.sol:FhevmMaterializeUpgrade', adminSigner, ctx.opt.admin, {
      ...existingEnv(ctx),
      ...sealedMigrationEnv(ctx),
    }),
  );
}

/**
 * The terminal conditions, in three layers that answer three different questions:
 *
 *   Solidity   did the intended changes happen — from a fresh recompile, against the seal
 *   survey     did anything ELSE change — every v12 getter, and the sealed handles
 *   events     was the path between the two endpoints straight — one atomic upgrade, no authority moved
 *
 * Waits for the materialize block to be buried and finalized first: a verdict read off a block about to be
 * orphaned is not a verdict. Run it again later at greater depth; `verify-report.json` records each run.
 */
async function stageVerify(ctx: Ctx): Promise<void> {
  say('✅  verify');
  await waitForBlock(ctx, ctx.opt.minBlockOverride ?? ctx.nextMinBlock);
  await waitForMaterializeSettled(ctx);
  const out = checkBuildOut(ctx);
  const env = { ...scriptEnv(ctx, existingEnv(ctx)), ...generatedConfigEnv(ctx) };
  await step(ctx, 'verify', 'terminal conditions in Solidity, from a fresh recompile', async () => {
    if (!(await runReadOnly(ctx, 'FhevmVerifyUpgrade.s.sol', 'FhevmVerifyUpgrade', out, env))) {
      fail('Error: verify failed.');
    }
  });
  await step(ctx, 'verify', 'survey: every v12 getter and every sealed handle', () => {
    verifySurveyAndHandles(ctx);
  });
  const materialize = await step(ctx, 'verify', 'events: one atomic upgrade, nothing else', () => {
    const found = verifyUpgradeEvents(ctx);
    verifyNoAuthorityEvents(ctx);
    return found;
  });
  writeVerifyReport(ctx, materialize, out);
}

////////////////////////////////////////////////////////////////////////////////
// Settlement — every wait derives from the chain, none from the journal
////////////////////////////////////////////////////////////////////////////////

/** The ERC-1967 implementation of `proxy`, lower-cased, at `block` or the head; null if unreadable. */
function implementationSlot(ctx: Ctx, proxy: string, block?: number): string | null {
  const raw = capture('cast', [
    'storage',
    proxy,
    ERC1967_IMPLEMENTATION_SLOT,
    '--rpc-url',
    ctx.opt.rpcUrl,
    ...atBlock(block),
  ]);
  const word = /0x[0-9a-fA-F]{64}/.exec(raw.stdout)?.[0];
  return !raw.ok || word === undefined ? null : `0x${word.slice(-40)}`.toLowerCase();
}

/** Every op target already at its sealed implementation, read at the head. */
function isMaterialized(ctx: Ctx): boolean {
  const { address } = sealedManifest(ctx);
  return UPGRADED_ROLES.every(
    (role) => implementationSlot(ctx, address[role] ?? '') === (address[`IMPL_${role}`] ?? '').toLowerCase(),
  );
}

/**
 * Block until every sealed create holds code at the settled block.
 *
 * The chain-side reorg gate for everything that reads the creates and then decides — precheck, rehearse,
 * materialize. It replaces "wait N blocks after the stage that sent them", which only an uninterrupted
 * `all` can know; this is answered from the chain by any invocation, days later, with no journal.
 */
async function waitForCreatesSettled(ctx: Ctx): Promise<void> {
  const { address } = sealedManifest(ctx);
  const at = (role: string): string => address[role] ?? '';
  const head = headBlock(ctx);
  const absent = CREATE_ROLES.filter((role) => !hasCodeAt(ctx, at(role), head));
  if (absent.length > 0) {
    fail(
      `Error: ${String(absent.length)} of ${String(CREATE_ROLES.length)} creates have no code at block ${String(head)}:`,
      ...absent.map((role) => `         ${role}`),
      '       Run creates first.',
    );
  }
  for (;;) {
    const settled = settledBlock(ctx);
    const pending = CREATE_ROLES.filter((role) => !hasCodeAt(ctx, at(role), settled));
    if (pending.length === 0) {
      say(`  every create is settled at block ${String(settled)} (${settlementRule(ctx)})`);
      return;
    }
    say(
      `  waiting: ${String(pending.length)} create(s) not yet settled at block ${String(settled)} (${settlementRule(ctx)})`,
    );
    await sleep(12_000);
  }
}

/**
 * Block until the materialize transaction is settled, and return its block.
 *
 * Found through `HostUpgraded` on the ACLOwner rather than through a receipt, so a multisig's transaction
 * and a run whose journal was lost are handled the same way as the happy path.
 */
async function waitForMaterializeSettled(ctx: Ctx): Promise<number> {
  const { address, preUpgrade } = sealedManifest(ctx);
  const logs = scanLogs(ctx, address.ACL_OWNER ?? '', 'HostUpgraded(address,address)', preUpgrade.blockNumber + 1);
  const block = Math.max(0, ...logs.map((log) => hexToNumber(log.blockNumber) ?? 0));
  if (logs.length === 0 || block === 0) {
    fail(
      'Error: no HostUpgraded event since the seal: the upgrade has not been materialized, or its',
      '       transaction was reorged out. --stage status says which; --stage materialize is safe to re-run.',
    );
  }
  for (;;) {
    const settled = settledBlock(ctx);
    if (settled >= block) {
      say(
        `  the materialize block ${String(block)} is settled (${settlementRule(ctx)}, settled block ${String(settled)})`,
      );
      return block;
    }
    say(`  waiting for block ${String(block)} to settle (${settlementRule(ctx)}, settled block ${String(settled)})`);
    await sleep(12_000);
  }
}

function settlementRule(ctx: Ctx): string {
  return ctx.useFinality ? 'finalized' : `${String(ctx.opt.confirmations)} blocks deep`;
}

////////////////////////////////////////////////////////////////////////////////
// Progress — the ledger of every step, and the offline view of it
////////////////////////////////////////////////////////////////////////////////

type ProgressEntry = {
  readonly ts: string;
  readonly stage: string;
  readonly step: string;
  readonly status: 'start' | 'ok' | 'fail';
  readonly block?: number | null;
  readonly ms?: number;
};

function progressPath(ctx: Ctx): string {
  return join(ctx.outDir, 'progress.jsonl');
}

/** The step in flight, so a `fail()` — which exits — still leaves a `fail` line behind it. */
let openStep: { readonly ctx: Ctx; readonly stage: string; readonly step: string; readonly startedAt: number } | null =
  null;

process.on('exit', (code) => {
  if (openStep === null || code === 0) return;
  const { ctx, stage, step: name, startedAt } = openStep;
  appendJsonl(progressPath(ctx), [
    { ts: new Date().toISOString(), stage, step: name, status: 'fail', ms: Date.now() - startedAt },
  ]);
});

/**
 * One step of a stage: announced, timed, recorded.
 *
 * The ledger is an audit trail like the journal, and like the journal nothing reads it to decide anything.
 * What it adds is the intermediate checks, which send no transaction and so leave no other trace.
 */
async function step<T>(ctx: Ctx, stage: string, name: string, body: () => T | Promise<T>): Promise<T> {
  const startedAt = Date.now();
  ensureDir(ctx.outDir);
  appendJsonl(progressPath(ctx), [{ ts: new Date().toISOString(), stage, step: name, status: 'start' }]);
  openStep = { ctx, stage, step: name, startedAt };
  say(`  ▸ ${name}`);
  const result = await body();
  const ms = Date.now() - startedAt;
  const block = needsChain(stage) ? headBlock(ctx) : null;
  appendJsonl(progressPath(ctx), [{ ts: new Date().toISOString(), stage, step: name, status: 'ok', ms, block }]);
  openStep = null;
  say(`  ✔ ${name}  (${(ms / 1000).toFixed(1)}s${block === null ? '' : `, block ${String(block)}`})`);
  return result;
}

/**
 * `--stage params`: the init parameters this upgrade sends, decoded.
 *
 * The one op with arguments is `ProtocolConfig.initializeFromMigration`; this is its payload. After
 * `compute` it is read from the seal. Before, it is resolved exactly as `compute` would resolve it — the
 * live KMSVerifier plus `--migration` or the package defaults — and printed as a preview, so an operator
 * can see what WOULD be sealed before anything is. `precheck` prints the same values at send time.
 */
function stageParams(ctx: Ctx): void {
  const manifest = readJson<{ readonly preUpgrade?: PreUpgradeSnapshot }>(manifestPath(ctx));
  const sealed = manifest?.preUpgrade?.migration;
  const migration = sealed ?? previewMigration(ctx);
  say(
    sealed === undefined
      ? `🧬  init parameters  ${ctx.opt.deploymentId}  PREVIEW: not sealed yet, resolved as compute would`
      : `🧬  init parameters  ${ctx.opt.deploymentId}  (${manifestPath(ctx)})`,
    '',
  );
  say('  ProtocolConfig.initializeFromMigration(existingContextId, existingKmsNodes, existingThresholds)', '');
  say(`  existingContextId  ${migration.existingContextId}`);
  migration.existingKmsNodes.forEach((node, i) => {
    say(
      `  existingKmsNodes[${String(i)}]`,
      `    signerAddress    ${node.signerAddress}   <- read off the live v12 KMSVerifier`,
      `    txSenderAddress  ${node.txSenderAddress}   <- new in v13: --migration, or package default`,
      `    ipAddress        ${node.ipAddress}`,
      `    storageUrl       ${node.storageUrl}`,
    );
  });
  const t = migration.existingThresholds;
  say(
    '  existingThresholds',
    `    publicDecryption ${t.publicDecryption}`,
    `    userDecryption   ${t.userDecryption}`,
    `    kmsGen           ${t.kmsGen}`,
    `    mpc              ${t.mpc}`,
    '',
    '  the six other ops take no arguments (reinitializers / initializeFromEmptyProxy)',
  );
}

/** What `compute` would seal right now: the live KMSVerifier, plus the migration file or the defaults. */
function previewMigration(ctx: Ctx): SealedMigration {
  const kms = ctx.opt.existing.KMS_VERIFIER_ADDRESS ?? '';
  if (kms === '')
    fail('Error: KMS_VERIFIER_ADDRESS is not known - pass --previous-manifest, --kms-verifier or a config file.');
  const signers = callAddresses(ctx, kms, 'getKmsSigners()(address[])');
  const threshold = callUint(ctx, kms, 'getThreshold()(uint256)');
  const contextId = callUint(ctx, kms, 'getCurrentKmsContextId()(uint256)');
  if (signers === null || threshold === null || contextId === null) {
    fail(`Error: ${kms} did not answer as a live v12 KMSVerifier.`);
  }
  say(`  source  ${ctx.opt.migrationPath ?? 'package defaults (no --migration)'}`, `  live    ${kms} (KMSVerifier)`);
  return resolveMigration(ctx, contextId, signers, threshold);
}

/** `--stage progress`: the ledger, rendered. Reads local files only. */
function stageProgress(ctx: Ctx): void {
  const rows = readJsonl<ProgressEntry>(progressPath(ctx));
  if (rows.length === 0) {
    say(`No progress ledger at ${progressPath(ctx)} - nothing has run for this upgrade yet.`);
    return;
  }
  say(`📈  progress  (${progressPath(ctx)})`, '');
  say(`  ${pad('WHEN', 20)} ${pad('STAGE', 10)} ${pad('STATUS', 7)} ${pad('TOOK', 8)} ${pad('BLOCK', 9)} STEP`);
  const when = (row: ProgressEntry): string => pad(row.ts.slice(0, 19).replace('T', ' '), 20);
  const open = new Map<string, ProgressEntry>();
  for (const row of rows) {
    const key = `${row.stage}/${row.step}`;
    if (row.status === 'start') {
      open.set(key, row);
      continue;
    }
    open.delete(key);
    const took = row.ms === undefined ? '-' : `${(row.ms / 1000).toFixed(1)}s`;
    const block = row.block === null || row.block === undefined ? '-' : String(row.block);
    say(`  ${when(row)} ${pad(row.stage, 10)} ${pad(row.status, 7)} ${pad(took, 8)} ${pad(block, 9)} ${row.step}`);
  }
  for (const row of open.values()) {
    say(
      `  ${when(row)} ${pad(row.stage, 10)} ${pad('OPEN', 7)} ${pad('-', 8)} ${pad('-', 9)} ${row.step}  <- started, never finished`,
    );
  }
  say('', `  transcripts: ${join(ctx.outDir, 'logs')}`);
}

function stageStatus(ctx: Ctx): void {
  const manifest = readJson<Manifest & { readonly preUpgrade?: PreUpgradeSnapshot }>(manifestPath(ctx));
  if (manifest?.address === undefined || manifest.preUpgrade === undefined) {
    say(`no manifest at ${manifestPath(ctx)} — run compute first.`);
    return;
  }
  say('-'.repeat(RULE_WIDTH));
  say(`📋  status  ${ctx.opt.deploymentId} @ v0.13`);
  say('-'.repeat(RULE_WIDTH));

  const createRoles = CREATE_ROLES;
  let createsDone = 0;
  for (const role of createRoles) {
    const address = manifest.address[role] ?? '';
    const code = capture('cast', ['code', address, '--rpc-url', ctx.opt.rpcUrl]);
    const done = code.ok && code.stdout !== '' && code.stdout !== '0x';
    if (done) createsDone += 1;
    say(`  ${done ? 'done' : 'todo'}  ${pad(role, ROLE_WIDTH)} ${address}`);
  }
  say(`  creates: ${String(createsDone)}/${String(createRoles.length)}`);

  const upgradedRoles = UPGRADED_ROLES;
  const slotOf = (role: string): string | null => implementationSlot(ctx, manifest.address?.[role] ?? '');
  let complete = 0;
  let expectedPrevious = 0;
  let foreign = 0;
  for (const role of upgradedRoles) {
    const current = slotOf(role);
    if (current === null) {
      say(`  FATAL ${pad(role, ROLE_WIDTH)} implementation slot unreadable`);
      foreign += 1;
      continue;
    }
    const target = (manifest.address[`IMPL_${role}`] ?? '').toLowerCase();
    const previous =
      role === 'PROTOCOL_CONFIG_ADDRESS' || role === 'KMS_GENERATION_ADDRESS'
        ? (manifest.address.IMPL_EMPTY_UUPS_PROXY ?? '').toLowerCase()
        : (manifest.preUpgrade.implementation[role] ?? '').toLowerCase();
    if (current === target) complete += 1;
    else if (current === previous) expectedPrevious += 1;
    else {
      foreign += 1;
      say(`  FATAL ${pad(role, ROLE_WIDTH)} unsealed implementation ${current}`);
    }
  }
  if (foreign > 0) say('  materialize: FATAL — at least one proxy has a foreign implementation');
  else if (complete === upgradedRoles.length)
    say('  materialize: done — all seven implementation slots match the seal');
  else if (complete > 0) say(`  materialize: FATAL — partial atomic upgrade (${String(complete)}/7 slots match)`);
  else if (expectedPrevious === upgradedRoles.length && createsDone === createRoles.length) {
    say('  materialize: ready — all ten creates exist and all seven slots are at their sealed previous values');
  } else if (expectedPrevious === upgradedRoles.length) say('  materialize: BLOCKED — run creates first');
  else say('  materialize: FATAL — slot classification is inconsistent');

  // The two proxies outside the op list, which must still run what they ran at seal time.
  for (const role of UNTOUCHED_ROLES) {
    const current = slotOf(role);
    const sealed = (manifest.preUpgrade.implementation[role] ?? '').toLowerCase();
    if (current === sealed) say(`  ok    ${pad(role, ROLE_WIDTH)} untouched, still at its sealed implementation`);
    else
      say(
        `  FATAL ${pad(role, ROLE_WIDTH)} implementation moved to ${current ?? '(unreadable)'} - the upgrade must not touch it`,
      );
  }
}

////////////////////////////////////////////////////////////////////////////////

async function runStage(ctx: Ctx, stage: Stage): Promise<void> {
  switch (stage) {
    case 'compute':
      await stageCompute(ctx);
      return;
    case 'creates':
      await stageCreates(ctx);
      return;
    case 'precheck':
      await stagePrecheck(ctx);
      return;
    case 'rehearse':
      await stageRehearse(ctx);
      return;
    case 'materialize':
      await stageMaterialize(ctx);
      return;
    case 'verify':
      await stageVerify(ctx);
      return;
    case 'status':
      stageStatus(ctx);
      return;
    case 'log':
      showJournal(ctx);
      return;
    case 'report':
      stageReport(ctx);
      return;
    case 'progress':
      stageProgress(ctx);
      return;
    case 'params':
      stageParams(ctx);
      return;
    case 'all':
      fail("Error: 'all' is expanded by main, not run as a stage.");
  }
}

////////////////////////////////////////////////////////////////////////////////

/** This coordinator's half of the split with common.ts: the stages, the help, the two predicates. */
const UPGRADE_FLOW: Flow = {
  name: 'upgrade',
  help: HELP,
  defaultConfigName: DEFAULT_CONFIG_NAME,
  stages: ALL_STAGES,
  runOrder: RUN_ORDER,
  reportSteps: REPORT_STEPS,
  needsChain,
  needsDeployerKey,
};

////////////////////////////////////////////////////////////////////////////////

async function main(): Promise<void> {
  const cli = parseCliArgs(UPGRADE_FLOW, process.argv.slice(2));
  const { cfg, path: configPath } = loadConfigFile(UPGRADE_FLOW, cli.configPath);
  const opt = resolveOptions(UPGRADE_FLOW, cli, cfg, configPath);

  process.chdir(PACKAGE_ROOT);
  if (needsChain(opt.stage)) {
    requireTool('forge');
    requireTool('cast');
  }

  const ctx = buildContext(UPGRADE_FLOW, opt);
  if (needsChain(opt.stage)) {
    // Everything this invocation prints, forge's output included, kept beside the seal.
    const logs = join(ctx.outDir, 'logs');
    ensureDir(logs);
    startTranscript(join(logs, `${new Date().toISOString().replace(/[:.]/g, '-')}-${opt.stage}.log`));
    say(`📝  transcript: ${logs}`);
    preflight(ctx);
    checkExistingManifestIdentity(ctx);
  }

  if (opt.stage === 'all') {
    say('🗺   plan', ...RUN_ORDER.map((stage, i) => `    ${String(i + 1)}/${String(RUN_ORDER.length)}  ${stage}`), '');
    // Same rule as the deploy: `compute` alone is a hard error once anything was sent, but inside `all`
    // it is incidental, so a sealed upgrade past its first transaction skips it. Otherwise `--stage all`
    // could never resume the upgrade it started. Asked of the chain, so a run killed before it wrote its
    // journal still resumes rather than resealing.
    if (upgradeStarted(ctx)) {
      say('🍟 compute already sealed and past its first transaction - skipping (resume)');
    } else {
      await stageCompute(ctx);
    }
    for (const stage of RUN_ORDER.filter((s) => s !== 'compute')) {
      await runStage(ctx, stage);
    }
    say('done (all)');
    return;
  }

  await runStage(ctx, opt.stage as Stage);
  say(`done (${opt.stage})`);
}

await main();
