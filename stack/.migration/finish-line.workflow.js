// fhevm-cli redesign — FINISH-LINE WORKFLOW (phase-runner)
// ============================================================================
// This is a Workflow-engine script (NOT a node script — it uses the workflow
// runtime globals: agent/parallel/pipeline/phase/log/budget/args). It is the
// durable source of the orchestrator. To run ONE migration phase:
//
//   Workflow({ scriptPath: ".../finish-line.workflow.js", args: { phase: 0 } })
//
// CONTRACT (matches the /goal):
//  - Runs exactly ONE phase per invocation, then STOPS at a human checkpoint.
//    It never advances phases and never pushes/merges. A human merges the green
//    worktree and starts the next phase.
//  - "Pass" is the acceptance harness (gate), never an agent's claim. The
//    implementer must surface the harness's RAW output; an independent verifier
//    re-runs the harness and must agree; ≥2-of-3 adversarial skeptics must fail
//    to refute. Only then is the phase GREEN-awaiting-merge.
//  - kind-local only; ephemeral clusters; ghcr-read creds; no real clusters /
//    mainnet. Determinism: pinned mnemonic, frozen tags, committed normalize.sh.
//  - Golden masters are immutable (recorded once from the CURRENT CLI); agents
//    never regenerate them. The intended-diff allowlist is human-gated.
// ============================================================================

export const meta = {
  name: 'fhevm-cli-finish-line',
  description: 'Phase-runner for the fhevm-cli redesign: runs ONE acceptance-gated migration phase (args.phase) in an isolated worktree, then stops at a human checkpoint. Gate = the acceptance harness, not agents. No push/merge; kind-local only.',
  phases: [
    { title: 'Decompose' },
    { title: 'Implement' },
    { title: 'Gate' },
    { title: 'Verify' },
    { title: 'Report' },
  ],
}

const ROOT = '/Users/work/code/zama/fhevm/.claude/worktrees/fhevm-cli-exemplar'
const GOLDEN = ROOT + '/stack/.migration/golden'
const RESERVE = 60000        // stop opening work if budget drops below this
const MAX_ATTEMPTS = 3       // implement->gate retries per phase, then escalate
const A = (typeof args === 'string') ? JSON.parse(args) : (args || {}) // args can arrive as a JSON string; normalize it before use
const ISO = A.inPlace ? {} : { isolation: 'worktree' } // in-place mode (A.inPlace) skips per-agent isolation; agents use the absolute ROOT paths. Needed when worktree.baseRef='fresh'.

const GROUNDING =
  'Repo facts (do not re-derive): the real deployment charts are at ' + ROOT + '/charts/{anvil-node,contracts,coprocessor,kms-connector,listener}; they are release-scoped ({{ .Release.Name }}-*) and single-operator. Multiplicity = N helm releases of the coprocessor chart, NOT chart edits and NOT replicas. Topology = operators (N releases) + signer-set/threshold (contracts values: NUM_COPROCESSORS + COPROCESSOR_SIGNER_ADDRESS_0..N-1 + threshold) + replicas (per-operator HA). Contract addresses flow via the contracts sc-deploy Job -> sc-addresses ConfigMap -> consumer charts. Endpoint indirection = kmsCoreEndpoints / *_ENDPOINT. kms-core is the EXTERNAL companion (not a chart). The Stack API is promoted from RolloutRunContext in ' + ROOT + '/test-suite/fhevm/src/commands/rollout-run.ts. Private ghcr images pull into kind with a registry-credentials secret from a read:packages token (proven).'

