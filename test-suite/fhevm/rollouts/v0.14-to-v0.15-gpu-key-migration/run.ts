import path from "node:path";

import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { waitForContainer } from "../../src/flow/readiness";
import {
  COPROCESSOR_DB_CONTAINER,
  DEFAULT_POSTGRES_PASSWORD,
  DEFAULT_POSTGRES_USER,
  coprocessorDatabaseName,
  defaultHostChainKey,
  dockerArgs,
} from "../../src/layout";
import { kmsConnectorDbName, kmsConnectorPrefix, kmsPartyIds } from "../../src/kms-party";
import { hostReachableRpcUrl } from "../../src/utils/fs";
import { run, runStreaming } from "../../src/utils/process";
import {
  type ConnectorObservation,
  type OperatorMaterial,
  assertConnectorMigrationReady,
  assertOperatorMaterialAgreement,
} from "./checks";
import { connectorVersionKeys, migrationVersions, versionSources } from "./versions";

const CONNECTOR_PARTIES = 4;
const OPERATOR_COUNT = 2;
const CONNECTOR_SERVICES = ["db-migration", "gw-listener", "kms-worker", "tx-sender"] as const;
const KEY_WORKERS = ["tfhe-worker", "zkproof-worker", "sns-worker"] as const;

const logPhase = (label: string) => console.log(`\n[GPU key migration] ${label}`);

const sqlScalar = async (database: string, sql: string): Promise<string> => {
  const result = await run([
    "docker",
    "exec",
    "-e",
    `PGPASSWORD=${DEFAULT_POSTGRES_PASSWORD}`,
    COPROCESSOR_DB_CONTAINER,
    "psql",
    "-U",
    DEFAULT_POSTGRES_USER,
    "-d",
    database,
    "-t",
    "-A",
    "-c",
    sql,
  ]);
  return result.stdout.trim();
};

const waitUntil = async (
  label: string,
  check: () => Promise<boolean>,
  timeoutSeconds = Number(process.env.RFC029_MIGRATION_TIMEOUT_SECONDS || 3600),
) => {
  const deadline = Date.now() + timeoutSeconds * 1000;
  while (!(await check())) {
    if (Date.now() >= deadline) {
      throw new Error(`${label} timed out after ${timeoutSeconds}s`);
    }
    await Bun.sleep(2_000);
  }
  console.log(`[ready] ${label}`);
};

export const migrationScenario = (blueTag: string) => `version: 1
kind: blue-green
name: RFC 029 production migration
hostChains:
  - key: host
    chainId: "12345"
    rpcPort: 8545
  - key: chain-b
    chainId: "67890"
    rpcPort: 8547
topology:
  count: ${OPERATOR_COUNT}
  threshold: ${OPERATOR_COUNT}
bcs:
  source:
    mode: registry
    tag: ${JSON.stringify(blueTag)}
  env:
    FORCE_LEGACY_SERVER_KEY: "true"
gcs:
  source:
    mode: local
  stackVersion: "0.15.0"
kms:
  mode: threshold
  parties: ${CONNECTOR_PARTIES}
  threshold: 1
  fheParams: Test
`;

const connectorObservation = async (party: number): Promise<ConnectorObservation> => {
  const database = kmsConnectorDbName(party);
  const prefix = kmsConnectorPrefix(party);
  const [cursorText, schemaText, imageResult] = await Promise.all([
    sqlScalar(
      database,
      "SELECT COALESCE(block_number, -1) FROM last_block_polled_by_chain WHERE chain_name = 'ethereum';",
    ),
    sqlScalar(
      database,
      "SELECT EXISTS (SELECT 1 FROM information_schema.columns " +
        "WHERE table_name='keygen_requests' AND column_name='migration_key_id')::int;",
    ),
    run(["docker", "inspect", "--format", "{{.Config.Image}}", `${prefix}-gw-listener`]),
  ]);
  return {
    cursor: Number(cursorText || -1),
    hasMigrationSchema: schemaText === "1",
    image: imageResult.stdout.trim(),
    party,
  };
};

const connectorObservations = () =>
  Promise.all(kmsPartyIds(CONNECTOR_PARTIES).map(connectorObservation));

const upgradeFirstConnectorOnly = async () => {
  const names = CONNECTOR_SERVICES.map((suffix) => `kms-connector-${suffix}`);
  await runStreaming([...dockerArgs("kms-connector"), "build", ...names]);
  await runStreaming([
    ...dockerArgs("kms-connector"),
    "up",
    "-d",
    "--no-deps",
    "--force-recreate",
    "kms-connector-db-migration",
  ]);
  await waitForContainer("kms-connector-db-migration", "complete");
  const runtime = names.filter((name) => !name.endsWith("db-migration"));
  await runStreaming([
    ...dockerArgs("kms-connector"),
    "up",
    "-d",
    "--no-deps",
    "--force-recreate",
    ...runtime,
  ]);
  for (const name of runtime) {
    await waitForContainer(name, "running");
  }
};

