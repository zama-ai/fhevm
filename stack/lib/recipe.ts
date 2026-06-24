// EXEMPLAR — declarative boot recipe, the spec→code bridge for `up`.
//
// This file encodes the PROVEN boot recipe (stack/EXEMPLAR.md §0) as an ordered list of
// phases over the engine-agnostic Stack primitives (./stack.ts). It is the single source of
// truth the `up` driver walks; the CLI and runbooks are front-ends. It mirrors the fhevm-cli
// `up` step list (test-suite/fhevm/src/types.ts) — base → kms-signer → gateway-deploy →
// host-deploy → discover → regenerate → coprocessor → kms-connector → bootstrap → relayer.
//
// THE keystone (what makes this thin instead of 10k LoC): discover→regenerate. NOTHING below
// hardcodes a contract address. Each deploy emits its actual addresses; a discover step reads
// them (here: from the deploy's own logs via ctx.logs); regenerate threads them into every
// consumer's ConfigMap via ./render.threadDiscovery. Contract addresses are deploy-ORDER
// dependent (deploying the mocked-OFT first shifts every gateway proxy) — assuming them is the
// exact mistake that turns this into an unbootable frankenstein.

import { helmUpgrade } from "./helm";
import { kubectlApply } from "./kubectl";
import { threadDiscovery } from "./render";
import type { Stack } from "./stack";

// ---------------------------------------------------------------------------
// Recipe configuration (STATIC only — every contract address is discovered)
// ---------------------------------------------------------------------------

/**
 * Static wiring for a boot. Holds only values that are genuinely fixed: namespaces, dirs,
 * image versions, the two anvil chains (chainId + mnemonic, which fund their own deployers),
 * and the public vault host. Contract addresses, the KMS signer and the realized key id are
 * NOT here — they are discovered at runtime (see Discovery) and threaded by ./render.
 */
export type RecipeConfig = {
  namespace: string;
  /** Repo-root charts directory (charts/) — the anvil-node chart is current; others stale. */
  chartsDir: string;
  /** Raw v0.13 manifests + chart values (the ~0.55k YAML data layer). */
  dataDir: string;
  versions: {
    gatewayHost: string; // GATEWAY_VERSION / HOST_VERSION  — v0.13.0-6
    core: string; //        CORE_VERSION                    — v0.13.20-0 (external companion)
    coprocessor: string; // COPROCESSOR_*/CONNECTOR_*/LISTENER — v0.13.0-6
    relayer: string; //     RELAYER_VERSION                 — v0.13.0-6
    /** Client SDK: @fhevm/sdk (js-sdk). Its bundled tfhe wasm MUST match the keys' tfhe
     *  version — v0.13.0-6/kms keys are tfhe 1.6.x, so the js-sdk needs the 1.6.2 wasm
     *  (PR #2812); the 1.5.3 wasm cannot deserialize the public key (§0). */
    jsSdkTfhe: string;
  };
  hostChain: { chainId: number; mnemonic: string; svc: string; port: number };
  gatewayChain: { chainId: number; mnemonic: string; svc: string; port: number };
  /** The connector's keygen tx-sender — funded on host; its on-chain registered signer must
   *  equal kms-core's discovered signer, else KmsSignerDoesNotMatchTxSender. */
  keygenTxSender: string;
  /** minio public vault base URL (a host NOT literally "minio:9000" — the host-listener
   *  v0.13.0-6 rewrites that substring to 172.17.0.1:9000; DNAT it back on kind, §0). */
  publicVaultUrl: string;
};

