import { describe, expect, test } from "bun:test";

import {
  type OperatorMaterial,
  assertConnectorMigrationReady,
  assertLocalConnectorUpgrade,
  assertOperatorMaterialAgreement,
} from "../rollouts/v0.15-to-v0.15.1-gpu-key-migration/checks";
import {
  adoptionScenario,
  migrationScenario,
} from "../rollouts/v0.15-to-v0.15.1-gpu-key-migration/run";
import {
  connectorVersionKeys,
  coprocessorVersionKeys,
  migrationPhaseVersions,
  migrationVersions,
} from "../rollouts/v0.15-to-v0.15.1-gpu-key-migration/versions";
import { parseBlueGreenScenario, parseCoprocessorScenario } from "./scenario/resolve";

const material = (overrides: Partial<OperatorMaterial> = {}): OperatorMaterial => ({
  blockNumber: 10,
  chainId: "12345",
  compressed: true,
  digest: "aa",
  existingKeyId: "01",
  keyId: "02",
  legacy: true,
  operator: 0,
  status: "activated",
  storedMatchesVerified: true,
  ...overrides,
});

describe("RFC 029 rollout gates", () => {
  test("requires an exact 0.15 Blue tag", () => {
    expect(() => migrationVersions({})).toThrow("RFC029_BLUE_TAG is required");
    expect(() => migrationVersions({ RFC029_BLUE_TAG: "latest" })).toThrow("release tag or an exact commit SHA");
    expect(migrationVersions({ RFC029_BLUE_TAG: "v0.15.0-0" }).blueTag).toBe("v0.15.0-0");
    expect(migrationVersions({ RFC029_BLUE_TAG: "04fb072ac5f4d46e562c56f0f1ff1aea94df1c4c" }).blueTag).toBe("04fb072");
  });

  test("uses the last published 0.14 images only to restore production-style legacy state", () => {
    const versions = migrationVersions({ RFC029_BLUE_TAG: "v0.15.0-0" });
    expect(versions.baselineTag).toBe("v0.14.0-10");
    expect(versions.baseline.HOST_VERSION).toBe("v0.14.0-9");
    expect(versions.baseline.GATEWAY_VERSION).toBe("v0.14.0-10");
  });

  test("forces the 0.15 migration workers to stay on legacy material", () => {
    const scenario = parseCoprocessorScenario(migrationScenario(), "generated RFC 029 migration scenario");
    expect(scenario.instances).toHaveLength(2);
    expect(
      scenario.instances?.every((instance) => instance.env?.FORCE_LEGACY_SERVER_KEY === "true"),
    ).toBe(true);
    expect(scenario.kms).toEqual({ mode: "threshold", parties: 4, threshold: 1, fheParams: "Test" });
  });

  test("forces Blue to legacy and leaves the safeguard off Green", () => {
    const scenario = parseBlueGreenScenario(
      adoptionScenario("v0.14.0-10"),
      "generated RFC 029 adoption scenario",
    );
    expect(scenario.bcs?.env?.FORCE_LEGACY_SERVER_KEY).toBe("true");
    expect(scenario.gcs.env?.FORCE_LEGACY_SERVER_KEY).toBeUndefined();
    expect(scenario.gcs.deferredStart).toBe(true);
    expect(scenario.gcs.stackVersion).toBe("0.15.1");
    expect(scenario.hostChains).toHaveLength(2);
    expect(scenario.kms).toEqual({ mode: "threshold", parties: 4, threshold: 1, fheParams: "Test" });
  });

  test("changes only the intended deployment unit in each version lock", () => {
    const baseline: Record<string, string> = {
      ...migrationVersions({ RFC029_BLUE_TAG: "v0.15.0-0" }).baseline,
      LISTENER_CORE_VERSION: "baseline-listener",
      RELAYER_VERSION: "baseline-relayer",
      TEST_SUITE_VERSION: "baseline-test-suite",
    };
    const target = {
      ...baseline,
      CORE_VERSION: "main-core",
      HOST_VERSION: "main-host",
      ...Object.fromEntries(connectorVersionKeys.map((key) => [key, `main-${key}`])),
      ...Object.fromEntries(coprocessorVersionKeys.map((key) => [key, `main-${key}`])),
    };
    const phases = migrationPhaseVersions(baseline, target, "v0.15.0-0");

    expect(phases.contract.HOST_VERSION).toBe("main-host");
    expect(phases.contract.CONNECTOR_KMS_WORKER_VERSION).toBe(baseline.CONNECTOR_KMS_WORKER_VERSION);
    expect(phases.contract.COPROCESSOR_TFHE_WORKER_VERSION).toBe(baseline.COPROCESSOR_TFHE_WORKER_VERSION);
    expect(phases.connector.CONNECTOR_KMS_WORKER_VERSION).toBe("main-CONNECTOR_KMS_WORKER_VERSION");
    expect(phases.connector.CORE_VERSION).toBe("main-core");
    expect(phases.connector.COPROCESSOR_TFHE_WORKER_VERSION).toBe(baseline.COPROCESSOR_TFHE_WORKER_VERSION);
    expect(phases.blue.COPROCESSOR_TFHE_WORKER_VERSION).toBe("v0.15.0-0");
    expect(phases.blue.CORE_VERSION).toBe("main-core");
    expect(phases.blue.LISTENER_CORE_VERSION).toBe("baseline-listener");
    expect(phases.contract.RELAYER_VERSION).toBe("baseline-relayer");
    expect(phases.connector.TEST_SUITE_VERSION).toBe("baseline-test-suite");
  });

  test("blocks a mixed connector deployment", () => {
    expect(() =>
      assertConnectorMigrationReady([
        { cursor: 100, hasMigrationSchema: true, images: ["repo:target|sha-a", "repo:target|sha-b"], party: 1 },
        { cursor: 100, hasMigrationSchema: false, images: ["repo:legacy|sha-c", "repo:target|sha-b"], party: 2 },
      ], 90, ["repo:target|sha-a", "repo:target|sha-b"]),
    ).toThrow("connector service images differ across parties");
  });

  test("requires the first connector party to run newly built local images", () => {
    expect(() =>
      assertLocalConnectorUpgrade(
        ["repo:legacy|sha-a"],
        ["repo:target|sha-b"],
      ),
    ).toThrow("locally built image");
    expect(() =>
      assertLocalConnectorUpgrade(
        ["repo:legacy|sha-a"],
        ["repo:fhevm-local|sha-b"],
      ),
    ).not.toThrow();
  });

  test("blocks a connector behind the deployment boundary", () => {
    expect(() =>
      assertConnectorMigrationReady([
        { cursor: 89, hasMigrationSchema: true, images: ["repo:target|sha-a"], party: 1 },
        { cursor: 100, hasMigrationSchema: true, images: ["repo:target|sha-a"], party: 2 },
      ], 90, ["repo:target|sha-a"]),
    ).toThrow("listener cursor is before deployment block");
  });

  test("blocks incomplete or disagreeing operator material", () => {
    expect(() =>
      assertOperatorMaterialAgreement([
        material(),
        material({ compressed: false, operator: 1, status: "ready" }),
      ]),
    ).toThrow("operator 1 is incomplete");
    expect(() =>
      assertOperatorMaterialAgreement([
        material(),
        material({ digest: "bb", operator: 1 }),
      ]),
    ).toThrow("applied material differs across operators");
    expect(() =>
      assertOperatorMaterialAgreement([
        material(),
        material({ keyId: "03", operator: 1 }),
      ]),
    ).toThrow("applied material differs across operators");
    expect(() =>
      assertOperatorMaterialAgreement([
        material({ storedMatchesVerified: false }),
      ]),
    ).toThrow("stored bytes differ from the verified download");
  });
});