const operatorMaterial = async (operator: number): Promise<OperatorMaterial | undefined> => {
  const database = coprocessorDatabaseName(operator);
  const value = await sqlScalar(
    database,
    `SELECT encode(e.key_id, 'hex') || '|' || encode(e.key_digest, 'hex') || '|' ||
            e.status || '|' || (k.sks_key IS NOT NULL)::int || '|' ||
            (k.compressed_xof_keyset IS NOT NULL)::int
       FROM kms_compressed_key_material_events e
       JOIN keys k ON k.key_id = e.key_id
      WHERE e.status = 'applied'
      ORDER BY e.block_number DESC
      LIMIT 1;`,
  );
  if (!value) return undefined;
  const [keyId, digest, status, legacy, compressed] = value.split("|");
  return {
    compressed: compressed === "1",
    digest: digest ?? "",
    keyId: keyId ?? "",
    legacy: legacy === "1",
    operator,
    status: status ?? "",
  };
};

const forcesLegacyServerKey = async (container: string): Promise<boolean> => {
  const result = await run(["docker", "inspect", "--format", "{{range .Config.Env}}{{println .}}{{end}}", container]);
  return result.stdout
    .split("\n")
    .some((line) => line === "FORCE_LEGACY_SERVER_KEY=true");
};

const assertFleetSafeguard = async () => {
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const prefix = operator === 0 ? "coprocessor-" : `coprocessor${operator}-`;
    for (const worker of KEY_WORKERS) {
      const blue = `${prefix}${worker}`;
      const green = `${prefix}gcs-${worker}`;
      if (!(await forcesLegacyServerKey(blue))) {
        throw new Error(`${blue} is not forced to use legacy material`);
      }
      if (await forcesLegacyServerKey(green)) {
        throw new Error(`${green} must not receive the force-legacy safeguard`);
      }
    }
  }
};

const upgradeKmsGeneration = async (ctx: RolloutRunContext, targetLock: string) => {
  await ctx.snapshotContracts("host");
  await ctx.applyVersionLock("RFC 029 host contract source", {
    lockFile: targetLock,
    allowedVersionKeys: ["HOST_VERSION"],
    overrides: [{ group: "host-contracts" }],
  });
  await ctx.runHostContractTask(
    [
      "npx hardhat task:upgradeKMSGeneration",
      "--current-implementation previous-contracts/KMSGeneration.sol:KMSGeneration",
      "--new-implementation contracts/KMSGeneration.sol:KMSGeneration",
      "--verify-contract false",
      "--use-internal-proxy-address true",
    ].join(" "),
  );
};

