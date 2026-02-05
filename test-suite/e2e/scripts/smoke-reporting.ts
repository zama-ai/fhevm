export type SmokeContext = {
  hostname?: string;
  network?: string;
  chainId?: string;
  phase?: string;
  signer?: string;
  contract?: string;
  handle?: string;
  lastTx?: {
    label: string;
    nonce: number;
    hash?: string;
  };
};

type RelayerMeta = {
  url?: string;
  fetchMethod?: string;
  operation?: string;
  status?: number;
  jobId?: string;
  bodyJson?: unknown;
};

export type SmokeFailureReport = {
  errorMessage: string;
  summary: string;
  relayerMetaJson: string;
  rawErrorBlob: string;
  heartbeatPayload: string;
};

export const formatLogValue = (value: unknown, maxLen = 500): string => {
  if (value === null || value === undefined) return String(value);
  if (typeof value === 'string') return value.length > maxLen ? `${value.slice(0, maxLen)}…` : value;
  if (typeof value === 'bigint') return value.toString();
  if (typeof value === 'number' || typeof value === 'boolean') return String(value);
  try {
    const json = JSON.stringify(value, (_key, val) => (typeof val === 'bigint' ? val.toString() : val));
    if (typeof json !== 'string') return String(value);
    return json.length > maxLen ? `${json.slice(0, maxLen)}…` : json;
  } catch {
    const str = String(value);
    return str.length > maxLen ? `${str.slice(0, maxLen)}…` : str;
  }
};

export const describeError = (error: unknown): string => {
  if (error instanceof Error) {
    const err = error as Error & {
      code?: string;
      reason?: string;
      shortMessage?: string;
      data?: unknown;
    };
    const details: string[] = [];
    if (err.code) details.push(`code=${err.code}`);
    if (err.reason) details.push(`reason=${err.reason}`);
    if (err.shortMessage) details.push(`shortMessage=${err.shortMessage}`);
    if (err.data !== undefined) details.push(`data=${formatLogValue(err.data)}`);
    return details.length > 0 ? `${err.message} (${details.join(' ')})` : err.message;
  }
  return String(error);
};

const extractRelayerMeta = (error: unknown): RelayerMeta => {
  if (!(error instanceof Error)) return {};
  const err = error as Error & Record<string, unknown>;

  const meta: RelayerMeta = {};
  meta.url = typeof err.url === 'string' ? err.url : typeof err._url === 'string' ? err._url : undefined;
  meta.fetchMethod =
    typeof err.fetchMethod === 'string'
      ? err.fetchMethod
      : typeof err._fetchMethod === 'string'
        ? err._fetchMethod
        : undefined;
  meta.operation =
    typeof err.operation === 'string' ? err.operation : typeof err._operation === 'string' ? err._operation : undefined;
  meta.status = typeof err.status === 'number' ? err.status : typeof err._status === 'number' ? err._status : undefined;
  meta.jobId = typeof err.jobId === 'string' ? err.jobId : typeof err._jobId === 'string' ? err._jobId : undefined;
  meta.bodyJson = err.bodyJson ?? err._bodyJson;
  return meta;
};

const firstLine = (value: string): string => value.split('\n')[0] ?? value;

const shortenUrl = (url: string, maxLen = 160): string => {
  try {
    const parsed = new URL(url);
    const short = `${parsed.host}${parsed.pathname}`;
    return short.length > maxLen ? `${short.slice(0, maxLen)}…` : short;
  } catch {
    return url.length > maxLen ? `${url.slice(0, maxLen)}…` : url;
  }
};

const getAlertHint = (error: unknown, failureClass: string): string | undefined => {
  if (failureClass === 'decrypt_timeout') return 'decrypt_timeout';
  const message = error instanceof Error ? error.message : String(error);
  if (/Fetch (GET|POST) failed\\./.test(message)) return 'relayer_fetch_failed';
  if (/Bad JSON\\./.test(message)) return 'relayer_bad_json';
  if (/does not match the expected schema/.test(message)) return 'relayer_unexpected_response';
  if (/No clean signer available/.test(message) || /No clean signer has sufficient balance/.test(message))
    return 'no_clean_signer';
  if (/All \\d+ attempts failed/.test(message)) return 'tx_all_attempts_failed';
  return undefined;
};

