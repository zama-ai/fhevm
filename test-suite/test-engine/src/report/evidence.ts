import type { EvidencePayload } from '../api/evidence.js';
import type { RunReport } from './report.js';

/**
 * Contract of `evidence.json`: every piece of evidence of a run, in one machine-readable file.
 * Two producers, by scope: `run` and `scenario` entries come from the engine (run metadata,
 * profile, plan decisions, outcomes); `step` and `hook` entries come from `evidence.record()`
 * attachments in the scenario message streams.
 */

export const EVIDENCE_SCHEMA_VERSION = 1;
export const EVIDENCE_FILE = 'evidence.json';

export type EvidenceScope = 'run' | 'scenario' | 'step' | 'hook';

export interface EvidenceEntry {
  scope: EvidenceScope;
  kind: string;
  label?: string;
  data: Record<string, unknown>;
  timestamp: string;
  /** Absent for `run` scope. */
  scenarioId?: string;
  /** `step` and `hook` scope only. */
  testCase?: { name: string; uri: string; line?: number };
  step?: { keyword: string; text: string; line?: number };
  hook?: { keyword: string; name: string; source?: string };
}

export interface EvidenceFile {
  schemaVersion: typeof EVIDENCE_SCHEMA_VERSION;
  runId: string;
  entries: EvidenceEntry[];
}

const fromPayload = (payload: EvidencePayload) => ({
  kind: payload.kind,
  ...(payload.label !== undefined ? { label: payload.label } : {}),
  data: payload.data,
  timestamp: payload.timestamp,
});

export function buildEvidence(report: RunReport): EvidenceFile {
  const { run, engine } = report;
  const entries: EvidenceEntry[] = [
    {
      scope: 'run',
      kind: 'engine.run',
      label: 'Run metadata',
      timestamp: run.startedAt,
      data: {
        runId: run.id,
        mode: run.mode,
        engine: engine.name,
        engineVersion: engine.version,
        node: engine.node,
        ...(run.gitCommit ? { gitCommit: run.gitCommit } : {}),
        startedAt: run.startedAt,
        finishedAt: run.finishedAt,
        filters: run.filters,
        interrupted: run.interrupted,
      },
    },
    {
      scope: 'run',
      kind: 'engine.profile',
      label: `Profile ${run.profile.name}`,
      timestamp: run.startedAt,
      data: { ...run.profile },
    },
  ];

  for (const scenario of report.scenarios) {
    entries.push({
      scope: 'scenario',
      kind: 'engine.scenario',
      label: `Scenario ${scenario.id}`,
      timestamp: scenario.startedAt,
      scenarioId: scenario.id,
      data: {
        decision: scenario.decision,
        outcome: scenario.outcome,
        ...(scenario.reason !== undefined ? { reason: scenario.reason } : {}),
        startedAt: scenario.startedAt,
        finishedAt: scenario.finishedAt,
        durationMs: scenario.durationMs,
      },
    });
    for (const testCase of scenario.cucumber?.testCases ?? []) {
      const testCaseRef = {
        name: testCase.name,
        uri: testCase.uri,
        ...(testCase.line !== undefined ? { line: testCase.line } : {}),
      };
      for (const step of testCase.steps) {
        for (const payload of step.evidence ?? []) {
          entries.push(
            step.kind === 'hook'
              ? {
                  scope: 'hook',
                  ...fromPayload(payload),
                  scenarioId: scenario.id,
                  testCase: testCaseRef,
                  hook: { keyword: step.keyword, name: step.text, ...(step.source ? { source: step.source } : {}) },
                }
              : {
                  scope: 'step',
                  ...fromPayload(payload),
                  scenarioId: scenario.id,
                  testCase: testCaseRef,
                  step: {
                    keyword: step.keyword,
                    text: step.text,
                    ...(step.line !== undefined ? { line: step.line } : {}),
                  },
                },
          );
        }
      }
    }
  }
  return { schemaVersion: EVIDENCE_SCHEMA_VERSION, runId: run.id, entries };
}
