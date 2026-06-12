import path from "node:path";
import { describe, expect, test } from "bun:test";

import { partyContainers, quorumPlan } from "./commands/kms-generation";
import { resolveUpgradePlan } from "./flow/repair";
import { renderEnvMaps } from "./generate/env";
import {
  KMS_THRESHOLD_CONFIG_NAME,
  THRESHOLD_PEERS_MARKER,
  buildKmsThresholdOverride,
  kmsRenderOptionsFor,
  renderThresholdCoreConfig,
  renderThresholdPeers,
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
import { stackSpecForState } from "./stack-spec/stack-spec";
import { testDefaultScenario } from "./test-fixtures";
import type { State } from "./types";
import { readEnvFile } from "./utils/fs";

const RENDER_OPTS = kmsRenderOptionsFor("c57f52f");
const fourParty = resolveKmsTopology({ mode: "threshold", parties: 4, threshold: 1 });

describe("resolveKmsTopology", () => {
  test("absent block keeps today's centralized single node", () => {
    expect(resolveKmsTopology(undefined)).toEqual({
      mode: "centralized",
      parties: 1,
      threshold: 1,
      fheParams: "Default",
    });
  });

  test("threshold defaults to 4 parties / t=1 / Test params", () => {
    expect(resolveKmsTopology({ mode: "threshold" })).toEqual({
      mode: "threshold",
      parties: 4,
      threshold: 1,
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

  test("rejects Default params for threshold (deferred to a follow-up PR)", () => {
    expect(() => resolveKmsTopology({ mode: "threshold", parties: 4, threshold: 1, fheParams: "Default" })).toThrow(
      /Test/,
    );
  });

  test("rejects Test params for centralized (it would be a silent no-op)", () => {
    expect(() => resolveKmsTopology({ mode: "centralized", fheParams: "Test" })).toThrow(/threshold/);
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
    // Guards the KMS reference flow: `--cmd` defaults to `all` (which pre-generates FHE key shares +
    // CRS centrally) and `--num-parties` defaults to 4 (too small for a larger cluster).
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
});

describe("threshold core config", () => {
  test("renderThresholdPeers lists every party with its address and mpc port", () => {
    const peers = renderThresholdPeers(fourParty);
    expect(peers).toContain("party_id = 1");
    expect(peers).toContain("party_id = 4");
    expect(peers).toContain('address = "kms-core"');
    expect(peers).toContain('address = "kms-core-4"');
    expect(peers).toContain("port = 50004");
  });

  test("renderThresholdCoreConfig injects the roster and leaves no marker", () => {
    const config = renderThresholdCoreConfig(`[threshold]\nmy_id = 0\n\n${THRESHOLD_PEERS_MARKER}\n`, fourParty);
    expect(config).not.toContain(THRESHOLD_PEERS_MARKER);
    expect(config).toContain("party_id = 4");
  });

  test("renderThresholdCoreConfig fails fast when the template marker is missing", () => {
    expect(() => renderThresholdCoreConfig("[threshold]\n", fourParty)).toThrow(/marker/);
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
      scenario: testDefaultScenario({ kms: { mode: "threshold", parties: 4, threshold: 1, fheParams: "Test" } }),
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

describe("resolveUpgradePlan (threshold guard)", () => {
  const thresholdState = {
    overrides: [],
    scenario: testDefaultScenario({ kms: { mode: "threshold", parties: 4, threshold: 1, fheParams: "Test" } }),
  };

  test("refuses to upgrade the single-core/connector KMS groups on a threshold cluster", () => {
    expect(() => resolveUpgradePlan(thresholdState, "kms", { lockFile: true })).toThrow(/threshold KMS/);
    expect(() => resolveUpgradePlan(thresholdState, "kms-core", { lockFile: true })).toThrow(/threshold KMS/);
  });
});
