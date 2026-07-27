const BOOT_ID_PATTERN = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
const TOKEN_PATTERN = /^[A-Za-z0-9_-]{43}$/;
const DEMO_ORIGIN = 'http://127.0.0.1:5173';
const FAUCET_ORIGIN = 'http://127.0.0.1:8090';

type BrowserDemoAuthorization = {
  readonly bootId: string;
  readonly token: string;
};

let authorization: BrowserDemoAuthorization | undefined;

const isLaunchAuthorizationHash = (hash: string): boolean => {
  const params = new URLSearchParams(hash.startsWith('#') ? hash.slice(1) : hash);
  return params.has('boot') || params.has('token');
};

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

export const initializeDemoLaunchAuthorization = (
  browser: {
    readonly location: Pick<Location, 'hash' | 'pathname' | 'search'>;
    readonly history: Pick<History, 'replaceState'>;
    addEventListener(type: 'hashchange', listener: () => void): void;
    removeEventListener(type: 'hashchange', listener: () => void): void;
  } = window,
): (() => void) => {
  const existing = launchAuthorizationListeners.get(browser);
  if (existing !== undefined) return existing;

  const consume = () => {
    if (!isLaunchAuthorizationHash(browser.location.hash)) return;
    consumeDemoLaunchAuthorization(browser.location, browser.history);
  };
  browser.addEventListener('hashchange', consume);
  consume();

  const dispose = () => {
    browser.removeEventListener('hashchange', consume);
    if (launchAuthorizationListeners.get(browser) === dispose) {
      launchAuthorizationListeners.delete(browser);
    }
  };
  launchAuthorizationListeners.set(browser, dispose);
  return dispose;
};

const launchAuthorizationListeners = new WeakMap<object, () => void>();

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