const getCauseSummary = (error: unknown): string | undefined => {
  if (!(error instanceof Error)) return undefined;
  const maybeCause = (error as Error & { cause?: unknown }).cause;
  if (!maybeCause) return undefined;
  if (maybeCause instanceof Error) return firstLine(maybeCause.message);
  if (typeof maybeCause === 'string') return firstLine(maybeCause);
  if (typeof maybeCause === 'object') {
    const causeObj = maybeCause as Record<string, unknown>;
    const msg = typeof causeObj.message === 'string' ? causeObj.message : undefined;
    return msg ? firstLine(msg) : undefined;
  }
  return undefined;
};

const buildResponderSummary = (params: { failureClass: string; error: unknown; context: SmokeContext }): string => {
  const { failureClass, error, context } = params;
  const hint = getAlertHint(error, failureClass);
  const relayer = extractRelayerMeta(error);
  const cause = getCauseSummary(error);
  const errorName = error instanceof Error ? error.name : undefined;
  const message = error instanceof Error ? error.message : String(error);
  const msg = firstLine(message);
  const parts = [
    'smoke_failed',
    `class=${failureClass}`,
    hint ? `hint=${hint}` : null,
    context.network ? `network=${context.network}` : null,
    context.chainId ? `chainId=${context.chainId}` : null,
    context.phase ? `phase=${context.phase}` : null,
    context.hostname ? `host=${context.hostname}` : null,
    context.signer ? `signer=${context.signer}` : null,
    context.contract ? `contract=${context.contract}` : null,
    context.handle ? `handle=${context.handle}` : null,
    context.lastTx?.label ? `txLabel=${context.lastTx.label}` : null,
    context.lastTx?.nonce !== undefined ? `nonce=${context.lastTx.nonce}` : null,
    context.lastTx?.hash ? `txHash=${context.lastTx.hash}` : null,
    relayer.operation ? `relayerOp=${relayer.operation}` : null,
    relayer.fetchMethod ? `relayerMethod=${relayer.fetchMethod}` : null,
    relayer.status !== undefined ? `relayerStatus=${relayer.status}` : null,
    relayer.jobId ? `relayerJobId=${relayer.jobId}` : null,
    relayer.url ? `relayerUrl=${shortenUrl(relayer.url)}` : null,
    errorName ? `error=${errorName}` : null,
    cause ? `cause=${formatLogValue(cause, 180)}` : null,
    `msg=${formatLogValue(msg, 220)}`,
    relayer.bodyJson !== undefined ? `relayerBody=${formatLogValue(relayer.bodyJson, 300)}` : null,
  ]
    .filter(Boolean)
    .join(' ');

  return parts.length > 900 ? `${parts.slice(0, 900)}…` : parts;
};

const redactSecrets = (text: string): string => {
  const candidates = [
    process.env.ZAMA_FHEVM_API_KEY,
    process.env.MNEMONIC,
    process.env.PRIVATE_KEY,
    process.env.SECRET_KEY,
  ].filter((value): value is string => typeof value === 'string' && value.length >= 12);

  return candidates.reduce((acc, secret) => acc.split(secret).join('<redacted>'), text);
};

const buildRawErrorBlob = (error: unknown): string => {
  if (!(error instanceof Error)) return String(error);

  const err = error as Error & { cause?: unknown };
  const rawParts: string[] = [err.stack ?? err.message];

  const cause = err.cause;
  if (cause) {
    rawParts.push('\n--- cause ---');
    if (cause instanceof Error) rawParts.push(cause.stack ?? cause.message);
    else rawParts.push(typeof cause === 'string' ? cause : formatLogValue(cause, 4000));
  }

  return redactSecrets(rawParts.join('\n'));
};

const truncateText = (text: string, maxLen: number): string => {
  return text.length > maxLen ? `${text.slice(0, maxLen)}…` : text;
};

export const buildSmokeFailureReport = (params: {
  failureClass: string;
  error: unknown;
  context: SmokeContext;
  maxRawErrorLen?: number;
  maxHeartbeatLen?: number;
}): SmokeFailureReport => {
  const { failureClass, error, context, maxRawErrorLen = 6500, maxHeartbeatLen = 10_000 } = params;

  const errorMessage = describeError(error);
  const summary = buildResponderSummary({ failureClass, error, context });
  const relayerMetaJson = formatLogValue(extractRelayerMeta(error), 4000);
  const rawErrorBlob = truncateText(buildRawErrorBlob(error), maxRawErrorLen);

  const heartbeatPayload = truncateText(
    `${summary}\n\nRELAYER_META\n${relayerMetaJson}\n\nRAW_ERROR\n${rawErrorBlob}\n\nERROR_MESSAGE\n${errorMessage}`,
    maxHeartbeatLen,
  );

  return { errorMessage, summary, relayerMetaJson, rawErrorBlob, heartbeatPayload };
};
