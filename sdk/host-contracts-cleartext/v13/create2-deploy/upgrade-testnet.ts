// Upgrade a LIVE v12 cleartext stack to v13, through the canonical CREATE2 factory.
//
// See plans/CREATE2_TESTNET_UPGRADE_PLAN.md. Section references in the form §N are that document's;
// references to the deploy plan are marked as such.
//
// The sibling of deploy-testnet.ts, and everything that is not a stage comes from common.ts: argument
// parsing, the config file, the out-dir identity check, the chain and factory preflight, signer
// resolution, the reorg/finality waits, the journal, the seal gate and broadcast().
//
// ## Two invariants, structural rather than checked-and-hoped (§1)
//
// OWNERSHIP NEVER CHANGES. The live stack's ACL is already owned by a standing ACLOwner, and that
// ACLOwner is already owned by the admin; the upgrade runs entirely THROUGH that existing root. So this
// coordinator has no offer/accept stages at all — not stages that no-op, but absent — and it therefore
// cannot move ownership even when invoked wrongly. `verify` asserts it anyway.
//
// THE PAUSERS STAY THE SAME. No pauser stage either. PauserSet is untouched: same contract, same
// membership. Also asserted rather than assumed, because PauserSet exposes no enumeration — see §7.
//
// ## What is NOT here
//
// The nonce path (scripts/deploy.sh) and the TypeScript `updateV12ToV13` in pkg/ts are both untouched and
// remain what they were. This is a third path, for a stack whose addresses have to be predictable before
// anything is sent.

import { existsSync } from 'node:fs';
import { join } from 'node:path';

import {
  capture,
  ensureDir,
  fail,
  pad,
  readJson,
  readJsonl,
  removeIfPresent,
  requireTool,
  run,
  say,
  warn,
} from './utils.ts';
import {
  broadcast,
  buildContext,
  type Ctx,
  EXISTING_FLAGS,
  EXISTING_ROLES,
  type Flow,
  generatedConfigEnv,
  type JournalEntry,
  loadConfigFile,
  type Manifest,
  manifestPath,
  PACKAGE_ROOT,
  parseCliArgs,
  preflight,
  requireBuiltArtifacts,
  resolveOptions,
  ROLE_WIDTH,
  RULE_WIDTH,
  SCRIPT_DIR,
  scriptEnv,
  showJournal,
  stageReport,
  traceArgs,
} from './common.ts';

////////////////////////////////////////////////////////////////////////////////
// Stages (§2)
////////////////////////////////////////////////////////////////////////////////

/**
 * Four real stages against the deploy's nine.
 *
 * Absent by design, and their absence is the enforcement of §1: `pausers`, `offer-acl`, `accept-acl`,
 * `offer-admin`, `accept-admin`. An upgrade that cannot name those stages cannot run them.
 */
export type Stage = 'compute' | 'creates' | 'materialize' | 'verify' | 'status' | 'log' | 'report' | 'all';

const ALL_STAGES: readonly Stage[] = ['compute', 'creates', 'materialize', 'verify', 'status', 'log', 'report', 'all'];

/** What `--stage all` runs, in order. */
const RUN_ORDER: readonly Stage[] = ['compute', 'creates', 'materialize', 'verify'];

const REPORT_STEPS: ReadonlyArray<{ readonly label: string; readonly title: string }> = [
  { label: 'creates', title: 'every CREATE2 through the factory' },
  { label: 'D', title: 'ACLOwner.upgrade — 2 materializations + 5 reinitializations' },
];

const DEFAULT_CONFIG_NAME = 'upgrade.config.json';

/** `log` and `report` read only local files, so they work with no network at all. */
function needsChain(stage: string): boolean {
  return stage !== 'log' && stage !== 'report';
}

/**
 * Which stages need the deployer resolved from the KEYSTORE, at the cost of a password prompt.
 *
 * `compute` is on this side of the line for the same reason it is in the deploy: the new addresses are a
 * function of the deployer, so taking it from anywhere but the key itself would let a typo seal a set
 * nobody holds the key to.
 */
function needsDeployerKey(stage: string): boolean {
  return stage !== 'verify' && stage !== 'status' && stage !== 'report' && stage !== 'log';
}

////////////////////////////////////////////////////////////////////////////////