/** Centralized/default scenario. */
export const DEFAULT_CONFIG: RecipeConfig = {
  namespace: "fhevm",
  chartsDir: "charts",
  dataDir: "stack/manifests",
  versions: {
    gatewayHost: "v0.13.0-6",
    core: "v0.13.20-0",
    coprocessor: "v0.13.0-6",
    relayer: "v0.13.0-6",
    jsSdkTfhe: "1.6.2",
  },
  hostChain: {
    chainId: 12345,
    mnemonic:
      "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer",
    svc: "host-node",
    port: 8545,
  },
  gatewayChain: {
    chainId: 54321,
    mnemonic: "coyote sketch defense hover finger envelope celery urge panther venue verb cheese",
    svc: "gateway-node",
    port: 8546,
  },
  keygenTxSender: "0x31De9c8ac5ECD5EacEddDdEE531e9BaD8AC9c2A5",
  publicVaultUrl: "http://minio:9000/kms-public",
};

// ---------------------------------------------------------------------------
// Discovery — everything learned at runtime (no value here is ever hardcoded)
// ---------------------------------------------------------------------------

/** Host-chain contract addresses, parsed from the host deploy's own log. */
export type HostAddresses = {
  acl?: string;
  fhevmExecutor?: string;
  kmsGeneration?: string; // v0.13: KMSGeneration is a HOST contract
  kmsVerifier?: string;
  inputVerifier?: string;
  hcuLimit?: string; // HCULimit (per-block compute caps) — the erc20 HCU tests need its address
};

/** Gateway-chain contract addresses, parsed from the gateway deploy's own log. */
export type GatewayAddresses = {
  gatewayConfig?: string;
  inputVerification?: string;
  ciphertextCommits?: string;
  decryption?: string;
  protocolPayment?: string; // order-dependent — MUST be discovered (the mocked-payment bug)
  pauserSet?: string;
  zamaOft?: string;
};

/**
 * Discovered values accumulated as phases run. Later phases read what earlier phases
 * discovered; ./render.threadDiscovery turns this into ConfigMap patches.
 */
export type Discovery = {
  /** kms-core's LIVE signer — scraped from its own log; kms-gen-keys is non-deterministic. */
  kmsSigner?: string;
  host: HostAddresses;
  gateway: GatewayAddresses;
  /** Realized FHE key id (on-chain ActivateKey after keygen) — feeds coprocessor + relayer keyurl. */
  fheKeyId?: string;
  /** Realized CRS id (on-chain ActivateCrs). */
  crsId?: string;
};

const ADDR = "(0x[0-9a-fA-F]{40})";
const grab = (log: string, re: RegExp): string | undefined => re.exec(log)?.[1];

/** Parse host contract addresses from `deployAllHostContracts` log output. */
export function parseHostAddresses(log: string): HostAddresses {
  return {
    acl: grab(log, new RegExp(`ACL\\b.*?${ADDR}`)),
    fhevmExecutor: grab(log, new RegExp(`FHEVMExecutor\\b.*?${ADDR}`)),
    kmsGeneration: grab(log, new RegExp(`KMSGeneration\\b.*?${ADDR}`)),
    kmsVerifier: grab(log, new RegExp(`KMSVerifier\\b.*?${ADDR}`)),
    inputVerifier: grab(log, new RegExp(`InputVerifier\\b.*?${ADDR}`)),
    hcuLimit: grab(log, new RegExp(`HCULimit\\b.*?${ADDR}`)),
  };
}

/** Parse gateway contract addresses from `deployAllGatewayContracts` + mocked-OFT log output. */
export function parseGatewayAddresses(log: string): GatewayAddresses {
  return {
    gatewayConfig: grab(log, new RegExp(`GatewayConfig address ${ADDR}`)),
    inputVerification: grab(log, new RegExp(`InputVerification address ${ADDR}`)),
    ciphertextCommits: grab(log, new RegExp(`CiphertextCommits address ${ADDR}`)),
    decryption: grab(log, new RegExp(`Decryption address ${ADDR}`)),
    protocolPayment: grab(log, new RegExp(`ProtocolPayment address ${ADDR}`)),
    pauserSet: grab(log, new RegExp(`PauserSet address ${ADDR}`)),
    zamaOft: grab(log, new RegExp(`ZamaOFT (?:deployed successfully at address: )?${ADDR}`)),
  };
}

