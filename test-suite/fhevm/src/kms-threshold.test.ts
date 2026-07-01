import path from "node:path";
import { describe, expect, test } from "bun:test";

import { partyContainers, quorumPlan } from "./commands/kms-generation";
import { resolveUpgradePlan } from "./flow/repair";
import { buildKmsConnectorOverride } from "./generate/compose";
import { buildGatewayScSwapEnv, buildHostScSwapEnv, renderEnvMaps } from "./generate/env";
import {
  KMS_THRESHOLD_CONFIG_NAME,
  KMS_THRESHOLD_SPARE_CONFIG_NAME,
  THRESHOLD_PEERS_MARKER,
  buildKmsThresholdOverride,
  kmsRenderOptionsFor,
  renderThresholdCoreConfig,
  renderThresholdPeers,
  renderThresholdSpareConfig,
  thresholdCoreEnv,
} from "./generate/kms-core";
import {
  kmsConnectorDbName,
  kmsConnectorEnvName,
  kmsConnectorPrefix,
  kmsCoreName,
  reconstructionThreshold,
} from "./kms-party";
import { COMPONENTS, TEMPLATE_ENV_DIR } from "./layout";
import { presetBundle } from "./resolve/target";
import { resolveKmsTopology } from "./scenario/resolve";
import { stackSpecForState, type StackSpec } from "./stack-spec/stack-spec";
import { testDefaultScenario } from "./test-fixtures";
import type { State } from "./types";
import { readEnvFile } from "./utils/fs";

const RENDER_OPTS = kmsRenderOptionsFor("c57f52f");
const fourParty = resolveKmsTopology({ mode: "threshold", parties: 4, threshold: 1 });
// 5 cores, 4-node committee (t=1), 1 spare — the node-swap substrate.
const swapTopology = resolveKmsTopology({ mode: "threshold", parties: 5, threshold: 1, committeeSize: 4 });

describe("resolveKmsTopology", () => {
  test("absent block keeps today's centralized single node", () => {
    expect(resolveKmsTopology(undefined)).toEqual({
      mode: "centralized",
      parties: 1,
      threshold: 1,
      committeeSize: 1,
      fheParams: "Default",
    });
  });

  test("threshold defaults to 4 parties / t=1 / Test params", () => {
    expect(resolveKmsTopology({ mode: "threshold" })).toEqual({
      mode: "threshold",
      parties: 4,
      threshold: 1,
      committeeSize: 4,
      fheParams: "Test",
    });
  });

  test("accepts the next valid 3t+1 cluster size (7 parties, t=2)", () => {
    expect(resolveKmsTopology({ mode: "threshold", parties: 7, threshold: 2 })).toMatchObject({
      parties: 7,
      threshold: 2,
    });
  });

  test("rejects topologies that violate parties === 3*threshold + 1", () => {
    expect(() => resolveKmsTopology({ mode: "threshold", parties: 4, threshold: 2 })).toThrow(/3\*threshold/);
  });

  test("rejects fewer than 4 parties", () => {
    expect(() => resolveKmsTopology({ mode: "threshold", parties: 3, threshold: 1 })).toThrow();
  });

  test("committeeSize defaults to parties (no spares)", () => {
    expect(resolveKmsTopology({ mode: "threshold", parties: 4, threshold: 1 }).committeeSize).toBe(4);
    expect(resolveKmsTopology({ mode: "threshold", parties: 7, threshold: 2 }).committeeSize).toBe(7);
  });

  test("accepts a spare cluster: committeeSize < parties (4-of-5 for a node swap)", () => {
    expect(swapTopology).toMatchObject({ parties: 5, threshold: 1, committeeSize: 4 });
  });

  test("enforces 3*t+1 on the committee, not the cluster size", () => {
    // 5 cores with a 4-node committee (t=1) is valid; a 5-node committee is not (5 != 3t+1).
    expect(() =>
      resolveKmsTopology({ mode: "threshold", parties: 5, threshold: 1, committeeSize: 5 }),
    ).toThrow(/3\*threshold/);
  });

  test("rejects committeeSize greater than parties", () => {
    expect(() =>
      resolveKmsTopology({ mode: "threshold", parties: 4, threshold: 1, committeeSize: 7 }),
    ).toThrow(/committeeSize/);
  });

  test("rejects Default params for threshold (deferred to a follow-up PR)", () => {
    expect(() => resolveKmsTopology({ mode: "threshold", parties: 4, threshold: 1, fheParams: "Default" })).toThrow(
      /Test/,
    );
  });

  test("rejects Test params for centralized (it would be a silent no-op)", () => {
    expect(() => resolveKmsTopology({ mode: "centralized", fheParams: "Test" })).toThrow(/threshold/);
  });

  test("rejects an empty `kms:` key (YAML null) instead of crashing", () => {
    expect(() => resolveKmsTopology(null as unknown as undefined)).toThrow(/must be a map/);
  });

  test("rejects unknown fheParams values in centralized mode (e.g. a typo)", () => {
    expect(() =>
      resolveKmsTopology({ mode: "centralized", fheParams: "default" as unknown as "Default" }),
    ).toThrow(/must be "Test" or "Default"/);
  });
});

