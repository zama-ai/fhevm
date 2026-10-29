import { describe, expect, test } from "bun:test";
import path from "node:path";

import {
  type OperatorMaterial,
  assertConnectorMigrationReady,
  assertConnectorUpgraded,
  assertOperatorMaterialAgreement,
} from "../rollouts/key-migration-from-v0.14/checks";
import { parseBlueGreenScenario, resolveBlueGreenScenario } from "./scenario/resolve";

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

describe("key migration from v0.14", () => {
  const scenario = async (name: string) => {
    const file = path.join(import.meta.dir, "..", "scenarios", `${name}.yaml`);
    return resolveBlueGreenScenario(file, parseBlueGreenScenario(await Bun.file(file).text(), name));
  };

  test("boots 0.14 Blue and defers the upgrade and the locally built Green with the legacy safeguard", async () => {
    for (const name of ["key-migration-from-v0.14", "key-migration-from-v0.14-gpu"]) {
      const resolved = await scenario(name);
      expect(resolved.bootstrap).toEqual({ tag: "v0.14.2-0", upgrade: "deferred" });
      expect(resolved.bcs.source).toEqual({ mode: "registry", tag: "v0.14.2-0" });
      expect(resolved.gcs.source).toEqual({ mode: "local" });
      expect(resolved.gcs.env.FORCE_LEGACY_SERVER_KEY).toBe("true");
      expect(resolved.gcs.deferredStart).toBe(true);
      expect(resolved.topology).toEqual({ count: 2, threshold: 2 });
      expect(resolved.hostChains).toHaveLength(2);
    }
    expect((await scenario("key-migration-from-v0.14")).kms).toEqual({
      parties: 4,
      threshold: 1,
      committeeSize: 4,
      fheParams: "Test",
    });
  });

  test("selects Default parameters at baseline creation for the GPU phase", async () => {
    expect((await scenario("key-migration-from-v0.14-gpu")).kms).toEqual({
      parties: 4,
      threshold: 1,
      committeeSize: 4,
      fheParams: "Default",
      insecureTestKeygen: true,
    });
  });

  test("blocks a mixed connector deployment", () => {
    expect(() =>
      assertConnectorMigrationReady([
        { cursor: 100, hasMigrationSchema: true, images: ["repo:target|sha-a", "repo:target|sha-b"], party: 1 },
        { cursor: 100, hasMigrationSchema: false, images: ["repo:legacy|sha-c", "repo:target|sha-b"], party: 2 },
      ], 90, ["repo:target|sha-a", "repo:target|sha-b"]),
    ).toThrow("connector service images differ across parties");
  });

  test("requires every service of the first upgraded connector party to change image", () => {
    expect(() => assertConnectorUpgraded(["repo:legacy|sha-a"], ["repo:legacy|sha-a"])).toThrow(
      "still runs its previous image",
    );
    expect(() => assertConnectorUpgraded(["repo:legacy|sha-a"], ["repo:target|"])).toThrow("still runs its previous image");
    expect(() => assertConnectorUpgraded(["repo:legacy|sha-a"], ["repo:target|sha-b"])).not.toThrow();
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
