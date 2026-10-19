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
//      passes with two rebuilds between them. This is the largest piece of work CREATE2 adds
//      over the nonce path.
//
//   2. WAIT FOR A TRANSACTION FROM SOMEONE ELSE. Step E only offers; ACLOwner is Ownable2Step and
//      the admin must send acceptOwnership() from its own key. Nothing here can produce that.
//
// The nonce path (scripts/deploy.sh) is UNTOUCHED and remains the only path for chain 31337.
// This adds a second path; it replaces nothing.

import { existsSync } from 'node:fs';
import { join } from 'node:path';

import {
  captureOrFail,
  ensureDir,
  fail,
  readJson,
  readJsonl,
  removeIfPresent,
  run,
  sameAddress,
  say,
  requireTool,
  sleep,
} from './utils.ts';
import {
  broadcast,
  buildContext,
  type Ctx,
  type Flow,
  generatedConfigEnv,
  headBlock,
  PACKAGE_ROOT,
  type JournalEntry,
  loadConfigFile,
  type Manifest,
  manifestPath,
  parseCliArgs,
  preflight,
  recordObservation,
  requireBuiltArtifacts,
  resolveOptions,
  SCRIPT_DIR,
  scriptEnv,
  showJournal,
  stageReport,
  traceArgs,
} from './common.ts';

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

/** Looked for when --config is not given. Its absence is not an error. */
const DEFAULT_CONFIG_NAME = 'deploy.config.json';

/** Every key a config file may carry — anything else is a typo, and typos here select addresses. */
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
const REPORT_STEPS: ReadonlyArray<{ readonly label: string; readonly title: string }> = [
  { label: 'creates', title: 'every CREATE2 through the factory' },
  { label: "A/A'", title: 'register the pausers' },
  { label: 'B', title: 'offer ACL ownership to the ACLOwner (offers only)' },
  { label: 'C', title: 'accept ACL ownership - ownership MOVES here' },
  { label: 'D', title: 'materialize the stack (one atomic tx)' },
  { label: 'E', title: 'offer the ACLOwner to the admin (offers only)' },
  { label: 'F', title: 'admin accepts - the deployer is no longer root' },
];

/** The broadcasting stages of a full run, in run order. */
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
                       answer anvil_nodeInfo or this is refused
  --admin 0x...        final owner of ACLOwner. Mandatory, no default — except under the
                       anvil default above, where it is anvil account 1
  --deployment-id ID   operator-chosen string; a fresh one gives a disjoint address set
  --pauser 0x...       optional operator pauser, step A'
  --confirmations N    reorg DEPTH floor for the between-stage waits (default 3). This is
                       the value the Solidity gate enforces, so it is the one a different
                       orchestrator also has to honor
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
                         verify               the terminal conditions           (no tx)
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
 * The deployer key owns the ACLOwner — root over the whole stack — until step F completes, so
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
function needsChain(stage: string): boolean {
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
 * the deployer, so taking it from anywhere but the key itself would let a typo seal a stack
 * nobody holds the key to.
 */
function needsDeployerKey(stage: string): boolean {
  return stage !== 'verify' && stage !== 'status' && stage !== 'report' && stage !== 'log';
}

////////////////////////////////////////////////////////////////////////////////

function stageCompute(ctx: Ctx): void {
  say('🎃 compute (3 passes, 2 rebuilds)');

  // Recomputing after transactions have been sent would move the sealed address set out from under a
  // stack that is already partly deployed — the creates stage would then either report drift or,
  // worse, start building a second disjoint set alongside the first. A
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
  // journal.jsonl and broadcast/, which are the audit trail and belong to the deploy stages.
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
 * The seal must be committed AND PUSHED before any transaction — for a stronger reason than
 * audit trail. The addresses are a function of the init-code hashes, so retrying a failed create
 * needs the byte-exact ones, and a resumed run's first act is computing which addresses to probe.
 * Lose the seal and a half-finished stack is unfinishable.
 *
 * NOT automated: pushing to a shared remote is the operator's call, not this script's.
 */
async function stageCreates(ctx: Ctx): Promise<void> {
  say('🧀  creates (one CREATE2 per create, each gated on getCode)');
  ctx.stageLabel = 'creates';
  await broadcast(ctx, 'FhevmDeployCreates.s.sol:FhevmDeployCreates');
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Steps A and A'. The only part of the sequence that is not the ownership handover, and the only
 * part still reachable after the run, via ACLOwner.execute.
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
 * Step B. Needed no gate invented for it — step C's precondition is already
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
 * Step E — the deployer gives up root. Only OFFERS; it has no precondition on D, so the script
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
 * There is no step F in the stage list: the sequence stops at E, and this is described only as prose — "the admin must send
 * acceptOwnership()… the runner waits for and verifies it". That prose is a step. It has a sender, a
 * predicate, a precondition, and a terminal condition that fails without it, and until it lands
 * the DEPLOYER still holds ACLOwner.execute — an unrestricted call as ACL.owner(), i.e. root.
 *
 * Two paths, because the admin is not necessarily a key we can sign with:
 *
 *   --admin-account NAME   a forge keystore account for the admin: send it, gated like every other
 *                          step. This is the local-key / single-signer case.
 *   (not given)            the multisig case. Nobody here can produce that
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

/** The terminal conditions. Reverts non-zero if any is unmet. */
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

/**
 * This coordinator's half of the split: the stage list, the help text and the two stage predicates.
 * Everything else it needs comes from `common.ts`.
 */
const DEPLOY_FLOW: Flow = {
  name: 'deploy',
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
  const cli = parseCliArgs(DEPLOY_FLOW, process.argv.slice(2));
  const { cfg, path: configPath } = loadConfigFile(DEPLOY_FLOW, cli.configPath);
  const opt = resolveOptions(DEPLOY_FLOW, cli, cfg, configPath);

  process.chdir(PACKAGE_ROOT);
  if (needsChain(opt.stage)) {
    requireTool('forge');
    requireTool('cast');
  }

  const ctx = buildContext(DEPLOY_FLOW, opt);

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
      say('🎃 compute already sealed and past its first transaction - skipping (resume)');
    } else {
      stageCompute(ctx);
    }

    for (const stage of RUN_ORDER) await runStage(ctx, stage);
    stageVerify(ctx);
  } else {
    await runStage(ctx, opt.stage as Stage);
  }

  say('', `done (${opt.stage})`);
}

////////////////////////////////////////////////////////////////////////////////

await main();
