// checkBrowserReality — acceptance #5, run as a step of the demo-acceptance workflow.
//
// A Vite-origin dApp (#1761) reaches the relayer, the proof service and the faucet with browser
// fetch(), so each must answer a CORS preflight (OPTIONS) with a permissive
// `access-control-allow-origin`, and the faucet's own endpoints must be callable cross-origin. This
// exercises exactly that, from a browser Origin, and exits non-zero (naming the failing endpoint) if
// any check fails. TS rather than a bash curl script because the header assertions are logic.
//
// Reads the seeded demo-config for the relayer + proof-service URLs; the faucet URL is its own
// well-known local port. The Origin is `DEMO_ORIGIN` (the dApp dev server), defaulting to Vite's.

import { readDemoConfig } from "./config";

const ORIGIN = process.env.DEMO_ORIGIN ?? "http://localhost:5173";
const FAUCET_URL = process.env.DEMO_FAUCET_URL ?? "http://127.0.0.1:8090";

type Check = { readonly name: string; readonly run: () => Promise<void> };

/** Asserts an OPTIONS preflight from `ORIGIN` comes back with a permissive allow-origin header. */
const preflightAllowsOrigin = async (name: string, url: string): Promise<void> => {
  const response = await fetch(url, {
    method: "OPTIONS",
    headers: {
      origin: ORIGIN,
      "access-control-request-method": "POST",
      "access-control-request-headers": "content-type",
    },
  });
  const allowOrigin = response.headers.get("access-control-allow-origin");
  if (allowOrigin !== "*" && allowOrigin !== ORIGIN) {
    throw new Error(
      `${name}: CORS preflight at ${url} did not permit origin ${ORIGIN} (access-control-allow-origin=${allowOrigin ?? "<none>"}, status ${response.status})`,
    );
  }
};

const main = async (): Promise<void> => {
  const config = await readDemoConfig();

  const checks: Check[] = [
    // The relayer only carries the CORS layer when RELAYER_PERMISSIVE_CORS is set on its container;
    // this preflight is what proves the demo bring-up wired that env through to the relayer service.
    { name: "relayer", run: () => preflightAllowsOrigin("relayer", `${config.relayerUrl}/v2/input-proof`) },
    { name: "proof-service", run: () => preflightAllowsOrigin("proof-service", `${config.proofServiceUrl}/health/readiness`) },
    { name: "faucet preflight", run: () => preflightAllowsOrigin("faucet", `${FAUCET_URL}/mint-usdc`) },
    {
      // The faucet's health must be reachable cross-origin (the dApp polls it before showing the button).
      name: "faucet reachable cross-origin",
      run: async () => {
        const response = await fetch(`${FAUCET_URL}/health`, { headers: { origin: ORIGIN } });
        if (!response.ok) throw new Error(`faucet /health returned ${response.status}`);
        if (response.headers.get("access-control-allow-origin") !== "*") {
          throw new Error("faucet /health did not return a permissive access-control-allow-origin");
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
