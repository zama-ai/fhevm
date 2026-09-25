import type { StepReport, StepStatus, TestCaseReport } from './messages.js';
import type { RunReport, ScenarioReport } from './report.js';

/**
 * Renders the consolidated run report as a single self-contained HTML file (no external assets),
 * readable offline and attachable as a CI artifact.
 */

const escapeHtml = (value: string): string =>
  value.replace(/[&<>"']/g, (char) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[char]!);

function formatDuration(ms: number): string {
  if (ms < 1000) return `${Math.round(ms)} ms`;
  if (ms < 60_000) return `${(ms / 1000).toFixed(2)} s`;
  const minutes = Math.floor(ms / 60_000);
  return `${minutes} min ${Math.round((ms % 60_000) / 1000)} s`;
}

type Tone = 'ok' | 'bad' | 'warn' | 'muted';

function outcomeTone(outcome: ScenarioReport['outcome']): Tone {
  if (outcome === 'passed') return 'ok';
  if (outcome === 'skipped') return 'muted';
  if (outcome === 'timed-out') return 'warn';
  return 'bad';
}

function stepTone(status: StepStatus): Tone {
  if (status === 'passed') return 'ok';
  if (status === 'failed' || status === 'ambiguous' || status === 'undefined') return 'bad';
  if (status === 'pending') return 'warn';
  return 'muted';
}

/** In dry-run, "passed" means "valid": nothing was executed. */
function outcomeLabel(outcome: ScenarioReport['outcome'], dryRun: boolean): string {
  if (!dryRun) return outcome;
  if (outcome === 'passed') return 'valid';
  if (outcome === 'failed') return 'invalid';
  return outcome;
}

const badge = (label: string, tone: Tone) => `<span class="badge ${tone}">${escapeHtml(label)}</span>`;

function renderStep(step: StepReport): string {
  const error = step.error ? `<pre class="error">${escapeHtml(step.error)}</pre>` : '';
  return `<tr>
    <td>${badge(step.status, stepTone(step.status))}</td>
    <td><span class="kw">${escapeHtml(step.keyword)}</span> ${escapeHtml(step.text)}${error}</td>
    <td class="num">${step.line ?? ''}</td>
    <td class="num">${formatDuration(step.durationMs)}</td>
  </tr>`;
}

function renderTestCase(testCase: TestCaseReport): string {
  const tags = testCase.tags.map((tag) => `<span class="tag">${escapeHtml(tag)}</span>`).join('');
  return `<div class="case">
    <div class="case-head">
      ${badge(testCase.status, stepTone(testCase.status))}
      <strong>${escapeHtml(testCase.name)}</strong>
      <span class="muted">${escapeHtml(testCase.uri)}${testCase.line !== undefined ? `:${testCase.line}` : ''}</span>
      ${tags}
      <span class="spacer"></span>
      <span class="muted">${formatDuration(testCase.durationMs)}</span>
    </div>
    <table>
      <thead><tr><th>Status</th><th>Step</th><th class="num">Line</th><th class="num">Duration</th></tr></thead>
      <tbody>${testCase.steps.map(renderStep).join('')}</tbody>
    </table>
  </div>`;
}

function renderScenario(scenario: ScenarioReport, dryRun: boolean): string {
  const tone = outcomeTone(scenario.outcome);
  const cucumber = scenario.cucumber;
  const issues = cucumber?.issues.length
    ? `<ul class="issues">${[...new Set(cucumber.issues)].map((issue) => `<li>${escapeHtml(issue)}</li>`).join('')}</ul>`
    : '';
  const artifacts = scenario.artifacts
    ? `<p class="links">
        <a href="${escapeHtml(scenario.artifacts.cucumberHtml)}">Cucumber report</a>
        <a href="${escapeHtml(scenario.artifacts.messages)}">Message stream</a>
        <a href="${escapeHtml(scenario.artifacts.console)}">Console log</a>
      </p>`
    : '';
  const meta: Array<[string, string]> = [
    ['Owner', scenario.owner],
    ['Tags', scenario.tags.join(', ') || '—'],
    ['Manifest', scenario.manifestPath],
    ['Feature', scenario.featurePath],
    ['Timeout', `${scenario.timeoutSeconds} s`],
    ...(scenario.origin ? ([['Origin', scenario.origin]] as Array<[string, string]>) : []),
  ];
  return `<details class="scenario ${tone}"${scenario.outcome === 'passed' || scenario.outcome === 'skipped' ? '' : ' open'}>
    <summary>
      ${badge(outcomeLabel(scenario.outcome, dryRun), tone)}
      <code>${escapeHtml(scenario.id)}</code>
      <span>${escapeHtml(scenario.name)}</span>
      <span class="spacer"></span>
      <span class="muted">${formatDuration(scenario.durationMs)}</span>
    </summary>
    <div class="body">
      <p>${escapeHtml(scenario.description)}</p>
      ${scenario.reason ? `<p class="reason">${escapeHtml(scenario.reason)}</p>` : ''}
      <dl>${meta.map(([key, value]) => `<dt>${key}</dt><dd>${escapeHtml(value)}</dd>`).join('')}</dl>
      ${artifacts}
      ${issues}
      ${cucumber?.testCases.map(renderTestCase).join('') ?? ''}
    </div>
  </details>`;
}

export function renderHtmlReport(report: RunReport): string {
  const dryRun = report.run.mode === 'dry-run';
  const { summary } = report;
  const tiles: Array<[string, number, Tone]> = [
    ['Total', summary.total, 'muted'],
    [dryRun ? 'Valid' : 'Passed', summary.passed, 'ok'],
    [dryRun ? 'Invalid' : 'Failed', summary.failed, 'bad'],
    ['Timed out', summary['timed-out'], 'warn'],
    ['Errors', summary.error, 'bad'],
    ['Skipped', summary.skipped, 'muted'],
  ];
  const filters = [
    report.run.filters.ids.length ? `ids: ${report.run.filters.ids.join(', ')}` : '',
    report.run.filters.tagExpression ? `tags: ${report.run.filters.tagExpression}` : '',
  ].filter(Boolean);

  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Test Engine Report</title>
<style>
  :root {
    --bg: #f7f7f8; --surface: #ffffff; --text: #1d1d22; --muted: #6b6b76; --border: #e2e2e7;
    --ok: #1a7f37; --ok-bg: #e6f4ea; --bad: #c62828; --bad-bg: #fdecea; --warn: #9a6700; --warn-bg: #fff4d6;
    --neutral-bg: #eeeef1; --code-bg: #f1f1f4;
  }
  @media (prefers-color-scheme: dark) {
    :root:not([data-theme="light"]) {
      --bg: #141417; --surface: #1d1d22; --text: #ececf1; --muted: #9a9aa6; --border: #2e2e36;
      --ok: #5cc27a; --ok-bg: #15301e; --bad: #ff7b72; --bad-bg: #3a1a1a; --warn: #e3b341; --warn-bg: #33290f;
      --neutral-bg: #2a2a31; --code-bg: #26262c;
    }
  }
  :root[data-theme="dark"] {
    --bg: #141417; --surface: #1d1d22; --text: #ececf1; --muted: #9a9aa6; --border: #2e2e36;
    --ok: #5cc27a; --ok-bg: #15301e; --bad: #ff7b72; --bad-bg: #3a1a1a; --warn: #e3b341; --warn-bg: #33290f;
    --neutral-bg: #2a2a31; --code-bg: #26262c;
  }
  * { box-sizing: border-box; }
  body { margin: 0; background: var(--bg); color: var(--text); font: 14px/1.5 system-ui, -apple-system, "Segoe UI", sans-serif; }
  main { max-width: 1100px; margin: 0 auto; padding: 24px 16px 48px; }
  h1 { font-size: 22px; margin: 0 0 4px; }
  .muted { color: var(--muted); }
  .run-meta { display: flex; flex-wrap: wrap; gap: 4px 16px; margin: 0 0 20px; color: var(--muted); }
  .tiles { display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 8px; margin-bottom: 24px; }
  .tile { background: var(--surface); border: 1px solid var(--border); border-radius: 8px; padding: 10px 12px; }
  .tile .value { font-size: 24px; font-weight: 600; }
  .tile.ok .value { color: var(--ok); } .tile.bad .value { color: var(--bad); } .tile.warn .value { color: var(--warn); }
  .scenario { background: var(--surface); border: 1px solid var(--border); border-left: 4px solid var(--border); border-radius: 8px; margin-bottom: 10px; }
  .scenario.ok { border-left-color: var(--ok); } .scenario.bad { border-left-color: var(--bad); } .scenario.warn { border-left-color: var(--warn); }
  summary { display: flex; align-items: center; gap: 10px; padding: 10px 14px; cursor: pointer; flex-wrap: wrap; }
  .body { padding: 0 14px 14px; overflow-x: auto; }
  .spacer { flex: 1; }
  .badge { display: inline-block; padding: 1px 8px; border-radius: 999px; font-size: 12px; font-weight: 600; text-transform: uppercase; letter-spacing: .02em; }
  .badge.ok { color: var(--ok); background: var(--ok-bg); } .badge.bad { color: var(--bad); background: var(--bad-bg); }
  .badge.warn { color: var(--warn); background: var(--warn-bg); } .badge.muted { color: var(--muted); background: var(--neutral-bg); }
  code, pre { font-family: ui-monospace, SFMono-Regular, Menlo, monospace; font-size: 12.5px; }
  code { background: var(--code-bg); padding: 1px 6px; border-radius: 4px; }
  dl { display: grid; grid-template-columns: max-content 1fr; gap: 2px 16px; margin: 8px 0; }
  dt { color: var(--muted); } dd { margin: 0; word-break: break-all; }
  .reason { background: var(--neutral-bg); padding: 6px 10px; border-radius: 6px; }
  .links a { margin-right: 14px; color: inherit; }
  .issues { color: var(--bad); }
  .case { border-top: 1px solid var(--border); padding-top: 10px; margin-top: 10px; }
  .case-head { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; margin-bottom: 6px; }
  .tag { font-size: 12px; color: var(--muted); }
  table { width: 100%; border-collapse: collapse; }
  th, td { text-align: left; padding: 4px 8px; border-bottom: 1px solid var(--border); vertical-align: top; }
  th { font-weight: 500; color: var(--muted); font-size: 12px; }
  .num { text-align: right; white-space: nowrap; }
  .kw { font-weight: 600; }
  pre.error { white-space: pre-wrap; color: var(--bad); background: var(--bad-bg); padding: 8px; border-radius: 6px; margin: 6px 0 0; }
</style>
</head>
<body>
<main>
  <h1>Test engine ${dryRun ? 'dry-run' : 'run'} report</h1>
  <p class="run-meta">
    <span>Run <code>${escapeHtml(report.run.id)}</code></span>
    <span>${escapeHtml(report.run.startedAt)}</span>
    <span>${formatDuration(report.run.durationMs)}</span>
    <span>${escapeHtml(report.engine.name)} ${escapeHtml(report.engine.version)} · Node ${escapeHtml(report.engine.node)}</span>
    ${filters.length ? `<span>${escapeHtml(filters.join(' · '))}</span>` : ''}
    <span>${badge(summary.success ? 'success' : 'failure', summary.success ? 'ok' : 'bad')}</span>
  </p>
  <section class="tiles">
    ${tiles.map(([label, value, tone]) => `<div class="tile ${tone}"><div class="muted">${label}</div><div class="value">${value}</div></div>`).join('')}
  </section>
  ${report.scenarios.map((scenario) => renderScenario(scenario, dryRun)).join('')}
</main>
</body>
</html>
`;
}
