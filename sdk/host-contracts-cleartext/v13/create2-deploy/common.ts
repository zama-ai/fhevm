// Shared machinery for the two CREATE2 coordinators, `deploy-testnet.ts` and `upgrade-testnet.ts`.
//
// The split is by WHAT VARIES, not by size. Both flows want the same everything-except-the-stages:
// argument parsing, the config file, the out-dir identity check, the chain and factory preflight, signer
// resolution, the reorg/finality waits, the journal, the seal gate, and the one `forge script` invocation
// that broadcasts a stage. What differs is the stage list, the help text, and which stages need a chain
// or a key — so those seven things arrive as a `Flow` descriptor and everything else lives here.
//
// Why not a base class or a single coordinator with a `--mode` flag: the two flows have genuinely
// different preconditions. A deploy asserts nothing exists yet; an upgrade asserts a specific stack
// already does, and must never touch its ownership or its pausers. Sharing the plumbing while keeping the
// stage tables apart is what keeps each one's preconditions readable in one place.
//
// Distinct from `utils.ts`, which is dependency-free and knows nothing about this deploy at all.

import { existsSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';

import {
  appendJsonl,
  capture,
  captureOrFail,
  confirm,
  fail,
  hexToNumber,
  isInside,
  pad,
  readJson,
  readJsonl,
  run,
  sameAddress,
  say,
  sleep,
  warn,
} from './utils.ts';

////////////////////////////////////////////////////////////////////////////////

/**
 * A stage name. Left as a plain string here because the two flows have different stage sets; each
 * coordinator declares its own union and passes the list in via `Flow`.
 */
export type Stage = string;

/**
 * Everything that differs between the deploy and the upgrade.
 *
 * Seven fields, which is the honest measure of how much the two flows actually diverge: the rest of this
 * file is identical for both. Adding a flow means writing one of these plus its stage functions.
 */
export type Flow = {
  /** For messages and the journal, e.g. `deploy` or `upgrade`. */
  readonly name: string;
  /** `--help` output. */
  readonly help: string;
  /** Auto-discovered config file name, e.g. `deploy.config.json`. */
  readonly defaultConfigName: string;
  /** Every accepted `--stage` value, including the pseudo-stages and `all`. */
  readonly stages: readonly Stage[];
  /** What `--stage all` runs, in order. */
  readonly runOrder: readonly Stage[];
  /** The steps `--report` tabulates. */
  readonly reportSteps: ReadonlyArray<{ readonly label: string; readonly title: string }>;
  /** Does this stage talk to a node at all? */
  readonly needsChain: (stage: Stage) => boolean;
  /** Does this stage need the deployer resolved from the keystore, at the cost of a password prompt? */
  readonly needsDeployerKey: (stage: Stage) => boolean;
};

export type Options = {
  readonly rpcUrl: string;
  readonly account: string;
  /** How stages sign. Null only for the read-only stages, which sign nothing. */
  readonly signer: Signer | null;
  /** Who signs step F on the admin's behalf, when anything does. */
  readonly adminSigner: Signer | null;
  readonly admin: string;
  readonly deploymentId: string;
  readonly pauser: string | null;
  readonly adminAccount: string | null;
  readonly confirmations: number;
  readonly minBlockOverride: number | null;
  readonly outDirArg: string | null;
  readonly stage: Stage;
  readonly dryRun: boolean;
  readonly useFinality: boolean;
  readonly noConfirm: boolean;
  /** false = this deployment needs no git-committed seal. See confirmSealed. */
  readonly requireGitSeal: boolean;
  /** Trust the artifacts already in the out dir instead of rebuilding. See stageCompute. */
  readonly noBuild: boolean;
  /** Pass -vvvv to forge, so its execution traces are printed. See traceArgs. */
  readonly verbose: boolean;
  /** Where the stable half came from, or null if it was all typed. Shown in the preflight banner. */
  readonly configPath: string | null;

  // ---- upgrade only; empty / null for a deploy ----

  /** The live stack being upgraded (see ExistingAddresses). Empty for a deploy. */
  readonly existing: ExistingAddresses;
  /** Path to the KMS migration seed, or null to reconstruct it from the live stack plus defaults. */
  readonly migrationPath: string | null;
  /**
   * Cleartext handles already recorded in the live `CleartextDB`, whose values must survive the upgrade.
   *
   * Optional and repeatable, and its emptiness is REPORTED rather than silently tolerated: without one,
   * verify can only prove the stack still works, not that existing data survived. Two different claims.
   */
  readonly handles: readonly string[];
};

/**
 * The JSON config file: what this deployment IS.
 *
 * Deliberately does NOT include stage, dryRun, minBlock or yes — those are what a single invocation
 * DOES, and pinning them in a file turns every invocation into the same one. loadConfigFile rejects
 * them by name rather than ignoring them.
 */
export type ConfigFile = {
  readonly rpcUrl?: string;
  readonly account?: string;
  readonly admin?: string;
  readonly deploymentId?: string;
  readonly pauser?: string;
  readonly adminAccount?: string;
  readonly confirmations?: number;
  readonly outDir?: string;
  /** Positive form of --no-finality: `false` disables the finality wait. */
  readonly finality?: boolean;
  /** Positive form of --no-git: `false` means this deployment needs no committed seal. */
  readonly git?: boolean;
  /** Upgrade only: the live stack's addresses. Nine flags is unreasonable to retype. */
  readonly existing?: ExistingAddresses;
  /** Upgrade only: path to the KMS migration seed. */
  readonly migration?: string;
  /** Upgrade only: handles whose cleartext values must survive. */
  readonly handles?: readonly string[];
};

/** Command line before the config file is merged under it. `null` means "not given". */
export type CliArgs = {
  configPath: string | null;
  rpcUrl: string | null;
  account: string | null;
  admin: string | null;
  deploymentId: string | null;
  pauser: string | null;
  adminAccount: string | null;
  confirmations: number | null;
  outDirArg: string | null;
  useFinality: boolean | null;
  minBlockOverride: number | null;
  stage: Stage | null;
  dryRun: boolean;
  noConfirm: boolean;
  requireGitSeal: boolean | null;
  noBuild: boolean;
  verbose: boolean;
  /** Upgrade only. Merged over the config file's `existing` block, so a single address can be overridden. */
  existing: Record<string, string>;
  migrationPath: string | null;
  handles: string[];
};

/** Mutable run state. The shell version kept these as globals; they are threaded explicitly here. */
export type Ctx = {
  /** What varies between the deploy and the upgrade. */
  readonly flow: Flow;
  readonly opt: Options;
  readonly outDir: string;
  readonly buildOut: string;
  readonly broadcastDir: string;
  readonly journalPath: string;
  readonly deployer: string;
  chainId: string;
  useFinality: boolean;
  /** Block the next broadcasting stage may not start before. */
  nextMinBlock: number;
  /** Block that must FINALIZE before the next stage. 0 = nothing sent yet. */
  finalityTarget: number;
  stageLabel: string;
};

export type JournalEntry = {
  readonly stage: string;
  readonly script?: string;
  readonly hash: string | null;
  readonly type?: string | null;
  readonly contract?: string | null;
  readonly address?: string | null;
  readonly function?: string | null;
  readonly block: number | null;
  readonly gasUsed?: number | null;
  readonly status: 'ok' | 'REVERTED' | 'unmined';
  readonly observed?: boolean;
  readonly note?: string;
  readonly ts?: number | null;
};

/** The shape of forge's broadcast/<Script>/<chainId>/run-latest.json that this reads. */
export type ForgeRun = {
  readonly timestamp?: number;
  readonly transactions?: ReadonlyArray<{
    readonly hash: string;
    readonly transactionType?: string;
    readonly contractName?: string;
    readonly contractAddress?: string;
    readonly function?: string | null;
  }>;
  readonly receipts?: ReadonlyArray<{
    readonly transactionHash: string;
    readonly blockNumber?: string;
    readonly status?: string;
    readonly gasUsed?: string;
  }>;
};

export type Manifest = {
  readonly chainId?: number;
  readonly deploymentId?: string;
  readonly deployer?: string;
  readonly admin?: string;
  readonly address?: Record<string, string>;
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Located from this file's own path, not from the caller's working directory, so the tool can be run
 * from anywhere. main() chdirs to PACKAGE_ROOT before touching forge, because forge resolves script
 * paths, remappings and fs_permissions against the directory holding foundry.toml.
 *
 * One consequence worth knowing: loadConfigFile runs BEFORE that chdir, so a relative --config
 * resolves against the caller's directory — which is what anyone typing it would expect. A relative
 * --out-dir does not: it resolves against FS_ROOT, because it has to land somewhere forge is allowed
 * to write (see resolveOutDir).
 */

////////////////////////////////////////////////////////////////////////////////

export const DRAFT_DIR = import.meta.dirname;
export const PACKAGE_ROOT = dirname(DRAFT_DIR);
/** Relative on purpose: forge resolves script paths against the project root it runs in. */
export const SCRIPT_DIR = 'create2-deploy/script';

/** Baked into every salt. MAJOR_MINOR only — a patch release must not move the addresses. */
export const FHEVM_VERSION = '0.13';
export const CONFIG_PREFIX = `fhevm-config-${FHEVM_VERSION}.0/`;

export const FACTORY = '0x4e59b44847b379578588920cA78FbF26c0B4956C';

/**
 * The factory's runtime code hash (§3).
 *
 * WHY THIS IS A CONSTANT RATHER THAN COMPUTED. Computing it is trivial — checkFactory already does
 * `keccak(eth_getCode(FACTORY))` on the target chain. But deriving the EXPECTED value from the same
 * chain being checked compares a value against itself and can never fail. The gate exists to catch a
 * DIFFERENT contract squatting this address on some chain, and "different" is only meaningful
 * against a reference that did not come from that chain. The nonce path makes the identical argument
 * about a different value in scripts/deploy.sh: "an 'expected' value fetched from the thing under
 * test always matches, which is not a check."
 *
 * NOT Sepolia-specific, despite where it was first read: mainnet, Sepolia, Holesky and base-sepolia
 * all return this, and it is also what anvil pre-deploys locally. The factory is the same deployed
 * bytes everywhere by construction — that is the entire point of a deterministic-deployment proxy.
 *
 * PROVENANCE, per §3's "read it off mainnet or Sepolia; do not transcribe it from memory or from a
 * blog post": read off mainnet and Sepolia, which agree. Re-verify with
 *
 *     cast keccak "$(cast code 0x4e59b44847b379578588920cA78FbF26c0B4956C --rpc-url <rpc>)"
 *
 * The runtime is the EIP-3860-aware variant, whose leading PUSH32 mask rejects initcode at or above
 * the 49152-byte limit. Shorter bytecode for this address circulates in older write-ups; it is stale,
 * which is exactly why §3 says to read the value rather than recall it.
 */
export const FACTORY_CODEHASH = '0x2fa86add0aed31f33a762c9d88e807c475bd51d0f52bd0955754b2608f7e4989';

/**
 * §1: testnets only. This is the cleartext stack — FHE is replaced by plaintext and the KMS /
 * coprocessor signer keys derive from the published FHEVM_MNEMONIC at documented HD paths. On a
 * testnet that is the POINT: the js-sdk relayer must hold those keys for cleartext decryption to
 * work. On mainnet it is total compromise.
 *
 * Read §11 R1 before trusting this list to do more than it does. It binds OUR tooling and nobody
 * else's — the address set is replayable onto mainnet by anyone, and no allow-list here can stop it.
 */
export const ALLOWED_CHAIN_IDS: readonly string[] = ['11155111', '17000', '84532', '421614'];

/** Where forge may write. Must match foundry.toml's fs_permissions — see resolveOutDir. */
export const FS_ROOT = DRAFT_DIR;

////////////////////////////////////////////////////////////////////////////////

export const CONFIG_KEYS: readonly string[] = [
  'rpcUrl',
  'account',
  'admin',
  'deploymentId',
  'pauser',
  'adminAccount',
  'confirmations',
  'outDir',
  'finality',
  'git',
  'existing',
  'migration',
  'handles',
];

/** Rejected in a config file by name, so the message can say where they belong instead. */
export const CLI_ONLY_KEYS: readonly string[] = [
  'stage',
  'dryRun',
  'minBlock',
  'noConfirm',
  'report',
  'noBuild',
  'verbose',
];

////////////////////////////////////////////////////////////////////////////////

export const HOST_ROLES: readonly string[] = [
  'ACL_ADDRESS',
  'FHEVM_EXECUTOR_ADDRESS',
  'KMS_VERIFIER_ADDRESS',
  'INPUT_VERIFIER_ADDRESS',
  'HCU_LIMIT_ADDRESS',
  'PROTOCOL_CONFIG_ADDRESS',
  'KMS_GENERATION_ADDRESS',
  'CLEARTEXT_ARITHMETIC_ADDRESS',
  'CLEARTEXT_DB_ADDRESS',
  'PAUSER_SET_ADDRESS',
];

/** Widest role name is IMPL_CLEARTEXT_ARITHMETIC_ADDRESS, at 34. */
export const ROLE_WIDTH = 34;

/** Width of the rule under a report step's header. Sized to the block+status+hash columns. */
export const RULE_WIDTH = 92;

/**
 * The steps a report accounts for, in plan order, keyed by the label each stage tags its journal
 * entries with. `compute` is absent because it sends nothing — its evidence is the manifest.
 */

////////////////////////////////////////////////////////////////////////////////

export const ANVIL_MNEMONIC = 'test test test test test test test test test test test junk';

/** Account 0 deploys; account 1 is the admin that takes root in step F. Matches anvil-config.json. */
export const ANVIL_DEPLOYER_INDEX = 0;
export const ANVIL_ADMIN_INDEX = 1;

/**
 * The nine addresses of a stack that ALREADY EXISTS, supplied by the operator.
 *
 * Only the upgrade uses this; a deploy derives every address it needs. Keyed by the role names the
 * manifest and the generated `addresses.sol` use, so a value read here can be written straight out
 * without a second naming convention to keep in step.
 *
 * Deliberately supplied rather than read from a previous manifest: a stack may have been deployed by the
 * nonce path, by an older revision, or by someone else, and requiring a manifest this tooling happened to
 * write would make the upgrade unusable exactly when it matters. The cost is that a typo bakes into the
 * new implementations, which is why every entry is validated against the live chain before anything is
 * computed.
 */
export type ExistingAddresses = Readonly<Record<string, string>>;

/** The role names an upgrade must be given, in the order the help and the banner list them. */
export const EXISTING_ROLES: readonly string[] = [
  'ACL_ADDRESS',
  'FHEVM_EXECUTOR_ADDRESS',
  'KMS_VERIFIER_ADDRESS',
  'INPUT_VERIFIER_ADDRESS',
  'HCU_LIMIT_ADDRESS',
  'CLEARTEXT_ARITHMETIC_ADDRESS',
  'CLEARTEXT_DB_ADDRESS',
  'PAUSER_SET_ADDRESS',
  'ACL_OWNER',
];

/** `--acl` -> `ACL_ADDRESS`. The CLI spelling of each role above. */
export const EXISTING_FLAGS: Readonly<Record<string, string>> = {
  '--acl': 'ACL_ADDRESS',
  '--fhevm-executor': 'FHEVM_EXECUTOR_ADDRESS',
  '--kms-verifier': 'KMS_VERIFIER_ADDRESS',
  '--input-verifier': 'INPUT_VERIFIER_ADDRESS',
  '--hcu-limit': 'HCU_LIMIT_ADDRESS',
  '--cleartext-arithmetic': 'CLEARTEXT_ARITHMETIC_ADDRESS',
  '--cleartext-db': 'CLEARTEXT_DB_ADDRESS',
  '--pauser-set': 'PAUSER_SET_ADDRESS',
  '--acl-owner': 'ACL_OWNER',
};

/**
 * How a stage signs: a forge keystore account, or an index into the public anvil mnemonic.
 *
 * A tagged union rather than a nullable account name because the two produce DIFFERENT forge flags,
 * and because it puts the anvil-only restriction in one place — `resolveSigner` is the only thing that
 * can mint the `anvil` variant, and it refuses unless the node answers `anvil_nodeInfo`.
 */
export type Signer =
  { readonly kind: 'keystore'; readonly account: string } | { readonly kind: 'anvil'; readonly index: number };

/**
 * Flags for `forge script`. Note the PLURAL names: forge takes `--mnemonics` / `--mnemonic-indexes`,
 * while `cast wallet` takes the singular `--mnemonic` / `--mnemonic-index`. They are not interchangeable,
 * and passing the wrong pair fails with an unhelpful clap error.
 */
export function forgeSignerArgs(signer: Signer): string[] {
  return signer.kind === 'keystore'
    ? ['--account', signer.account]
    : ['--mnemonics', ANVIL_MNEMONIC, '--mnemonic-indexes', String(signer.index)];
}

/** Flags for `cast wallet address` — the singular spellings. */
export function castSignerArgs(signer: Signer): string[] {
  return signer.kind === 'keystore'
    ? ['--account', signer.account]
    : ['--mnemonic', ANVIL_MNEMONIC, '--mnemonic-index', String(signer.index)];
}

export function signerAddress(signer: Signer): string {
  return captureOrFail('cast', ['wallet', 'address', ...castSignerArgs(signer)]);
}

export function describeSigner(signer: Signer): string {
  return signer.kind === 'keystore' ? `keystore '${signer.account}'` : `anvil account ${String(signer.index)}`;
}

/**
 * Is this RPC an anvil?
 *
 * `anvil_nodeInfo` is the discriminator: anvil answers it, and every other node returns JSON-RPC
 * -32601 "Method not found". Chain id is NOT used — anvil happily runs with any chain id (`--chain-id`,
 * or a fork inheriting the upstream one), so 31337 both misses forked anvils and could be spoofed by a
 * private chain configured to claim it.
 */
export function isAnvil(rpcUrl: string): boolean {
  return capture('cast', ['rpc', 'anvil_nodeInfo', '--rpc-url', rpcUrl]).ok;
}

export function rejectRawPrivateKey(flag: string, value: string): void {
  if (!/^(0x)?[0-9a-fA-F]{64}$/.test(value)) return;
  fail(
    `Error: ${flag} takes a forge KEYSTORE NAME, not a private key.`,
    '       (The value looks like one, so it is not being echoed here. If it is a real key,',
    '        treat it as compromised — it may already be in your shell history.)',
    '',
    '       Import it once, then pass the name:',
    '         cast wallet import my-deployer --interactive',
    `         ${flag} my-deployer`,
  );
}

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

export function parseCliArgs(flow: Flow, argv: readonly string[]): CliArgs {
  const cli: CliArgs = {
    configPath: null,
    rpcUrl: null,
    account: null,
    admin: null,
    deploymentId: null,
    pauser: null,
    adminAccount: null,
    confirmations: null,
    outDirArg: null,
    useFinality: null,
    minBlockOverride: null,
    stage: null,
    dryRun: false,
    noConfirm: false,
    requireGitSeal: null,
    noBuild: false,
    verbose: false,
    existing: {},
    migrationPath: null,
    handles: [],
  };

  const need = (i: number, flag: string): string => {
    const v = argv[i + 1];
    if (v === undefined) fail(`Error: ${flag} requires a value.`);
    return v;
  };

  for (let i = 0; i < argv.length; i++) {
    // argv[i] is `string | undefined` under noUncheckedIndexedAccess, and the loop bound makes it
    // never actually undefined — but the switch has to say so rather than assert it.
    const a = argv[i] ?? '';
    switch (a) {
      case '--config':
        cli.configPath = need(i, a);
        i++;
        break;
      case '--rpc-url':
        cli.rpcUrl = need(i, a);
        i++;
        break;
      case '--account':
        cli.account = need(i, a);
        i++;
        break;
      case '--admin':
        cli.admin = need(i, a);
        i++;
        break;
      case '--deployment-id':
        cli.deploymentId = need(i, a);
        i++;
        break;
      case '--pauser':
        cli.pauser = need(i, a);
        i++;
        break;
      case '--admin-account':
        cli.adminAccount = need(i, a);
        i++;
        break;
      case '--confirmations':
        cli.confirmations = Number(need(i, a));
        i++;
        break;
      case '--min-block':
        cli.minBlockOverride = Number(need(i, a));
        i++;
        break;
      case '--out-dir':
        cli.outDirArg = need(i, a);
        i++;
        break;
      case '--stage':
        cli.stage = need(i, a);
        i++;
        break;
      case '--report':
        cli.stage = 'report';
        break;
      case '--dry-run':
        cli.dryRun = true;
        break;
      case '--no-finality':
        cli.useFinality = false;
        break;
      case '--no-confirm':
        cli.noConfirm = true;
        break;
      case '--no-git':
        cli.requireGitSeal = false;
        break;
      case '--no-build':
        cli.noBuild = true;
        break;
      case '-v':
      case '--verbose':
        cli.verbose = true;
        break;
      case '--migration':
        cli.migrationPath = need(i, a);
        i += 1;
        break;
      case '--handle':
        // Repeatable rather than comma-separated: a handle is 66 characters and a missed comma would
        // produce one unusable value instead of an error.
        cli.handles.push(need(i, a));
        i += 1;
        break;
      case '-h':
      case '--help':
        say(flow.help);
        process.exit(0);
      // The --help case above ends in `process.exit`, which is `never` — but the rule cannot see that,
      // and `allowUnreachableCode: false` forbids adding the `break` that would otherwise silence it.
      // The directive has to be the line immediately before `default`, or it applies to a comment.
      // eslint-disable-next-line no-fallthrough
      default: {
        // The nine `existing` address flags, from one table rather than nine cases.
        const role = EXISTING_FLAGS[a];
        if (role !== undefined) {
          cli.existing[role] = need(i, a);
          i += 1;
          break;
        }
        fail(`Error: unknown argument '${a}'. Try --help.`);
      }
    }
  }
  return cli;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Load the JSON config, if there is one.
 *
 * A deployment spans many invocations, and retyping five identity flags every time is how they drift
 * — which is the whole reason preflight has to check them against the seal. A config file removes
 * the retyping, so the drift it guards against becomes much less likely in the first place.
 *
 * It holds only the STABLE half of the arguments: what this deployment IS. The per-invocation half —
 * which stage to run, whether to dry-run, the reorg floor for one manual step — stays on the command
 * line, and is rejected here with a pointer rather than silently accepted. A config that pinned
 * `stage: "all"` would turn every invocation into a full deploy.
 *
 * Unknown keys are rejected too. A typo in `deploymentId` would otherwise surface as "missing
 * required argument" three functions later, or worse, silently select a different address set.
 */

////////////////////////////////////////////////////////////////////////////////

export function loadConfigFile(
  flow: Flow,
  explicitPath: string | null,
): { readonly cfg: ConfigFile; readonly path: string | null } {
  const path = explicitPath ?? join(FS_ROOT, flow.defaultConfigName);

  if (!existsSync(path)) {
    // An explicit --config that is not there is an error; the conventional path simply not existing
    // is the normal case for someone passing everything on the command line.
    if (explicitPath !== null) fail(`Error: no config file at ${path}`);
    return { cfg: {}, path: null };
  }

  const cfg = readJson<Record<string, unknown>>(path);
  if (cfg === null || typeof cfg !== 'object' || Array.isArray(cfg)) {
    fail(`Error: ${path} is not a JSON object.`);
  }

  for (const key of Object.keys(cfg)) {
    if (CONFIG_KEYS.includes(key)) continue;
    if (CLI_ONLY_KEYS.includes(key)) {
      fail(
        `Error: '${key}' is not allowed in ${path}.`,
        '       The config file holds what this deployment IS, not what one invocation DOES.',
        `       Pass --${key.replace(/[A-Z]/g, (c) => '-' + c.toLowerCase())} on the command line.`,
      );
    }
    fail(`Error: unknown key '${key}' in ${path}.`, `       allowed: ${CONFIG_KEYS.join(', ')}`);
  }

  return { cfg: cfg, path };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Merge the config file under the command line, then validate.
 *
 * PRECEDENCE: an explicit flag always wins. That is what makes a config file safe to keep around —
 * `--stage creates --dry-run` against a pinned deployment needs no other arguments, and a one-off
 * `--rpc-url` override does not require editing the file.
 */

////////////////////////////////////////////////////////////////////////////////

export function resolveOptions(flow: Flow, cli: CliArgs, cfg: ConfigFile, configPath: string | null): Options {
  const stage: Stage = cli.stage ?? 'all';
  if (!flow.stages.includes(stage)) {
    fail(`Error: unknown --stage '${stage}'.`, `       one of: ${flow.stages.join(', ')}`);
  }

  const rpcUrl = cli.rpcUrl ?? cfg.rpcUrl ?? '';
  const account = cli.account ?? cfg.account ?? '';
  let admin = cli.admin ?? cfg.admin ?? '';
  const deploymentId = cli.deploymentId ?? cfg.deploymentId ?? '';
  const adminAccount = cli.adminAccount ?? cfg.adminAccount ?? null;

  const missing = (flag: string, key: string): string =>
    configPath === null
      ? `Error: ${flag} is required (or put "${key}" in a config file - see --help).`
      : `Error: ${flag} is required, and "${key}" is not in ${configPath}.`;

  if (rpcUrl === '') fail(missing('--rpc-url', 'rpcUrl'));

  // Before the value reaches `cast`, which would print it in its own error. See §12.
  if (account !== '') rejectRawPrivateKey('--account', account);
  if (adminAccount !== null) rejectRawPrivateKey('--admin-account', adminAccount);

  // --account is optional in exactly one case: a local anvil, where the funded accounts come from a
  // mnemonic that is public knowledge. Anywhere else it stays mandatory, and §12's keystone holds —
  // the deployer key owns ACLOwner until step F, so a testnet run must not accept an unprotected key.
  //
  // The probe runs only when the operator has actually omitted --account, so an unreachable node
  // cannot break the flows that supplied one. Read-only stages sign nothing and skip it entirely.
  let signer: Signer | null = null;
  if (account !== '') {
    signer = { kind: 'keystore', account };
  } else if (flow.needsChain(stage)) {
    if (!isAnvil(rpcUrl)) {
      fail(
        missing('--account', 'account'),
        '',
        `       ${rpcUrl} did not answer anvil_nodeInfo, so it is not an anvil. The keystore-free`,
        '       default exists only for a local anvil rehearsal, whose funded accounts come from a',
        '       PUBLIC mnemonic. On any other chain the deployer key owns ACLOwner until step F',
        '       completes (plan section 12), so it has to be a keystore:',
        '',
        '         cast wallet import my-deployer --interactive',
        '         --account my-deployer',
      );
    }
    signer = { kind: 'anvil', index: ANVIL_DEPLOYER_INDEX };
  }

  // Step F is sent by the admin. With a keystore that is --admin-account; on anvil it is simply the
  // next account, so the rehearsal completes unattended instead of stopping to poll for a transaction
  // no one is going to send.
  const adminSigner: Signer | null =
    adminAccount !== null
      ? { kind: 'keystore', account: adminAccount }
      : signer?.kind === 'anvil'
        ? { kind: 'anvil', index: ANVIL_ADMIN_INDEX }
        : null;

  // --admin is an ADDRESS, and mandatory by plan section 7 — except on the anvil default, where the
  // only sensible value is the account that adminSigner will sign with. Deriving it keeps the two from
  // disagreeing, which preflight would otherwise reject.
  if (admin === '' && adminSigner?.kind === 'anvil') {
    admin = signerAddress(adminSigner);
  }
  if (admin === '') fail(missing('--admin', 'admin') + ' (plan section 7)');
  if (deploymentId === '') fail(missing('--deployment-id', 'deploymentId') + ' (plan section 14.2)');

  // A dry run of `all` would be theatre: nothing is sent, so stage 2 simulates against a chain where
  // stage 1 never happened, and every later stage reports blocked on a precondition a real run would
  // have satisfied.
  if (cli.dryRun && stage === 'all') {
    fail(
      "Error: --dry-run needs a specific --stage. Simulating 'all' would report every stage",
      '       after the first as blocked, because a dry run sends nothing.',
    );
  }

  return {
    rpcUrl,
    account,
    signer,
    adminSigner,
    admin,
    deploymentId,
    adminAccount,
    pauser: cli.pauser ?? cfg.pauser ?? null,
    confirmations: cli.confirmations ?? cfg.confirmations ?? 3,
    minBlockOverride: cli.minBlockOverride,
    outDirArg: cli.outDirArg ?? cfg.outDir ?? null,
    stage,
    dryRun: cli.dryRun,
    useFinality: cli.useFinality ?? cfg.finality ?? true,
    noConfirm: cli.noConfirm,
    requireGitSeal: cli.requireGitSeal ?? cfg.git ?? true,
    noBuild: cli.noBuild,
    verbose: cli.verbose,
    configPath,

    // Upgrade inputs. CLI merged OVER the config file, so one address can be overridden without editing
    // the file. Empty for a deploy, which derives every address it needs.
    existing: { ...(cfg.existing ?? {}), ...cli.existing },
    migrationPath: cli.migrationPath ?? cfg.migration ?? null,
    handles: cli.handles.length > 0 ? cli.handles : (cfg.handles ?? []),
  };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolve --out-dir, and refuse anything forge could not write to.
 *
 * The compute passes write addresses.sol, pass2.json and manifest.json with `vm.writeFile`, and
 * forge rejects any path not granted by `fs_permissions` in foundry.toml:
 *
 *     vm.createDir: the path /... is not allowed to be accessed for write operations
 *
 * That list is static config, so --out-dir can only ever reach inside it. Absolute paths anywhere on
 * disk do work IF foundry.toml grants them — forge accepts entries outside the project root — but
 * granting one per deployment does not scale, so this keeps every out dir under a single root.
 *
 * foundry.toml needs, alongside the nonce path's own entry:
 *
 *     fs_permissions = [
 *         { access = "read-write", path = "./internal/.deploy-config" },   # nonce path
 *         { access = "read-write", path = "./create2-deploy" },      # this path
 *     ]
 *
 * Checked here rather than left to forge, which would only notice midway through pass 1 — after two
 * builds — complaining about a path the operator never typed.
 */

////////////////////////////////////////////////////////////////////////////////

export function resolveOutDir(outDirArg: string | null): string {
  const outDir = outDirArg === null ? join(FS_ROOT, '.out') : resolve(FS_ROOT, outDirArg);

  if (!isInside(outDir, FS_ROOT)) {
    fail(
      `Error: --out-dir must be inside ${FS_ROOT}`,
      `         resolved to: ${outDir}`,
      "       forge only writes where foundry.toml's fs_permissions allows, and that is",
      '       static config. To use somewhere else, add it there first.',
    );
  }
  return outDir;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Does this stage touch the network?
 *
 * `log` and `report` read the journal and nothing else, so they must work with no RPC, no keystore
 * and no foundry installed — which is exactly when you want them: reading back what happened from a
 * machine that cannot reach the chain, or after the deployer key has been put away.
 */

////////////////////////////////////////////////////////////////////////////////

export function buildContext(flow: Flow, opt: Options): Ctx {
  const outDir = resolveOutDir(opt.outDirArg);

  // §12: the deployer key owns ACLOwner — root over the stack — until step F completes. Keystore
  // only. A raw private key is accepted by scripts/deploy.sh for 31337 and is NOT accepted here.
  // "Testnet" is not "throwaway": these stacks are what the js-sdk integration story runs against.
  //
  // Read-only stages take the deployer off the manifest instead, and so never unlock anything. That
  // does make checkOutDirIdentity's deployer comparison vacuous for them — which is the point: that
  // check guards stages that SEND, and there is nothing to protect when nothing is sent.
  let deployer = '';
  if (!flow.needsDeployerKey(opt.stage)) {
    deployer = readJson<Manifest>(join(outDir, 'manifest.json'))?.deployer ?? '';
  }
  if (deployer === '' && flow.needsChain(opt.stage)) {
    deployer = opt.signer === null ? '' : signerAddress(opt.signer);
  }

  return {
    flow,
    opt,
    outDir,
    buildOut: join(outDir, 'build'),
    broadcastDir: join(outDir, 'broadcast'),
    journalPath: join(outDir, 'journal.jsonl'),
    deployer,
    chainId: '',
    useFinality: opt.useFinality,
    nextMinBlock: 0,
    finalityTarget: 0,
    stageLabel: '',
  };
}

////////////////////////////////////////////////////////////////////////////////

export function manifestPath(ctx: Ctx): string {
  return join(ctx.outDir, 'manifest.json');
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The environment every forge script in this path reads. No script takes a CLI argument.
 *
 * FHEVM_DEPLOYER is an ADDRESS, not a key: the scripts only predict with it and check it against
 * msg.sender, while forge authenticates via --account/--sender. No script here ever holds a key.
 */
export function scriptEnv(ctx: Ctx, extra?: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  return {
    FHEVM_VERSION,
    FHEVM_DEPLOYMENT_ID: ctx.opt.deploymentId,
    FHEVM_DEPLOYER: ctx.deployer,
    FHEVM_ADMIN: ctx.opt.admin,
    FHEVM_CONFIRMATIONS: String(ctx.opt.confirmations),
    FHEVM_OUT_DIR: ctx.outDir,
    ...(ctx.opt.pauser !== null && ctx.opt.pauser !== '' ? { FHEVM_PAUSER_0: ctx.opt.pauser } : {}),
    // Redirect forge's own per-run records into the out dir, so a run leaves nothing in the package
    // root and the raw artifacts sit beside the journal distilled from them.
    FOUNDRY_BROADCAST: ctx.broadcastDir,
    ...extra,
  };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * `-vvvv` when --verbose, nothing otherwise.
 *
 * Worth knowing where the traces come from: `forge script` runs the script in its OWN EVM against a
 * fork, fetching state with eth_getCode / eth_getStorageAt and executing locally. A `view` call like
 * ACL.getFHEVMExecutorAddress() therefore never reaches the node as an eth_call, and never appears
 * in anvil's log — it is not a transaction and, mostly, not even a request. forge prints the trace on
 * failure; on success you have to ask.
 */
export function traceArgs(ctx: Ctx): readonly string[] {
  return ctx.opt.verbose ? ['-vvvv'] : [];
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Point the `fhevm-config-X.Y.0/` import prefix at the GENERATED addresses.sol.
 *
 * Every stage after compute's first pass needs this. It is NOT applied globally on purpose: compute
 * pass 1 must build against the committed placeholders, because it is what PRODUCES the generated
 * config, and a global would quietly make pass 1 depend on its own output (or on last run's).
 *
 * Overrides just this one prefix and leaves openzeppelin/forge-std to be discovered as usual, so
 * remappings.txt is never edited and there is no restore-on-failure to get wrong.
 */
export function generatedConfigEnv(ctx: Ctx): NodeJS.ProcessEnv {
  return { FOUNDRY_REMAPPINGS: `${CONFIG_PREFIX}=${ctx.outDir}/` };
}

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

export function checkChainAllowed(ctx: Ctx): void {
  if (ALLOWED_CHAIN_IDS.includes(ctx.chainId)) return;

  // An anvil is exempt whatever chain id it reports, and that is not a hole in the rule — it is the
  // rule read properly. What the allow-list protects against is BROADCASTING a stack whose KMS keys
  // come from a published mnemonic onto a network other people use. An anvil is a local sandbox: it
  // reaches nothing, so there is nothing to protect. A plain `anvil` starts on 31337, which is
  // excluded from the list for an unrelated reason (it is the nonce path's chain), and requiring
  // `--chain-id 11155111` just to rehearse was friction with no safety behind it.
  //
  // Keyed on `anvil_nodeInfo` rather than on the chain id, so a private chain that simply claims 31337
  // gets no exemption. A mainnet-FORKED anvil does qualify, and should: it reports chain id 1 while
  // still being a sandbox that sends nothing to mainnet.
  if (isAnvil(ctx.opt.rpcUrl)) {
    say(`  chain id ${ctx.chainId} allowed: the node answers anvil_nodeInfo, so it is a local anvil`);
    return;
  }

  fail(
    `Error: chain id ${ctx.chainId} is not in the testnet allow-list, and ${ctx.opt.rpcUrl} is not an anvil.`,
    '       This stack derives its KMS/coprocessor keys from a PUBLISHED mnemonic, so it may only be',
    `       broadcast to a testnet (${ALLOWED_CHAIN_IDS.join(', ')}) or to a local anvil.`,
  );
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Do the arguments of this session match the ones the manifest was sealed with?
 *
 * A deployment spans many invocations, often days apart, and every one of them retypes the whole
 * argument list. The manifest is the record of what the first one decided (§9), so it is also the
 * only thing that can catch the second one from drifting. Four fields, four distinct failures:
 *
 *   wrong chain          --out-dir was not changed when --rpc-url was. The next `compute` would
 *                        reseal over another network's record.
 *   wrong deploymentId   the salts have changed, so this is a DIFFERENT address set (§14.2) that
 *                        happens to be pointed at the same directory. Every read-only stage would
 *                        compute new salts while reading old addresses and report drift on all of them.
 *   wrong deployer       --account points at another key. The deployer is baked into the ACL
 *                        proxy's initcode, so this too is a different address set — but only SOME
 *                        of the 22 move, which is why the symptom is so misleading without this.
 *   wrong admin          moves no address, but silently redirects who ends up with root.
 *
 * In every case the standing stack is unharmed — but its manifest is how it is verified and upgraded
 * for the rest of its life, and overwriting that is the actual loss.
 */
export function checkOutDirIdentity(ctx: Ctx): void {
  const manifest = readJson<Manifest>(manifestPath(ctx));
  if (manifest === null) return;

  if (manifest.chainId !== undefined && String(manifest.chainId) !== ctx.chainId) {
    fail(
      `Error: ${ctx.outDir} holds a manifest sealed for chain ${manifest.chainId}, not ${ctx.chainId}.`,
      `       Use one --out-dir per chain, e.g. --out-dir .out-${ctx.chainId}`,
    );
  }

  if (manifest.deploymentId !== undefined && manifest.deploymentId !== ctx.opt.deploymentId) {
    fail(
      `Error: ${ctx.outDir} belongs to deployment '${manifest.deploymentId}', not '${ctx.opt.deploymentId}'.`,
      '       A different --deployment-id is a different set of salts, so a different',
      '       address set entirely (plan section 14.2) - it needs its own --out-dir.',
      `         --deployment-id ${ctx.opt.deploymentId} --out-dir .out-${ctx.opt.deploymentId}`,
      `       '${manifest.deploymentId}' stays where it is; its stack is untouched and still standing.`,
    );
  }

  // A --account pointed at a different key between sessions. Caught HERE because the failure it
  // otherwise produces is actively misleading: the deployer is baked into the ACL proxy's initcode,
  // so `creates` would stop at ACL_ADDRESS reporting "build drift", blaming the build for what is
  // really a changed key — and only some of them would have moved, since the shared proxies and
  // PauserSet do not reference the deployer at all.
  if (manifest.deployer !== undefined && !sameAddress(manifest.deployer, ctx.deployer)) {
    fail(
      `Error: '${ctx.opt.deploymentId}' was sealed by a different deployer.`,
      `         sealed:            ${manifest.deployer}`,
      `         ${ctx.opt.signer === null ? 'the deployer' : describeSigner(ctx.opt.signer)} resolves to ${ctx.deployer}`,
      '       Every address in this stack derives from the deployer (plan section 5.2), so this is',
      '       a different address set — not a different way of reaching the same one.',
      '       Use the keystore account that sealed it, or start a new deployment with its own',
      '       --deployment-id and --out-dir.',
    );
  }

  // The admin moves no address (§5.2), so this is not about the address set — it is about who ends
  // up with root. Step E's predicate is "offered to THIS admin", so a changed --admin would not be
  // seen as already-done: it would offer again, silently redirecting ownership of the whole stack to
  // an address the seal never named.
  if (manifest.admin !== undefined && !sameAddress(manifest.admin, ctx.opt.admin)) {
    fail(
      `Error: '${ctx.opt.deploymentId}' was sealed for a different admin.`,
      `         sealed: ${manifest.admin}`,
      `         --admin ${ctx.opt.admin}`,
      '       Continuing would offer ownership of the ACLOwner - root over the whole stack - to an',
      "       address this deployment's seal never named. Rotating the admin after the run is the",
      "       standing admin's own transferOwnership call, not a re-run with a different --admin.",
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * `--admin-account` must resolve to `--admin`.
 *
 * The two are not alternatives and one does not override the other. `--admin` is the ADDRESS that
 * gets root: it is sealed into the manifest, read by every script as FHEVM_ADMIN, and needed at
 * compute time — long before any admin key is involved, and in the multisig case §7 is written for,
 * where no admin keystore exists at all. `--admin-account` is only a signing credential for step F.
 *
 * Checked in preflight rather than inside step F, where it lives conceptually, because step F is the
 * LAST stage: a mismatch would otherwise survive compute, the seal, all creates and steps A-E
 * before surfacing, on a run that may have started days earlier.
 */
export function checkAdminAccount(ctx: Ctx): void {
  if (ctx.opt.adminSigner === null) return;

  const resolved = signerAddress(ctx.opt.adminSigner);
  if (!sameAddress(resolved, ctx.opt.admin)) {
    fail(
      `Error: the admin signer (${describeSigner(ctx.opt.adminSigner)}) resolves to ${resolved},`,
      `       but --admin is ${ctx.opt.admin}.`,
      '       --admin is the address that gets root and is sealed in the manifest; --admin-account',
      '       only signs step F on its behalf. They have to be the same account.',
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * §3, hard gate. A different contract squatting 0x4e59… on some testnet is the one realistic way
 * §8's "fatal mismatch" actually fires, and it would produce addresses nothing was compiled for.
 */
export function checkFactory(ctx: Ctx): void {
  const code = capture('cast', ['code', FACTORY, '--rpc-url', ctx.opt.rpcUrl]);
  if (!code.ok || code.stdout === '0x' || code.stdout === '') {
    fail(
      `Error: no CREATE2 factory at ${FACTORY} on chain ${ctx.chainId}.`,
      '       Fallback is the standard presigned deployment, with two conditions this',
      "       script will not hide: funding goes to the factory's one-time EOA",
      '       0x3fAB184622Dc19b6109349B94811493BF2a45362, not to our deployer; and that',
      '       transaction is PRE-EIP-155 legacy, which some chains reject outright. On such',
      '       a chain the canonical factory can never exist and this path is unavailable.',
    );
  }

  const hash = captureOrFail('cast', ['keccak', code.stdout]);
  if (hash !== FACTORY_CODEHASH) {
    fail(
      `Error: factory runtime code hash mismatch at ${FACTORY}.`,
      `         expected ${FACTORY_CODEHASH}`,
      `         observed ${hash}`,
    );
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Does this chain serve the `finalized` tag?
 *
 * Probed once, rather than discovered mid-run: a chain without it would make waitForBlock loop
 * forever on a query that can never be satisfied. Degrades to the depth floor LOUDLY — silently
 * dropping to a weaker guarantee is the failure mode worth avoiding.
 */
export function probeFinality(ctx: Ctx): string {
  if (!ctx.useFinality) return 'disabled (--no-finality)';

  const r = capture('cast', ['block-number', 'finalized', '--rpc-url', ctx.opt.rpcUrl]);
  if (r.ok && r.stdout !== '') return r.stdout;

  ctx.useFinality = false;
  warn(
    "this chain does not serve the 'finalized' tag.",
    `Falling back to a depth of ${ctx.opt.confirmations} blocks between stages, which is a`,
    'heuristic, not a consensus guarantee.',
  );
  return 'unsupported';
}

////////////////////////////////////////////////////////////////////////////////

/**
 * §11 R3, quantified before starting rather than discovered at send time.
 *
 * Deploying via the factory pays initcode as CALLDATA (16 gas per non-zero byte), and the
 * implementations of up to ~24 KB runtime each add materially per create. Faucet-funded deployers
 * run dry mid-run. This prints; measuring a real threshold against a fork is still a gap.
 */
export function preflight(ctx: Ctx): void {
  say('🍖 preflight');

  ctx.chainId = captureOrFail('cast', ['chain-id', '--rpc-url', ctx.opt.rpcUrl]);
  checkChainAllowed(ctx);
  checkOutDirIdentity(ctx);
  if (ctx.flow.needsDeployerKey(ctx.opt.stage)) checkAdminAccount(ctx);
  checkFactory(ctx);

  const finalized = probeFinality(ctx);
  const balanceWei = captureOrFail('cast', ['balance', ctx.deployer, '--rpc-url', ctx.opt.rpcUrl]);
  const balanceEth = captureOrFail('cast', ['to-unit', balanceWei, 'ether']);

  say(
    `  chain            ${ctx.chainId}`,
    `  factory          ${FACTORY} (hash pinned, ok)`,
    `  finalized block  ${finalized}`,
    `  deployer         ${ctx.deployer}`,
    `  balance          ${balanceEth} ETH`,
    `  admin            ${ctx.opt.admin}`,
    `  deploymentId     ${ctx.opt.deploymentId} @ v${FHEVM_VERSION}`,
    `  out dir          ${ctx.outDir}`,
    `  config           ${ctx.opt.configPath ?? '(none - all arguments on the command line)'}`,
    '',
  );
}

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

export function headBlock(ctx: Ctx): number {
  return Number(captureOrFail('cast', ['block-number', '--rpc-url', ctx.opt.rpcUrl]));
}

////////////////////////////////////////////////////////////////////////////////

export function finalizedBlock(ctx: Ctx): number {
  return Number(captureOrFail('cast', ['block-number', 'finalized', '--rpc-url', ctx.opt.rpcUrl]));
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The reorg gate (§11 R2), shell half.
 *
 * Steps A-F each REQUIRE FHEVM_MIN_BLOCK and refuse to run until the chain has reached it. Every one
 * of them decides what to do by reading state a previous step wrote, so a predicate evaluated one
 * block after the transaction it asks about can be answering from a block about to be orphaned — and
 * these predicates decide whether a step is SKIPPED.
 *
 * Two halves, both needed: this waits so the normal path does not fail; the script refuses so a
 * different orchestrator cannot proceed early just because it did not implement the wait.
 *
 * Depth is a heuristic — ~3 min at 15 blocks vs ~12.8 min to PoS finality — so this waits for the
 * `finalized` tag as well, when the chain serves it. A Solidity script cannot read `finalized`
 * through block.number, which is why the depth floor lives there and the finality wait lives here.
 */
export async function waitForBlock(ctx: Ctx, target: number): Promise<void> {
  let head = headBlock(ctx);
  if (head < target) {
    say(`  waiting for block ${target} (at ${head}, reorg depth ${ctx.opt.confirmations})`);
    while (head < target) {
      await sleep(4000);
      head = headBlock(ctx);
    }
  }

  if (!ctx.useFinality || ctx.finalityTarget <= 0) return;

  let fin = finalizedBlock(ctx);
  if (fin < ctx.finalityTarget) {
    say(`  waiting for block ${ctx.finalityTarget} to FINALIZE (finalized at ${fin})`);
    while (fin < ctx.finalityTarget) {
      await sleep(12000);
      fin = finalizedBlock(ctx);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Refuse to start while `who` has transactions in flight.
 *
 * Every broadcasting stage is idempotent, and re-running one IS the resume path — the predicates are
 * chain queries, so there is no journal to repair. The one case that is NOT safe is aborting while
 * transactions are still in the mempool: `forge script` simulates against a fork at the head, which
 * does not see them, so a re-run's predicates report "not deployed" for creates about to land. It
 * then re-sends them, and those revert — the canonical factory reverts when CREATE2 returns zero.
 * Wasted gas and a burnt nonce; the ADDRESSES are unharmed, so the next run succeeds.
 *
 * A difference between the pending and latest nonce is exactly "this account has work in flight".
 */
export function requireNoPendingTxs(ctx: Ctx, who: string): void {
  const latest = Number(captureOrFail('cast', ['nonce', who, '--block', 'latest', '--rpc-url', ctx.opt.rpcUrl]));
  const pending = Number(captureOrFail('cast', ['nonce', who, '--block', 'pending', '--rpc-url', ctx.opt.rpcUrl]));
  if (pending === latest) return;

  fail(
    `Error: ${who} has ${pending - latest} transaction(s) in the mempool.`,
    '       Starting now would re-send creates for addresses that are about to have code,',
    '       and those transactions would revert. Nothing is corrupted and no address is',
    '       burnt — wait for them to be mined and run the same command again.',
    '       (`--stage log` shows what this run has sent so far.)',
    `         latest nonce  ${latest}`,
    `         pending nonce ${pending}`,
  );
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Distil forge's run-latest.json into the journal (§9).
 *
 * Called whether the stage SUCCEEDED OR NOT — a stage that died halfway is precisely when the record
 * matters, and forge has already written what it managed to send.
 *
 * Invents no facts: forge already records every transaction and receipt, and this flattens them into
 * one append-only stream across stages, tagged with which stage sent what. Reading ten separate
 * run-latest.json files in the right order is the thing this saves you at 2am.
 */

////////////////////////////////////////////////////////////////////////////////

export function recordJournal(ctx: Ctx, target: string): void {
  // `?? target` is unreachable in practice — a forge target is always `File.sol:Contract` — but
  // noUncheckedIndexedAccess types index access as possibly-undefined, and a non-null assertion here
  // would be a claim rather than a handled case. A target without a colon IS its own file name.
  const file = target.split(':')[0] ?? target;
  const runPath = join(ctx.broadcastDir, file, ctx.chainId, 'run-latest.json');
  const forgeRun = readJson<ForgeRun>(runPath);
  if (forgeRun === null) return;

  const receipts = forgeRun.receipts ?? [];
  const rows: JournalEntry[] = (forgeRun.transactions ?? []).map((tx) => {
    const rc = receipts.find((r) => r.transactionHash === tx.hash);
    return {
      stage: ctx.stageLabel,
      script: file,
      hash: tx.hash,
      type: tx.transactionType ?? null,
      contract: tx.contractName ?? null,
      address: tx.contractAddress ?? null,
      function: tx.function ?? null,
      block: hexToNumber(rc?.blockNumber),
      gasUsed: hexToNumber(rc?.gasUsed),
      status: rc === undefined ? 'unmined' : rc.status === '0x1' ? 'ok' : 'REVERTED',
      ts: forgeRun.timestamp ?? null,
    };
  });

  // Skip transactions the journal already has.
  //
  // forge does NOT clear run-latest.json when a re-run broadcasts nothing — it just leaves the
  // previous run's file in place. Every idempotent re-run of an already-done stage would therefore
  // re-read and re-append the same transactions, and since re-running IS the resume path here, the
  // journal filled up with duplicates of its own history.
  //
  // Keyed on the transaction hash, which is unique by construction, so a hash already recorded is
  // never new information. Observations (step F, hash null) are always kept — they are not
  // transactions this run sent, and there is nothing to collide on.
  const known = new Set(
    readJsonl<JournalEntry>(ctx.journalPath)
      .map((r) => r.hash)
      .filter((h): h is string => h !== null),
  );
  const fresh = rows.filter((r) => r.hash === null || !known.has(r.hash));

  appendJsonl(ctx.journalPath, fresh);

  // A reverted transaction is not fatal on this path — a failed create does not burn its address
  // (§2) — but it must never scroll past unnoticed. Counted from what was actually appended, so a
  // re-run of a stage that once reverted does not warn about it again.
  const reverted = fresh.filter((r) => r.status === 'REVERTED').length;
  if (reverted > 0) warn(`${reverted} transaction(s) REVERTED in this stage - see --stage log`);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Record something on chain that this script did NOT send.
 *
 * Only step F's polling path uses it: the admin's acceptOwnership() comes from a key we do not hold,
 * so there is no local receipt to distil — but "the deployment finished at block N" is the single
 * most useful line in the whole journal, and omitting it because we were not the sender would be
 * pedantry.
 */
export function recordObservation(ctx: Ctx, stage: string, note: string, block: number): void {
  const entry: JournalEntry = { stage, observed: true, note, block, hash: null, status: 'ok' };
  appendJsonl(ctx.journalPath, [entry]);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The journal, with duplicate transactions collapsed.
 *
 * recordJournal now refuses to append a hash it already holds, but journals written before that
 * carry repeats — one per idempotent re-run of an already-done stage, because forge leaves the
 * previous run-latest.json in place when a run broadcasts nothing. Collapsing on read means an
 * existing journal reports correctly instead of needing to be hand-edited.
 *
 * Entries with no hash are observations, not transactions (step F), and are always kept.
 */
export function readJournal(ctx: Ctx): JournalEntry[] {
  const seen = new Set<string>();
  return readJsonl<JournalEntry>(ctx.journalPath).filter((r) => {
    if (r.hash === null) return true;
    if (seen.has(r.hash)) return false;
    seen.add(r.hash);
    return true;
  });
}

////////////////////////////////////////////////////////////////////////////////

/** What has been executed. The other half of `--stage status`, which says what remains. */
export function showJournal(ctx: Ctx): void {
  const rows = readJournal(ctx);
  if (rows.length === 0) {
    say(`No journal at ${ctx.journalPath} - nothing has been broadcast for this deployment yet.`);
    return;
  }

  say(`📜  log  (${ctx.journalPath})`, '');
  say(`  ${pad('STAGE', 10)} ${pad('STATUS', 9)} ${pad('BLOCK', 9)} ${pad('WHAT', 30)} ADDRESS / TX`);
  for (const r of rows) {
    // WHAT is truncated rather than left to overflow: a full signature would push the address column
    // out of line on one row and not the others.
    const what = r.contract ?? r.function ?? r.note ?? '-';
    say(
      `  ${pad(r.stage, 10)} ${pad(r.status, 9)} ${pad(r.block === null ? '-' : String(r.block), 9)} ` +
        `${pad(what, 30)} ${r.address ?? r.hash ?? '-'}`,
    );
  }

  const reverted = rows.filter((r) => r.status === 'REVERTED').length;
  say('', `  ${rows.length} entries, ${reverted} reverted`, `  raw forge records: ${ctx.broadcastDir}`);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * What has been executed for this deployment, step by step, with the transaction that did it.
 *
 * Reads the journal and the manifest, so — like `log` — it needs no network, no keystore and no
 * foundry: this is the view you want when the RPC is down, when the key has been put away, or when
 * someone asks months later what was deployed and when.
 *
 * The three read-only views answer three different questions, and it is worth keeping them apart:
 *
 *   report   which STEPS ran, and which transactions did them        (reads the journal)
 *   log      every transaction in the order it was sent              (reads the journal)
 *   status   what is DONE and what is BLOCKED, right now, and why    (reads the chain)
 *
 * A step can appear more than once. That is not a bug to hide: re-running a stage after a failure is
 * the normal path here (§2), so a report that collapsed the retries would be hiding the interesting
 * part.
 */
export function stageReport(ctx: Ctx): void {
  const manifest = readJson<Manifest>(manifestPath(ctx));
  const rows = readJournal(ctx);

  say(`📋  report  ${ctx.opt.deploymentId}`);
  if (manifest === null) {
    say(`  no manifest at ${manifestPath(ctx)} - this deployment has not been computed yet.`);
    return;
  }
  say(
    `  chain      ${manifest.chainId ?? '?'}`,
    `  deployer   ${manifest.deployer ?? '?'}`,
    `  admin      ${manifest.admin ?? '?'}`,
    `  sealed     ${manifestPath(ctx)}`,
    '',
    `  ✅  ${pad('compute', 8)} addresses computed and sealed (no transactions)`,
  );

  let executed = 0;
  for (const step of ctx.flow.reportSteps) {
    const entries = rows.filter((r) => r.stage === step.label);

    // One blank line before every step, so a step and its transactions read as one block rather than
    // as an undifferentiated wall.
    say('');

    // 🍔 rather than ⬜ for "not run": the white square is nearly invisible on a light terminal,
    // which is the opposite of what a status mark is for.
    if (entries.length === 0) {
      say(`  🍔  ${pad(step.label, 8)} ${step.title}`);
      say(`      ${'-'.repeat(RULE_WIDTH)}`);
      say('      <Not yet executed>');
      continue;
    }
    executed++;

    // ❌ means "something in this step reverted", not "this step did not achieve its goal" — those
    // are different questions and only the chain can answer the second one, which is what
    // `--stage status` is for. A revert followed by a successful retry still gets the cross, because
    // the point of the mark is to send you to the lines below it.
    const failed = entries.filter((e) => e.status === 'REVERTED').length;
    const suffix = failed > 0 ? `  (${failed} reverted)` : '';
    say(`  ${failed > 0 ? '❌' : '✅'}  ${pad(step.label, 8)} ${step.title}${suffix}`);

    say(`      ${'-'.repeat(RULE_WIDTH)}`);
    for (const e of entries) say(`      ${reportTxLine(e)}`);
  }

  const reverted = rows.filter((r) => r.status === 'REVERTED').length;
  say(
    '',
    `  ${executed}/${ctx.flow.reportSteps.length} steps executed, ${rows.length} transactions, ${reverted} reverted`,
    ...(reverted > 0 ? ['  A reverted create does NOT burn its address (plan section 2) - re-run the stage.'] : []),
  );

  reportAddresses(manifest);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The stack's addresses, from the manifest — the report's conclusion.
 *
 * Read from the seal, not the chain, so this works with no network like the rest of `report`. Which
 * also means it says where the stack IS, not that it is there; `--stage verify` answers that.
 *
 * Split three ways because the three groups answer different questions. The host addresses are what
 * a dApp compiles against and what goes in an SDK config. ACL_OWNER is the trust root — whoever owns
 * it can upgrade any of them. The implementations are what those proxies currently run, and are
 * the only group that changes on an upgrade.
 */
export function reportAddresses(manifest: Manifest): void {
  const addr = manifest.address;
  if (addr === undefined) return;

  const line = (role: string): void => {
    const value = addr[role];
    if (value !== undefined) say(`      ${pad(role, ROLE_WIDTH)} ${value}`);
  };

  say('', '  📇  addresses  the deployed stack, as sealed');
  say(`      ${'-'.repeat(RULE_WIDTH)}`);
  for (const role of HOST_ROLES) line(role);

  say('');
  say(`      ${pad('ACL_OWNER', ROLE_WIDTH)} ${addr.ACL_OWNER ?? '-'}   <- trust root, owned by the admin`);

  say('');
  for (const role of Object.keys(addr)
    .filter((k) => k.startsWith('IMPL_'))
    .sort())
    line(role);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * One transaction in a report: block, status, hash, and what it did.
 *
 * The hash is printed in full rather than abbreviated, because the point of having it here is to
 * paste it into an explorer. Step F's line has no hash when the admin's transaction was merely
 * observed — see recordObservation — and says so rather than printing a misleading blank.
 */
export function reportTxLine(e: JournalEntry): string {
  const block = e.block === null ? 'unmined' : `block ${e.block}`;
  const what = e.contract ?? e.function ?? e.note ?? '-';
  const hash = e.hash ?? '(no local receipt - sent externally)';
  return `${pad(block, 15)} ${pad(e.status, 9)} ${hash}  ${what}`;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * One broadcasting stage.
 *
 * --sender alongside --account: every script requires msg.sender == FHEVM_DEPLOYER, because the whole
 * address set is a function of the deployer (§5.2) and broadcasting from another account produces
 * creates that land where nothing was compiled for.
 *
 * Step F is sent by the ADMIN, not the deployer — that inversion is what Ownable2Step is for — so
 * account and sender are parameters rather than constants.
 */

////////////////////////////////////////////////////////////////////////////////

export async function broadcast(ctx: Ctx, target: string, signer?: Signer, sender?: string): Promise<void> {
  const from = sender ?? ctx.deployer;
  const key = signer ?? ctx.opt.signer;
  if (key === null) {
    fail(`Error: stage '${ctx.opt.stage}' sends transactions but no signer was resolved.`);
  }
  const base = [
    'script',
    `${SCRIPT_DIR}/${target}`,
    '--rpc-url',
    ctx.opt.rpcUrl,
    '--out',
    ctx.buildOut,
    ...traceArgs(ctx),
  ];

  // --dry-run: the same script, simulated, sending nothing.
  //
  // `forge script` WITHOUT --broadcast still simulates the whole run against a fork at the head, so
  // every predicate and precondition executes and reverts exactly as it would for real. That makes
  // it a genuine readiness check rather than a separate code path that can drift from the one that
  // matters. Nothing is signed, so --account is dropped and only --sender is passed.
  //
  // It does not WAIT: a dry run's job is to say whether you are ready now, so a too-early run should
  // fail with the Solidity gate's block countdown rather than block for ten minutes.
  if (ctx.opt.dryRun) {
    say('  (dry run: simulating, nothing will be sent)');
    const env = {
      ...scriptEnv(ctx),
      ...generatedConfigEnv(ctx),
      FHEVM_MIN_BLOCK: String(ctx.opt.minBlockOverride ?? 0),
    };
    const code = run('forge', [...base, '--sender', from], env);
    if (code !== 0) process.exit(code);
    return;
  }

  // §9's gate. No-ops unless this is the deployment's first transaction; never reached by a dry run,
  // which returned above.
  await confirmSealed(ctx);

  requireNoPendingTxs(ctx, from);
  const minBlock = ctx.opt.minBlockOverride ?? ctx.nextMinBlock;
  await waitForBlock(ctx, minBlock);

  const env = { ...scriptEnv(ctx), ...generatedConfigEnv(ctx), FHEVM_MIN_BLOCK: String(minBlock) };

  // --slow: one transaction at a time, waiting for each receipt. §6's two hard edges (impl₁ before
  // the ACL proxy, impl₃ before the rest) are satisfied by nonce ordering alone, but --slow turns a
  // mid-run failure into "stop here" instead of "the rest also fail in the same block".
  //
  // The exit code is captured rather than thrown, so the journal is written even when the stage
  // dies. A half-finished stage is the case the audit trail exists for.
  const code = run('forge', [...base, ...forgeSignerArgs(key), '--sender', from, '--slow', '--broadcast'], env);

  recordJournal(ctx, target);
  if (code !== 0) {
    console.error(`  stage failed (forge exit ${code}). What was sent is in --stage log.`);
    process.exit(code);
  }

  // Derived from the head AFTER the stage rather than from a receipt: --slow means every transaction
  // is already mined by now, so the head is at or past the last of them. Erring later is the safe
  // direction for a reorg gate.
  ctx.finalityTarget = headBlock(ctx);
  ctx.nextMinBlock = ctx.finalityTarget + ctx.opt.confirmations;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Refuse to proceed on an out dir that has no artifacts in it.
 *
 * `--no-build` skips the compile, so this is the only thing standing between "reuse what is there"
 * and "compute addresses from nothing". A missing artifact makes `vm.getCode` return empty, and an
 * empty initcode still hashes to something — it just hashes to the wrong thing, silently.
 *
 * Spot-checks the artifacts every pass reads rather than counting files, so a half-written or
 * partially cleaned out dir fails here rather than three passes later.
 */

////////////////////////////////////////////////////////////////////////////////

export function requireBuiltArtifacts(ctx: Ctx): void {
  const required = [
    'ERC1967Proxy.sol/ERC1967Proxy.json',
    'EmptyUUPSProxyACL.sol/EmptyUUPSProxyACL.json',
    'EmptyUUPSProxy.sol/EmptyUUPSProxy.json',
    'PauserSet.sol/PauserSet.json',
    'ACLOwner.sol/ACLOwner.json',
    'ACL.sol/ACL.json',
  ];
  const missing = required.filter((r) => !existsSync(join(ctx.buildOut, r)));
  if (missing.length === 0) return;

  fail(
    `Error: --no-build, but ${ctx.buildOut} is missing ${missing.length} of ${required.length} artifacts:`,
    ...missing.map((m) => `         ${m}`),
    '       Run once without --no-build to populate it.',
  );
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Compute the address set — the three-build pipeline (§5.3).
 *
 * Each pass is: build, then compute. The build comes FIRST every time, because a pass computes an
 * address by hashing bytecode, and that bytecode has to already contain whatever the previous pass
 * worked out.
 *
 *   pass 1   build, then compute the ACL address and write it into addresses.sol
 *   pass 2   rebuild (contracts now hold the real ACL address), then compute every other address
 *   pass 3   rebuild (implementations now hold every address), check nothing moved, seal
 *
 * An address depends on the bytecode, and the bytecode contains addresses. forge cannot recompile
 * itself mid-run, which is why this stage is here and not in Solidity.
 */

////////////////////////////////////////////////////////////////////////////////

export async function confirmSealed(ctx: Ctx): Promise<void> {
  if (ctx.opt.noConfirm) return;

  // Fires on the CONDITION — "this deployment has never sent a transaction" — rather than on how the
  // run was invoked. It used to hang off the `--stage all` branch, which meant the manual path, the
  // one a real deployment actually uses, was never asked at all. An empty journal is the same signal
  // `compute` uses to decide whether recomputing is still safe.
  if (readJsonl<JournalEntry>(ctx.journalPath).length > 0) return;

  // `--no-git` / `"git": false` — this deployment does not need a committed seal at all.
  //
  // NOT the same claim as --no-confirm, which asserts the seal HAS been pushed and stays silent.
  // This one says the seal does not matter here, and warns every time, because it is a real loss:
  // without it a failed create cannot be retried at the same address, so a half-finished stack is
  // unfinishable. Legitimate for a throwaway rehearsal, wrong for anything standing.
  if (!ctx.opt.requireGitSeal) {
    warn(
      'no git seal for this deployment (--no-git).',
      'If a stage fails midway there is no committed record of the init-code hashes,',
      'and this stack cannot be resumed. Fine for a rehearsal.',
    );
    return;
  }

  // Relative to the package root, which main() has already chdir'd to, so the lines below can be
  // pasted straight into the shell this was launched from.
  const dir = relative(PACKAGE_ROOT, ctx.outDir);

  say(
    '',
    '  ---------------------------------------------------------------------------',
    '  About to send the FIRST transaction for this deployment.',
    '',
    '  GIT COMMIT AND PUSH the seal first. It is not a formality: the addresses ARE',
    '  the init-code hashes, so retrying a failed create needs the byte-exact ones,',
    '  and a resumed run computes which addresses to probe from them. Lose the seal',
    '  and a half-finished stack cannot be finished.',
    '',
    `    git add -f ${dir}/manifest.json ${dir}/addresses.sol`,
    `    git commit -m "seal: ${ctx.opt.deploymentId}"`,
    '    git push',
    '  ---------------------------------------------------------------------------',
  );
  if (!(await confirm('  Pushed to git? [y/N] '))) fail('Aborted before the first transaction.');
}

////////////////////////////////////////////////////////////////////////////////