describe("reconstructionThreshold", () => {
  test("is 2t+1", () => {
    expect(reconstructionThreshold(1)).toBe(3);
    expect(reconstructionThreshold(2)).toBe(5);
  });
});

describe("quorumPlan", () => {
  test("4 parties / t=1: stop 1 to reach the 2t+1=3 quorum, stop 2 to drop below it", () => {
    expect(quorumPlan(4, 1)).toEqual({ reconstruct: 3, stopForTolerance: 1, stopForFloor: 2 });
  });

  test("7 parties / t=2: stop 2 to reach 2t+1=5, stop 3 to drop below it", () => {
    expect(quorumPlan(7, 2)).toEqual({ reconstruct: 5, stopForTolerance: 2, stopForFloor: 3 });
  });
});

describe("partyContainers", () => {
  test("party 1 uses the bare names; later parties get the -{i}- suffix", () => {
    expect(partyContainers(1)).toEqual(["kms-core", "kms-connector-gw-listener", "kms-connector-kms-worker", "kms-connector-tx-sender"]);
    expect(partyContainers(3)).toEqual(["kms-core-3", "kms-connector-3-gw-listener", "kms-connector-3-kms-worker", "kms-connector-3-tx-sender"]);
  });
});

describe("kms-party naming conventions", () => {
  test("party 1 keeps the bare single-node names everywhere", () => {
    expect(kmsCoreName(1)).toBe("kms-core");
    expect(kmsConnectorPrefix(1)).toBe("kms-connector");
    expect(kmsConnectorEnvName(1)).toBe("kms-connector");
    expect(kmsConnectorDbName(1)).toBe("kms-connector");
  });

  test("parties 2..N get the documented per-artifact suffix forms", () => {
    expect(kmsCoreName(2)).toBe("kms-core-2");
    expect(kmsConnectorPrefix(2)).toBe("kms-connector-2");
    expect(kmsConnectorEnvName(2)).toBe("kms-connector.2"); // dot: instance env-file convention
    expect(kmsConnectorDbName(2)).toBe("kms-connector-2");
  });
});