const HELP = `
Upgrade a live v12 cleartext stack to v13, via the canonical CREATE2 factory.
See plans/CREATE2_TESTNET_UPGRADE_PLAN.md.

Usage: node create2-deploy/upgrade-testnet.ts --rpc-url URL [--account NAME] --admin 0x...
                             --deployment-id ID <the nine existing addresses> [--handle 0x...]

  --rpc-url URL        node to upgrade on (required)
  --account NAME       forge keystore account to broadcast from. Required on every chain EXCEPT a
                       local anvil, where accounts 0 and 1 of anvil's public mnemonic are used
  --admin 0x...        the CURRENT ACLOwner owner. Not a value this sets — a value it VERIFIES, since
                       ownership must not change (section 1)
  --deployment-id ID   reuse the deployment's own id. The salt mixes the version, so "0.13" here and
                       "0.12" for the original deploy already give disjoint addresses (section 5)

THE LIVE STACK (section 3) — nine addresses, best supplied through the config file:

  --acl 0x...                    --hcu-limit 0x...
  --fhevm-executor 0x...         --cleartext-arithmetic 0x...
  --kms-verifier 0x...           --cleartext-db 0x...
  --input-verifier 0x...         --pauser-set 0x...
  --acl-owner 0x...

  Every one is validated against the live chain before anything is computed, because the seven v13
  implementations BAKE these addresses into their creation code: a wrong entry produces a stack that
  materializes cleanly and fails only in use. Eight are cross-checked against the stack's own wiring;
  --kms-verifier cannot be (no contract exposes a getter returning it), so it gets a weaker check and
  says so. See section 3.1.

  --handle 0x...       a cleartext handle already in the live CleartextDB, repeatable. compute records
                       its value; verify requires it unchanged. STRONGLY RECOMMENDED: without one,
                       verify can only prove the stack still works, not that existing data survived
                       (section 3.2)
  --migration PATH     the KMS migration seed. Omit to reconstruct it from the live KMSVerifier plus
                       the package defaults, which is refused if the live signer set disagrees
                       (section 6)

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
                         materialize   one atomic ACLOwner.upgrade
                         verify        versions, wiring, and the invariants           (no tx)
                       or a read-only view: status, log, report
                       default: all
  -v, --verbose        pass -vvvv to forge
  -h, --help           this
`;

////////////////////////////////////////////////////////////////////////////////
// The live stack (§3)
////////////////////////////////////////////////////////////////////////////////

/**
 * One `cast call` returning an address, or null if the call reverted or answered zero.
 *
 * `capture`, not `captureOrFail`: a revert is a legitimate ANSWER here rather than a failure. It is what
 * a wrong address looks like when it holds no code, or holds something without that getter — and the
 * caller is what decides whether that is fatal.
 */
