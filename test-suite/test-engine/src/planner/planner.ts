import { parse as parseTagExpression } from '@cucumber/tag-expressions';

import type { DiscoveredScenario } from '../discovery/discovery.js';

/**
 * Contract of the Planner component: given the discovered scenarios and the user's selection,
 * decide what runs and what is skipped, always with a reason. No side effects.
 */

export interface SelectionFilters {
  /** Explicit scenario ids. Empty means "every scenario". */
  ids: string[];
  /** Cucumber tag expression evaluated against the manifest tags (e.g. `@smoke and not @slow`). */
  tagExpression?: string;
}

export type PlanDecision = { action: 'run' } | { action: 'skip'; reason: string };

export interface PlanItem {
  scenario: DiscoveredScenario;
  decision: PlanDecision;
}

export interface ExecutionPlan {
  filters: SelectionFilters;
  /** Name of the profile the plan was made for. */
  profile: string;
  /** Selected scenarios, in execution order. Scenarios excluded by the filters are not listed. */
  items: PlanItem[];
}

export class PlanError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'PlanError';
  }
}

/** Tags are written without `@` in manifests and with `@` in tag expressions. */
function asCucumberTags(tags: string[]): string[] {
  return tags.map((tag) => `@${tag}`);
}

export function selectScenarios(scenarios: DiscoveredScenario[], filters: SelectionFilters): DiscoveredScenario[] {
  const known = new Set(scenarios.map((scenario) => scenario.manifest.metadata.id));
  const unknown = filters.ids.filter((id) => !known.has(id));
  if (unknown.length > 0) {
    throw new PlanError(
      `Unknown scenario id(s): ${unknown.join(', ')}. Available: ${[...known].join(', ') || '(none)'}`,
    );
  }

  let matchesTags: (tags: string[]) => boolean = () => true;
  if (filters.tagExpression !== undefined) {
    try {
      const expression = parseTagExpression(filters.tagExpression);
      matchesTags = (tags) => expression.evaluate(asCucumberTags(tags));
    } catch (error) {
      throw new PlanError(`Invalid tag expression '${filters.tagExpression}': ${(error as Error).message}`);
    }
  }

  const ids = new Set(filters.ids);
  return scenarios.filter(
    ({ manifest }) => (ids.size === 0 || ids.has(manifest.metadata.id)) && matchesTags(manifest.metadata.tags),
  );
}

/** What the planner needs to know about the target profile. */
export interface PlanningProfile {
  name: string;
  requiresConfirmation: boolean;
}

/**
 * Decides whether a scenario runs on the profile. The first matching rule wins:
 * 1. disabled → skip; 2. profile requires confirmation and is not listed in `targets` → skip;
 * 3. `targets` present without the profile → skip; 4. run.
 */
export function decide(scenario: DiscoveredScenario, profile: PlanningProfile): PlanDecision {
  const { spec } = scenario.manifest;
  if (!spec.enabled) {
    return { action: 'skip', reason: 'disabled in manifest (spec.enabled: false)' };
  }
  const targets = spec.targets;
  if (profile.requiresConfirmation && !targets?.includes(profile.name)) {
    return { action: 'skip', reason: `profile '${profile.name}' must be listed explicitly in spec.targets` };
  }
  if (targets && !targets.includes(profile.name)) {
    return { action: 'skip', reason: `profile '${profile.name}' not in spec.targets [${targets.join(', ')}]` };
  }
  return { action: 'run' };
}

export function planExecution(
  scenarios: DiscoveredScenario[],
  filters: SelectionFilters,
  profile: PlanningProfile,
): ExecutionPlan {
  const selected = selectScenarios(scenarios, filters);
  if (selected.length === 0) {
    throw new PlanError('No scenario matches the selection.');
  }
  const items = selected.map((scenario): PlanItem => ({ scenario, decision: decide(scenario, profile) }));
  return { filters, profile: profile.name, items };
}

/** True when the plan has at least one scenario to run. */
export function hasRunnableItems(plan: ExecutionPlan): boolean {
  return plan.items.some((item) => item.decision.action === 'run');
}

/**
 * Applies the confirmation rules of sensitive profiles: `run` on a profile with
 * `requiresConfirmation` needs `--confirm <name>`; a confirmation with a different name, or on a
 * profile that does not require it, is rejected (it usually means the wrong profile was selected).
 *
 * @throws PlanError
 */
export function checkConfirmation(
  command: 'run' | 'dry-run',
  profile: PlanningProfile,
  confirm: string | undefined,
): void {
  if (confirm !== undefined) {
    if (!profile.requiresConfirmation) {
      throw new PlanError(
        `--confirm is only accepted for profiles that require confirmation; '${profile.name}' does not`,
      );
    }
    if (confirm !== profile.name) {
      throw new PlanError(`--confirm '${confirm}' does not match the selected profile '${profile.name}'`);
    }
  }
  if (command === 'run' && profile.requiresConfirmation && confirm === undefined) {
    throw new PlanError(`profile '${profile.name}' requires confirmation: add --confirm ${profile.name}`);
  }
}
