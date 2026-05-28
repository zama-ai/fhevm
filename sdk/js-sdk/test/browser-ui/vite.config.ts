import type { IncomingMessage, ServerResponse } from 'node:http';
import type { ViteDevServer } from 'vite';
import { execFile } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { readFile } from 'node:fs/promises';
import { dirname, extname, isAbsolute, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';
import { defineConfig } from 'vite';

const __dirname = dirname(fileURLToPath(import.meta.url));
const execFileAsync = promisify(execFile);
const projectRoot = resolve(__dirname, '../..');
const rawWasmAssetPrefix = '/__raw_wasm';
const browserUiConfigPath = '/__browser_ui/config';
const browserUiEnsureFundsPath = '/__browser_ui/ensure-funded';
const localstackRelayerProxyPrefix = '/__localstack_relayer';
const localstackMinioProxyPrefix = '/__localstack_minio';
const localstackRelayerTarget = 'http://localhost:3000';
const localstackMinioTarget = 'http://localhost:9000';
const localFundingBalanceWei = '0x56BC75E2D63100000';

type BrowserUiChainTarget = 'testnet' | 'localstack' | 'localcleartext';

type BrowserUiConfig = {
  readonly targets: Record<
    BrowserUiChainTarget,
    {
      readonly rpcUrl: string;
      readonly mnemonic: string;
      readonly fheTestAddress: string;
    }
  >;
};

export default defineConfig({
  root: projectRoot,
  plugins: [browserUiPlugin()],
  build: {
    rollupOptions: {
      input: resolve(__dirname, 'index.html'),
    },
  },
  server: {
    port: 3335,
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
});

function browserUiPlugin() {
  return {
    name: 'browser-ui-local-services',
    configureServer(server: ViteDevServer) {
      server.middlewares.use(async (req: IncomingMessage, res: ServerResponse, next: () => void) => {
        try {
          const url = new URL(req.url ?? '/', 'http://localhost');

          if (url.pathname === browserUiConfigPath) {
            respondWithJson(res, loadBrowserUiConfig());
            return;
          }

          if (url.pathname === browserUiEnsureFundsPath) {
            await ensureSignerFunds(req, res);
            return;
          }

          if (url.pathname.startsWith(rawWasmAssetPrefix)) {
            await serveRawWasmAsset(req, res, url);
            return;
          }

          if (url.pathname.startsWith(localstackRelayerProxyPrefix)) {
            await proxyRequest({
              req,
              res,
              targetBaseUrl: localstackRelayerTarget,
              proxyPrefix: localstackRelayerProxyPrefix,
              rewriteRelayerKeyUrl: true,
            });
            return;
          }

          if (url.pathname.startsWith(localstackMinioProxyPrefix)) {
            await proxyRequest({
              req,
              res,
              targetBaseUrl: localstackMinioTarget,
              proxyPrefix: localstackMinioProxyPrefix,
              rewriteRelayerKeyUrl: false,
            });
            return;
          }

          next();
        } catch (err) {
          respondWithProxyError(res, err);
        }
      });
    },
  };
}

type ChainDefaults = {
  readonly rpcUrl: string;
  readonly mnemonic?: string;
  readonly fheTestAddress?: string;
};

const chainDefaultsPath = resolve(projectRoot, 'test/chains/chain-defaults.json');

function loadBrowserUiConfig(): BrowserUiConfig {
  const testDir = resolve(projectRoot, 'test');
  const sharedEnv = parseEnvFile(resolve(testDir, '.env'));
  const sharedMnemonic = sharedEnv.MNEMONIC ?? process.env.MNEMONIC;
  const defaults = JSON.parse(readFileSync(chainDefaultsPath, 'utf-8')) as Record<string, ChainDefaults>;

  return {
    targets: {
      testnet: resolveBrowserUiTarget(testDir, 'sepolia', defaults, sharedMnemonic),
      localstack: resolveBrowserUiTarget(testDir, 'localstack', defaults, sharedMnemonic),
      localcleartext: resolveBrowserUiTarget(testDir, 'localcleartext', defaults, sharedMnemonic),
    },
  };
}

function resolveBrowserUiTarget(
  testDir: string,
  chainKey: string,
  defaults: Record<string, ChainDefaults>,
  sharedMnemonic: string | undefined,
): { readonly rpcUrl: string; readonly mnemonic: string; readonly fheTestAddress: string } {
  const entry = defaults[chainKey];
  if (entry === undefined) {
    throw new Error(`Missing "${chainKey}" entry in ${chainDefaultsPath}`);
  }
  const chainEnv = parseEnvFile(resolve(testDir, `.env.${chainKey}`));
  const rpcUrl = chainEnv.RPC_URL ?? process.env.RPC_URL ?? entry.rpcUrl;
  if (rpcUrl === undefined || rpcUrl === '') {
    throw new Error(`Missing rpcUrl for "${chainKey}" in ${chainDefaultsPath} or test/.env.${chainKey}.`);
  }
  const mnemonic = sharedMnemonic ?? entry.mnemonic;
  if (mnemonic === undefined || mnemonic === '') {
    throw new Error(`Missing mnemonic for "${chainKey}" in ${chainDefaultsPath} or test/.env.`);
  }
  if (entry.fheTestAddress === undefined || entry.fheTestAddress === '') {
    throw new Error(`Missing "${chainKey}.fheTestAddress" in ${chainDefaultsPath}`);
  }
  return { rpcUrl, mnemonic, fheTestAddress: entry.fheTestAddress };
}

async function ensureSignerFunds(req: IncomingMessage, res: ServerResponse): Promise<void> {
  if (req.method !== 'POST') {
    respondWithStatus(res, 405, 'Method not allowed.');
    return;
  }

  const request = JSON.parse((await readRequestBody(req)).toString('utf-8')) as {
    readonly chainTarget?: string;
    readonly address?: string;
  };
  const chainTarget = request.chainTarget;
  const address = request.address;

  if (!isBrowserUiChainTarget(chainTarget)) {
    respondWithStatus(res, 400, 'Unknown chain target.');
    return;
  }
  if (address === undefined || !/^0x[0-9a-fA-F]{40}$/.test(address)) {
    respondWithStatus(res, 400, 'Invalid signer address.');
    return;
  }
  if (chainTarget === 'testnet') {
    respondWithJson(res, { funded: false, reason: 'testnet' });
    return;
  }

  const rpcUrl = loadBrowserUiConfig().targets[chainTarget].rpcUrl;
  await execFileAsync('cast', ['rpc', 'anvil_setBalance', address, localFundingBalanceWei, '--rpc-url', rpcUrl], {
    timeout: 10_000,
  });
  respondWithJson(res, {
    funded: true,
    address,
    balanceWei: localFundingBalanceWei,
    chainTarget,
    rpcUrl,
  });
}

function isBrowserUiChainTarget(value: unknown): value is BrowserUiChainTarget {
  return value === 'testnet' || value === 'localstack' || value === 'localcleartext';
}

function parseEnvFile(filePath: string): Record<string, string> {
  if (!existsSync(filePath)) {
    return {};
  }

  const values: Record<string, string> = {};
  for (const line of readFileSync(filePath, 'utf-8').split('\n')) {
    const trimmed = line.trim();
    if (trimmed === '' || trimmed.startsWith('#')) {
      continue;
    }

    const eqIndex = trimmed.indexOf('=');
    if (eqIndex === -1) {
      continue;
    }

    const key = trimmed.slice(0, eqIndex).trim();
    let value = trimmed.slice(eqIndex + 1).trim();
    if ((value.startsWith('"') && value.endsWith('"')) || (value.startsWith("'") && value.endsWith("'"))) {
      value = value.slice(1, -1);
    }
    values[key] = value;
  }
  return values;
}

function respondWithJson(res: ServerResponse, value: unknown): void {
  const body = JSON.stringify(value);
  res.statusCode = 200;
  res.setHeader('content-type', 'application/json');
  res.setHeader('cache-control', 'no-store');
  res.setHeader('content-length', Buffer.byteLength(body).toString());
  res.end(body);
}

async function serveRawWasmAsset(req: IncomingMessage, res: ServerResponse, url: URL): Promise<void> {
  const filePath = resolveRawWasmAssetPath(url.pathname);
  if (filePath === undefined) {
    respondWithStatus(res, 404, 'Raw WASM asset not found.');
    return;
  }

  let bytes: Buffer;
  try {
    bytes = await readFile(filePath);
  } catch {
    respondWithStatus(res, 404, 'Raw WASM asset not found.');
    return;
  }

  res.statusCode = 200;
  res.setHeader('content-type', contentTypeForPath(filePath));
  res.setHeader('content-length', bytes.byteLength.toString());
  res.setHeader('cache-control', 'no-store');
  res.setHeader('cross-origin-opener-policy', 'same-origin');
  res.setHeader('cross-origin-embedder-policy', 'require-corp');
  res.setHeader('cross-origin-resource-policy', 'same-origin');

  if (req.method === 'HEAD') {
    res.end();
    return;
  }

  res.end(bytes);
}

function resolveRawWasmAssetPath(pathname: string): string | undefined {
  const relativeUrlPath = decodeURIComponent(pathname.slice(rawWasmAssetPrefix.length)).replace(/^\/+/, '');
  if (relativeUrlPath === '') {
    return undefined;
  }

  const filePath = resolve(projectRoot, relativeUrlPath);
  const relativeFilePath = relative(projectRoot, filePath);
  if (
    relativeFilePath === '' ||
    relativeFilePath.startsWith('..') ||
    isAbsolute(relativeFilePath) ||
    !relativeFilePath.startsWith(`src${sep}wasm${sep}`)
  ) {
    return undefined;
  }

  return filePath;
}

function contentTypeForPath(path: string): string {
  switch (extname(path)) {
    case '.js':
    case '.mjs':
      return 'application/javascript';
    case '.wasm':
      return 'application/wasm';
    default:
      return 'application/octet-stream';
  }
}

async function proxyRequest(parameters: {
  readonly req: IncomingMessage;
  readonly res: ServerResponse;
  readonly targetBaseUrl: string;
  readonly proxyPrefix: string;
  readonly rewriteRelayerKeyUrl: boolean;
}): Promise<void> {
  const { req, res, targetBaseUrl, proxyPrefix, rewriteRelayerKeyUrl } = parameters;
  const sourceUrl = new URL(req.url ?? '/', 'http://localhost');
  const targetPath = sourceUrl.pathname.slice(proxyPrefix.length) || '/';
  const targetUrl = new URL(`${targetPath}${sourceUrl.search}`, targetBaseUrl);
  const method = req.method ?? 'GET';
  const body = method === 'GET' || method === 'HEAD' ? undefined : await readRequestBody(req);

  const init: RequestInit = {
    method,
    headers: toFetchHeaders(req.headers),
  };
  if (body !== undefined) {
    init.body = new Blob([new Uint8Array(body)]);
  }

  const upstream = await fetch(targetUrl, init);

  if (rewriteRelayerKeyUrl && targetUrl.pathname === '/v2/keyurl') {
    await respondWithRewrittenRelayerKeyUrl(req, res, upstream);
    return;
  }

  await pipeFetchResponse(res, upstream);
}

async function respondWithRewrittenRelayerKeyUrl(
  req: IncomingMessage,
  res: ServerResponse,
  upstream: Response,
): Promise<void> {
  const contentType = upstream.headers.get('content-type') ?? '';
  if (!contentType.includes('application/json')) {
    await pipeFetchResponse(res, upstream);
    return;
  }

  const json = (await upstream.json()) as unknown;
  const publicOrigin = `http://${req.headers.host ?? 'localhost:3335'}`;
  const rewritten = rewriteMinioUrls(json, publicOrigin);
  const body = JSON.stringify(rewritten);

  copyResponseHeaders(res, upstream.headers, ['content-length', 'content-encoding', 'transfer-encoding']);
  res.statusCode = upstream.status;
  res.statusMessage = upstream.statusText;
  res.setHeader('content-type', 'application/json');
  res.setHeader('content-length', Buffer.byteLength(body).toString());
  res.end(body);
}

async function pipeFetchResponse(res: ServerResponse, upstream: Response): Promise<void> {
  const body = Buffer.from(await upstream.arrayBuffer());

  copyResponseHeaders(res, upstream.headers, ['content-length', 'content-encoding', 'transfer-encoding']);
  res.statusCode = upstream.status;
  res.statusMessage = upstream.statusText;
  res.setHeader('content-length', body.byteLength.toString());
  res.end(body);
}

function copyResponseHeaders(res: ServerResponse, headers: Headers, skip: readonly string[]): void {
  const skipSet = new Set(skip.map((header) => header.toLowerCase()));
  for (const [name, value] of headers.entries()) {
    if (!skipSet.has(name.toLowerCase())) {
      res.setHeader(name, value);
    }
  }
}

function toFetchHeaders(headers: IncomingMessage['headers']): Headers {
  const result = new Headers();
  const skip = new Set(['connection', 'content-length', 'host']);

  for (const [name, value] of Object.entries(headers)) {
    if (skip.has(name.toLowerCase()) || value === undefined) {
      continue;
    }
    if (Array.isArray(value)) {
      for (const item of value) {
        result.append(name, item);
      }
    } else {
      result.set(name, value);
    }
  }

  return result;
}

function readRequestBody(req: IncomingMessage): Promise<Buffer> {
  return new Promise((resolveBody, reject) => {
    const chunks: Buffer[] = [];
    req.on('data', (chunk: Buffer | string) => {
      chunks.push(typeof chunk === 'string' ? Buffer.from(chunk) : chunk);
    });
    req.on('end', () => resolveBody(Buffer.concat(chunks)));
    req.on('error', reject);
  });
}

function rewriteMinioUrls(value: unknown, publicOrigin: string): unknown {
  if (typeof value === 'string') {
    return rewriteMinioUrl(value, publicOrigin);
  }

  if (Array.isArray(value)) {
    return value.map((item) => rewriteMinioUrls(item, publicOrigin));
  }

  if (value !== null && typeof value === 'object') {
    return Object.fromEntries(Object.entries(value).map(([key, item]) => [key, rewriteMinioUrls(item, publicOrigin)]));
  }

  return value;
}

function rewriteMinioUrl(value: string, publicOrigin: string): string {
  let url: URL;
  try {
    url = new URL(value);
  } catch {
    return value;
  }

  if (
    (url.protocol === 'http:' || url.protocol === 'https:') &&
    (url.hostname === 'minio' || url.hostname === 'localhost') &&
    url.port === '9000'
  ) {
    return `${publicOrigin}${localstackMinioProxyPrefix}${url.pathname}${url.search}`;
  }

  return value;
}

function respondWithProxyError(res: ServerResponse, err: unknown): void {
  if (res.headersSent) {
    res.destroy(err instanceof Error ? err : undefined);
    return;
  }

  res.statusCode = 502;
  res.setHeader('content-type', 'application/json');
  res.end(
    JSON.stringify({
      error: err instanceof Error ? err.message : String(err),
    }),
  );
}

function respondWithStatus(res: ServerResponse, statusCode: number, message: string): void {
  res.statusCode = statusCode;
  res.setHeader('content-type', 'text/plain');
  res.end(message);
}