// One phase per row. `gate.levels` = which acceptance levels must pass.
// `runnableNow` flags whether the gate can actually pass today (depends on
// earlier phases being merged); the runner reports HONESTLY when it can't.
const PHASES = [
  { id: 0, title: 'Phase 0 cleanup — honest L0',
    objective: 'Wire stack/values/{default,two-of-three}.yaml to the REAL chart value schemas so the cases diverge; commit stack/.migration/normalize.sh (the canonicalizer); make the L0 render expand a topology into (contracts NUM_COPROCESSORS/threshold) + (N coprocessor releases); make the assertion count documents, not just kinds.',
    lane: ['stack/values/', 'stack/.migration/'],
    gate: { levels: ['L0'], how: 'render default + two-of-three via the harness; assert the two normalized goldens DIFFER and doc-counts match expectations' },
    acceptance: 'L0: default != two-of-three (distinct sha); normalize.sh committed; doc-count assertion in place', runnableNow: true },

  { id: 1, title: 'Phase 1 — Stack API in place',
    objective: 'Extract the engine-agnostic Stack interface from RolloutRunContext, decoupled from its concrete compose/readiness/state/test imports. No behavior change to the current CLI.',
    lane: ['test-suite/fhevm/src/'],
    gate: { levels: ['typecheck'], how: 'bun run check (tsc --noEmit) green; existing runbooks type-check against the extracted interface' },
    acceptance: 'interface extracted; tsc green; runbooks unchanged in behavior', runnableNow: true },

  { id: 2, title: 'Phase 2 — kind backend + first slice',
    objective: 'Implement the kind backend for up(default): helm-install shared infra (anvil x chains, minio, postgres, kms-core) + contracts (sc-deploy Job -> sc-addresses ConfigMap) + 1 coprocessor release; test erc20 as a Job.',
    lane: ['stack/lib/', 'stack/cli/', 'stack/values/'],
    gate: { levels: ['L0', 'L2'], how: 'on a local kind cluster: up(default) reaches Ready; L2 erc20 green; L0 default matches golden' },
    acceptance: 'L2 erc20 GREEN on kind; L0 default matches', runnableNow: false },

  { id: 3, title: 'Phase 3 — topology',
    objective: 'two-of-three = 3 coprocessor releases + contracts NUM_COPROCESSORS=3/threshold=2; threshold-KMS = N kms-core behind kmsCoreEndpoints.',
    lane: ['stack/lib/', 'stack/values/', 'stack/runbooks/'],
    gate: { levels: ['L0', 'L2'], how: 'L0 cases distinct; L2 green for default + two-of-three + threshold-kms; 2-of-3 consensus exercised' },
    acceptance: 'consensus + threshold topologies green', runnableNow: false },

  { id: 4, title: 'Phase 4 — procedures to runbooks',
    objective: 'Move drift/db-revert/kms-gen out of test.ts into stack/runbooks/ over the Stack API (exec/sql/stop/start/logs/until); rollout runs through the API.',
    lane: ['stack/runbooks/', 'test-suite/'],
    gate: { levels: ['L3', 'L2'], how: 'v0.12-to-v0.13 receipt + per-phase L2 match golden; drift/db-revert runbooks pass on kind' },
    acceptance: 'L3 rollout receipts match; chaos runbooks pass', runnableNow: false },

  { id: 5, title: 'Phase 5 — manifest is the only version mechanism',
    objective: 'Delete resolve/ and the --target/--override/live-resolve paths; evict compat/ to old tags; apply endpoint indirection across env.',
    lane: ['stack/', 'test-suite/fhevm/src/'],
    gate: { levels: ['L0'], how: 'full-matrix L0 still matches golden; --build + manifest cover CI selective-rebuild' },
    acceptance: 'resolve/ + compat/ gone; full-matrix L0 green', runnableNow: false },

  { id: 6, title: 'Phase 6 — CI cutover',
    objective: 'Migrate e2e/orchestrate/threshold/rollout workflows one at a time old->new, each gated by acceptance.yml, behind FHEVM_CLI_IMPL=old.',
    lane: ['.github/workflows/'],
    gate: { levels: ['L2', 'L3'], how: 'each migrated workflow green in CI; fallback flag still works' },
    acceptance: 'CI e2e runs through the new driver, green', runnableNow: false },

  { id: 7, title: 'Phase 7 — relocate (late, separate)',
    objective: 'Move test-suite/fhevm -> top-level stack/; fix every hardcoded test-suite/fhevm reference in CI + SDK scripts.',
    lane: ['stack/', '.github/workflows/', 'sdk/'],
    gate: { levels: ['L0', 'L2'], how: 'full matrix green from the new path; no dangling test-suite/fhevm refs' },
    acceptance: 'relocated; matrix green from stack/', runnableNow: false },

  { id: 8, title: 'Phase 8 — soak + delete',
    objective: 'After one release behind FHEVM_CLI_IMPL=old, final commit deletes the old CLI (test-suite/fhevm) AND stack/.migration/ together.',
    lane: ['test-suite/', 'stack/.migration/'],
    gate: { levels: ['guard'], how: 'merge check fails if old CLI or stack/.migration/ still exists; permanent suite green on the new impl alone' },
    acceptance: 'old CLI + harness deleted; new impl stands alone', runnableNow: false },
]

const PATCH = {
  type: 'object', additionalProperties: false,
  properties: {
    summary: { type: 'string' },
    filesChanged: { type: 'array', items: { type: 'string' } },
    worktree: { type: 'string' },
    rawGateOutput: { type: 'string' }, // VERBATIM harness output the agent ran; the gate truth, not a self-judgment
  },
  required: ['summary', 'worktree', 'rawGateOutput'],
}
const GATE = {
  type: 'object', additionalProperties: false,
  properties: {
    pass: { type: 'boolean' },
    levels: { type: 'array', items: { type: 'string' } },
    detail: { type: 'string' },     // per-case/level result derived from the raw harness output
    skipped: { type: 'array', items: { type: 'string' } }, // anything sampled/not run — never silently
  },
  required: ['pass', 'detail'],
}
const VERDICT = {
  type: 'object', additionalProperties: false,
  properties: { refuted: { type: 'boolean' }, why: { type: 'string' } },
  required: ['refuted', 'why'],
}

const phaseNum = (typeof A.phase === 'number') ? A.phase : parseInt(A.phase, 10)
const P = Number.isInteger(phaseNum) ? PHASES[phaseNum] : undefined
if (!P) { log('unknown/blank phase arg (no silent fallback): ' + JSON.stringify(A.phase)); return { error: 'unknown phase', got: A.phase } }

