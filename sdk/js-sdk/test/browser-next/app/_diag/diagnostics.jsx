// Shared diagnostics for browser-next test cells: a log collector (with the SDK
// logger shape) plus a pretty renderer, so EVERY cell surfaces its logs the same
// readable way. Importantly, the `#log` block prints each entry on its own line with
// real newlines preserved — so the SDK's multi-line JSON debug dumps render legibly,
// instead of being escaped into one ugly `\n`-laden line by an outer JSON.stringify.
//
// Used by both client ('use client') and server-component cells; `DiagnosticsView` is
// purely presentational (no hooks/state), so it renders correctly in either context.
//
// Folder is `_diag` (underscore) so Next.js treats it as private (never routed).

// Collects log lines and exposes the SDK's logger shape (debug/info/warn/error).
export function createTestLogger() {
  const logs = [];
  const log = (message) => logs.push(message);
  const logger = {
    debug: (m) => log(`  [debug] ${m}`),
    info: (m) => log(`  [info] ${m}`),
    warn: (m) => log(`  [warn] ${m}`),
    error: (m, cause) => log(`  [error] ${m}${cause !== undefined ? `: ${String(cause)}` : ''}`),
  };
  return { logs, log, logger };
}

// Appends an error and its full `cause` chain as `[FAIL]` / `[cause]` lines.
export function logError(log, err) {
  log(`[FAIL] ${err instanceof Error ? (err.stack ?? err.message) : String(err)}`);
  let cause = err instanceof Error ? err.cause : undefined;
  while (cause !== undefined) {
    log(`  [cause] ${cause instanceof Error ? (cause.stack ?? cause.message) : String(cause)}`);
    cause = cause instanceof Error ? cause.cause : undefined;
  }
}

// Text for the `#log` <pre>: one entry per line, real newlines preserved.
export function renderLogs(logs) {
  return logs.join('\n');
}

// Presentational result view. `#result[data-status]` is what specs assert on; `#log`
// is the readable trace. No hooks → safe in client and server components alike.
//
// `idPrefix` namespaces the ids (`<prefix>-result` / `<prefix>-log`) and switches to a
// <section>/<h2> layout — used by the mixed cell, which renders two legs (ssr + csr)
// on one page. Without it, the default full-page <main>/<h1> with `#result` / `#log`.
export function DiagnosticsView({ title, status, logs, idPrefix }) {
  const resultId = idPrefix ? `${idPrefix}-result` : 'result';
  const logId = idPrefix ? `${idPrefix}-log` : 'log';
  const Tag = idPrefix ? 'section' : 'main';
  const Heading = idPrefix ? 'h2' : 'h1';
  return (
    <Tag>
      {title ? <Heading>{title}</Heading> : null}
      <p id={resultId} data-status={status}>
        {status}
      </p>
      <pre id={logId}>{renderLogs(logs)}</pre>
    </Tag>
  );
}