describe("buildKmsThresholdOverride", () => {
  test("emits the gen-keys job, one core per party, and the init job", () => {
    const names = Object.keys(buildKmsThresholdOverride(fourParty, RENDER_OPTS).services);
    expect(names).toEqual(
      expect.arrayContaining(["kms-core-gen-keys", "kms-core", "kms-core-2", "kms-core-3", "kms-core-4", "kms-core-init"]),
    );
  });

  test("gen-keys generates ONLY signing keys, sized to exactly N parties", () => {
    const entrypoint = JSON.stringify(buildKmsThresholdOverride(fourParty, RENDER_OPTS).services["kms-core-gen-keys"].entrypoint);
    // `--cmd signing-keys` and `--num-parties` are gated by `--help` probes (newer cores dropped both);
    // keep the probes so a pinned newer CORE_VERSION still boots.
    expect(entrypoint).toContain("if kms-gen-keys --help");
    expect(entrypoint).toContain("if kms-gen-keys threshold --help");
    expect(entrypoint).toContain("--cmd signing-keys");
    expect(entrypoint).toContain("--num-parties 4");
  });

  test("rejects a non-threshold topology", () => {
    expect(() => buildKmsThresholdOverride(resolveKmsTopology(undefined), RENDER_OPTS)).toThrow();
  });

  test("each core runs the binary directly (no shell), with config + creds from env", () => {
    const core = buildKmsThresholdOverride(fourParty, RENDER_OPTS).services["kms-core-3"];
    const env = core.environment as Record<string, string>;
    expect(env.KMS_CORE__THRESHOLD__MY_ID).toBe("3");
    expect(env.AWS_ACCESS_KEY_ID).toBe("fhevm-access-key");
    expect(JSON.stringify(core.volumes)).toContain(KMS_THRESHOLD_CONFIG_NAME);
    expect(JSON.stringify(core.entrypoint)).toContain(KMS_THRESHOLD_CONFIG_NAME);
    expect(JSON.stringify(core.entrypoint)).not.toContain("/bin/sh");
    expect(JSON.stringify(core.volumes)).not.toContain("minio_secrets");
  });

  test("cores publish no host ports (everything dials them over the docker network)", () => {
    const services = buildKmsThresholdOverride(fourParty, RENDER_OPTS).services;
    for (const name of ["kms-core", "kms-core-2", "kms-core-3", "kms-core-4"]) {
      expect(services[name].ports).toBeUndefined();
    }
  });

  test("provisions all parties; spares (party > committeeSize) mount the peers=None config", () => {
    const services = buildKmsThresholdOverride(swapTopology, RENDER_OPTS).services;
    // 5 cores provisioned: 4 committee + 1 spare.
    expect(Object.keys(services)).toEqual(expect.arrayContaining(["kms-core", "kms-core-4", "kms-core-5"]));
    // The committee core mounts the committee roster config; the spare mounts the peers=None config.
    expect(JSON.stringify(services["kms-core-4"].volumes)).toContain(KMS_THRESHOLD_CONFIG_NAME);
    expect(JSON.stringify(services["kms-core-4"].volumes)).not.toContain(KMS_THRESHOLD_SPARE_CONFIG_NAME);
    expect(JSON.stringify(services["kms-core-5"].volumes)).toContain(KMS_THRESHOLD_SPARE_CONFIG_NAME);
  });

  test("kms-init bootstraps the committee endpoints only (not the spare)", () => {
    const init = JSON.stringify(
      buildKmsThresholdOverride(swapTopology, RENDER_OPTS).services["kms-core-init"].entrypoint,
    );
    expect(init).toContain(`${kmsCoreName(4)}:`);
    expect(init).not.toContain(`${kmsCoreName(5)}:`);
  });
});

