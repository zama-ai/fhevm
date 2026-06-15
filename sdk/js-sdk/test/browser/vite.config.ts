import type { IncomingMessage, ServerResponse } from 'node:http';
import type { ViteDevServer } from 'vite';
import { readFile } from 'node:fs/promises';
import { dirname, extname, isAbsolute, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';

const __dirname = dirname(fileURLToPath(import.meta.url));
const projectRoot = resolve(__dirname, '../..');
const rawWasmAssetPrefix = '/__raw_wasm';

export default defineConfig({
  root: projectRoot,
  resolve: {
    alias: {
      '@fhevm/sdk': resolve(projectRoot, 'src'),
    },
  },
  plugins: [rawWasmAssetPlugin()],
  server: {
    port: 3333,
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
});

function rawWasmAssetPlugin() {
  return {
    name: 'browser-test-raw-wasm-assets',
    configureServer(server: ViteDevServer) {
      server.middlewares.use(async (req: IncomingMessage, res: ServerResponse, next: () => void) => {
        try {
          const url = new URL(req.url ?? '/', 'http://localhost');
          if (!url.pathname.startsWith(rawWasmAssetPrefix)) {
            next();
            return;
          }

          await serveRawWasmAsset(req, res, url);
        } catch (err) {
          respondWithStatus(res, 500, err instanceof Error ? err.message : String(err));
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

function respondWithStatus(res: ServerResponse, statusCode: number, message: string): void {
  res.statusCode = statusCode;
  res.setHeader('content-type', 'text/plain');
  res.end(message);
}
