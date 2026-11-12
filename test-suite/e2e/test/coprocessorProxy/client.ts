// The HTTPS client of the suites that call coprocessor-proxy.
import { readFileSync } from 'node:fs';
import type { IncomingHttpHeaders } from 'node:http';
import https from 'node:https';

export const envList = (name: string): string[] =>
  (process.env[name] ?? '')
    .split(',')
    .map((item) => item.trim())
    .filter(Boolean);

export interface Reply {
  status: number;
  headers: IncomingHttpHeaders;
  body: Buffer;
}

/** A request over TLS, trusting the proxies' self-signed test certificate. */
export function request(
  method: string,
  url: string,
  authorization: string,
  opts: { signal?: AbortSignal } = {},
): Promise<Reply> {
  const ca = readFileSync(process.env.COPROCESSOR_PROXY_CA_CERT!);
  const timeout = AbortSignal.timeout(30_000);
  const signal = opts.signal ? AbortSignal.any([timeout, opts.signal]) : timeout;
  return new Promise((resolve, reject) => {
    const req = https
      .request(url, { method, ca, headers: { authorization }, signal }, (res) => {
        const chunks: Buffer[] = [];
        res.on('data', (chunk: Buffer) => chunks.push(chunk));
        res.on('end', () =>
          resolve({ status: res.statusCode ?? 0, headers: res.headers, body: Buffer.concat(chunks) }),
        );
      })
      .on('error', (err) => reject(new Error(`${err.message}${req.reusedSocket ? ' on a reused connection' : ''}`)));
    req.end();
  });
}

/** Sums the series of counter `name` that have `label` in their labels. */
export async function metric(metricsUrl: string, name: string, label = ''): Promise<number> {
  const resp = await fetch(metricsUrl, { signal: AbortSignal.timeout(10_000) });
  return (await resp.text())
    .split('\n')
    .filter((line) => line.startsWith(`${name}{`) && line.includes(label))
    .reduce((sum, line) => sum + Number(line.split(' ').pop()), 0);
}
