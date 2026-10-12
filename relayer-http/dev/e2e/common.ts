// Shared by the relayer-http flow scripts. They run inside the e2e debug container
// (`npx hardhat run scripts/relayer-http/<flow>.ts --network staging`, see dev/scripts/e2e.sh) and talk to the
// relayer on the host. Imports are relative to /app/test-suite/e2e/scripts/relayer-http/.
import { ethers } from 'hardhat';

import { getSigners, initSigners } from '../../test/signers';

export const RELAYER = (process.env.RELAYER_HTTP_URL ?? 'http://host.docker.internal:8080').replace(/\/$/, '');
const TIMEOUT_MS = 120_000;

export interface Reply<T> {
  status: number;
  requestId: string;
  elapsedMs: number;
  body: T;
}

/** POST a JSON body to a relayer route; one line per call with the status, the timing and the request id. */
export async function post<T>(route: string, body: unknown): Promise<Reply<T>> {
  const started = Date.now();
  const resp = await fetch(`${RELAYER}${route}`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(body),
    signal: AbortSignal.timeout(TIMEOUT_MS),
  });
  const elapsedMs = Date.now() - started;
  const requestId = resp.headers.get('x-request-id') ?? '';
  const json = (await resp.json()) as T;
  console.log(`POST ${route} -> ${resp.status} in ${elapsedMs}ms  request_id=${requestId}`);
  return { status: resp.status, requestId, elapsedMs, body: json };
}

/** Connector outcomes the stack can turn into a success a moment later (fresh ciphertexts, cold KMS). */
const TRANSIENT = new Set(['ciphertext_not_found', 'copro_consensus_failed', 'upstream_transient', 'timeout']);
const ATTEMPTS = Number(process.env.ATTEMPTS ?? '6');
const RETRY_DELAY_MS = Number(process.env.RETRY_DELAY_MS ?? '5000');

/** `post`, re-submitted while the relayer answers a transient connector code (at most ATTEMPTS times). */
export async function postUntilSettled<T>(route: string, body: unknown): Promise<Reply<T>> {
  let reply = await post<T>(route, body);
  for (let attempt = 2; attempt <= ATTEMPTS && reply.status !== 200; attempt += 1) {
    const code = (reply.body as { code?: string })?.code ?? '';
    if (!TRANSIENT.has(code)) break;
    console.log(`  ${code}: retrying in ${RETRY_DELAY_MS}ms (attempt ${attempt}/${ATTEMPTS})`);
    await new Promise((r) => setTimeout(r, RETRY_DELAY_MS));
    reply = await post<T>(route, body);
  }
  return reply;
}

/** The relayer must be ready before a flow starts. */
export async function requireRelayer(): Promise<void> {
  try {
    const health = await fetch(`${RELAYER}/healthz`, { signal: AbortSignal.timeout(5_000) });
    console.log(`GET ${RELAYER}/healthz -> ${health.status} ${await health.text()}`);
    if (health.status === 200) return;
  } catch (e) {
    console.error(`relayer unreachable at ${RELAYER}: ${(e as Error).message}`);
  }
  console.error('start it with `make -C relayer-http/dev run`');
  process.exit(1);
}

/** Alice, funded by the harness. */
export async function alice() {
  await initSigners(2);
  const signers = await getSigners();
  return { signers, alice: signers.alice };
}

/** Deploys one of the compiled e2e fixture contracts with `signer`. */
export async function deploy(name: string, signer: Awaited<ReturnType<typeof alice>>['alice']) {
  const contract = await (await ethers.getContractFactory(name)).connect(signer).deploy();
  await contract.waitForDeployment();
  console.log(`${name} deployed at ${await contract.getAddress()}`);
  return contract;
}

export const show = (value: unknown): string =>
  JSON.stringify(value, (_, v) => (typeof v === 'bigint' ? v.toString() : v));

export function fail(step: string, reply: Reply<unknown>): never {
  console.error(`${step} failed: ${reply.status} ${show(reply.body)}`);
  process.exit(1);
}

/** The SDK client keeps provider handles open: flows exit explicitly. */
export function run(flow: () => Promise<void>): void {
  flow()
    .then(() => process.exit(0))
    .catch((e) => {
      console.error(e);
      process.exit(1);
    });
}