export default async function runMigration(ctx: RolloutRunContext) {
  const versions = migrationVersions();
  const baselineLock = await ctx.resolveVersionLock("rfc029-00-baseline", {
    versions: versions.baseline,
    sources: versionSources,
  });
  const targetLock = await ctx.resolveVersionLock("rfc029-01-target", {
    versions: { CORE_VERSION: versions.baseline.CORE_VERSION! },
    sources: versionSources,
  });
  const scenario = path.join(ctx.stateDir(), "rollout", "rfc029-blue-green.yaml");
  await Bun.write(scenario, migrationScenario(versions.blueTag));

  logPhase("00 boot a real legacy-key baseline");
  await ctx.up({
    lockFile: baselineLock,
    scenario,
    overrides: [{ group: "test-suite" }],
  });
  await assertFleetSafeguard();
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const state = await sqlScalar(
      coprocessorDatabaseName(operator),
      "SELECT (sks_key IS NOT NULL)::int || '|' || " +
        "(compressed_xof_keyset IS NOT NULL)::int FROM keys ORDER BY sequence_number DESC LIMIT 1;",
    );
    if (state !== "1|0") {
      throw new Error(`operator ${operator} did not start with legacy-only key material: ${state}`);
    }
  }
  await ctx.test("input-proof-compute-decrypt", { parallel: false });
  const preMigrationHandles = await Promise.all(
    Array.from({ length: OPERATOR_COUNT }, (_, operator) =>
      sqlScalar(
        coprocessorDatabaseName(operator),
        "SELECT encode(handle, 'hex') FROM ciphertexts ORDER BY created_at LIMIT 1;",
      ),
    ),
  );
  if (preMigrationHandles.some((handle) => !handle)) {
    throw new Error("baseline traffic did not create a ciphertext on every operator");
  }

  logPhase("01 upgrade KMSGeneration without invoking migration");
  await upgradeKmsGeneration(ctx, targetLock);
  const state = await ctx.readState();
  const hostKey = defaultHostChainKey(state.scenario.hostChains);
  const hostRpc = hostReachableRpcUrl(state.discovery!.endpoints.hosts[hostKey]!.http);
  const deploymentBoundary = Number((await run(["cast", "block-number", "--rpc-url", hostRpc])).stdout.trim());

  logPhase("02 prove a mixed connector fleet blocks the request");
  await ctx.applyVersionLock("RFC 029 connector source", {
    lockFile: targetLock,
    allowedVersionKeys: [...connectorVersionKeys],
    overrides: [{ group: "kms-connector" }],
  });
  await upgradeFirstConnectorOnly();
  let mixedBlocked = false;
  try {
    assertConnectorMigrationReady(await connectorObservations(), deploymentBoundary);
  } catch (error) {
    if (!(error instanceof Error) || !error.message.startsWith("connector gate blocked:")) {
      throw error;
    }
    mixedBlocked = true;
    console.log(`[expected] ${error.message}`);
  }
  if (!mixedBlocked) {
    throw new Error("connector gate accepted a mixed connector deployment");
  }

  logPhase("03 upgrade every connector and establish finalized listener boundaries");
  await ctx.upgradeRuntimeGroup("kms-connector");
  await waitUntil("all connector listeners crossed the deployment boundary", async () => {
    try {
      assertConnectorMigrationReady(await connectorObservations(), deploymentBoundary);
      return true;
    } catch {
      return false;
    }
  }, 300);

  logPhase("04 request compressed material for the existing active key");
  const keyIdHex = await sqlScalar(
    coprocessorDatabaseName(0),
    "SELECT encode(key_id, 'hex') FROM keys ORDER BY sequence_number DESC LIMIT 1;",
  );
  const keyId = BigInt(`0x${keyIdHex}`).toString();
  await run(["docker", "stop", "coprocessor1-gcs-host-listener"]);
  await ctx.runHostContractTask(
    `npx hardhat task:migrateToCompressedKeySet --key-id ${keyId} --use-internal-proxy-address true`,
  );
  await run(["docker", "restart", "kms-connector-4-kms-worker"]);

  logPhase("05 wait for threshold publication and every operator's passive ingestion");
  await waitUntil("operator 0 applied compressed material while operator 1 was delayed", async () =>
    Boolean(await operatorMaterial(0)),
  );
  if (await operatorMaterial(1)) {
    throw new Error("material gate accepted operator 1 while its Green listener was stopped");
  }
  await run(["docker", "start", "coprocessor1-gcs-host-listener"]);
  await waitForContainer("coprocessor1-gcs-host-listener", "running");
  let materialRows: OperatorMaterial[] = [];
  await waitUntil("all operators applied identical compressed material", async () => {
    const rows = await Promise.all(
      Array.from({ length: OPERATOR_COUNT }, (_, operator) => operatorMaterial(operator)),
    );
    if (rows.some((row) => !row)) return false;
    materialRows = rows as OperatorMaterial[];
    try {
      assertOperatorMaterialAgreement(materialRows);
      return true;
    } catch {
      return false;
    }
  });
  assertOperatorMaterialAgreement(materialRows);
  await assertFleetSafeguard();
  await run(["docker", "restart", "coprocessor1-gcs-tfhe-worker"]);
  await waitForContainer("coprocessor1-gcs-tfhe-worker", "running");

  // Publication is passive: Blue must still compute successfully after every
  // database contains the compressed representation.
  await ctx.test("input-proof-compute-decrypt", { parallel: false });

  logPhase("06 run the existing blue-green failure, retry, unanimity, and cutover battery");
  await ctx.test("blue-green", { parallel: false });

  logPhase("07 verify preserved history and post-cutover protocol paths");
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const handle = preMigrationHandles[operator]!;
    const present = await sqlScalar(
      coprocessorDatabaseName(operator),
      `SELECT EXISTS (SELECT 1 FROM ciphertexts WHERE handle=decode('${handle}', 'hex'))::int;`,
    );
    if (present !== "1") {
      throw new Error(`operator ${operator} lost pre-migration ciphertext ${handle}`);
    }
  }
  await ctx.test("rollout-standard", { parallel: false });
  await ctx.test("public-decryption", { parallel: false });
  await ctx.test("user-decryption", { parallel: false });
}
