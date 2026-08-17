import type { IncomingMessage, ServerResponse } from 'node:http';
import type { ViteDevServer } from 'vite';
import { readFile } from 'node:fs/promises';
import { defineConfig } from 'vite';
import { dirname, extname, isAbsolute, relative, resolve, sep } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const projectRoot = resolve(__dirname, '../..');
const rawWasmAssetPrefix = '/__raw_wasm';
const localstackRelayerProxyPrefix = '/__localstack_relayer';
const localstackMinioProxyPrefix = '/__localstack_minio';
const localstackRelayerTarget = 'http://localhost:3000';
const localstackMinioTarget = 'http://localhost:9000';

export default defineConfig({
  root: projectRoot,
  plugins: [localstackProxyPlugin()],
  resolve: {
    alias: {
      // The chain configs in test/chains/*.ts import from this
      // alias; the alias is normally provided by test/tsconfig.json paths,
      // which Vite does not read. Without this entry, dynamic-importing a
      // chain file from roundtrip.ts breaks at bundle time.
      '@fhevm/sdk/chains': resolve(projectRoot, 'src/core/chains/index.ts'),
    },
  },
  server: {
    port: 3334,
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
});

function localstackProxyPlugin() {
  return {
    name: 'multi-wasm-localstack-proxy',
    configureServer(server: ViteDevServer) {
      server.middlewares.use(async (req: IncomingMessage, res: ServerResponse, next: () => void) => {
        try {
          const url = new URL(req.url ?? '/', 'http://localhost');

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
  const publicOrigin = `http://${req.headers.host ?? 'localhost:3334'}`;
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
    if (skipSet.has(name.toLowerCase())) {
      continue;
    }
    res.setHeader(name, value);
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
