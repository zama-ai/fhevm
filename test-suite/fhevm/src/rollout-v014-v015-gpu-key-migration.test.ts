import { describe, expect, test } from "bun:test";

import {
  assertConnectorMigrationReady,
  assertOperatorMaterialAgreement,
} from "../rollouts/v0.14-to-v0.15-gpu-key-migration/checks";
import { migrationScenario } from "../rollouts/v0.14-to-v0.15-gpu-key-migration/run";
import { migrationVersions } from "../rollouts/v0.14-to-v0.15-gpu-key-migration/versions";
import { parseBlueGreenScenario } from "./scenario/resolve";

describe("RFC 029 rollout gates", () => {
  test("requires an exact Blue hotfix tag", () => {
    expect(() => migrationVersions({})).toThrow("RFC029_BLUE_HOTFIX_TAG is required");
    expect(migrationVersions({ RFC029_BLUE_HOTFIX_TAG: "v0.14.1-0" }).blueTag).toBe("v0.14.1-0");
  });

  test("forces Blue to legacy and leaves the safeguard off Green", () => {
    const scenario = parseBlueGreenScenario(migrationScenario("v0.14.1-0"), "generated RFC 029 scenario");
    expect(scenario.bcs?.env?.FORCE_LEGACY_SERVER_KEY).toBe("true");
    expect(scenario.gcs.env?.FORCE_LEGACY_SERVER_KEY).toBeUndefined();
    expect(scenario.hostChains).toHaveLength(2);
    expect(scenario.kms).toEqual({ mode: "threshold", parties: 4, threshold: 1, fheParams: "Test" });
  });

  test("blocks a mixed connector deployment", () => {
    expect(() =>
      assertConnectorMigrationReady([
        { cursor: 100, hasMigrationSchema: true, image: "target", party: 1 },
        { cursor: 100, hasMigrationSchema: false, image: "legacy", party: 2 },
      ], 90),
    ).toThrow("connector images differ");
  });

  test("blocks a connector behind the deployment boundary", () => {
    expect(() =>
      assertConnectorMigrationReady([
        { cursor: 89, hasMigrationSchema: true, image: "target", party: 1 },
        { cursor: 100, hasMigrationSchema: true, image: "target", party: 2 },
      ], 90),
    ).toThrow("listener cursor is before deployment block");
  });

  test("blocks delayed or disagreeing operator material", () => {
    expect(() =>
      assertOperatorMaterialAgreement([
        { compressed: true, digest: "aa", keyId: "01", legacy: true, operator: 0, status: "applied" },
        { compressed: false, digest: "aa", keyId: "01", legacy: true, operator: 1, status: "ready" },
      ]),
    ).toThrow("operator 1 is incomplete");
    expect(() =>
      assertOperatorMaterialAgreement([
        { compressed: true, digest: "aa", keyId: "01", legacy: true, operator: 0, status: "applied" },
        { compressed: true, digest: "bb", keyId: "01", legacy: true, operator: 1, status: "applied" },
      ]),
    ).toThrow("key ID or digest differs");
  });
});
