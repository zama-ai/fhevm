/**
 * Case catalogue for the `kms-context-qa-tests` profile.
 *
 * The profile grows one QA scenario at a time. Keeping the catalogue separate from the runner means
 * adding a scenario is one new file under `cases/` plus one entry in {@link QA_CASES} — the runner
 * itself is never edited, so a new case cannot regress the sequencing, preflight, evidence or
 * restore behaviour that every other case depends on.
 *
 * Everything here except the case imports is pure, so selection and requirement logic is unit
 * tested without a live stack.
 */
import { PreflightError } from "../errors";
import type { DecryptionRunner } from "../commands/kms-generation";
import type { SmokeRunner } from "../commands/kms-context-switch";
import type { State } from "../types";
import type { Owner } from "../kms-onchain";
import type { CaseEvidence } from "./evidence";
import type { NodeSupervisor } from "./nodes";
import type { ProtocolConfigTarget } from "./protocol-config";
import { contextSwitchCase } from "./cases/case-context-switch";
import { epochRotationCase } from "./cases/case-epoch-rotation";
import { epochRotationPendingCase } from "./cases/case-epoch-rotation-pending";
import { contextSwitchPendingCase } from "./cases/case-context-switch-pending";

/** The KMS topology fields a case may predicate on. Mirrors `state.scenario.kms`. */
export type KmsTopology = State["scenario"]["kms"];

/**
 * Runs an in-container check of the KMS-context extraData, given the pair the orchestrator observed
 * on chain. Throws when the spec fails or matches no tests.
 *
 * The spec reads the active pair itself, so it can run standalone; the injected values let it also
 * assert that the chain did not move between the orchestrator's read and its own.
 */
export type ExtraDataCheckRunner = (
  label: string,
  expected: {
    readonly contextId: bigint;
    readonly epochId: bigint;
    /** The superseded pair, when the scenario asserts it must NOT appear in the extraData. */
    readonly previousContextId?: bigint;
    readonly previousEpochId?: bigint;
    /**
     * Ids that exist on chain but must NOT appear in the extraData because they are not active yet —
     * a Pending rotation's or switch's target. The mirror of `previous*`: those are behind the
     * active pair, these are ahead of it.
     */
    readonly forbiddenContextId?: bigint;
    readonly forbiddenEpochId?: bigint;
  },
) => Promise<void>;

/**
 * Everything a case is allowed to touch.
 *
 * Injected rather than imported, so cases hold no module state, can run in any order, and are
 * trivially testable in isolation should we ever want to.
 */
export type QaCaseContext = {
  readonly state: State;
  readonly target: ProtocolConfigTarget;
  readonly owner: Owner;
  readonly nodes: NodeSupervisor;
  readonly evidence: CaseEvidence;
  /** Runs the e2e user-decryption grep inside the test-suite container. */
  readonly runDecryption: DecryptionRunner;
  /** Runs the e2e input-proof smoke inside the test-suite container. */
  readonly runSmoke: SmokeRunner;
  /** Runs the KMS-context extraData spec inside the test-suite container. */
  readonly runExtraDataCheck: ExtraDataCheckRunner;
};

/**
 * Topology preconditions for a case.
 *
 * Checked for every selected case *before the first one runs*, so a long run never dies halfway
 * through on something knowable up front.
 */
export type QaCaseRequirements = {
  /** Required KMS mode; `undefined` means any. */
  readonly mode?: KmsTopology["mode"];
  /** Minimum total cores provisioned in the cluster. */
  readonly minParties?: number;
  /** Minimum spare cores (`parties - committeeSize`), for cases that promote a spare. */
  readonly minSpares?: number;
  /** Minimum initial on-chain committee size. */
  readonly minCommitteeSize?: number;
};

/** One QA scenario, implemented host-side. */
export type QaCase = {
  /** Stable selector used by `KMS_QA_CASES`; never rename without updating qa-kms-context-scenario-1-epoch.md. */
  readonly id: string;
  /** One-line human title, printed in the run banner. */
  readonly title: string;
  /** The QA claim this case establishes, printed verbatim in the final summary. */
  readonly proves: string;
  readonly requirements: QaCaseRequirements;
  /**
   * True when the case advances on-chain context/epoch state, making the stack non-pristine.
   * Informational: it drives the "re-up before rerunning" warning in the run banner.
   */
  readonly mutatesLifecycle: boolean;
  readonly run: (ctx: QaCaseContext) => Promise<void>;
};

/**
 * Every implemented case, in execution order.
 *
 * Order is fixed here and never taken from user input: these cases mutate shared on-chain state, so
 * a run must be reproducible regardless of how the selector was typed.
 */
export const QA_CASES: readonly QaCase[] = [
  epochRotationCase,
  contextSwitchCase,
  epochRotationPendingCase,
  contextSwitchPendingCase,
];

/** Environment variable selecting which cases run. Unset or `all` runs everything. */
export const CASE_SELECTOR_ENV = "KMS_QA_CASES";

/**
 * Returns the human-readable reason `topology` cannot satisfy `requirements`, or undefined when it
 * can. Pure; exported for unit testing.
 */
export const checkCaseRequirements = (
  requirements: QaCaseRequirements,
  topology: KmsTopology,
): string | undefined => {
  if (requirements.mode && topology.mode !== requirements.mode) {
    return `requires a ${requirements.mode}-mode KMS, but the active scenario is ${topology.mode}`;
  }
  if (requirements.minParties !== undefined && topology.parties < requirements.minParties) {
    return `requires at least ${requirements.minParties} KMS core(s), but the active scenario has ${topology.parties}`;
  }
  if (requirements.minCommitteeSize !== undefined && topology.committeeSize < requirements.minCommitteeSize) {
    return `requires a committee of at least ${requirements.minCommitteeSize}, but the active scenario has ${topology.committeeSize}`;
  }
  if (requirements.minSpares !== undefined) {
    const spares = topology.parties - topology.committeeSize;
    if (spares < requirements.minSpares) {
      return `requires at least ${requirements.minSpares} spare core(s), but the active scenario has ${spares} (parties=${topology.parties}, committeeSize=${topology.committeeSize})`;
    }
  }
  return undefined;
};

/**
 * Resolves a `KMS_QA_CASES` value into the cases to run.
 *
 * Accepts a comma-separated list of ids, or `all` / empty / undefined for everything. The result is
 * always in registry order and deduplicated, never in the order the ids were typed — see
 * {@link QA_CASES}. Pure; exported for unit testing.
 *
 * @throws PreflightError listing the available ids when one is unknown.
 */
export const selectCases = (cases: readonly QaCase[], spec: string | undefined): QaCase[] => {
  const trimmed = (spec ?? "").trim();
  if (!trimmed || trimmed.toLowerCase() === "all") return [...cases];

  const requested = trimmed
    .split(",")
    .map((id) => id.trim())
    .filter(Boolean);
  if (!requested.length) return [...cases];

  const known = new Map(cases.map((item) => [item.id, item]));
  const unknown = requested.filter((id) => !known.has(id));
  if (unknown.length) {
    throw new PreflightError(
      `${CASE_SELECTOR_ENV}: unknown case id(s) ${unknown.join(", ")}. Available: ${
        cases.map((item) => `${item.id} (${item.title})`).join("; ") || "none"
      }`,
    );
  }
  const selected = new Set(requested);
  return cases.filter((item) => selected.has(item.id));
};
