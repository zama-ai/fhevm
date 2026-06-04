// Autonomous build driver for the Solana e2e PoC (Track 2).
//
// Run via the Claude Code `Workflow` tool: Workflow({ scriptPath: this file }),
// with budget.total set at launch to a fraction of the session window. It is a
// SINGLE frugal background loop (not a wide fan-out): it grinds the #1494 work
// queue item by item — implement -> self-test against run-oracle.sh -> adversarial
// verify -> local commit — parking honest-hard items and breaking the circuit only
// for "very good reasons". Resumable via resumeFromRunId.
//
// GUARDRAILS (encoded into every prompt; see fhevm-internal#1494 + HARNESS.md):
//   - Scope = the items below only. RFCs (021/024) and feature/solana are READ-ONLY
//     starting points; on conflict the #1494 plan wins (note the divergence, never
//     edit an RFC).
//   - No writes to RFCs, PRs, or issues. No `git push`. Local commits only.
//   - No hack / no glue: run-oracle.sh is the source of truth; never weaken a test
//     or the oracle to go green. Honest-pass or PARK.
//   - Mirror `main`: each adapt item names the EVM file whose shape/semantics it ports.

export const meta = {
  name: 'solana-poc-driver',
  description: 'Autonomous Solana e2e PoC build on test/solana-e2e within #1494 guardrails',
  phases: [{ title: 'Implement' }, { title: 'Verify' }, { title: 'Slice' }],
};

// --- Work queue: a starting encoding of #1494 (refine as the spec firms up). ---
// kind: 'adapt' (sync existing component) | 'new' (Solana-specific crypto seam)
// phase: 1 compute spine · 2 decrypt · 3 on-chain consumption
// oracle: extra command beyond run-oracle.sh that proves THIS item (L2 contract test)
// mirrors: the EVM file this item ports (adapt items)
const ITEMS = [
  { id: 'handles-keccak', kind: 'adapt', phase: 1, deps: [],
    desc: 'Derive all handles with keccak256 + canonical layout (chain_type high-bit); drop sha256.',
    mirrors: 'host-contracts/contracts/FHEVMExecutor.sol (_appendMetadataToPrehandle, _binaryOp keccak preimage)',
    oracle: 'cargo test -p zama-host handle_derivation' },
  { id: 'addr-bytes32', kind: 'adapt', phase: 1, deps: [],
    desc: 'bytes32 canonical host-chain addresses across the Solana surface (RFC-021).',
    mirrors: 'zkproof-worker/src/auxiliary.rs (92->128 aux), tech-spec RFC-021',
    oracle: 'cargo test -p zama-host bytes32_roundtrip' },
  { id: 'listener-wire', kind: 'adapt', phase: 1, deps: ['handles-keccak'],
    desc: 'Wire solana_adapter as the Solana host-listener: normalized rows into the coprocessor DB.',
    mirrors: 'coprocessor/fhevm-engine/host-listener (EVM decode path)',
    oracle: 'cargo test -p host-listener solana_adapter' },
  { id: 'input-bind-secp', kind: 'new', phase: 1, deps: ['handles-keccak'],
    desc: 'zama-host binds a coprocessor-verified input by verifying the copro EIP-712 attestation on-chain via secp256k1_recover; retire native verify_input_and_bind + mock-input.',
    mirrors: 'host-contracts FHEVMExecutor.verifyInput (coprocessor-sig check)',
    oracle: 'cargo test -p zama-host input_bind_secp256k1' },
  { id: 'acl-trait', kind: 'adapt', phase: 2, deps: ['listener-wire'],
    desc: 'KMS-worker ACL read becomes chain-dispatched; solana_acl is the Solana branch.',
    mirrors: 'kms-connector kms-worker/.../decryption.rs (acl_contracts isAllowedForDecryption)',
    oracle: 'cargo test -p kms-worker solana_acl' },
  { id: 'user-sig-ed25519', kind: 'new', phase: 2, deps: ['acl-trait'],
    desc: 'Solana signMessage/Ed25519 user-decrypt verify, chain-dispatched in the KMS worker.',
    mirrors: 'kms-connector .../decryption.rs verify_signature (EIP-712 path)',
    oracle: 'cargo test -p kms-worker ed25519_user_sig' },
  { id: 'relayer-addr', kind: 'adapt', phase: 2, deps: [],
    desc: 'Relayer base58 address validation + Solana host_chains config (RFC-021).',
    mirrors: 'relayer validations.rs / RFC-021 address.ts',
    oracle: 'cargo test -p relayer solana_address' },
  { id: 'cert-secp', kind: 'new', phase: 3, deps: ['user-sig-ed25519'],
    desc: 'On-chain secp256k1_recover verify of the KMS EIP-712 decryption cert for disclose/redeem.',
    mirrors: 'gateway KMSVerifier / Decryption.sol EIP-712 cert',
    oracle: 'cargo test -p confidential-token disclose_redeem_cert' },
];

