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

export function planExecution(scenarios: DiscoveredScenario[], filters: SelectionFilters): ExecutionPlan {
  const selected = selectScenarios(scenarios, filters);
  if (selected.length === 0) {
    throw new PlanError('No scenario matches the selection.');
  }

  const items = selected.map((scenario): PlanItem => {
    if (!scenario.manifest.spec.enabled) {
      return { scenario, decision: { action: 'skip', reason: 'disabled in manifest (spec.enabled: false)' } };
    }
    return { scenario, decision: { action: 'run' } };
  });

  return { filters, items };
}
