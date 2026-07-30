const DEMO_ORIGIN = 'http://127.0.0.1:5173';

const localDemoFetch = (
  path: string,
  init: RequestInit = {},
  fetcher: typeof fetch = fetch,
): Promise<Response> => {
  const url = new URL(path, DEMO_ORIGIN);
  if (url.origin !== DEMO_ORIGIN || window.location.origin !== DEMO_ORIGIN) {
    throw new Error(`refusing local demo request from ${window.location.origin}`);
  }
  return fetcher(url.toString(), { ...init, credentials: 'omit', redirect: 'error' });
};

export const demoApiFetch = (
  path: `/api/${string}`,
  init?: RequestInit,
  fetcher?: typeof fetch,
): Promise<Response> => localDemoFetch(path, init, fetcher);

export const demoFaucetFetch = (
  path: '/airdrop-sol' | '/mint-usdc',
  init?: RequestInit,
  fetcher?: typeof fetch,
): Promise<Response> => localDemoFetch(`/api/demo-faucet${path}`, init, fetcher);
