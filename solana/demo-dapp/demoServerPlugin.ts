import fs from 'node:fs/promises';
import type { IncomingMessage } from 'node:http';
import path from 'node:path';

import { createKeyPairSignerFromBytes } from '@solana/kit';
import type { Plugin } from 'vite';

import {
  readDemoAuthorizationFromEnv,
} from '../../test-suite/fhevm/demo/authorization';
import type { VaultMetrics } from './src/batchTypes';
import { parseDemoConfig } from './src/demoConfig';
import { encodeVaultMetrics, parseOperatorRequest } from './src/demoApi';

const appDirectory = import.meta.dirname;
const repoRoot = path.resolve(appDirectory, '../..');
const runtimeConfigPath = path.resolve(
  process.env.DEMO_CONFIG_PATH ?? path.join(repoRoot, '.fhevm/runtime/solana-demo.json'),
);
const aliceKeypairPath = path.join(repoRoot, 'solana/scripts/demo/demo-keypairs/alice.json');
const keeperKeypairPath = path.join(repoRoot, 'solana/scripts/demo/demo-keypairs/keeper.json');
const relayerKeyUrl = 'http://127.0.0.1:3000/v2/keyurl';

type DemoEncryptionKey = {
  readonly fingerprint: string;
  readonly publicKeyId: string;
  readonly publicKeyBase64: string;
  readonly crsId: string;
  readonly crsBase64: string;
};

type DemoEncryptionKeyDescriptor = {
  readonly fingerprint: string;
  readonly publicKeyId: string;
  readonly publicKeyUrl: string;
  readonly crsId: string;
  readonly crsUrl: string;
};

let encryptionKeyFingerprint: string | undefined;
let encryptionKeyPromise: Promise<DemoEncryptionKey> | undefined;

const isLoopback = (remoteAddress?: string): boolean =>
  remoteAddress === '127.0.0.1' || remoteAddress === '::1' || remoteAddress === '::ffff:127.0.0.1';
export const hasDemoPageContext = (
  request: Pick<IncomingMessage, 'headers' | 'method' | 'socket'>,
): boolean => {
  if (
    !isLoopback(request.socket.remoteAddress) ||
    request.headers.host !== '127.0.0.1:5173' ||
    request.headers['sec-fetch-site'] !== 'same-origin' ||
    request.headers['sec-fetch-dest'] !== 'empty'
  ) {
    return false;
  }
  const origin = request.headers.origin;
  if (
    (request.method !== 'GET' && origin !== 'http://127.0.0.1:5173') ||
    (origin !== undefined && origin !== 'http://127.0.0.1:5173')
  ) {
    return false;
  }
  const referer = request.headers.referer;
  if (referer === undefined) return false;
  try {
    const url = new URL(referer);
    return url.origin === 'http://127.0.0.1:5173' && url.username === '' && url.password === '';
  } catch {
    return false;
  }
};

export const runSingleFlight = async <T>(
  operations: Map<string, Promise<T>>,
  key: string,
  start: () => Promise<T>,
): Promise<T> => {
  const existing = operations.get(key);
  if (existing !== undefined) return existing;
  const operation = start();
  operations.set(key, operation);
  try {
    return await operation;
  } finally {
    if (operations.get(key) === operation) operations.delete(key);
  }
};

const hasJsonContentType = (request: IncomingMessage): boolean =>
  request.headers['content-type']?.split(';', 1)[0]?.trim().toLowerCase() === 'application/json';

const requiredString = (value: unknown, name: string): string => {
  if (typeof value !== 'string' || value.length === 0) throw new Error(`${name} must be a non-empty string`);
  return value;
};

const readJsonBody = async (request: IncomingMessage): Promise<unknown> => {
  const chunks: Buffer[] = [];
  let size = 0;
  for await (const chunk of request) {
    const bytes = Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk);
    size += bytes.length;
    if (size > 8_192) throw new Error('request body is too large');
    chunks.push(bytes);
  }
  return JSON.parse(Buffer.concat(chunks).toString('utf8'));
};

const loadDemoOperatorSession = async () => {
  const [configJson, keeperKeypairJson] = await Promise.all([
    fs.readFile(runtimeConfigPath, 'utf8'),
    fs.readFile(keeperKeypairPath, 'utf8'),
  ]);
  const config = parseDemoConfig(JSON.parse(configJson) as unknown);
  const keeper = await createKeyPairSignerFromBytes(Uint8Array.from(JSON.parse(keeperKeypairJson) as number[]));
  if (keeper.address !== config.personas.keeper) {
    throw new Error(`keeper signer ${keeper.address} does not match seeded keeper ${config.personas.keeper}`);
  }
  return { config, keeper };
};

