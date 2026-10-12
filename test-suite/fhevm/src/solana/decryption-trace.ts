// decryption-trace — follows one decryption from the client to the KMS Connectors.
//
// relayer-http answers every request with `x-request-id` and writes each of its log lines with that
// request id. Once it forwards a request it adds the Connectors' `decryption_id`, the id every
// kms-connector endpoint logs. Its response bodies carry neither the decryption id nor the per-node
// outcomes, so the logs are the only link from a client's call to what each node did. Reading them
// needs the local stack's containers.
import { RELAYER_HTTP_CONTAINER } from '../layout';
import { kmsConnectorPrefix } from '../kms-party';
import { run } from '../utils/process';
import { until } from '../utils/until';

const DECRYPTION_ROUTE = /\/v4\/(user|public)-decrypt$/;
/** The last line relayer-http writes about a request (`endpoint/docs.md` §9). */
const TERMINAL_MESSAGES = new Set(['request succeeded', 'request rejected', 'request failed']);
/** Wide enough for the slowest scenario step, narrow enough to keep a long run's logs cheap. */
const LOG_WINDOW = '30m';

/** One v4 decryption POST, as the client saw it. */
export type DecryptionCall = {
  readonly route: 'user-decrypt' | 'public-decrypt';
  readonly status: number;
  readonly requestId: string;
  /** The error body's code; absent on a 200. */
  readonly code?: string | undefined;
  readonly requestBody: string;
  readonly responseBody: string;
};

/** One relayer-http log line about a request: its message and its fields, span fields included. */
export type RelayerHttpLine = { readonly message: string; readonly fields: Readonly<Record<string, unknown>> };

/** What relayer-http logged about one KMS node's call: the node's last line for the request. */
export type RelayerHttpNodeOutcome = {
  readonly node: string;
  readonly message: string;
  readonly elapsedMs: number;
  readonly attempts: number | undefined;
  readonly code: string | undefined;
};

const containerLogs = async (container: string, since: string = LOG_WINDOW): Promise<string> => {
  const { stdout, stderr } = await run(['docker', 'logs', '--since', since, container]);
  return `${stdout}\n${stderr}`;
};

/** relayer-http's JSON lines, each with its span fields folded into its own. */
const parseRelayerHttpLines = (logs: string): RelayerHttpLine[] =>
  logs
    .split('\n')
    .filter((line) => line.startsWith('{'))
    .map((line) => {
      const { fields, span } = JSON.parse(line) as { fields?: Record<string, unknown>; span?: Record<string, unknown> };
      const merged = { ...span, ...fields };
      return { message: String(merged.message), fields: merged };
    });

const recordCall = async (route: string, init: RequestInit | undefined, response: Response): Promise<DecryptionCall> => {
  const responseBody = await response.clone().text();
  let code: string | undefined;
  if (!response.ok) {
    const parsed = JSON.parse(responseBody) as { code?: unknown };
    code = typeof parsed.code === 'string' ? parsed.code : undefined;
  }
  const requestId = response.headers.get('x-request-id');
  if (requestId === null) throw new Error(`relayer-http answered ${route} without x-request-id`);
  return {
    route: route as DecryptionCall['route'],
    status: response.status,
    requestId,
    code,
    requestBody: String(init?.body ?? ''),
    responseBody,
  };
};

/**
 * Wraps `fetch` so that every v4 decryption POST is handed to `onCall` once answered, and returns the
 * function that restores the previous `fetch`. Other requests pass through untouched.
 */
export const interceptDecryptionCalls = (onCall: (call: DecryptionCall) => Promise<void> | void): (() => void) => {
  const previous = globalThis.fetch;
  globalThis.fetch = (async (input: RequestInfo | URL, init?: RequestInit) => {
    const response = await previous(input as never, init);
    const route = DECRYPTION_ROUTE.exec(String(input instanceof Request ? input.url : input));
    if (route && (init?.method ?? 'GET') === 'POST') await onCall(await recordCall(`${route[1]}-decrypt`, init, response));
    return response;
  }) as typeof fetch;
  return () => {
    globalThis.fetch = previous;
  };
};

/** Runs `action` and returns the v4 decryption POSTs it made, in order. */
export const recordDecryptionCalls = async <T>(action: () => Promise<T>): Promise<{ result: T; calls: DecryptionCall[] }> => {
  const calls: DecryptionCall[] = [];
  const restore = interceptDecryptionCalls((call) => {
    calls.push(call);
  });
  try {
    return { result: await action(), calls };
  } finally {
    restore();
  }
};

/**
 * relayer-http's lines about `requestId`, once its last line is written. The aggregator's lines carry
 * the request id in their span, the handler's in their own fields.
 */