log('FINISH-LINE phase-runner — running ' + P.title + ' (gate: ' + P.gate.levels.join('+') + (P.runnableNow ? '' : ' — NOTE: gate likely not green until earlier phases are merged') + ')')
if (budget.total && budget.remaining() < RESERVE) { log('budget too low; aborting before any work'); return { phase: P.id, status: 'ABORTED_BUDGET' } }

// ---- implement -> gate loop (bounded; feeds gate failures back) ----
phase('Implement')
let attempt = 0, patch = null, gate = null, feedback = ''
while (attempt < MAX_ATTEMPTS && (!budget.total || budget.remaining() > RESERVE)) {
  attempt += 1
  patch = await agent(
    'PHASE ' + P.id + ' — ' + P.title + '. ' + GROUNDING + '\n\n' +
    'Objective: ' + P.objective + '\n' +
    'Stay strictly inside this lane (edit nothing else): ' + P.lane.join(', ') + '. Charts are structurally OFF-LIMITS. Do NOT push, do NOT merge, do NOT touch main. kind-local only; tear down any cluster you create.\n' +
    (feedback ? 'The previous attempt FAILED the gate — fix exactly this and retry:\n' + feedback + '\n' : '') +
    'Implement the change in this worktree. THEN run the acceptance gate for levels [' + P.gate.levels.join(', ') + '] (' + P.gate.how + ') and return the harness output VERBATIM in rawGateOutput — do not summarize it into a pass/fail yourself; the orchestrator and an independent verifier judge it.',
    { ...ISO, schema: PATCH, phase: 'Implement', model: 'sonnet', label: 'impl:P' + P.id + '#' + attempt }
  )
  if (!patch) { feedback = 'implementer returned nothing'; continue }
  // gate-as-oracle: a SEPARATE agent interprets the raw harness output into pass/fail.
  phase('Gate')
  gate = await agent(
    'You are the GATE for phase ' + P.id + '. Read the VERBATIM acceptance-harness output below and decide pass/fail for levels [' + P.gate.levels.join(', ') + '] against: ' + P.acceptance + '. Judge ONLY from the raw output; if it is missing, ambiguous, or shows any non-empty diff / failing test / skipped level, set pass=false. Never infer success. List anything skipped.\n\nRAW HARNESS OUTPUT:\n' + patch.rawGateOutput,
    { schema: GATE, phase: 'Gate', model: 'sonnet', label: 'gate:P' + P.id + '#' + attempt }
  )
  if (gate && gate.pass) break
  feedback = (gate && gate.detail) || 'gate failed without detail'
  log('P' + P.id + ' attempt ' + attempt + ': gate RED — ' + feedback)
}

if (!gate || !gate.pass) {
  phase('Report')
  return { phase: P.id, title: P.title, status: 'BLOCKED', attempts: attempt, gate, worktree: patch && patch.worktree,
           note: 'Stopped at the human checkpoint with the gate RED. Present blockers; do not advance.' }
}

// ---- adversarial verify (independent skeptics re-run / inspect the gate) ----
phase('Verify')
const skeptics = [
  'Re-run the L0/level checks for this phase in a FRESH worktree and confirm the gate result reproduces; refuted=true if it does not.',
  'Hunt for a masked regression: a non-empty diff hidden by over-aggressive normalization, a test that was skipped not passed, or an intended-diff that is actually a real behavior change. Default refuted=true if unsure.',
  'Check the change stayed inside the lane (' + P.lane.join(', ') + '), touched no chart structure, and made no push/merge/cluster-leak. refuted=true on any violation.',
]
const verdicts = (await parallel(skeptics.map((s, i) => () => agent(
  'Adversarially verify phase ' + P.id + ' (' + P.title + '). ' + GROUNDING + '\nWorktree: ' + (patch && patch.worktree) + '. Gate claimed PASS with: ' + gate.detail + '\nYour lens: ' + s,
  { schema: VERDICT, phase: 'Verify', model: 'sonnet', label: 'verify:P' + P.id + '#' + (i + 1) }
)))).filter(Boolean)
const verified = verdicts.filter((v) => !v.refuted).length >= 2

phase('Report')
const report = await agent(
  'Write ' + ROOT + '/stack/.migration/phase-' + P.id + '-report.md and return a tight summary. Phase: ' + P.title + '. Gate: ' + JSON.stringify(gate) + '. Verifier verdicts: ' + JSON.stringify(verdicts) + '. Worktree: ' + (patch && patch.worktree) + '. Cover: what changed, the gate evidence (the levels that passed + anything skipped), the verifier outcome, and the explicit next action for the human (review + merge this worktree, then start phase ' + (P.id + 1) + '). State clearly this run did NOT push, merge, or advance.',
  { phase: 'Report', model: 'sonnet', label: 'report:P' + P.id }
)
return {
  phase: P.id, title: P.title,
  status: verified ? 'GREEN — awaiting human merge' : 'GATE GREEN BUT VERIFICATION FAILED — needs human review',
  gate, verdicts, worktree: patch && patch.worktree, report,
}
