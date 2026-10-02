import { isIP } from "node:net";

/** Bind endpoint overrides to the one managed SNS queue and immutable image. */
export function storageRoute(snapshot: unknown, url?: string): { upstream: string; override: unknown } {
  const row = (snapshot as any)?.[0];
  const service = row?.Config?.Labels?.['com.docker.compose.service'];
  if (service !== 'coprocessor1-sns-worker' || !/^sha256:[a-f0-9]{64}$/.test(row?.Image ?? '') ||
      !Array.isArray(row?.Config?.Env)) throw new Error('unowned SNS route');
  const values = new Map<string, string>();
  for (const entry of row.Config.Env) {
    if (typeof entry !== 'string' || !entry.includes('=')) throw new Error('invalid runtime environment');
    const equals = entry.indexOf('=');
    const key = entry.slice(0, equals);
    if (values.has(key)) throw new Error('ambiguous runtime environment');
    values.set(key, entry.slice(equals + 1));
  }
  const upstream = values.get('AWS_ENDPOINT_URL_S3') || values.get('AWS_ENDPOINT_URL') || '';
  for (const value of [upstream, ...(url ? [url] : [])]) {
    const parsed = new URL(value);
    if (parsed.protocol !== 'http:' || parsed.username || parsed.password || parsed.pathname !== '/' || parsed.search || parsed.hash) throw new Error('isolated HTTP storage root required');
  }
  if (url && isIP(new URL(url).hostname) !== 4) throw new Error("IPv4 proxy endpoint required to preserve S3 path-style addressing");
  return { upstream, override: { services: { [service]: { image: row.Image,
    environment: { AWS_ENDPOINT_URL: url ?? upstream, AWS_ENDPOINT_URL_S3: url ?? upstream },
  } } } };
}