export const relayerHttpLines = async (requestId: string): Promise<RelayerHttpLine[]> =>
  until(
    async () => {
      const lines = parseRelayerHttpLines(await containerLogs(RELAYER_HTTP_CONTAINER)).filter(
        (line) => line.fields.request_id === requestId,
      );
      return lines.some((line) => TERMINAL_MESSAGES.has(line.message)) ? lines : undefined;
    },
    { description: `relayer-http's last log line for request ${requestId}`, timeoutMs: 10_000, intervalMs: 250 },
  );

/** Every request relayer-http finished since `since`, by request id. */
export const relayerHttpRequestsSince = async (since: Date): Promise<Map<string, RelayerHttpLine[]>> => {
  const requests = new Map<string, RelayerHttpLine[]>();
  for (const line of parseRelayerHttpLines(await containerLogs(RELAYER_HTTP_CONTAINER, since.toISOString()))) {
    const requestId = line.fields.request_id;
    if (typeof requestId === 'string') requests.set(requestId, [...(requests.get(requestId) ?? []), line]);
  }
  for (const [requestId, lines] of requests) if (!lines.some((line) => TERMINAL_MESSAGES.has(line.message))) requests.delete(requestId);
  return requests;
};

/** The Connectors' decryption id relayer-http logged for the request, once it forwarded it. */
export const loggedDecryptionId = (lines: readonly RelayerHttpLine[]): string | undefined => {
  const id = lines.map((line) => line.fields.decryption_id).find((value) => typeof value === 'string' && value !== 'none');
  return id as string | undefined;
};

/** True when relayer-http sent the request on to the Connectors. */
export const wasForwarded = (lines: readonly RelayerHttpLine[]): boolean =>
  lines.some((line) => line.message === 'request forwarded');

/**
 * Each node's last line for the request: how its call ended and how long it took. The code is the
 * one its last failed attempt logged, since `call failed` carries none.
 */
export const nodeOutcomes = (lines: readonly RelayerHttpLine[]): RelayerHttpNodeOutcome[] => {
  const last = new Map<string, RelayerHttpLine>();
  const codes = new Map<string, string>();
  for (const line of lines) {
    if (typeof line.fields.node !== 'string') continue;
    last.set(line.fields.node, line);
    if (typeof line.fields.code === 'string') codes.set(line.fields.node, line.fields.code);
  }
  return [...last.entries()].map(([node, line]) => ({
    node,
    message: line.message,
    elapsedMs: Number(line.fields.elapsed_ms),
    attempts: line.fields.attempts === undefined ? undefined : Number(line.fields.attempts),
    code: codes.get(node),
  }));
};

/** The aggregation's final line: `aggregation succeeded` or `aggregation failed`. */
export const aggregationOutcome = (lines: readonly RelayerHttpLine[]): RelayerHttpLine | undefined =>
  lines.find((line) => line.message === 'aggregation succeeded' || line.message === 'aggregation failed');

/** Party i's kms-connector endpoint lines that name `decryptionId`, colour codes removed. */
export const connectorEndpointLines = async (party: number, decryptionId: string): Promise<string[]> =>
  (await containerLogs(`${kmsConnectorPrefix(party)}-endpoint`))
    .split('\n')
    .map((line) => line.replace(/\x1b\[[0-9;]*m/g, ''))
    .filter((line) => line.includes(decryptionId));

/**
 * One call, followed through the logs: its request id, the decryption id relayer-http derived, each
 * node's outcome, and the endpoint lines of every party in `parties` that name that decryption id.
 */
export const describeDecryptionCall = async (call: DecryptionCall, parties: readonly number[]): Promise<string> => {
  const lines = await relayerHttpLines(call.requestId);
  const decryptionId = loggedDecryptionId(lines);
  const nodes = nodeOutcomes(lines).map(
    (node) => `  ${node.node}: ${node.message}${node.code === undefined ? '' : ` ${node.code}`} after ${node.elapsedMs} ms`,
  );
  const connectors =
    decryptionId === undefined
      ? ['  not forwarded to the Connectors']
      : (
          await Promise.all(
            parties.map(async (party) =>
              (await connectorEndpointLines(party, decryptionId)).map((line) => `  ${kmsConnectorPrefix(party)}-endpoint: ${line}`),
            ),
          )
        ).flat();
  return [
    `[decryption] ${call.route} -> ${call.status}${call.code === undefined ? '' : ` ${call.code}`} request_id=${call.requestId} decryption_id=${decryptionId ?? 'none'}`,
    ...nodes,
    ...connectors,
  ].join('\n');
};