// ---------------------------------------------------------------------------
// Phase model
// ---------------------------------------------------------------------------

export type Phase = {
  /** Stable id (used for resume / golden receipts). */
  id: string;
  title: string;
  /** §0 invariants this phase enforces — guardrails in plain language. */
  invariants?: string[];
  /** The work, over the Stack primitives. Mutates `disc` for later phases. */
  run: (ctx: Stack, cfg: RecipeConfig, disc: Discovery) => Promise<void>;
  /** Human-readable readiness gate that must hold before the next phase begins. */
  gate: string;
};

/** regenerate — thread everything discovered so far into the service ConfigMaps. Idempotent;
 *  run it after every discover step so each service starts with current addresses. */
const regenerate = async (ctx: Stack, cfg: RecipeConfig, disc: Discovery): Promise<void> => {
  for (const patch of threadDiscovery(cfg, disc)) {
    await ctx.patchConfigMap(patch.configMap, patch.data);
  }
};

// ---------------------------------------------------------------------------
// The recipe — the 16-step boot, with discover→regenerate woven through
// ---------------------------------------------------------------------------

export const RECIPE: Phase[] = [
  {
    id: "cluster",
    title: "kind cluster + registry-credentials secret",
    gate: "cluster reachable; private ghcr images pullable (read:packages token)",
    run: async () => {
      // Out-of-band: `kind create cluster` + `kubectl create secret docker-registry
      // registry-credentials` from a read:packages token. Every workload references it.
    },
  },
  {
    id: "chains",
    title: "anvil host (12345) + gateway (54321) + alias Services",
    invariants: [
      "anvil REQUIRES a mnemonic; per-chain mnemonics differ (host adapt…, gateway coyote…)",
      "anvil-node chart adds --state for persistence; a SIGKILL mid-dump corrupts state.json " +
        "(boot once from genesis; do NOT reset a chain mid-flight — §0)",
    ],
    gate: "both RPCs answer eth_chainId; host-node/gateway-node alias Services resolve",
    run: async (ctx, cfg) => {
      for (const release of ["host", "gateway"] as const) {
        await helmUpgrade(release, `${cfg.chartsDir}/anvil-node`, {
          namespace: cfg.namespace,
          install: true,
          wait: true,
          valuesFiles: [`${cfg.dataDir}/anvil-${release}.yaml`],
        });
      }
      await ctx.until(async () => (await ctx.chain<string>("eth_chainId")) != null);
    },
  },
  {
    id: "data-plane",
    title: "base env ConfigMaps + postgres (3 dbs) + minio",
    invariants: [
      "apply the base env ConfigMaps (envs.yaml) FIRST — every later regenerate (kms-signer, " +
        "deploys, await-keygen) PATCHES them in place, so they must already exist",
      "postgres AND minio need PVCs — ephemeral storage loses all DBs / FHE keys on restart " +
        "and cascades the whole stack into failure (§0 persistence finding)",
      "minio public vault MUST be EMPTY before keygen — it never overwrites existing objects, " +
        "so stale bytes survive and break on-chain digest validation",
      "order is db-init → service-migration → service (a fresh DB needs its migration first)",
    ],
    gate: "env ConfigMaps applied; postgres ready; db-init Completed (3 DBs); minio buckets ready",
    run: async (ctx, cfg) => {
      // alias Services (db/minio/kms-core + host-node/gateway-node) — stable in-cluster DNS that
      // every consumer addresses; they may precede their target pods (endpoints fill in later).
      await kubectlApply({ path: `${cfg.dataDir}/services.yaml` }, { namespace: cfg.namespace });
      await kubectlApply({ path: `${cfg.dataDir}/envs.yaml` }, { namespace: cfg.namespace });
      await kubectlApply({ path: `${cfg.dataDir}/data-plane.yaml` }, { namespace: cfg.namespace });
      await kubectlApply({ path: `${cfg.dataDir}/setup.yaml` }, { namespace: cfg.namespace });
      await ctx.until(async () =>
        (await ctx.sql(
          "deploy/db",
          "SELECT count(*) FROM pg_database WHERE datname IN ('coprocessor','kms-connector','relayer_db')",
        )).trim() === "3",
      );
    },
  },
  {
    id: "kms-core",
    title: "kms-core (external companion, centralized)",
    invariants: [
      "persist /app/kms/.../keys on a PVC so the signing key is stable across restarts",
      "kms-gen-keys signing key is NON-deterministic — its address must be DISCOVERED, never " +
        "hardcoded (the next phase)",
    ],
    gate: "kms-core serving :50051; signing key written to minio + PVC",
    run: async (_ctx, cfg) => {
      await kubectlApply({ path: `${cfg.dataDir}/kms-core.yaml` }, { namespace: cfg.namespace });
    },
  },
  {
    id: "kms-signer",
    title: "DISCOVER kms-core's live signer → regenerate deploy envs",
    invariants: [
      "scrape the signer from kms-core's OWN log (`stored … ethereum address 0x…`); deploy " +
        "envs register THIS signer on-chain so responses verify (else KmsSignerDoesNotMatchTxSender)",
    ],
    gate: "disc.kmsSigner set; host-sc-env/gateway-sc-env KMS_SIGNER_ADDRESS_0 patched",
    run: async (ctx, cfg, disc) => {
      // WAIT for kms-core to come up and log its (non-deterministic) signer, then assert it —
      // deploying before kms-core is serving registers a STALE signer on-chain (the silent-undefined
      // trap: the first boot here deployed the gateway with a wrong signer because kms-core lagged).
      await ctx.until(async () => {
        const log = await ctx.logs("deploy/kms-core", { tail: 4000 });
        disc.kmsSigner = grab(log, /ethereum address (0x[0-9a-fA-F]{40})/);
        return Boolean(disc.kmsSigner);
      }, 180_000, 3_000);
      await regenerate(ctx, cfg, disc);
    },
  },
  {
    id: "gateway-deploy",
    title: "deploy gateway contracts (+ mocked ZamaOFT) → DISCOVER addresses",
    invariants: [
      "deploy order is ORDER-DEPENDENT: the mocked-OFT shares the gateway deployer nonce space, " +
        "so it shifts every gateway proxy — discover the ACTUAL addresses, never assume them",
      "all gateway-sc steps share an /app/addresses volume so the generated GatewayAddresses.sol " +
        "is visible to the wiring steps (which compile contracts importing it)",
    ],
    gate: "gateway deploy Completed; disc.gateway populated from the deploy log",
    run: async (ctx, cfg, disc) => {
      await kubectlApply({ path: `${cfg.dataDir}/gateway-deploy.yaml` }, { namespace: cfg.namespace });
      await ctx.waitForJob("gateway-deploy"); // MUST complete before parsing — else partial log
      disc.gateway = parseGatewayAddresses(await ctx.logs("job/gateway-deploy", { tail: 4000 }));
      await regenerate(ctx, cfg, disc);
    },
  },
  {
    id: "host-deploy",
    title: "deploy host contracts (KMS_SIGNER = discovered) → DISCOVER addresses",
    invariants: [
      "KMSGeneration is a HOST contract in v0.13 (the gateway one is view-only)",
      "host-sc-env already carries the discovered KMS_SIGNER (kms-signer phase regenerated it)",
    ],
    gate: "host deploy Completed; disc.host populated from the deploy log",
    run: async (ctx, cfg, disc) => {
      await kubectlApply({ path: `${cfg.dataDir}/host-deploy.yaml` }, { namespace: cfg.namespace });
      await ctx.waitForJob("host-deploy"); // MUST complete before parsing — else partial log
      disc.host = parseHostAddresses(await ctx.logs("job/host-deploy", { tail: 4000 }));
      await regenerate(ctx, cfg, disc);
    },
  },
  {
    id: "gateway-wire",
    title: "gateway wiring: mocked-payment + host-chain registration + pausers",
    invariants: [
      "REQUIRES the discovered addresses (regenerated into gateway-mocked-payment-env): the " +
        "mocked-payment approve must target the DISCOVERED ProtocolPayment, or input-proof " +
        "verification reverts (the gateway charges INPUT_VERIFICATION_PRICE)",
      "addHostChainsToGatewayConfig registers the host chain (ACL/executor) so the gateway can " +
        "route input-proofs + decryptions; contracts deploy PAUSED — add-pausers unpauses them",
    ],
    gate: "host chain registered in GatewayConfig; pausers added; tx-sender approved on ProtocolPayment",
    run: async (ctx, cfg) => {
      // set-payment (discovered ProtocolPayment) → add-host-chains → add-pausers; shares the
      // gateway-addr volume for GatewayAddresses.sol.
      await kubectlApply({ path: `${cfg.dataDir}/gateway-wire.yaml` }, { namespace: cfg.namespace });
      await ctx.waitForJob("gateway-wire");
    },
  },
  {
    id: "fund-tx-sender",
    title: "fund the connector keygen tx-sender on the host chain",
    invariants: ["operational EOAs that are NOT mnemonic-prefunded need anvil_setBalance"],
    gate: "cfg.keygenTxSender balance > 0 on host chain",
    run: async (ctx, cfg) => {
      await ctx.chain("anvil_setBalance", [cfg.keygenTxSender, "0x56BC75E2D63100000"]);
    },
  },
  {
    id: "coprocessor",
    title: "coprocessor (db-migration + 7 services) — UP BEFORE keygen",
    invariants: [
      "ORDER: host-listener must be polling BEFORE keygen — it learns the key only from the " +
        "on-chain ActivateKey event and does NOT backfill (§0 finding 1)",
      "coprocessor-env already carries discovered host+gateway addresses (regenerated)",
      "host-listener v0.13.0-6 rewrites minio:9000 → 172.17.0.1:9000 (aws_s3.rs); a minio-dnat " +
        "initContainer DNATs it back to minio's CURRENT ClusterIP (resolve at runtime, §0)",
      "AWS_ENDPOINT_URL MUST be minio's ClusterIP (an IP), not the `minio` hostname: the S3 SDK " +
        "uses virtual-hosted-style for hostnames (ct128.minio:9000 → DNS fail) but path-style for " +
        "IPs. Discovered here, not hardcoded — the ClusterIP changes when minio is recreated (§0)",
      "host_chains MUST be seeded (chain_id → DISCOVERED ACL) after db-migration: the zkproof-worker " +
        "looks up the host chain there to verify input proofs; empty → `Unknown chain ID` → every " +
        "encrypted-input op (all transfers) stalls forever (plaintext mint is unaffected). Seed " +
        "BEFORE any proof request (erc20) so the worker never caches the miss (§0 finding 17)",
    ],
    gate: "all 7 services Running; host-listener polling KMSGeneration; host_chains seeded",
    run: async (ctx, cfg, disc) => {
      const minioIp = await ctx.serviceClusterIP("minio");
      await ctx.patchConfigMap("coprocessor-env", { AWS_ENDPOINT_URL: `http://${minioIp}:9000` });
      await kubectlApply({ path: `${cfg.dataDir}/coprocessor.yaml` }, { namespace: cfg.namespace });
      await ctx.waitForJob("coprocessor-db-migration"); // creates host_chains before we seed it
      if (disc.host.acl) {
        await ctx.sql(
          "deploy/db",
          `INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES ` +
            `(${cfg.hostChain.chainId}, 'host', '${disc.host.acl}') ON CONFLICT DO NOTHING`,
          "coprocessor",
        );
        // The zkproof-worker caches host_chains at startup and was applied (empty) above —
        // restart it so it re-reads the seeded chain, else it silently never picks up
        // input-proof work for chain 12345 and encryptUint64 hangs (§0 finding 17).
        await ctx.restart("coprocessor-zkproof-worker");
      }
      await ctx.waitForLog("deploy/coprocessor-host-listener", /KMSGeneration/);
    },
  },
  {
    id: "kms-connector",
    title: "kms-connector (gw-listener + kms-worker + tx-sender)",
    invariants: [
      "watches the HOST KMSGeneration over host-node:8545; wait for `Started KMSGeneration " +
        "polling from block N` before triggering (no backfill)",
    ],
    gate: "gw-listener logs `Started KMSGeneration polling from block N`",
    run: async (ctx, cfg) => {
      await kubectlApply({ path: `${cfg.dataDir}/kms-connector.yaml` }, { namespace: cfg.namespace });
      await ctx.waitForLog("deploy/kmsconn-gw-listener", /Started KMSGeneration polling/);
    },
  },
  {
    id: "trigger-keygen",
    title: "trigger keygen + crsgen host-side (two-phase: prep → keygen)",
    invariants: [
      "trigger HOST-side with the deploy's /app/addresses mounted (else HH404)",
      "keygen is two-phase: the prep response must land on-chain to emit the real KeygenRequest",
      "the tx-sender needs ETH (fund-tx-sender) and its registered signer must equal kms-core's",
    ],
    gate: "kms-core logs keygen `exiting normally`; objects stored fresh in the empty vault",
    run: async (ctx, cfg) => {
      await kubectlApply({ path: `${cfg.dataDir}/host-trigger.yaml` }, { namespace: cfg.namespace });
      await ctx.waitForJob("host-trigger-2");
    },
  },
  {
    id: "await-keygen",
    title: "wait for the active key; DISCOVER realized ids → regenerate",
    invariants: [
      "the realized FHE/CRS ids are DISCOVERED here and threaded into coprocessor + relayer",
      "the coprocessor registers the key from the ActivateKey event (download from minio); " +
        "the relayer keyurl is CONFIG-driven (no on-chain relay) — both fed from this discovery",
    ],
    gate: "coprocessor `keys` table populated (key activated); relayer keyurl serves disc.fheKeyId",
    run: async (ctx, cfg, disc) => {
      await ctx.until(async () => {
        // RAW hex, NO 0x prefix — both the minio object key (PUB/PublicKey/<id>) and the
        // coprocessor FHE_KEY_ID use the bare hex; a 0x prefix 404s the key URL.
        const keyHex = (await ctx.sql("deploy/db", "SELECT encode(key_id,'hex') FROM keys LIMIT 1", "coprocessor")).trim();
        if (!keyHex) return false;
        disc.fheKeyId = keyHex;
        const crsHex = (await ctx.sql(
          "deploy/db",
          "SELECT encode(crs_id,'hex') FROM kms_crs_activation_events WHERE status='activated' LIMIT 1",
          "coprocessor",
        )).trim();
        if (crsHex) disc.crsId = crsHex;
        return Boolean(disc.crsId);
      }, 600_000, 5_000);
      await regenerate(ctx, cfg, disc);
    },
  },
  {
    id: "relayer",
    title: "relayer (+ relayer-migrate one-shot + relayer_db)",
    invariants: [
      "relayer-migrate /bin/server only migrates then exits — scale it to 0 after (a fresh " +
        "relayer_db must be migrated before the relayer starts)",
      "keyurl is CONFIG-driven: relayer-env APP_KEYURL__* carry the discovered key id " +
        "(await-keygen regenerated them) — the relayer serves /v2/keyurl from config, not chain",
    ],
    gate: "relayer logs `All servers are ready`; GET /v2/keyurl returns disc.fheKeyId",
    run: async (ctx, cfg) => {
      // relayer-config (local.yaml) is mounted by relayer.yaml — apply it first.
      await kubectlApply({ path: `${cfg.dataDir}/relayer-config.yaml` }, { namespace: cfg.namespace });
      await kubectlApply({ path: `${cfg.dataDir}/relayer.yaml` }, { namespace: cfg.namespace });
      await ctx.waitForLog("deploy/relayer", /All servers are ready/);
      await ctx.stop("relayer-migrate");
    },
  },
  {
    id: "erc20",
    title: "erc20 e2e (encrypt → compute → decrypt) → record L2 golden",
    invariants: [
      "run the client IN-CLUSTER (the on-chain key URL is minio:9000, in-cluster only)",
      "use @fhevm/sdk (js-sdk) with the tfhe-1.6.2 wasm — it must match the keys' tfhe version " +
        "and serves /v2/keyurl (no relayer-sdk version pin; §0)",
      "erc20-env already carries every DISCOVERED host+gateway address (regenerated); the Job " +
        "consumes them via envFrom — the boot drives the test self-contained, no hand-set env",
      "DEPLOYER_PRIVATE_KEY (erc20-env) must be the host deployer = ACL owner, so the HCU " +
        "block-cap tests can mutate the protocol HCU caps (else NotHostOwner)",
    ],
    gate: "erc20-e2e Job succeeds (all 11 EncryptedERC20 tests pass) — the L2 behavioral golden",
    run: async (ctx, cfg) => {
      await kubectlApply({ path: `${cfg.dataDir}/erc20.yaml` }, { namespace: cfg.namespace });
      // Job exit 0 == hardhat all-green; exit 1 == a failing test → waitForJob rejects (the gate).
      // 20min: SDK init + heavy CPU FHE + the HCU "accumulate until block cap" tx-storm.
      await ctx.waitForJob("erc20-e2e", 1_200_000);
    },
  },
];