function callAddress(ctx: Ctx, target: string, sig: string): string | null {
  const r = capture('cast', ['call', target, sig, '--rpc-url', ctx.opt.rpcUrl]);
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

////////////////////////////////////////////////////////////////////////////////

/**
 * §3.1 — every supplied address, checked against the live chain before anything is computed.
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
    const flagFor = (role: string): string =>
      Object.entries(EXISTING_FLAGS).find(([, r]) => r === role)?.[0] ?? `--${role.toLowerCase()}`;
    fail(
      `Error: the live stack is not fully specified — ${String(missing.length)} of ${String(EXISTING_ROLES.length)} addresses missing:`,
      ...missing.map((role) => `         ${flagFor(role)}  (${role})`),
      '',
      `       Put them in the "existing" block of ${DEFAULT_CONFIG_NAME}, or pass the flags. An upgrade`,
      '       derives nothing from the chain it does not first verify — see section 3 of the plan.',
    );
  }

  const at = (role: string): string => existing[role] ?? '';
  const acl = at('ACL_ADDRESS');
  const executor = at('FHEVM_EXECUTOR_ADDRESS');
  const arithmetic = at('CLEARTEXT_ARITHMETIC_ADDRESS');
  const aclOwner = at('ACL_OWNER');

  say('🔬  validating the live stack (section 3.1)');

  let bad = 0;
  const expect = (role: string, from: string, sig: string, source: string): void => {
    const got = callAddress(ctx, from, sig);
    const want = at(role);
    if (got === null) {
      say(`    ?    ${pad(role, ROLE_WIDTH)} ${source} did not answer — is that address a live v12 contract?`);
      bad += 1;
      return;
    }
    if (got.toLowerCase() !== want.toLowerCase()) {
      say(`    FAIL ${pad(role, ROLE_WIDTH)} ${source} says ${got}, you passed ${want}`);
      bad += 1;
      return;
    }
    say(`    ok   ${pad(role, ROLE_WIDTH)} confirmed by ${source}`);
  };

  // Eight addresses, each corroborated by a contract OTHER than itself. ACL by three.
  expect('ACL_ADDRESS', executor, 'getACLAddress()(address)', 'FHEVMExecutor.getACLAddress');
  expect('ACL_ADDRESS', at('CLEARTEXT_DB_ADDRESS'), 'getACLAddress()(address)', 'CleartextDB.getACLAddress');
  expect('ACL_ADDRESS', aclOwner, 'acl()(address)', 'ACLOwner.acl');
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

  // §1: --admin is verified, never set. The upgrade must not move ownership, so a mismatch here means
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
  // KMSVerifier" — NOT that it is this stack's. §7's post-upgrade signer-set comparison is what would
  // catch the residual case of pointing at another deployment's verifier on the same chain.
  const kms = at('KMS_VERIFIER_ADDRESS');
  const kmsVersion = callString(ctx, kms, 'getVersion()(string)');
  const kmsSigners = capture('cast', ['call', kms, 'getKmsSigners()(address[])', '--rpc-url', ctx.opt.rpcUrl]);
  if (kmsVersion === null || !kmsSigners.ok) {
    say(
      `    FAIL ${pad('KMS_VERIFIER_ADDRESS', ROLE_WIDTH)} not an initialized KMSVerifier (no getVersion/getKmsSigners)`,
    );
    bad += 1;
  } else {
    say(`    ok   ${pad('KMS_VERIFIER_ADDRESS', ROLE_WIDTH)} reports ${kmsVersion} — WEAK: not corroborated by wiring`);
    warn('       KMS_VERIFIER_ADDRESS is the one address the live stack cannot confirm (section 3.1).');
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
 * The Solidity half of this coordinator is not written yet (plan §8). Rather than let forge fail with
 * "script not found", each stage that needs one says which file is missing and where it is specified.
 *
 * This exists so the parts that ARE implemented can be run and tested on their own — §3.1's validation
 * needs no Solidity at all, and it is the check most likely to save an operator.
 */
function requireScript(name: string, planSection: string): string {
  const path = join(SCRIPT_DIR, name);
  if (!existsSync(join(PACKAGE_ROOT, path))) {
    fail(
      `Error: ${path} does not exist yet.`,
      `       It is specified in plans/CREATE2_TESTNET_UPGRADE_PLAN.md ${planSection}.`,
      '',
      '       Implemented and runnable today: --stage compute performs the section 3.1 validation of the',
      '       live stack before it reaches this point, so it is worth running on its own.',
    );
  }
  return path;
}

////////////////////////////////////////////////////////////////////////////////

function stageCompute(ctx: Ctx): void {
  say('🍟 compute (2 passes, 1 rebuild)');

  // Same reasoning as the deploy: recomputing after transactions have been sent would move the sealed
  // address set out from under a half-applied upgrade. An upgrade is worse than a deploy here, because
  // `materialize` is NOT idempotent — the reinitializers are reinitializer(n)-guarded, so a second run
  // reverts rather than no-oping (plan §11).
  if (readJsonl<JournalEntry>(ctx.journalPath).length > 0) {
    fail(
      `Error: '${ctx.opt.deploymentId}' has already sent transactions (see ${ctx.journalPath}),`,
      '       so its addresses are not safe to recompute.',
      `       To discard this upgrade's record and start over: rm -rf ${ctx.outDir}`,
    );
  }

  // §3.1 — before anything is computed, and before any build. This is the cheap check that prevents the
  // expensive mistake.
  validateExisting(ctx);
  recordHandleValues(ctx);

  ensureDir(ctx.outDir);
  removeIfPresent(ctx.buildOut, join(ctx.outDir, 'addresses.sol'), join(ctx.outDir, 'pass2.json'), manifestPath(ctx));

  const script = requireScript('FhevmComputeUpgradeAddresses.s.sol', '§4 and §8');
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
  const pass = (n: number, env: NodeJS.ProcessEnv): void => {
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
    if (run('forge', args, { ...env, FHEVM_PASS: String(n) }) !== 0) {
      fail(`Error: compute pass ${String(n)} failed.`);
    }
  };

  // Two passes, not the deploy's three (§4): the ACL already exists, so nothing computed here feeds
  // anything the live stack has already fixed.
  const env = scriptEnv(ctx, existingEnv(ctx));
  build(env);
  pass(1, env);
  build({ ...env, ...generatedConfigEnv(ctx) });
  pass(2, { ...env, ...generatedConfigEnv(ctx) });
}

////////////////////////////////////////////////////////////////////////////////

/**
 * §3.2 — the value of each `--handle`, read before the upgrade and required unchanged after it.
 *
 * Reported rather than silently skipped when empty, because the two situations give different guarantees
 * and conflating them in the output would overstate what `verify` proved.
 */
function recordHandleValues(ctx: Ctx): void {
  if (ctx.opt.handles.length === 0) {
    warn(
      '  no --handle given: verify will be able to prove the stack still WORKS after the upgrade, but',
      '  not that existing cleartext data survived it. Those are different claims (section 3.2).',
    );
    return;
  }
  const db = ctx.opt.existing.CLEARTEXT_DB_ADDRESS ?? '';
  for (const handle of ctx.opt.handles) {
    const r = capture('cast', ['call', db, 'get(bytes32)(uint256)', handle, '--rpc-url', ctx.opt.rpcUrl]);
    if (!r.ok) {
      fail(
        `Error: --handle ${handle} does not resolve in the live CleartextDB at ${db}.`,
        '       A handle that cannot be read now cannot be shown to survive.',
      );
    }
    say(`    ${pad(handle, ROLE_WIDTH)} = ${r.stdout}`);
  }
}

/** The live stack, passed to the forge scripts as FHEVM_EXISTING_<ROLE>. */
function existingEnv(ctx: Ctx): NodeJS.ProcessEnv {
  const env: NodeJS.ProcessEnv = {};
  for (const role of EXISTING_ROLES) {
    env[`FHEVM_EXISTING_${role}`] = ctx.opt.existing[role] ?? '';
  }
  return env;
}

////////////////////////////////////////////////////////////////////////////////

async function stageCreates(ctx: Ctx): Promise<void> {
  say('🥩 creates (one CREATE2 per create, each gated on getCode)');
  ctx.stageLabel = 'creates';
  const script = requireScript('FhevmUpgradeCreates.s.sol', '§5 and §8');
  await broadcast(ctx, `${script}:FhevmUpgradeCreates`);
}

async function stageMaterialize(ctx: Ctx): Promise<void> {
  say('🧩  materialize — one atomic ACLOwner.upgrade');
  ctx.stageLabel = 'D';
  const script = requireScript('FhevmMaterializeUpgrade.s.sol', '§6 and §8');
  await broadcast(ctx, `${script}:FhevmMaterializeUpgrade`);
}

function stageVerify(ctx: Ctx): void {
  say('✅  verify');
  const script = requireScript('FhevmVerifyUpgrade.s.sol', '§7 and §8');
  const args = [
    'script',
    `${script}:FhevmVerifyUpgrade`,
    '--out',
    ctx.buildOut,
    '--rpc-url',
    ctx.opt.rpcUrl,
    ...traceArgs(ctx),
  ];
  if (run('forge', args, { ...scriptEnv(ctx, existingEnv(ctx)), ...generatedConfigEnv(ctx) }) !== 0) {
    fail('Error: verify failed.');
  }
  // §7.1's ABI-enumerated survey and the ownership/pauser log scans live HERE rather than in Solidity,
  // which cannot enumerate an ABI. Not yet implemented; the plan is explicit that a missing survey must
  // be reported rather than inferred as "nothing changed".
  warn('  the section 7.1 survey (every readable value unchanged) is not implemented yet.');
}

function stageStatus(ctx: Ctx): void {
  const manifest = readJson<Manifest>(manifestPath(ctx));
  if (manifest === null) {
    say(`no manifest at ${manifestPath(ctx)} — run compute first.`);
    return;
  }
  say('-'.repeat(RULE_WIDTH));
  say(`📋  status  ${ctx.opt.deploymentId} @ v${ctx.opt.stage}`);
  say('-'.repeat(RULE_WIDTH));
  // materialize is NOT idempotent (plan §11), so "already done" has to read as success, not as a reason
  // to retry into a revert.
  warn('  status detail for the upgrade is not implemented yet (plan section 8).');
}

////////////////////////////////////////////////////////////////////////////////

async function runStage(ctx: Ctx, stage: Stage): Promise<void> {
  switch (stage) {
    case 'compute':
      stageCompute(ctx);
      return;
    case 'creates':
      await stageCreates(ctx);
      return;
    case 'materialize':
      await stageMaterialize(ctx);
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
  if (needsChain(opt.stage)) preflight(ctx);

  if (opt.stage === 'all') {
    for (const stage of RUN_ORDER) {
      await runStage(ctx, stage);
    }
    say('done (all)');
    return;
  }

  await runStage(ctx, opt.stage as Stage);
  say(`done (${opt.stage})`);
}

await main();
