const BOOT_ID_PATTERN = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
const TOKEN_PATTERN = /^[A-Za-z0-9_-]{43}$/;
const DEMO_ORIGIN = 'http://127.0.0.1:5173';
const FAUCET_ORIGIN = 'http://127.0.0.1:8090';

type BrowserDemoAuthorization = {
  readonly bootId: string;
  readonly token: string;
};

let authorization: BrowserDemoAuthorization | undefined;

export const consumeDemoLaunchAuthorization = (
  location: Pick<Location, 'hash' | 'pathname' | 'search'> = window.location,
  history: Pick<History, 'replaceState'> = window.history,
): void => {
  const params = new URLSearchParams(location.hash.startsWith('#') ? location.hash.slice(1) : location.hash);
  const bootId = params.get('boot');
  const token = params.get('token');
  authorization =
    bootId !== null && token !== null && BOOT_ID_PATTERN.test(bootId) && TOKEN_PATTERN.test(token)
      ? { bootId, token }
      : undefined;
  if (location.hash.length > 0) history.replaceState(null, '', `${location.pathname}${location.search}`);
};

const authorizedFetch = (
  input: string,
  expectedOrigin: string,
  init: RequestInit = {},
  fetcher: typeof fetch = fetch,
): Promise<Response> => {
  if (authorization === undefined) {
    throw new Error('Open the current demo launch URL to authorize local demo actions');
  }
  const url = new URL(input, window.location.origin);
  if (url.origin !== expectedOrigin) throw new Error(`refusing to send demo authorization to ${url.origin}`);
  const headers = new Headers(init.headers);
  headers.set('authorization', `Bearer ${authorization.token}`);
  headers.set('x-fhevm-demo-boot-id', authorization.bootId);
  return fetcher(url.toString(), { ...init, headers });
};

export const demoApiFetch = (
  path: `/api/${string}`,
  init?: RequestInit,
  fetcher?: typeof fetch,
): Promise<Response> => authorizedFetch(`${DEMO_ORIGIN}${path}`, DEMO_ORIGIN, init, fetcher);

export const demoFaucetFetch = (
  path: '/airdrop-sol' | '/mint-usdc',
  init?: RequestInit,
  fetcher?: typeof fetch,
): Promise<Response> => authorizedFetch(`${FAUCET_ORIGIN}${path}`, FAUCET_ORIGIN, init, fetcher);