const readDemoEncryptionKeyDescriptor = async (): Promise<DemoEncryptionKeyDescriptor> => {
  const keyUrlResponse = await fetch(relayerKeyUrl, {
    headers: { accept: 'application/json', 'x-api-key': 'local' },
  });
  if (!keyUrlResponse.ok) throw new Error(`relayer key URL failed with HTTP ${keyUrlResponse.status}`);
  const body = (await keyUrlResponse.json()) as {
    readonly response?: {
      readonly fheKeyInfo?: readonly [
        { readonly fhePublicKey?: { readonly dataId?: unknown; readonly urls?: unknown } },
      ];
      readonly crs?: Record<string, { readonly dataId?: unknown; readonly urls?: unknown }>;
    };
  };
  const publicKey = body.response?.fheKeyInfo?.[0]?.fhePublicKey;
  const crs = body.response?.crs?.['2048'];
  const publicKeyUrl = Array.isArray(publicKey?.urls) ? publicKey.urls[0] : undefined;
  const crsUrl = Array.isArray(crs?.urls) ? crs.urls[0] : undefined;
  const hostUrl = (value: unknown, name: string): string => {
    const url = new URL(requiredString(value, name));
    if (url.protocol !== 'http:' || url.port !== '9000') {
      throw new Error(`${name} must use the local MinIO HTTP endpoint`);
    }
    if (url.hostname === 'minio') url.hostname = '127.0.0.1';
    if (url.hostname !== '127.0.0.1') {
      throw new Error(`${name} must use the local MinIO host`);
    }
    return url.toString();
  };
  const resolvedPublicKeyUrl = hostUrl(publicKeyUrl, 'public key URL');
  const resolvedCrsUrl = hostUrl(crsUrl, 'CRS URL');
  const [publicKeyHead, crsHead] = await Promise.all([
    fetch(resolvedPublicKeyUrl, { method: 'HEAD' }),
    fetch(resolvedCrsUrl, { method: 'HEAD' }),
  ]);
  if (!publicKeyHead.ok) throw new Error(`public key metadata failed with HTTP ${publicKeyHead.status}`);
  if (!crsHead.ok) throw new Error(`CRS metadata failed with HTTP ${crsHead.status}`);
  const publicKeyId = requiredString(publicKey?.dataId, 'public key dataId');
  const crsId = requiredString(crs?.dataId, 'CRS dataId');
  const publicKeyTag = publicKeyHead.headers.get('etag') ?? publicKeyHead.headers.get('last-modified') ?? 'unknown';
  const crsTag = crsHead.headers.get('etag') ?? crsHead.headers.get('last-modified') ?? 'unknown';
  return {
    fingerprint: `${publicKeyId}:${publicKeyTag}:${crsId}:${crsTag}`,
    publicKeyId,
    publicKeyUrl: resolvedPublicKeyUrl,
    crsId,
    crsUrl: resolvedCrsUrl,
  };
};

const readDemoEncryptionKey = async (descriptor: DemoEncryptionKeyDescriptor): Promise<DemoEncryptionKey> => {
  const [publicKeyResponse, crsResponse] = await Promise.all([
    fetch(descriptor.publicKeyUrl),
    fetch(descriptor.crsUrl),
  ]);
  if (!publicKeyResponse.ok) throw new Error(`public key fetch failed with HTTP ${publicKeyResponse.status}`);
  if (!crsResponse.ok) throw new Error(`CRS fetch failed with HTTP ${crsResponse.status}`);
  const [publicKeyBytes, crsBytes] = await Promise.all([publicKeyResponse.arrayBuffer(), crsResponse.arrayBuffer()]);
  return {
    fingerprint: descriptor.fingerprint,
    publicKeyId: descriptor.publicKeyId,
    publicKeyBase64: Buffer.from(publicKeyBytes).toString('base64'),
    crsId: descriptor.crsId,
    crsBase64: Buffer.from(crsBytes).toString('base64'),
  };
};