describe("threshold core config", () => {
  test("renderThresholdPeers lists every party with its address and mpc port", () => {
    const peers = renderThresholdPeers(fourParty);
    expect(peers).toContain("party_id = 1");
    expect(peers).toContain("party_id = 4");
    expect(peers).toContain('address = "kms-core"');
    expect(peers).toContain('address = "kms-core-4"');
    expect(peers).toContain("port = 50004");
    // mpc_identity must equal the on-chain KMS_NODE_MPC_IDENTITY (kmsCoreName) so a node's identity
    // matches across the config-seeded baseline context and a dynamically-added switch context.
    expect(peers).toContain('mpc_identity = "kms-core"');
    expect(peers).toContain('mpc_identity = "kms-core-4"');
  });

  test("renderThresholdCoreConfig injects the roster and leaves no marker", () => {
    const config = renderThresholdCoreConfig(`[threshold]\nmy_id = 0\n\n${THRESHOLD_PEERS_MARKER}\n`, fourParty);
    expect(config).not.toContain(THRESHOLD_PEERS_MARKER);
    expect(config).toContain("party_id = 4");
  });

  test("renderThresholdCoreConfig fails fast when the template marker is missing", () => {
    expect(() => renderThresholdCoreConfig("[threshold]\n", fourParty)).toThrow(/marker/);
  });

  test("renderThresholdPeers rosters the committee, not the whole cluster", () => {
    // 5 cores, 4-node committee: roster has the 4 committee members; the spare (party 5) is absent.
    const peers = renderThresholdPeers(swapTopology);
    expect(peers).toContain("party_id = 4");
    expect(peers).not.toContain("party_id = 5");
  });

  test("renderThresholdSpareConfig drops the roster entirely (peers=None)", () => {
    const spare = renderThresholdSpareConfig(`[threshold]\nmy_id = 0\n\n${THRESHOLD_PEERS_MARKER}\n`);
    expect(spare).not.toContain(THRESHOLD_PEERS_MARKER);
    expect(spare).not.toContain("threshold.peers");
    expect(spare).not.toContain("party_id");
  });

  test("thresholdCoreEnv overrides per-party identity, ports and prefixes", () => {
    const env = thresholdCoreEnv(2, fourParty, RENDER_OPTS);
    expect(env.KMS_CORE__THRESHOLD__MY_ID).toBe("2");
    expect(env.KMS_CORE__THRESHOLD__THRESHOLD).toBe("1");
    expect(env.KMS_CORE__SERVICE__LISTEN_PORT).toBe("50200");
    expect(env.KMS_CORE__THRESHOLD__LISTEN_PORT).toBe("50002");
    expect(env.KMS_CORE__PUBLIC_VAULT__STORAGE__S3__PREFIX).toBe("PUB-p2");
    expect(env.KMS_CORE__PRIVATE_VAULT__STORAGE__S3__PREFIX).toBe("PRIV-p2");
    expect(env.AWS_ACCESS_KEY_ID).toBe("fhevm-access-key");
  });
});

describe("buildKmsConnectorOverride (--override kms-connector)", () => {
  const thresholdSpec = (overrides: State["overrides"]) =>
    stackSpecForState({
      target: "latest-main",
      requiresGitHub: true,
      versions: presetBundle("latest-main", "abcdef0", "latest-main.json"),
      overrides,
      scenario: testDefaultScenario({ kms: { mode: "threshold", parties: 4, threshold: 1, committeeSize: 4, fheParams: "Test" } }),
    });

  test("without an override every party runs the resolved published image (no build specs)", async () => {
    const services = (await buildKmsConnectorOverride(thresholdSpec([]))).services;
    for (const [name, service] of Object.entries(services)) {
      expect(service.build, name).toBeUndefined();
      expect(String(service.image), name).not.toContain("fhevm-local");
    }
  });

  test("override retags every party to the one locally built image; only party 1 builds it", async () => {
    const services = (await buildKmsConnectorOverride(thresholdSpec([{ group: "kms-connector" }]))).services;
    const base = services["kms-connector-gw-listener"];
    const clone = services["kms-connector-3-gw-listener"];
    expect(String(base.image)).toEndWith(":fhevm-local");
    expect(base.build).toBeDefined();
    // Same image tag, no per-party rebuild: maybeBuild builds by base service name once.
    expect(clone.image).toBe(base.image);
    expect(clone.build).toBeUndefined();
  });
});