const IMPL = { type: 'object', required: ['oracleGreen', 'escalate'], properties: {
  oracleGreen: { type: 'boolean' }, escalate: { type: 'boolean' },
  escalateReason: { type: 'string', description: 'cheat-required|undecided-architecture|env-unrecoverable|scope-conflict|""' },
  diffSummary: { type: 'string' }, oracleTail: { type: 'string' }, rfcDivergence: { type: 'string' } } };
const VERDICT = { type: 'object', required: ['refuted', 'hack'], properties: {
  refuted: { type: 'boolean' }, hack: { type: 'boolean' }, why: { type: 'string' } } };

const COMMON = `
GUARDRAILS (hard):
- Scope is ONLY this item. Do not touch unrelated files or the moving parts (Decryption.sol churn) — if blocked by scope, escalate scope-conflict.
- RFCs 021/024 and feature/solana are READ-ONLY references; if they conflict with the #1494 plan, the plan wins — record it in rfcDivergence, never edit an RFC.
- No writes to RFCs, PRs, or issues. No git push, and do NOT commit yourself — leave your changes in the working tree; the driver commits an item only after verification passes.
- run-oracle.sh is the source of truth. NEVER weaken a test, stub a check, or add glue to go green. If you cannot pass honestly, set oracleGreen=false (the loop will retry/park) — do not cheat.
- Work in your worktree; keep the diff minimal and in our code-quality contract (no new >500-line files, no unwrap/panic/TODO/glue — check-form enforces this).
- If your change would grow a file listed in scripts/poc/form-allow.txt (e.g. zama-host/src/state/mod.rs, which currently holds the handle-derivation code), do NOT grow it: EXTRACT the code you touch into a new focused <500-line module so the allowlisted file shrinks. Editing form-allow.txt or check-form.sh is never allowed, and hitting this is NOT a scope-conflict — splitting the god-file is the intended path and advances our quality goal.
- Need keccak256 in zama-host? Add \`solana-keccak-hasher = "3.1.0"\` to programs/zama-host/Cargo.toml (the keccak twin of the existing solana-sha256-hasher; that version is already in Cargo.lock) and use its hashv. This crate is PRE-APPROVED in scripts/poc/dep-allow.txt, so the dep-guard allows it. Do NOT add any OTHER crate. NOTE: anchor_lang::solana_program does NOT re-export keccak — do not rely on it.`;

const implPrompt = (it) => `Implement #1494 item "${it.id}" (${it.kind}, phase ${it.phase}) on test/solana-e2e.
Task: ${it.desc}
${it.mirrors ? `Mirror this EVM component's shape/semantics: ${it.mirrors}` : ''}
First run \`${it.oracle}\` and \`bash solana/scripts/poc/run-oracle.sh\`; if the item is ALREADY satisfied (green, e.g. committed by a prior run), make no changes and report oracleGreen=true with diffSummary="already-satisfied". Otherwise implement it and iterate until both are green.
Report oracleGreen, a one-line diffSummary, the oracle tail, and any rfcDivergence.
If the only way to green is to cheat / the architecture is undecided / the env is unrecoverable, set escalate=true with escalateReason and stop.
${COMMON}`;

