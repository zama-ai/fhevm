// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// DRAFT — see README.md. Deploy a cleartext FHEVM stack to a public EVM testnet via the canonical
// CREATE2 factory. Coordinator for create2-deploy/script/*.
//
//   node create2-deploy/deploy-testnet.ts --help
//
// Plain `node` (>= 22.6) runs this directly — types are stripped at load, no tsx, no build step, no
// dependencies. That constrains the syntax to the erasable subset: no `enum`, no `namespace`, no
// parameter properties, and relative imports carry their `.ts` extension.
//
// ---------------------------------------------------------------------------------------------
// Why a coordinator exists at all, rather than one `forge script`
// ---------------------------------------------------------------------------------------------
//
// Two things forge cannot do inside a single run, and they are the two things this path needs:
//
//   1. RECOMPILE MID-RUN. EmptyUUPSProxy and PauserSet bake aclAdd as a compiled-in immediate, and
//      this path forbids bytecode patching, so their init-code hashes — and therefore their
//      addresses — only exist after a build against a config holding the real aclAdd. Hence three
//      passes with two rebuilds between them (§5.3). This is the largest piece of work CREATE2 adds
//      over the nonce path.
//
//   2. WAIT FOR A TRANSACTION FROM SOMEONE ELSE. Step E only offers; ACLOwner is Ownable2Step and
//      the admin must send acceptOwnership() from its own key. Nothing here can produce that.
//
// The nonce path (scripts/deploy.sh) is UNTOUCHED and remains the only path for chain 31337.
// This adds a second path; it replaces nothing.

import { existsSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';

import {
  appendJsonl,
  capture,
  captureOrFail,
  confirm,
  ensureDir,
  fail,
  hexToNumber,
  isInside,
  pad,
  readJson,
  readJsonl,
  removeIfPresent,
  requireTool,
  run,
  sameAddress,
  say,
  sleep,
  warn,
} from './utils.ts';

////////////////////////////////////////////////////////////////////////////////

export type Stage =
  | 'compute'
  | 'creates'
  | 'pausers'
  | 'offer-acl'
  | 'accept-acl'
  | 'materialize'
  | 'offer-admin'
  | 'accept-admin'
  | 'verify'
  | 'status'
  | 'log'
  | 'report'
  | 'all';

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
};

/** Command line before the config file is merged under it. `null` means "not given". */
type CliArgs = {
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
};