describe("renderEnvMaps (threshold)", () => {
  const deriveWallet = async (_mnemonic: string, index: number) => ({
    address: `0x${String(index + 1).padStart(40, "1")}`,
    privateKey: `0x${String(index + 1).padStart(64, "2")}`,
  });

  test("projects N-party counts, 2t+1 thresholds and Test params into gateway + host envs", async () => {
    const templateEnvs = Object.fromEntries(
      await Promise.all(
        COMPONENTS.map(async (component) => [
          component,
          await readEnvFile(path.join(TEMPLATE_ENV_DIR, `.env.${component}`)),
        ]),
      ),
    ) as Record<string, Record<string, string>>;
    const state: State = {
      target: "latest-main",
      lockPath: "/tmp/latest-main.json",
      requiresGitHub: true,
      versions: presetBundle("latest-main", "abcdef0", "latest-main.json"),
      overrides: [],
      scenario: testDefaultScenario({ kms: { mode: "threshold", parties: 4, threshold: 1, committeeSize: 4, fheParams: "Test" } }),
      completedSteps: [],
      updatedAt: "2026-03-30T00:00:00.000Z",
    };

    const rendered = await renderEnvMaps({ discovery: undefined }, stackSpecForState(state), templateEnvs, deriveWallet);
    const gw = rendered.componentEnvs["gateway-sc"];
    const host = rendered.componentEnvs["host-sc"];

    expect(gw.NUM_KMS_NODES).toBe("4");
    expect(gw.MPC_THRESHOLD).toBe("1");
    expect(host.MPC_THRESHOLD).toBe("1"); // mirrored to host-sc so ProtocolConfig agrees with the gateway
    expect(gw.PUBLIC_DECRYPTION_THRESHOLD).toBe("3"); // 2t+1
    expect(gw.USER_DECRYPTION_THRESHOLD).toBe("3");
    expect(gw.KMS_GENERATION_THRESHOLD).toBe("3");
    // one distinct tx-sender per party
    expect(gw.KMS_TX_SENDER_ADDRESS_0).toBeDefined();
    expect(gw.KMS_TX_SENDER_ADDRESS_3).toBeDefined();
    expect(gw.KMS_TX_SENDER_ADDRESS_0).not.toBe(gw.KMS_TX_SENDER_ADDRESS_3);
    // Test params drive the on-chain keygen/crsgen triggers
    expect(rendered.versionsEnv.KEYGEN_PARAMS_TYPE).toBe("1");
  });
});