export const demoServerPlugin = (): Plugin => ({
  name: 'solana-demo-session',
  async configureServer(server) {
        const authorization = await readDemoAuthorizationFromEnv();
        const authorizationHeaders = {
          authorization: `Bearer ${authorization.token}`,
          'x-fhevm-demo-boot-id': authorization.bootId,
        };
        type HarvestResult = { readonly before: VaultMetrics; readonly after: VaultMetrics };
        let harvestInFlight: Promise<HarvestResult> | undefined;
        const operatorInFlight = new Map<string, Promise<void>>();

        server.middlewares.use('/api/demo-faucet', async (request, response) => {
          response.setHeader('content-type', 'application/json');
          response.setHeader('cache-control', 'no-store');
          response.setHeader('x-content-type-options', 'nosniff');
          if (request.method !== 'POST' || !hasDemoPageContext(request) || !hasJsonContentType(request)) {
            response.statusCode = request.method === 'POST' ? 403 : 405;
            response.end(
              JSON.stringify({ error: request.method === 'POST' ? 'local demo page only' : 'method not allowed' }),
            );
            return;
          }
          const path = request.url?.split('?', 1)[0];
          if (path !== '/airdrop-sol' && path !== '/mint-usdc') {
            response.statusCode = 404;
            response.end(JSON.stringify({ error: 'unknown faucet action' }));
            return;
          }
          try {
            const upstream = await fetch(`http://127.0.0.1:8090${path}`, {
              method: 'POST',
              headers: {
                'content-type': 'application/json',
                origin: 'http://127.0.0.1:5173',
                ...authorizationHeaders,
              },
              body: JSON.stringify(await readJsonBody(request)),
            });
            response.statusCode = upstream.status;
            response.end(await upstream.text());
          } catch (error) {
            response.statusCode = 503;
            response.end(JSON.stringify({ error: error instanceof Error ? error.message : String(error) }));
          }
        });

        server.middlewares.use('/api/demo-encryption-key-meta', async (request, response) => {
          response.setHeader('content-type', 'application/json');
          response.setHeader('cache-control', 'no-store');
          if (request.method !== 'GET' || !isLoopback(request.socket.remoteAddress)) {
            response.statusCode = request.method === 'GET' ? 403 : 405;
            response.end(JSON.stringify({ error: request.method === 'GET' ? 'loopback only' : 'method not allowed' }));
            return;
          }
          try {
            const descriptor = await readDemoEncryptionKeyDescriptor();
            response.statusCode = 200;
            response.end(JSON.stringify({ fingerprint: descriptor.fingerprint }));
          } catch (error) {
            response.statusCode = 503;
            response.end(JSON.stringify({ error: error instanceof Error ? error.message : String(error) }));
          }
        });
        server.middlewares.use('/api/demo-encryption-key', async (request, response) => {
          response.setHeader('content-type', 'application/json');
          response.setHeader('cache-control', 'no-store');
          if (request.method !== 'GET') {
            response.statusCode = 405;
            response.end(JSON.stringify({ error: 'method not allowed' }));
            return;
          }
          if (!isLoopback(request.socket.remoteAddress)) {
            response.statusCode = 403;
            response.end(JSON.stringify({ error: 'demo encryption key is loopback-only' }));
            return;
          }
          try {
            const descriptor = await readDemoEncryptionKeyDescriptor();
            if (encryptionKeyFingerprint !== descriptor.fingerprint || encryptionKeyPromise === undefined) {
              encryptionKeyFingerprint = descriptor.fingerprint;
              const pending = readDemoEncryptionKey(descriptor).catch((error) => {
                if (encryptionKeyPromise === pending) {
                  encryptionKeyFingerprint = undefined;
                  encryptionKeyPromise = undefined;
                }
                throw error;
              });
              encryptionKeyPromise = pending;
            }
            response.statusCode = 200;
            response.end(JSON.stringify(await encryptionKeyPromise));
          } catch (error) {
            response.statusCode = 503;
            response.end(JSON.stringify({ error: error instanceof Error ? error.message : String(error) }));
          }
        });
        server.middlewares.use('/api/demo-operator', async (request, response) => {
          response.setHeader('content-type', 'application/json');
          response.setHeader('cache-control', 'no-store');
          if (
            request.method !== 'POST' ||
            !isLoopback(request.socket.remoteAddress) ||
            !hasDemoPageContext(request) ||
            !hasJsonContentType(request)
          ) {
            response.statusCode = request.method === 'POST' ? 403 : 405;
            response.end(
              JSON.stringify({ error: request.method === 'POST' ? 'local demo origin only' : 'method not allowed' }),
            );
            return;
          }
          try {
            const { action, direction, position } = parseOperatorRequest(await readJsonBody(request));
            const operationKey = `${direction}:${position.batch}:${action}`;
            await runSingleFlight(operatorInFlight, operationKey, async () => {
              const session = await loadDemoOperatorSession();
              const operator = (await server.ssrLoadModule('/src/settlement.ts')) as typeof import('./src/settlement');
              if (action === 'dispatch') {
                await operator.dispatchVaultBatch(session, position, direction);
              } else {
                await operator.settleVaultBatch(session, position, direction);
              }
            });
            response.statusCode = 200;
            response.end(JSON.stringify({ ok: true }));
          } catch (error) {
            response.statusCode = 503;
            response.end(JSON.stringify({ error: error instanceof Error ? error.message : String(error) }));
          }
        });
        server.middlewares.use('/api/demo-vault-metrics', async (request, response) => {
          response.setHeader('content-type', 'application/json');
          response.setHeader('cache-control', 'no-store');
          if (request.method !== 'GET' || !isLoopback(request.socket.remoteAddress)) {
            response.statusCode = request.method === 'GET' ? 403 : 405;
            response.end(JSON.stringify({ error: request.method === 'GET' ? 'loopback only' : 'method not allowed' }));
            return;
          }
          try {
            const session = await loadDemoOperatorSession();
            const module = (await server.ssrLoadModule(
              '/src/harvestOperator.ts',
            )) as typeof import('./src/harvestOperator');
            const metrics = await module.readDemoVaultMetrics(session.config);
            response.statusCode = 200;
            response.end(
              JSON.stringify(encodeVaultMetrics(metrics)),
            );
          } catch (error) {
            response.statusCode = 503;
            response.end(JSON.stringify({ error: error instanceof Error ? error.message : String(error) }));
          }
        });
        server.middlewares.use('/api/demo-harvest', async (request, response) => {
          response.setHeader('content-type', 'application/json');
          response.setHeader('cache-control', 'no-store');
          if (
            request.method !== 'POST' ||
            !isLoopback(request.socket.remoteAddress) ||
            !hasDemoPageContext(request) ||
            !hasJsonContentType(request)
          ) {
            response.statusCode = request.method === 'POST' ? 403 : 405;
            response.end(
              JSON.stringify({ error: request.method === 'POST' ? 'local demo origin only' : 'method not allowed' }),
            );
            return;
          }
          try {
            harvestInFlight ??= (async () => {
              const session = await loadDemoOperatorSession();
              const module = (await server.ssrLoadModule(
                '/src/harvestOperator.ts',
              )) as typeof import('./src/harvestOperator');
              return module.harvestDemoVault(session.config, session.keeper, authorizationHeaders);
            })().finally(() => {
              harvestInFlight = undefined;
            });
            const result = await harvestInFlight;
            response.statusCode = 200;
            response.end(
              JSON.stringify({
                before: encodeVaultMetrics(result.before),
                after: encodeVaultMetrics(result.after),
              }),
            );
          } catch (error) {
            response.statusCode = 503;
            response.end(JSON.stringify({ error: error instanceof Error ? error.message : String(error) }));
          }
        });
        server.middlewares.use('/api/demo-session', async (request, response) => {
          response.setHeader('content-type', 'application/json');
          response.setHeader('cache-control', 'no-store');
          if (request.method !== 'GET') {
            response.statusCode = 405;
            response.end(JSON.stringify({ error: 'method not allowed' }));
            return;
          }
          if (!hasDemoPageContext(request)) {
            response.statusCode = 403;
            response.end(JSON.stringify({ error: 'demo session is available only to the local demo page' }));
            return;
          }
          try {
            const config = JSON.parse(await fs.readFile(runtimeConfigPath, 'utf8')) as {
              source?: string;
              rpcUrl?: string;
            };
            if (config.source !== 'demo-config' || config.rpcUrl !== 'http://127.0.0.1:8899') {
              throw new Error('refusing to expose a burner wallet outside the seeded local validator');
            }
            const aliceKeypair = await fs
              .readFile(aliceKeypairPath, 'utf8')
              .then((value) => JSON.parse(value) as number[]);
            response.statusCode = 200;
            response.end(JSON.stringify({ config, aliceKeypair }));
          } catch (error) {
            response.statusCode = 503;
            response.end(JSON.stringify({ error: error instanceof Error ? error.message : String(error) }));
          }
        });
        server.middlewares.use('/api/demo-config', async (request, response) => {
          response.setHeader('content-type', 'application/json');
          response.setHeader('cache-control', 'no-store');
          if (request.method !== 'GET') {
            response.statusCode = 405;
            response.end(JSON.stringify({ error: 'method not allowed' }));
            return;
          }
          if (!hasDemoPageContext(request)) {
            response.statusCode = 403;
            response.end(JSON.stringify({ error: 'demo config is available only to the local demo page' }));
            return;
          }
          try {
            const config = JSON.parse(await fs.readFile(runtimeConfigPath, 'utf8')) as {
              source?: string;
              rpcUrl?: string;
            };
            if (config.source !== 'demo-config' || config.rpcUrl !== 'http://127.0.0.1:8899') {
              throw new Error('refusing to expose configuration outside the seeded local validator');
            }
            response.statusCode = 200;
            response.end(JSON.stringify({ config }));
          } catch (error) {
            response.statusCode = 503;
            response.end(JSON.stringify({ error: error instanceof Error ? error.message : String(error) }));
          }
        });
  },
});