const refutePrompt = (it, impl, lens) => `Adversarially verify #1494 item "${it.id}" via the ${lens} lens. Default to refuted=true unless you can prove otherwise — but judge the WORKING-TREE diff (\`git diff\`), not git history.
CONVENTIONS — do NOT refute or set hack on these (they are by design, not defects):
- Changes are UNCOMMITTED in the working tree on purpose; the driver commits after you pass. "Not committed yet" is never a reason to refute.
- Re-implementing canonical logic inside the Solana program/crate is EXPECTED and NOT a cheat when that crate cannot import the EVM/coprocessor source (e.g. mirroring zkproof-worker auxiliary.rs in an on-chain module). Only treat duplication as a problem if it could trivially be shared WITHIN the same crate.
Claimed: ${impl.diffSummary}
- correctness: find a path where the oracle passes but behavior is wrong (tautological test, both sides share the same wrong derivation, stubbed check).
- matches-evm: confirm it preserves the semantics of ${it.mirrors || 'the mirrored component'}.
- no-hack/form: set hack=true ONLY for a genuine cheat — a weakened/stubbed/deleted test, a faked oracle, an edited gate/test file, or added glue (unwrap/panic/TODO). Necessary cross-crate reimplementation is NOT a hack.
Return refuted, hack, why.`;

// --- The frugal loop. Sequential (no worktree isolation needed): each impl agent
//     works in-tree, the driver commits an item only after verify passes and reverts
//     the working tree on a failed attempt so the next item starts clean.
//     maxTokens (from args, set by the driver at launch) is the self-imposed ceiling
//     on budget.spent(); falls back to the turn budget directive, else park/escalate-bounded.
const MAX_TOKENS = (args && typeof args === 'object' && args.maxTokens) || null;
const RESERVE = 80_000;
const withinBudget = () =>
  MAX_TOKENS ? budget.spent() < MAX_TOKENS : (budget.total ? budget.remaining() > RESERVE : true);
const LENSES = ['correctness', 'matches-evm', 'no-hack/form'];
const queue = ITEMS.map((it) => ({ ...it, status: 'todo', tries: 0 }));
const done = (id) => queue.find((q) => q.id === id)?.status === 'green';
const nextReady = () => queue.find((q) => q.status === 'todo' && q.deps.every(done));
const escalations = [];
const divergences = [];

const commitItem = (it) =>
  agent(`Commit ONLY the working-tree changes for #1494 item "${it.id}" on test/solana-e2e: \`git add -A && git commit\` with a conventional message referencing the item and zama-ai/fhevm-internal#1494. If there is nothing to commit (item was already satisfied), do nothing and return "no-op". Do NOT push. Return the commit hash.`,
    { label: `commit:${it.id}`, phase: 'Implement', model: 'sonnet' });
const revertTree = (it) =>
  agent(`Discard the UNCOMMITTED working-tree changes from the failed attempt at "${it.id}" so the next item starts clean (\`git checkout -- . && git clean -fd\` under solana/). Never touch committed history. Confirm the tree is clean.`,
    { label: `revert:${it.id}`, phase: 'Implement', model: 'sonnet' });

while (withinBudget() && nextReady() && !escalations.length) {
  const it = nextReady();
  phase('Implement');
  const impl = await agent(implPrompt(it), { label: `impl:${it.id}`, phase: 'Implement', schema: IMPL });
  if (!impl) { it.status = 'parked'; continue; }
  if (impl.escalate) { escalations.push({ id: it.id, reason: impl.escalateReason }); break; }
  if (impl.rfcDivergence) divergences.push({ id: it.id, note: impl.rfcDivergence });
  if (!impl.oracleGreen) { await revertTree(it); it.tries += 1; if (it.tries >= 3) it.status = 'parked'; continue; }

  phase('Verify');
  const verdicts = (await parallel(LENSES.map((lens) => () =>
    agent(refutePrompt(it, impl, lens), { label: `verify:${it.id}:${lens}`, phase: 'Verify', schema: VERDICT,
      model: lens === 'correctness' ? undefined : 'sonnet' })))).filter(Boolean);
  if (verdicts.some((v) => v.hack)) { await revertTree(it); escalations.push({ id: it.id, reason: 'cheat-detected', detail: verdicts.find((v) => v.hack)?.why }); break; }
  if (verdicts.length === LENSES.length && verdicts.every((v) => !v.refuted)) {
    await commitItem(it); it.status = 'green'; log(`green: ${it.id}`);
  } else {
    await revertTree(it); it.tries += 1; if (it.tries >= 3) it.status = 'parked';
  }
}

return {
  green: queue.filter((q) => q.status === 'green').map((q) => q.id),
  parked: queue.filter((q) => q.status === 'parked').map((q) => q.id),
  todo: queue.filter((q) => q.status === 'todo').map((q) => q.id),
  escalations,
  rfcDivergences: divergences,
  spent: budget.total ? budget.spent() : null,
};
