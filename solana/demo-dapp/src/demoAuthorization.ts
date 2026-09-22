// The page's own origin serves `/api/*`: the dev server proxies it to the demo operator and adds
// the boot capability there, so no credential lives in the browser and none is sent from it.
export const demoApiFetch = (
  path: `/api/${string}`,
  init: RequestInit = {},
  fetcher: typeof fetch = fetch,
): Promise<Response> => fetcher(path, { ...init, credentials: 'omit', redirect: 'error' });

export const demoFaucetFetch = (
  path: '/airdrop-sol' | '/mint-usdc',
  init?: RequestInit,
  fetcher?: typeof fetch,
): Promise<Response> => demoApiFetch(`/api/demo-faucet${path}`, init, fetcher);