describe("buildHostScSwapEnv / buildGatewayScSwapEnv (node swap)", () => {
  const swapPlan = { kms: swapTopology } as unknown as StackSpec;
  const noSwapPlan = { kms: fourParty } as unknown as StackSpec;

  // Rendered host-sc has the spare's identity + discovered signer/CA-cert at index 4, but NOT its
  // connection vars (host copies only the committee 0..3); those live in gateway-sc for every party.
  const hostSc = (): Record<string, string> => ({
    NUM_KMS_NODES: "4",
    KMS_TX_SENDER_ADDRESS_3: "0xtx4",
    KMS_SIGNER_ADDRESS_3: "0xsigner4",
    KMS_SIGNER_ADDRESS_4: "0xsigner5",
    KMS_NODE_IP_3: "http://kms-core-4:50004",
    KMS_NODE_STORAGE_URL_3: "http://minio/kms-public",
    KMS_NODE_PARTY_ID_3: "4",
    KMS_NODE_PARTY_ID_4: "5",
    KMS_NODE_MPC_IDENTITY_3: "kms-core-4",
    KMS_NODE_MPC_IDENTITY_4: "kms-core-5",
    KMS_NODE_CA_CERT_3: "cert4",
    KMS_NODE_CA_CERT_4: "cert5",
    KMS_NODE_STORAGE_PREFIX_3: "PUB-p4",
    KMS_NODE_STORAGE_PREFIX_4: "PUB-p5",
  });
  const gatewaySc = (): Record<string, string> => ({
    NUM_KMS_NODES: "4",
    KMS_TX_SENDER_ADDRESS_3: "0xtx4",
    KMS_TX_SENDER_ADDRESS_4: "0xtx5",
    KMS_SIGNER_ADDRESS_3: "0xsigner4",
    KMS_SIGNER_ADDRESS_4: "0xsigner5",
    KMS_NODE_IP_ADDRESS_3: "http://kms-core-4:50004",
    KMS_NODE_IP_ADDRESS_4: "http://kms-core-5:50005",
    KMS_NODE_STORAGE_URL_3: "http://minio/kms-public",
    KMS_NODE_STORAGE_URL_4: "http://minio/kms-public",
  });

  test("host swap env promotes the spare (node 5) into the dropped committee slot, keeping NUM_KMS_NODES", () => {
    const swap = buildHostScSwapEnv(hostSc(), gatewaySc(), swapPlan)!;
    expect(swap).toBeDefined();
    expect(swap.NUM_KMS_NODES).toBe("4"); // still a 4-node committee, just a different member
    // The spare takes the dropped node's MPC position, so the party id stays positional (the core
    // rejects ids outside 1..n); only the node's identity/address/material move to the spare.
    expect(swap.KMS_NODE_PARTY_ID_3).toBe("4");
    expect(swap.KMS_SIGNER_ADDRESS_3).toBe("0xsigner5");
    expect(swap.KMS_TX_SENDER_ADDRESS_3).toBe("0xtx5"); // sourced from gateway-sc (absent from host-sc)
    expect(swap.KMS_NODE_IP_3).toBe("http://kms-core-5:50005");
    expect(swap.KMS_NODE_MPC_IDENTITY_3).toBe("kms-core-5"); // physical core is the spare
    expect(swap.KMS_NODE_CA_CERT_3).toBe("cert5");
    expect(swap.KMS_NODE_STORAGE_PREFIX_3).toBe("PUB-p5");
  });

  test("gateway swap env promotes the spare's (txSender, signer, ip, storageUrl) into the slot", () => {
    const swap = buildGatewayScSwapEnv(gatewaySc(), swapPlan)!;
    expect(swap).toBeDefined();
    expect(swap.KMS_TX_SENDER_ADDRESS_3).toBe("0xtx5");
    expect(swap.KMS_SIGNER_ADDRESS_3).toBe("0xsigner5");
    expect(swap.KMS_NODE_IP_ADDRESS_3).toBe("http://kms-core-5:50005");
    expect(swap.NUM_KMS_NODES).toBe("4");
  });

  test("returns undefined for a non-swap topology (no spare)", () => {
    expect(buildHostScSwapEnv(hostSc(), gatewaySc(), noSwapPlan)).toBeUndefined();
    expect(buildGatewayScSwapEnv(gatewaySc(), noSwapPlan)).toBeUndefined();
  });

  test("returns undefined before the spare's signer is discovered (so no half-built env is written)", () => {
    const hostPre = hostSc();
    delete hostPre.KMS_SIGNER_ADDRESS_4;
    expect(buildHostScSwapEnv(hostPre, gatewaySc(), swapPlan)).toBeUndefined();
    const gwPre = gatewaySc();
    delete gwPre.KMS_SIGNER_ADDRESS_4;
    expect(buildGatewayScSwapEnv(gwPre, swapPlan)).toBeUndefined();
  });
});

describe("resolveUpgradePlan (threshold guard)", () => {
  const thresholdState = {
    overrides: [],
    scenario: testDefaultScenario({ kms: { mode: "threshold", parties: 4, threshold: 1, committeeSize: 4, fheParams: "Test" } }),
  };

  test("refuses to upgrade the single-core/connector KMS groups on a threshold-mode cluster", () => {
    expect(() => resolveUpgradePlan(thresholdState, "kms", { lockFile: true })).toThrow(/threshold-mode KMS/);
    expect(() => resolveUpgradePlan(thresholdState, "kms-core", { lockFile: true })).toThrow(/threshold-mode KMS/);
  });
});
