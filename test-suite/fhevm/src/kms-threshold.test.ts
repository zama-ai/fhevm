import path from "node:path";
import { describe, expect, test } from "bun:test";

import { resolveUpgradePlan } from "./flow/repair";
import { renderEnvMaps } from "./generate/env";
import {
  buildKmsThresholdOverride,
  kmsRenderOptionsFor,
  reconstructionThreshold,
  renderThresholdConfigToml,
} from "./generate/kms-core";
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
});

describe("renderThresholdConfigToml", () => {
  test("pins my_id, the reconstruction threshold and the full peer list", () => {
    const toml = renderThresholdConfigToml(2, fourParty, RENDER_OPTS);
    expect(toml).toContain("my_id = 2");
    expect(toml).toContain("threshold = 1");
    expect(toml).toContain("party_id = 4");
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
