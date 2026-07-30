// checkBrowserReality — acceptance #5, run as a step of the solana-e2e workflow's demo phase.
//
// A Vite-origin dApp (#1761) reaches the relayer, the proof service and the faucet with browser
// fetch(), so each must answer the exact dApp origin's CORS preflight (OPTIONS), and the protected
// faucet must accept only the current lifecycle boot capability, while the dApp session must be
// same-origin and expose that same boot. This exercises exactly that, from a browser Origin, and
// exits non-zero (naming the failing endpoint) if any check fails. TS rather than a bash curl script
// because the header assertions are logic.
//
// Reads the seeded demo-config for the relayer + proof-service URLs; the faucet and dApp use their
// lifecycle-owned loopback ports. The browser origin is deliberately the exact Vite origin.

import { readDemoConfig } from "./config";
import { readCurrentDemoAuthorization } from "./lifecycle";

const ORIGIN = "http://127.0.0.1:5173";
const FAUCET_URL = process.env.DEMO_FAUCET_URL ?? "http://127.0.0.1:8090";
const DAPP_URL = "http://127.0.0.1:5173";

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
  const [config, authorization] = await Promise.all([
    readDemoConfig(),
    readCurrentDemoAuthorization(),
  ]);
  const authorizationHeaders = {
    authorization: `Bearer ${authorization.token}`,
    "x-fhevm-demo-boot-id": authorization.bootId,
  };

  const checks: Check[] = [
    // The relayer only carries the CORS layer when RELAYER_PERMISSIVE_CORS is set on its container;
    // this preflight is what proves the demo bring-up wired that env through to the relayer service.
    { name: "relayer", run: () => preflightAllowsOrigin("relayer", `${config.relayerUrl}/v2/input-proof`) },
    { name: "proof-service", run: () => preflightAllowsOrigin("proof-service", `${config.proofServiceUrl}/health/readiness`) },
    {
      name: "faucet preflight",
      run: () =>
        preflightAllowsOrigin("faucet", `${FAUCET_URL}/mint-usdc`, {
          requestHeaders: [
            "authorization",
            "content-type",
            "x-fhevm-demo-boot-id",
          ],
          requireExactOrigin: true,
        }),
    },
    {
      // Public health remains browser-readable, but only from the exact dApp origin.
      name: "faucet reachable cross-origin",
      run: async () => {
        const response = await fetch(`${FAUCET_URL}/health`, { headers: { origin: ORIGIN } });
        if (!response.ok) throw new Error(`faucet /health returned ${response.status}`);
        if (response.headers.get("access-control-allow-origin") !== ORIGIN) {
          throw new Error("faucet /health did not return the exact dApp access-control-allow-origin");
        }
      },
    },
    {
      name: "faucet current boot authorization",
      run: async () => {
        const response = await fetch(`${FAUCET_URL}/mint-usdc`, {
          method: "POST",
          headers: {
            ...authorizationHeaders,
            "content-type": "application/json",
            origin: ORIGIN,
          },
          body: "{}",
        });
        if (response.status !== 400) {
          throw new Error(`authorized no-op faucet request returned ${response.status}, expected 400`);
        }
      },
    },
    {
      name: "dApp current boot session",
      run: async () => {
        const response = await fetch(`${DAPP_URL}/api/demo-session`, {
          headers: {
            referer: `${ORIGIN}/`,
            "sec-fetch-dest": "empty",
            "sec-fetch-site": "same-origin",
          },
        });
        if (!response.ok) {
          throw new Error(`same-origin demo session returned ${response.status}`);
        }
        const body = (await response.json()) as {
          readonly aliceKeypair?: unknown;
          readonly config?: { readonly demoBootId?: unknown };
        };
        if (!Array.isArray(body.aliceKeypair) || body.aliceKeypair.length !== 64) {
          throw new Error("same-origin demo session did not return the burner wallet");
        }
        if (body.config?.demoBootId !== authorization.bootId) {
          throw new Error("same-origin demo session did not return the current boot");
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