// ---------------------------------------------------------------------------
// The thin driver — walk the recipe phases in order
// ---------------------------------------------------------------------------

/** Emitted per phase; the L3 runbook receipt is a stream of these (timestamps stripped). */
export type PhaseReceipt = { id: string; title: string; status: "ok" | "failed"; error?: string };

/**
 * bootStack — the entire `up` orchestration: walk RECIPE in order, running each phase over the
 * Stack primitives and carrying forward discovered values. This is the WHOLE driver — all
 * sequencing/knowledge lives in the declarative RECIPE above, not here.
 */
export const bootStack = async (
  ctx: Stack,
  cfg: RecipeConfig = DEFAULT_CONFIG,
  opts: { from?: string; until?: string; onPhase?: (r: PhaseReceipt) => void } = {},
): Promise<PhaseReceipt[]> => {
  const disc: Discovery = { host: {}, gateway: {} };
  const receipts: PhaseReceipt[] = [];
  const startIdx = opts.from ? RECIPE.findIndex((p) => p.id === opts.from) : 0;
  if (startIdx < 0) throw new Error(`unknown phase id: ${opts.from}`);
  const untilIdx = opts.until ? RECIPE.findIndex((p) => p.id === opts.until) : RECIPE.length - 1;
  if (untilIdx < 0) throw new Error(`unknown phase id: ${opts.until}`);

  for (const phase of RECIPE.slice(startIdx, untilIdx + 1)) {
    try {
      await phase.run(ctx, cfg, disc);
      const r: PhaseReceipt = { id: phase.id, title: phase.title, status: "ok" };
      receipts.push(r);
      opts.onPhase?.(r);
    } catch (error) {
      const r: PhaseReceipt = {
        id: phase.id,
        title: phase.title,
        status: "failed",
        error: error instanceof Error ? error.message : String(error),
      };
      receipts.push(r);
      opts.onPhase?.(r);
      throw new Error(`phase "${phase.id}" failed (gate: ${phase.gate}): ${r.error}`);
    }
  }
  return receipts;
};
