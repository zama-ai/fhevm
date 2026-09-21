// checkBrowserReality — acceptance #5, run as a step of the solana-e2e workflow's demo phase.
//
// The page reaches the relayer and operator through the dApp's same-origin proxy. Verify
// that relayer requests work without client credentials and that the config advertises that
// proxy. The operator must still enforce its boot capability and exact-origin CORS policy.

import { LOCAL_SOLANA_ENDPOINTS } from "../src/solana/endpoints";
import { readDemoAuthorization } from "./lifecycle";

const DAPP_URL = process.env.DEMO_DAPP_URL ?? LOCAL_SOLANA_ENDPOINTS.demoDapp;
const ORIGIN = DAPP_URL;
const OPERATOR_URL = process.env.DEMO_OPERATOR_URL ?? LOCAL_SOLANA_ENDPOINTS.demoOperator;

type Check = { readonly name: string; readonly run: () => Promise<void> };

/**
 * Asserts an OPTIONS preflight from `ORIGIN` comes back permissive for the request the dApp will
 * actually make: not just the origin, but also the method (POST) and header (content-type) it asks
 * for. A browser BLOCKS the real request unless the preflight allows all three, so checking origin
 * alone would pass here yet still fail in a real browser.
 */
const preflightAllowsOrigin = async (
  name: string,
  url: string,
  options: {
    readonly requestHeaders?: readonly string[];
    readonly requireExactOrigin?: boolean;
  } = {},
): Promise<void> => {
  const requestMethod = "POST";
  const requestHeaders = options.requestHeaders ?? ["content-type"];
  const response = await fetch(url, {
    method: "OPTIONS",
    headers: {
      origin: ORIGIN,
      "access-control-request-method": requestMethod,
      "access-control-request-headers": requestHeaders.join(", "),
    },
  });

  const allowOrigin = response.headers.get("access-control-allow-origin");
  if (
    options.requireExactOrigin
      ? allowOrigin !== ORIGIN
      : allowOrigin !== "*" && allowOrigin !== ORIGIN
  ) {
    throw new Error(
      `${name}: CORS preflight at ${url} did not permit origin ${ORIGIN} (access-control-allow-origin=${allowOrigin ?? "<none>"}, status ${response.status})`,
    );
  }

  // Accept either a wildcard or an explicit allow-list that contains the requested token (both are
  // valid CORS ways to permit it). Comparison is case-insensitive: header tokens are not case-sensitive.
  const allowsToken = (headerValue: string | null, token: string): boolean => {
    if (headerValue === null) return false;
    if (headerValue.trim() === "*") return true;
    return headerValue
      .split(",")
      .map((entry) => entry.trim().toLowerCase())
      .includes(token.toLowerCase());
  };

  const allowMethods = response.headers.get("access-control-allow-methods");
  if (!allowsToken(allowMethods, requestMethod)) {
    throw new Error(
      `${name}: CORS preflight at ${url} did not permit method ${requestMethod} (access-control-allow-methods=${allowMethods ?? "<none>"}, status ${response.status})`,
    );
  }

  const allowHeaders = response.headers.get("access-control-allow-headers");
  for (const requestHeader of requestHeaders) {
    if (!allowsToken(allowHeaders, requestHeader)) {
      throw new Error(
        `${name}: CORS preflight at ${url} did not permit header ${requestHeader} (access-control-allow-headers=${allowHeaders ?? "<none>"}, status ${response.status})`,
      );
    }
  }
};

const main = async (): Promise<void> => {
  const authorization = await readDemoAuthorization();
  const authorizationHeaders = {
    authorization: `Bearer ${authorization.token}`,
    "x-fhevm-demo-boot-id": authorization.bootId,
  };

  const checks: Check[] = [
    {
      name: "dApp same-origin relayer proxy",
      run: async () => {
        // No API key: the Vite proxy supplies it server-side, just as for the browser.
        const response = await fetch(`${DAPP_URL}/api/relayer/v2/keyurl`, {
          headers: { origin: ORIGIN, accept: "application/json" },
        });
        if (!response.ok) throw new Error(`proxied relayer key URL returned ${response.status}`);
        const body = await response.json() as {
          response?: { fheKeyInfo?: unknown[]; crs?: Record<string, unknown> };
        };
        if (!body.response?.fheKeyInfo?.length || !body.response.crs?.["2048"]) {
          throw new Error("proxied relayer did not return FHE key and CRS metadata");
        }
      },
    },
    {
      name: "operator preflight",
      run: () =>
        preflightAllowsOrigin("operator", `${OPERATOR_URL}/demo-faucet/mint-usdc`, {
          requestHeaders: ["authorization", "content-type", "x-fhevm-demo-boot-id"],
          requireExactOrigin: true,
        }),
    },
    {
      // Public health remains browser-readable, but only from the exact dApp origin.
      name: "operator reachable cross-origin",
      run: async () => {
        const response = await fetch(`${OPERATOR_URL}/health`, { headers: { origin: ORIGIN } });
        if (!response.ok) throw new Error(`operator /health returned ${response.status}`);
        if (response.headers.get("access-control-allow-origin") !== ORIGIN) {
          throw new Error("operator /health did not return the exact dApp access-control-allow-origin");
        }
      },
    },
    {
      name: "operator refuses a request without the capability",
      run: async () => {
        const response = await fetch(`${OPERATOR_URL}/demo-config`);
        if (response.status !== 401) {
          throw new Error(`unauthenticated operator request returned ${response.status}, expected 401`);
        }
      },
    },
    {
      name: "operator current boot authorization",
      run: async () => {
        const response = await fetch(`${OPERATOR_URL}/demo-faucet/mint-usdc`, {
          method: "POST",
          headers: { ...authorizationHeaders, "content-type": "application/json", origin: ORIGIN },
          body: "{}",
        });
        if (response.status !== 400) {
          throw new Error(`authorized no-op faucet request returned ${response.status}, expected 400`);
        }
      },
    },
    {
      // The page calls its own origin with no credentials; the dev server adds the capability.
      name: "dApp proxied current boot config",
      run: async () => {
        const response = await fetch(`${DAPP_URL}/api/demo-config`);
        if (!response.ok) throw new Error(`proxied demo config returned ${response.status}`);
        const text = await response.text();
        const body = JSON.parse(text) as { readonly config?: { readonly demoBootId?: unknown; readonly relayerUrl?: unknown } };
        if (body.config?.demoBootId !== authorization.bootId) {
          throw new Error("proxied demo config did not return the current boot");
        }
        if (body.config?.relayerUrl !== `${DAPP_URL}/api/relayer`) {
          throw new Error("proxied demo config did not advertise the same-origin relayer proxy");
        }
        if (/keypair/i.test(text) || text.includes(authorization.token)) {
          throw new Error("proxied demo config leaked key material or the capability");
        }
      },
    },
  ];

  const failures: string[] = [];
  for (const check of checks) {
    try {
      await check.run();
      console.log(`ok   ${check.name}`);
    } catch (error) {
      failures.push(`${check.name}: ${error instanceof Error ? error.message : String(error)}`);
      console.error(`FAIL ${check.name}`);
    }
  }

  if (failures.length > 0) {
    console.error(`\nbrowser-reality checks failed:\n  - ${failures.join("\n  - ")}`);
    process.exit(1);
  }
  console.log("\nall browser-reality checks passed");
};

await main();