/** Mutable run state. The shell version kept these as globals; they are threaded explicitly here. */
export type Ctx = {
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
type ForgeRun = {
  readonly timestamp?: number;
  readonly transactions?: readonly {
    readonly hash: string;
    readonly transactionType?: string;
    readonly contractName?: string;
    readonly contractAddress?: string;
    readonly function?: string | null;
  }[];
  readonly receipts?: readonly {
    readonly transactionHash: string;
    readonly blockNumber?: string;
    readonly status?: string;
    readonly gasUsed?: string;
  }[];
};

type Manifest = {
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
const DRAFT_DIR = import.meta.dirname;
const PACKAGE_ROOT = dirname(DRAFT_DIR);
/** Relative on purpose: forge resolves script paths against the project root it runs in. */
const SCRIPT_DIR = 'create2-deploy/script';

/** Baked into every salt. MAJOR_MINOR only — a patch release must not move the addresses. */
const FHEVM_VERSION = '0.13';
const CONFIG_PREFIX = `fhevm-config-${FHEVM_VERSION}.0/`;

const FACTORY = '0x4e59b44847b379578588920cA78FbF26c0B4956C';

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
const FACTORY_CODEHASH = '0x2fa86add0aed31f33a762c9d88e807c475bd51d0f52bd0955754b2608f7e4989';

/**
 * §1: testnets only. This is the cleartext stack — FHE is replaced by plaintext and the KMS /
 * coprocessor signer keys derive from the published FHEVM_MNEMONIC at documented HD paths. On a
 * testnet that is the POINT: the js-sdk relayer must hold those keys for cleartext decryption to
 * work. On mainnet it is total compromise.
 *
 * Read §11 R1 before trusting this list to do more than it does. It binds OUR tooling and nobody
 * else's — the address set is replayable onto mainnet by anyone, and no allow-list here can stop it.
 */
const ALLOWED_CHAIN_IDS: readonly string[] = ['11155111', '17000', '84532', '421614'];

/** Where forge may write. Must match foundry.toml's fs_permissions — see resolveOutDir. */
const FS_ROOT = DRAFT_DIR;

/** Looked for when --config is not given. Its absence is not an error. */
const DEFAULT_CONFIG_NAME = 'deploy.config.json';

/** Every key a config file may carry — anything else is a typo, and typos here select addresses. */
const CONFIG_KEYS: readonly string[] = [
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
];

/** Rejected in a config file by name, so the message can say where they belong instead. */
const CLI_ONLY_KEYS: readonly string[] = ['stage', 'dryRun', 'minBlock', 'noConfirm', 'report', 'noBuild', 'verbose'];

const ALL_STAGES: readonly Stage[] = [
  'compute',
  'creates',
  'pausers',
  'offer-acl',
  'accept-acl',
  'materialize',
  'offer-admin',
  'accept-admin',
  'verify',
  'status',
  'log',
  'report',
  'all',
];

/**
 * The addresses a consumer needs, in the order the generated addresses.sol declares them.
 *
 * These ten are the stack as far as anything outside it is concerned. ACL_OWNER and the eleven
 * implementations are reported separately: they are how the stack is GOVERNED and what it currently
 * RUNS, not what a dApp is compiled against.
 */
const HOST_ROLES: readonly string[] = [
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
const ROLE_WIDTH = 34;

/** Width of the rule under a report step's header. Sized to the block+status+hash columns. */
const RULE_WIDTH = 92;

/**
 * The steps a report accounts for, in plan order, keyed by the label each stage tags its journal
 * entries with. `compute` is absent because it sends nothing — its evidence is the manifest.
 */
const REPORT_STEPS: readonly { readonly label: string; readonly title: string }[] = [
  { label: 'creates', title: 'every CREATE2 through the factory' },
  { label: "A/A'", title: 'register the pausers' },
  { label: 'B', title: 'offer ACL ownership to the ACLOwner (offers only)' },
  { label: 'C', title: 'accept ACL ownership - ownership MOVES here' },
  { label: 'D', title: 'materialize the stack (one atomic tx)' },
  { label: 'E', title: 'offer the ACLOwner to the admin (offers only)' },
  { label: 'F', title: 'admin accepts - the deployer is no longer root' },
];

/** The broadcasting stages of a full run, in plan order. */
const RUN_ORDER: readonly Stage[] = [
  'creates',
  'pausers',
  'offer-acl',
  'accept-acl',
  'materialize',
  'offer-admin',
  'accept-admin',
];

const HELP = `
Usage: node create2-deploy/deploy-testnet.ts --rpc-url URL --account NAME
                                                   --admin 0x... --deployment-id ID [options]

  --rpc-url URL        node to deploy to (required)
  --account NAME       forge keystore account to broadcast from. Required on every chain EXCEPT a
                       local anvil: omit it there and accounts 0 and 1 of anvil's public mnemonic are
                       used as deployer and admin, so a rehearsal needs no keystore. The node must
                       answer anvil_nodeInfo or this is refused — see plan section 12
  --admin 0x...        final owner of ACLOwner. Mandatory, no default (plan §7) — except under the
                       anvil default above, where it is anvil account 1
  --deployment-id ID   operator-chosen string; a fresh one gives a disjoint address set (§14.2)
  --pauser 0x...       optional operator pauser, step A' (§6.1)
  --confirmations N    reorg DEPTH floor for the between-stage waits (default 3, §11 R2). This is
                       the value the Solidity gate enforces, so it is the one a different
                       orchestrator also has to honour
  --no-finality        between stages wait only for --confirmations of depth, NOT for the previous
                       stage to finalize. Depth is a heuristic — ~3 min at 15 blocks vs ~12.8 min to
                       PoS finality — and testnets are where that gap bites
  --admin-account NAME forge keystore account that SIGNS step F on the admin's behalf. Does not
                       replace --admin, which stays the authoritative address and is sealed in the
                       manifest; this must resolve to the same account. Without it, step F polls
                       until the admin's own transaction lands (the multisig case)
  --out-dir PATH       where this deployment's seal, generated config and journal are written
                       (default: .out). Relative to create2-deploy/, and MUST stay inside it:
                       forge writes only where foundry.toml's fs_permissions allows, which is static
                       config. ONE PER (chain, deployment-id)
  --dry-run            run the chosen stage WITHOUT --broadcast. Same script, same predicates, same
                       preconditions, simulated against the head. Not valid with --stage all
  --min-block N        FHEVM_MIN_BLOCK for a single manual --stage run: steps A-F refuse to start
                       until the chain reaches block N. Derived per stage in \`all\` mode
  --no-confirm         do not ask "have you pushed the seal to git?" before the deployment's
                       FIRST transaction. That prompt fires once per deployment, whichever stage
                       sends it; this ASSERTS you have already pushed
  -v, --verbose        pass -vvvv to forge, printing its execution traces. Those traces are the
                       only place a view call shows up: forge runs the script in its own EVM
                       against a fork, so a getter never reaches the node and never appears in
                       anvil's log
  --no-build           skip the three forge builds in compute and reuse the artifacts already in
                       the out dir. Refuses if they are not there. It cannot tell whether they are
                       up to date — that is what you assert by passing it
  --no-git             this deployment needs no git-committed seal at all. Different claim from
                       --no-confirm: it does not skip a question you have answered, it drops the
                       requirement, and warns every run. Without a seal a failed create cannot be
                       retried at the same address, so a half-finished stack is unfinishable.
                       For throwaway rehearsals. Also settable as "git": false in a config file
  --config PATH        JSON file holding the stable arguments, so they are not retyped every
                       invocation: rpcUrl, account, admin, deploymentId, pauser, adminAccount,
                       confirmations, outDir, finality. An explicit flag always overrides it.
                       Without --config, ./create2-deploy/deploy.config.json is used if it
                       exists. Unknown keys are rejected, and so are stage/dryRun/minBlock/noConfirm —
                       those are what one invocation DOES, not what the deployment IS

  --stage STAGE        one of, in order:
                         compute       3 builds + 3 passes, writes the manifest      (no tx)
                         creates       every CREATE2 through the factory
                         pausers       A, A'  register the pausers
                         offer-acl     B      ACL.transferOwnership(ACLOwner)   — offers only
                         accept-acl    C      ACLOwner.acceptACLOwnership()     — ownership MOVES
                         materialize   D      ACLOwner.upgrade(ops)             — one atomic tx
                         offer-admin   E      ACLOwner.transferOwnership(admin) — offers only
                         accept-admin  F      ACLOwner.acceptOwnership()  — SENT BY THE ADMIN
                         verify               the §7 terminal conditions        (no tx)
                       or \`all\` (default) to run every one of them in that order.
                       Three more are accepted out of band, and none of them sends anything:
                         status        what is done, what is left, and WHY   (reads the chain)
                         report        which STEPS ran, with the tx hash and block that did each
                         log           every transaction, in the order it was sent
                       report and log read only local files: no RPC, no keystore, no foundry.
                       --report is accepted as shorthand for --stage report
`;

////////////////////////////////////////////////////////////////////////////////

/**
 * Refuse a raw private key where a keystore NAME is expected.
 *
 * §12: the deployer key owns the ACLOwner — root over the whole stack — until step F completes, so
 * this path accepts keystore accounts only. scripts/deploy.sh takes a raw --private-key because
 * chain 31337 keys are throwaway; "testnet" is not "throwaway" here, since these stacks are what the
 * js-sdk integration story runs against.
 *
 * Without this the value still fails — `cast` reports `Keystore file "…/0xac09…" does not exist` —
 * but only after PRINTING THE KEY to stderr, and from there to scrollback, CI logs, or a pasted bug
 * report. So the check exists for the error message, not for the failure. Note it deliberately never
 * echoes the value back.
 */
/**
 * The mnemonic anvil prints on startup and funds accounts 0..9 from. Hardcoding it is safe precisely
 * because it is public: every anvil in the world uses it, so it protects nothing and can leak nothing.
 *
 * It exists so a LOCAL REHEARSAL needs no keystore. Rehearsing the CREATE2 path is how you find out
 * that a salt, an ordinal or a stage gate is wrong, and requiring an unlockable keystore for that put
 * the rehearsal behind a password prompt — which meant it did not get run.
 */
const ANVIL_MNEMONIC = 'test test test test test test test test test test test junk';

/** Account 0 deploys; account 1 is the admin that takes root in step F. Matches anvil-config.json. */
const ANVIL_DEPLOYER_INDEX = 0;
const ANVIL_ADMIN_INDEX = 1;

/**
 * How a stage signs: a forge keystore account, or an index into the public anvil mnemonic.
 *
 * A tagged union rather than a nullable account name because the two produce DIFFERENT forge flags,
 * and because it puts the anvil-only restriction in one place — `resolveSigner` is the only thing that
 * can mint the `anvil` variant, and it refuses unless the node answers `anvil_nodeInfo`.
 */
type Signer =
  { readonly kind: 'keystore'; readonly account: string } | { readonly kind: 'anvil'; readonly index: number };

/**
 * Flags for `forge script`. Note the PLURAL names: forge takes `--mnemonics` / `--mnemonic-indexes`,
 * while `cast wallet` takes the singular `--mnemonic` / `--mnemonic-index`. They are not interchangeable,
 * and passing the wrong pair fails with an unhelpful clap error.
 */
function forgeSignerArgs(signer: Signer): string[] {
  return signer.kind === 'keystore'
    ? ['--account', signer.account]
    : ['--mnemonics', ANVIL_MNEMONIC, '--mnemonic-indexes', String(signer.index)];
}

/** Flags for `cast wallet address` — the singular spellings. */
function castSignerArgs(signer: Signer): string[] {
  return signer.kind === 'keystore'
    ? ['--account', signer.account]
    : ['--mnemonic', ANVIL_MNEMONIC, '--mnemonic-index', String(signer.index)];
}

function signerAddress(signer: Signer): string {
  return captureOrFail('cast', ['wallet', 'address', ...castSignerArgs(signer)]);
}

function describeSigner(signer: Signer): string {
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
function isAnvil(rpcUrl: string): boolean {
  return capture('cast', ['rpc', 'anvil_nodeInfo', '--rpc-url', rpcUrl]).ok;
}

function rejectRawPrivateKey(flag: string, value: string): void {
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

function parseCliArgs(argv: readonly string[]): CliArgs {
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
  };

  const need = (i: number, flag: string): string => {
    const v = argv[i + 1];
    if (v === undefined) fail(`Error: ${flag} requires a value.`);
    return v;
  };

  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
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
        cli.stage = need(i, a) as Stage;
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
      case '-h':
      case '--help':
        say(HELP);
        process.exit(0);
      default:
        fail(`Error: unknown argument '${a}'. Try --help.`);
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
function loadConfigFile(explicitPath: string | null): { readonly cfg: ConfigFile; readonly path: string | null } {
  const path = explicitPath ?? join(FS_ROOT, DEFAULT_CONFIG_NAME);

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

  return { cfg: cfg as ConfigFile, path };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Merge the config file under the command line, then validate.
 *
 * PRECEDENCE: an explicit flag always wins. That is what makes a config file safe to keep around —
 * `--stage creates --dry-run` against a pinned deployment needs no other arguments, and a one-off
 * `--rpc-url` override does not require editing the file.
 */
function resolveOptions(cli: CliArgs, cfg: ConfigFile, configPath: string | null): Options {
  const stage: Stage = cli.stage ?? 'all';
  if (!ALL_STAGES.includes(stage)) {
    fail(`Error: unknown --stage '${stage}'.`, `       one of: ${ALL_STAGES.join(', ')}`);
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

  if (!rpcUrl) fail(missing('--rpc-url', 'rpcUrl'));

  // Before the value reaches `cast`, which would print it in its own error. See §12.
  if (account) rejectRawPrivateKey('--account', account);
  if (adminAccount !== null) rejectRawPrivateKey('--admin-account', adminAccount);

  // --account is optional in exactly one case: a local anvil, where the funded accounts come from a
  // mnemonic that is public knowledge. Anywhere else it stays mandatory, and §12's keystone holds —
  // the deployer key owns ACLOwner until step F, so a testnet run must not accept an unprotected key.
  //
  // The probe runs only when the operator has actually omitted --account, so an unreachable node
  // cannot break the flows that supplied one. Read-only stages sign nothing and skip it entirely.
  let signer: Signer | null = null;
  if (account) {
    signer = { kind: 'keystore', account };
  } else if (needsChain(stage)) {
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
  if (!admin && adminSigner?.kind === 'anvil') {
    admin = signerAddress(adminSigner);
  }
  if (!admin) fail(missing('--admin', 'admin') + ' (plan section 7)');
  if (!deploymentId) fail(missing('--deployment-id', 'deploymentId') + ' (plan section 14.2)');

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
function resolveOutDir(outDirArg: string | null): string {
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
function needsChain(stage: Stage): boolean {
  return stage !== 'log' && stage !== 'report';
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Which stages need the deployer resolved from the KEYSTORE, at the cost of a password prompt.
 *
 * A foundry keystore holds no plaintext address — only `crypto`, `id`, `version` — so
 * `cast wallet address --account` has to decrypt, and that means asking. Worth it for anything that
 * derives an address from the deployer or signs with it; pure nuisance for a read-only look at a
 * deployment that already recorded its deployer in the manifest.
 *
 * `compute` is on this side of the line and must stay there: the whole address set is a function of
 * the deployer (§5.2), so taking it from anywhere but the key itself would let a typo seal a stack
 * nobody holds the key to.
 */
function needsDeployerKey(stage: Stage): boolean {
  return stage !== 'verify' && stage !== 'status' && stage !== 'report' && stage !== 'log';
}

////////////////////////////////////////////////////////////////////////////////

function buildContext(opt: Options): Ctx {
  const outDir = resolveOutDir(opt.outDirArg);

  // §12: the deployer key owns ACLOwner — root over the stack — until step F completes. Keystore
  // only. A raw private key is accepted by scripts/deploy.sh for 31337 and is NOT accepted here.
  // "Testnet" is not "throwaway": these stacks are what the js-sdk integration story runs against.
  //
  // Read-only stages take the deployer off the manifest instead, and so never unlock anything. That
  // does make checkOutDirIdentity's deployer comparison vacuous for them — which is the point: that
  // check guards stages that SEND, and there is nothing to protect when nothing is sent.
  let deployer = '';
  if (!needsDeployerKey(opt.stage)) {
    deployer = readJson<Manifest>(join(outDir, 'manifest.json'))?.deployer ?? '';
  }
  if (deployer === '' && needsChain(opt.stage)) {
    deployer = opt.signer === null ? '' : signerAddress(opt.signer);
  }

  return {
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

function manifestPath(ctx: Ctx): string {
  return join(ctx.outDir, 'manifest.json');
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The environment every forge script in this path reads. No script takes a CLI argument.
 *
 * FHEVM_DEPLOYER is an ADDRESS, not a key: the scripts only predict with it and check it against
 * msg.sender, while forge authenticates via --account/--sender. No script here ever holds a key.
 */
function scriptEnv(ctx: Ctx, extra?: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  return {
    FHEVM_VERSION,
    FHEVM_DEPLOYMENT_ID: ctx.opt.deploymentId,
    FHEVM_DEPLOYER: ctx.deployer,
    FHEVM_ADMIN: ctx.opt.admin,
    FHEVM_CONFIRMATIONS: String(ctx.opt.confirmations),
    FHEVM_OUT_DIR: ctx.outDir,
    ...(ctx.opt.pauser ? { FHEVM_PAUSER_0: ctx.opt.pauser } : {}),
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
function traceArgs(ctx: Ctx): readonly string[] {
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
function generatedConfigEnv(ctx: Ctx): NodeJS.ProcessEnv {
  return { FOUNDRY_REMAPPINGS: `${CONFIG_PREFIX}=${ctx.outDir}/` };
}

////////////////////////////////////////////////////////////////////////////////

function checkChainAllowed(ctx: Ctx): void {
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
function checkOutDirIdentity(ctx: Ctx): void {
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
function checkAdminAccount(ctx: Ctx): void {
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
function checkFactory(ctx: Ctx): void {
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
function probeFinality(ctx: Ctx): string {
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
function preflight(ctx: Ctx): void {
  say('🔎  preflight');

  ctx.chainId = captureOrFail('cast', ['chain-id', '--rpc-url', ctx.opt.rpcUrl]);
  checkChainAllowed(ctx);
  checkOutDirIdentity(ctx);
  if (needsDeployerKey(ctx.opt.stage)) checkAdminAccount(ctx);
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

function headBlock(ctx: Ctx): number {
  return Number(captureOrFail('cast', ['block-number', '--rpc-url', ctx.opt.rpcUrl]));
}

////////////////////////////////////////////////////////////////////////////////

function finalizedBlock(ctx: Ctx): number {
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
async function waitForBlock(ctx: Ctx, target: number): Promise<void> {
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
function requireNoPendingTxs(ctx: Ctx, who: string): void {
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
function recordJournal(ctx: Ctx, target: string): void {
  const file = target.split(':')[0];
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
function recordObservation(ctx: Ctx, stage: string, note: string, block: number): void {
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
function readJournal(ctx: Ctx): JournalEntry[] {
  const seen = new Set<string>();
  return readJsonl<JournalEntry>(ctx.journalPath).filter((r) => {
    if (r.hash === null || r.hash === undefined) return true;
    if (seen.has(r.hash)) return false;
    seen.add(r.hash);
    return true;
  });
}

////////////////////////////////////////////////////////////////////////////////

/** What has been executed. The other half of `--stage status`, which says what remains. */
function showJournal(ctx: Ctx): void {
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
function stageReport(ctx: Ctx): void {
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
  for (const step of REPORT_STEPS) {
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
    `  ${executed}/${REPORT_STEPS.length} steps executed, ${rows.length} transactions, ${reverted} reverted`,
    ...(reverted > 0 ? ['  A reverted create does NOT burn its address (plan section 2) - re-run the stage.'] : []),
  );

  reportAddresses(ctx, manifest);
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
function reportAddresses(ctx: Ctx, manifest: Manifest): void {
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
function reportTxLine(e: JournalEntry): string {
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
async function broadcast(ctx: Ctx, target: string, signer?: Signer, sender?: string): Promise<void> {
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
function requireBuiltArtifacts(ctx: Ctx): void {
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
function stageCompute(ctx: Ctx): void {
  say('🧮  compute (3 passes, 2 rebuilds)');

  // Recomputing after transactions have been sent would move the sealed address set out from under a
  // stack that is already partly deployed — the creates stage would then either report drift or,
  // worse, start building a second disjoint set alongside the first. §14.2 is explicit that a
  // redeploy takes a FRESH deploymentId. Preflight rejects the mismatches, so reaching here with a
  // non-empty journal really does mean "this deployment has already sent transactions".
  if (readJsonl<JournalEntry>(ctx.journalPath).length > 0) {
    fail(
      `Error: '${ctx.opt.deploymentId}' has already sent transactions (see ${ctx.journalPath}),`,
      '       so its addresses are not safe to recompute.',
      '       For a new address set, use a fresh --deployment-id AND a fresh --out-dir:',
      '         --deployment-id <new-id> --out-dir .out-<new-id>',
      `       To discard this deployment's record and start it over: rm -rf ${ctx.outDir}`,
    );
  }

  // Clears only what compute itself produces. NOT the whole out dir: that would also take
  // journal.jsonl and broadcast/, which are the audit trail (§9) and belong to the deploy stages.
  ensureDir(ctx.outDir);
  removeIfPresent(ctx.buildOut, join(ctx.outDir, 'addresses.sol'), join(ctx.outDir, 'pass2.json'), manifestPath(ctx));

  const script = `${SCRIPT_DIR}/FhevmComputeCreate2Addresses.s.sol:FhevmComputeCreate2Addresses`;
  // --no-build: trust what is already in the out dir rather than rebuilding.
  //
  // The three builds are the slow part of compute, and on a rehearsal you re-run it constantly
  // against unchanged sources. Skipping them is only safe if the artifacts are actually there — a
  // pass reads initcode with `vm.getCode`, and an absent artifact makes it compute an address from
  // nothing. So the skip is not silent: it checks, and refuses if the out dir is not populated.
  //
  // It cannot check whether the artifacts are UP TO DATE — that is what you are asserting by passing
  // the flag. Pass 3's assertion is still the backstop: if the artifacts disagree with what pass 2
  // computed from, the init-code hashes will not match and the seal fails rather than proceeding.
  const build = (env: NodeJS.ProcessEnv): void => {
    if (ctx.opt.noBuild) {
      requireBuiltArtifacts(ctx);
      say('  (--no-build: using the artifacts already in the out dir)');
      return;
    }
    if (run('forge', ['build', '--out', ctx.buildOut, '--skip', 'test'], env) !== 0) {
      fail('Error: forge build failed.');
    }
  };
  // --rpc-url, even though a pass computes addresses and touches no chain state.
  //
  // Without it forge runs the script against its own in-memory EVM, where `block.chainid` is 31337 —
  // and _loadConfig reads exactly that into the manifest. Every deployment would then be sealed as
  // chain 31337 whatever it was really for, and preflight's identity check would block the very
  // deployment it had just created, on any chain but that one.
  const pass = (n: number, env: NodeJS.ProcessEnv): void => {
    const args = ['script', script, '--out', ctx.buildOut, '--rpc-url', ctx.opt.rpcUrl, ...traceArgs(ctx)];
    if (run('forge', args, { ...env, FHEVM_PASS: String(n) }) !== 0) {
      fail(`Error: compute pass ${n} failed.`);
    }
  };

  // Pass 1 builds against the COMMITTED placeholder config: EmptyUUPSProxyACL and ERC1967Proxy
  // reference no host address, so pass 1 is independent of its own output.
  const base = scriptEnv(ctx);
  say('--- pass 1: ACL');
  build(base);
  pass(1, base);

  // From here on the contracts must see the generated config — pass 1's output is pass 2's input.
  const withConfig = { ...base, ...generatedConfigEnv(ctx) };

  say('--- pass 2: proxies, PauserSet, ACLOwner');
  build(withConfig);
  pass(2, withConfig);

  say('--- pass 3: implementations, assert, seal');
  build(withConfig);
  pass(3, withConfig);

  // `forge script` can report success for a run that reverted, so check the artifact, not the code.
  if (!existsSync(manifestPath(ctx))) fail('Error: pass 3 wrote no manifest.json.');
  say('', `  sealed: ${manifestPath(ctx)}`);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The seal must be committed AND PUSHED before any transaction (§9) — for a stronger reason than
 * audit trail. The addresses are a function of the init-code hashes, so retrying a failed create
 * needs the byte-exact ones, and a resumed run's first act is computing which addresses to probe.
 * Lose the seal and a half-finished stack is unfinishable.
 *
 * NOT automated: pushing to a shared remote is the operator's call, not this script's.
 */
async function confirmSealed(ctx: Ctx): Promise<void> {
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

async function stageCreates(ctx: Ctx): Promise<void> {
  say('🧱  creates (one CREATE2 per create, each gated on getCode)');
  ctx.stageLabel = 'creates';
  await broadcast(ctx, 'FhevmDeployCreates.s.sol:FhevmDeployCreates');
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Steps A and A'. The only part of the sequence that is not the ownership handover, and the only
 * part still reachable after the run, via ACLOwner.execute (§6.1).
 *
 * Needs ACL.owner() == deployer, which stops being true at step C — so running accept-acl without a
 * prior pausers stage fails there, on FhevmAcceptACLOwnership's PauserSet.isPauser(ACLOwner) gate,
 * rather than producing a stack with no reachable emergency stop.
 */
async function stepARegisterPausers(ctx: Ctx): Promise<void> {
  say("🚨  pausers (steps A, A')");
  ctx.stageLabel = "A/A'";
  await broadcast(ctx, 'FhevmRegisterPausers.s.sol:FhevmRegisterPausers');
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Step B. Needed no gate invented for it — step C's §8 precondition is already
 * ACL.pendingOwner() == aclOwner, and nothing but this stage can make that true.
 *
 * It only OFFERS. ACL is Ownable2Step, so ACL.owner() is still the deployer after this returns, and
 * `pausers` is equally callable before or after it.
 */
async function stepBOfferAclOwnership(ctx: Ctx): Promise<void> {
  say('📤  offer ACL ownership (step B)');
  ctx.stageLabel = 'B';
  await broadcast(ctx, 'FhevmOfferACLOwnership.s.sol:FhevmOfferACLOwnership');
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Step C — where ownership actually MOVES. Everything gated on ACL.owner() answers "the ACLOwner"
 * from here on, so steps A and A' must already have landed; C checks for them rather than trusting
 * this file's ordering.
 */
async function stepCAcceptAclOwnership(ctx: Ctx): Promise<void> {
  say('🚚  accept ACL ownership (step C)');
  ctx.stageLabel = 'C';
  await broadcast(ctx, 'FhevmAcceptACLOwnership.s.sol:FhevmAcceptACLOwnership');
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Step D — the empty proxies become the real stack, in ONE transaction. The atomicity is why this
 * stage cannot be resumed halfway: see the tri-state note in FhevmMaterializeStack.
 */
async function stepDMaterializeStack(ctx: Ctx): Promise<void> {
  say('🍔  materialize the stack (step D)');
  ctx.stageLabel = 'D';
  await broadcast(ctx, 'FhevmMaterializeStack.s.sol:FhevmMaterializeStack');
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Step E — the deployer gives up root. Only OFFERS; §8 gives it no precondition on D, so the script
 * warns rather than refuses if the stack is not materialized (see its header).
 */
async function stepEOfferOwnerToAdmin(ctx: Ctx): Promise<void> {
  say('🥬  offer the ACLOwner to the admin (step E)');
  ctx.stageLabel = 'E';
  await broadcast(ctx, 'FhevmOfferACLOwnerToAdmin.s.sol:FhevmOfferACLOwnerToAdmin');
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Step F — ACLOwner.acceptOwnership(), sent BY THE ADMIN. The transaction that ends the deployment.
 *
 * The plan has no step F: §6 stops at E, and §7 describes this only as prose — "the admin must send
 * acceptOwnership()… the runner waits for and verifies it". That prose is a step. It has a sender, a
 * predicate, a precondition, and a §7 terminal condition that fails without it, and until it lands
 * the DEPLOYER still holds ACLOwner.execute — an unrestricted call as ACL.owner(), i.e. root.
 *
 * Two paths, because the admin is not necessarily a key we can sign with:
 *
 *   --admin-account NAME   a forge keystore account for the admin: send it, gated like every other
 *                          step. This is the local-key / single-signer case.
 *   (not given)            the multisig case §7 is really written for. Nobody here can produce that
 *                          transaction, so POLL until it lands. Ctrl-C is safe: nothing is in flight
 *                          and `--stage verify` picks up wherever it got to.
 */
async function stepFAcceptOwnershipAsAdmin(ctx: Ctx): Promise<void> {
  say('📥  accept ownership as the admin (step F)');
  ctx.stageLabel = 'F';

  const manifest = readJson<Manifest>(manifestPath(ctx));
  const aclOwner = manifest?.address?.ACL_OWNER;
  if (aclOwner === undefined) fail(`Error: no ACL_OWNER in ${manifestPath(ctx)} - run compute first.`);

  // Already proved to resolve to --admin, in preflight.
  if (ctx.opt.adminSigner !== null) {
    await broadcast(
      ctx,
      'FhevmAcceptOwnershipAsAdmin.s.sol:FhevmAcceptOwnershipAsAdmin',
      ctx.opt.adminSigner,
      ctx.opt.admin,
    );
    return;
  }

  if (ctx.opt.dryRun) {
    say("  (dry run: no --admin-account, so this stage would poll for the admin's transaction)");
    return;
  }

  say(
    '  No --admin-account given, so this stage cannot send the transaction.',
    '  The admin must send it from its own key:',
    '',
    `    cast send ${aclOwner} 'acceptOwnership()' --rpc-url ${ctx.opt.rpcUrl} --account <admin>`,
    '',
    '  Waiting for it to land. Ctrl-C is safe - nothing is in flight, and',
    "  '--stage verify' will pick up from wherever this got to.",
  );

  for (;;) {
    const owner = captureOrFail('cast', ['call', aclOwner, 'owner()(address)', '--rpc-url', ctx.opt.rpcUrl]);
    if (sameAddress(owner, ctx.opt.admin)) break;
    await sleep(15000);
  }

  say('  F  accepted. The deployer key is no longer root over this stack.');
  ctx.finalityTarget = headBlock(ctx);
  ctx.nextMinBlock = ctx.finalityTarget + ctx.opt.confirmations;
  recordObservation(ctx, 'F', 'admin accepted ACLOwner ownership (sent externally)', ctx.finalityTarget);
}

////////////////////////////////////////////////////////////////////////////////

/** §7's terminal conditions. Reverts non-zero if any is unmet. */
function stageVerify(ctx: Ctx): void {
  say('✅  verify');
  const code = run(
    'forge',
    [
      'script',
      `${SCRIPT_DIR}/FhevmVerify.s.sol:FhevmVerify`,
      '--rpc-url',
      ctx.opt.rpcUrl,
      '--out',
      ctx.buildOut,
      ...traceArgs(ctx),
    ],
    { ...scriptEnv(ctx), ...generatedConfigEnv(ctx) },
  );
  if (code !== 0) process.exit(code);
}

////////////////////////////////////////////////////////////////////////////////

/**
 * What is done, what is left, and why. Read-only, and — unlike `verify` — it does NOT fail on a bad
 * stack: it is meant to be run WHEN something is wrong, so it classifies and reports instead of
 * stopping at the first problem.
 *
 * `verify` answers "is this stack correct and finished?" with an exit code. This answers "where did
 * I get to, and what is stopping the next step?" with a board.
 */
function stageStatus(ctx: Ctx): void {
  say('📊  status');
  run(
    'forge',
    [
      'script',
      `${SCRIPT_DIR}/FhevmStatus.s.sol:FhevmStatus`,
      '--rpc-url',
      ctx.opt.rpcUrl,
      '--out',
      ctx.buildOut,
      ...traceArgs(ctx),
    ],
    { ...scriptEnv(ctx), ...generatedConfigEnv(ctx) },
  );
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Dispatch one stage.
 *
 * The order of the cases is documentation, not enforcement. This program is one orchestrator, an
 * operator running individual --stage invocations is another; neither can be the thing that
 * guarantees ordering. Every constraint that matters is a precondition on chain state inside the
 * Solidity script that would be harmed by the wrong order — see the table in README.md.
 */
async function runStage(ctx: Ctx, stage: Stage): Promise<void> {
  switch (stage) {
    case 'compute':
      stageCompute(ctx);
      return;
    case 'creates':
      await stageCreates(ctx);
      return;
    case 'pausers':
      await stepARegisterPausers(ctx);
      return;
    case 'offer-acl':
      await stepBOfferAclOwnership(ctx);
      return;
    case 'accept-acl':
      await stepCAcceptAclOwnership(ctx);
      return;
    case 'materialize':
      await stepDMaterializeStack(ctx);
      return;
    case 'offer-admin':
      await stepEOfferOwnerToAdmin(ctx);
      return;
    case 'accept-admin':
      await stepFAcceptOwnershipAsAdmin(ctx);
      return;
    case 'verify':
      stageVerify(ctx);
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
    case 'all':
      fail("Error: 'all' is expanded by main(), not dispatched.");
  }
}

////////////////////////////////////////////////////////////////////////////////

async function main(): Promise<void> {
  const cli = parseCliArgs(process.argv.slice(2));
  const { cfg, path: configPath } = loadConfigFile(cli.configPath);
  const opt = resolveOptions(cli, cfg, configPath);

  process.chdir(PACKAGE_ROOT);
  if (needsChain(opt.stage)) {
    requireTool('forge');
    requireTool('cast');
  }

  const ctx = buildContext(opt);

  // `log` and `report` read only local files, so they work with no network at all — which is
  // exactly when you want them. Every other stage needs the chain id, the factory check and the
  // allow-list before it is allowed to do anything.
  if (needsChain(opt.stage)) preflight(ctx);

  if (opt.stage === 'all') {
    // Every other stage is idempotent and skips what is already done, which is what makes re-running
    // `all` the resume path. `compute` cannot be: recomputing after transactions have been sent would
    // move the sealed addresses out from under a half-deployed stack, so run on its own it is a hard
    // error. Here it is incidental rather than requested, so a sealed deployment that has already
    // sent something simply skips it and carries on — otherwise `--stage all` could never resume the
    // deployment it started.
    if (readJsonl<JournalEntry>(ctx.journalPath).length > 0) {
      say('🧮  compute  already sealed and past its first transaction - skipping (resume)');
    } else {
      stageCompute(ctx);
    }

    for (const stage of RUN_ORDER) await runStage(ctx, stage);
    stageVerify(ctx);
  } else {
    await runStage(ctx, opt.stage);
  }

  say('', `done (${opt.stage})`);
}

////////////////////////////////////////////////////////////////////////////////

await main();
