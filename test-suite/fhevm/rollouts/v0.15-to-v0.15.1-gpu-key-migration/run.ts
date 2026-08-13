import { createHash } from "node:crypto";
import path from "node:path";

import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { waitForContainer } from "../../src/flow/readiness";
import {
  COPROCESSOR_DB_CONTAINER,
  DEFAULT_POSTGRES_PASSWORD,
  DEFAULT_POSTGRES_USER,
  coprocessorDatabaseName,
  defaultHostChainKey,
  MINIO_EXTERNAL_URL,
  TEST_SUITE_CONTAINER,
} from "../../src/layout";
import {
  kmsConnectorDbName,
  kmsConnectorPrefix,
  kmsPartyIds,
  kmsPublicPrefix,
} from "../../src/kms-party";
import { hostReachableRpcUrl } from "../../src/utils/fs";
import { run, runStreaming } from "../../src/utils/process";
import {
  type ConnectorObservation,
  type OperatorMaterial,
  assertConnectorMigrationReady,
  assertLocalConnectorUpgrade,
  assertOperatorMaterialAgreement,
} from "../v0.14-to-v0.15-gpu-key-migration/checks";
import { migrationPhaseVersions, migrationVersions, versionSources } from "./versions";

const CONNECTOR_PARTIES = 4;
const OPERATOR_COUNT = 2;
const CONNECTOR_SERVICES = ["db-migration", "gw-listener", "kms-worker", "tx-sender"] as const;
const KEY_WORKERS = ["tfhe-worker", "zkproof-worker", "sns-worker"] as const;
const ACTIVE_MIGRATION_SERVICES = ["host-listener", "host-listener-poller", "host-listener-consumer", ...KEY_WORKERS];

const logPhase = (label: string) => console.log(`\n[GPU key migration] ${label}`);

const runKeyContinuity = async (mode: "prepare" | "reuse", contract?: string) => {
  const result = await run([
    "docker",
    "exec",
    "-e",
    `RFC029_KEY_CONTINUITY_MODE=${mode}`,
    ...(contract ? ["-e", `RFC029_KEY_CONTINUITY_CONTRACT=${contract}`] : []),
    TEST_SUITE_CONTAINER,
    "npx",
    "hardhat",
    "run",
    "--no-compile",
    "scripts/rfc029-key-continuity.ts",
    "--network",
    "staging",
  ]);
  return result.stdout;
};

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

const activeKeyStateSql =
  "SELECT encode(key_id, 'hex') || '|' || encode(key_id_gw, 'hex') || '|' || md5(sks_key) || '|' || " +
  "md5(pks_key) || '|' || COALESCE(md5(cks_key), 'NULL') || '|' || COALESCE(md5(lo_get(sns_pk)), 'NULL') || '|' || " +
  "sequence_number || '|' || COALESCE(chain_id::text, 'NULL') || '|' || " +
  "COALESCE(encode(block_hash, 'hex'), 'NULL') || '|' || count(*) OVER () " +
  "FROM keys ORDER BY sequence_number DESC LIMIT 1;";

const hostChains = `hostChains:
  - key: host
    chainId: "12345"
    rpcPort: 8545
  - key: chain-b
    chainId: "67890"
    rpcPort: 8547`;

const kmsTopology = `kms:
  mode: threshold
  parties: ${CONNECTOR_PARTIES}
  threshold: 1
  fheParams: Test`;

export const adoptionScenario = (blueTag: string) => `version: 1
kind: blue-green
name: RFC 029 0.15 to 0.15.1 compressed key adoption
${hostChains}
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
  stackVersion: "0.15.1"
  deferredStart: true
${kmsTopology}
`;

const connectorObservation = async (party: number): Promise<ConnectorObservation> => {
  const database = kmsConnectorDbName(party);
  const prefix = kmsConnectorPrefix(party);
  const [cursorText, schemaText, imageResults] = await Promise.all([
    sqlScalar(
      database,
      "SELECT COALESCE(block_number, -1) FROM last_block_polled_by_chain WHERE chain_name = 'ethereum';",
    ),
    sqlScalar(
      database,
      "SELECT (COUNT(*) = 2)::int FROM information_schema.columns " +
        "WHERE table_name IN ('prep_keygen_requests', 'keygen_requests') AND column_name='existing_key_id';",
    ),
    Promise.all(
      CONNECTOR_SERVICES.map((service) =>
        run(["docker", "inspect", "--format", "{{.Config.Image}}|{{.Image}}", `${prefix}-${service}`]),
      ),
    ),
  ]);
  return {
    cursor: Number(cursorText || -1),
    hasMigrationSchema: schemaText === "1",
    images: imageResults.map((result) => result.stdout.trim()),
    party,
  };
};

const connectorObservations = () =>
  Promise.all(kmsPartyIds(CONNECTOR_PARTIES).map(connectorObservation));

const waitForConnectorBoundary = (
  party: number,
  deploymentBoundary: number,
  expectedImages: readonly string[],
) =>
  waitUntil(`KMS operator ${party} listener crossed the deployment boundary`, async () => {
    try {
      assertConnectorMigrationReady([await connectorObservation(party)], deploymentBoundary, expectedImages);
      return true;
    } catch {
      return false;
    }
  }, 300);

const assertKmsCoreVersions = async (parties: readonly number[], expectedVersion: string) => {
  for (const party of parties) {
    const container = party === 1 ? "kms-core" : `kms-core-${party}`;
    const image = (await run(["docker", "inspect", "--format", "{{.Config.Image}}", container])).stdout.trim();
    if (!image.endsWith(`:${expectedVersion}`)) {
      throw new Error(`${container} uses ${image}; expected version ${expectedVersion}`);
    }
  }
};

const kmsPublicMaterialDigests = async (
  type: "ServerKey" | "CompressedXofKeySet",
  keyId: string,
) =>
  Promise.all(
    kmsPartyIds(CONNECTOR_PARTIES).map(async (party) => {
      const response = await fetch(
        `${MINIO_EXTERNAL_URL}/kms-public/${kmsPublicPrefix(party)}/${type}/${keyId}`,
      );
      if (!response.ok) {
        throw new Error(
          `KMS operator ${party} does not serve ${type}/${keyId}: HTTP ${response.status}`,
        );
      }
      return createHash("sha256").update(Buffer.from(await response.arrayBuffer())).digest("hex");
    }),
  );

const operatorMaterial = async (
  operator: number,
  expectedKeyId: string,
  expectedChainId: string,
  afterBlock: number,
): Promise<OperatorMaterial | undefined> => {
  const database = coprocessorDatabaseName(operator);
  const value = await sqlScalar(
    database,
    `SELECT e.chain_id || '|' || e.block_number || '|' ||
            encode(e.key_id, 'hex') || '|' || encode(e.existing_key_id, 'hex') || '|' ||
            encode(e.key_digest_server, 'hex') || '|' ||
            e.status || '|' || (k.sks_key IS NOT NULL)::int || '|' ||
            (k.compressed_xof_keyset IS NOT NULL)::int || '|' ||
            (k.compressed_xof_keyset = e.key_content_compressed_xof_keyset)::int
       FROM kms_key_activation_events e
       JOIN keys k ON k.key_id = e.existing_key_id
       JOIN host_chain_blocks_valid b ON b.chain_id = e.chain_id AND b.block_hash = e.block_hash
      WHERE e.status = 'activated'
        AND b.block_status = 'finalized'
        AND e.existing_key_id IS NOT NULL
        AND e.key_content_compressed_xof_keyset IS NOT NULL
        AND e.chain_id = ${BigInt(expectedChainId).toString()}
        AND e.existing_key_id = decode('${expectedKeyId}', 'hex')
        AND e.block_number > ${afterBlock}
      ORDER BY e.block_number DESC
      LIMIT 1;`,
  );
  if (!value) return undefined;
  const [chainId, blockNumber, keyId, existingKeyId, digest, status, legacy, compressed, storedMatchesVerified] =
    value.split("|");
  return {
    blockNumber: Number(blockNumber),
    chainId: chainId ?? "",
    compressed: compressed === "1",
    digest: digest ?? "",
    existingKeyId: existingKeyId ?? "",
    keyId: keyId ?? "",
    legacy: legacy === "1",
    operator,
    storedMatchesVerified: storedMatchesVerified === "1",
    status: status ?? "",
  };
};

const forcesLegacyServerKey = async (container: string): Promise<boolean> => {
  const result = await run(["docker", "inspect", "--format", "{{range .Config.Env}}{{println .}}{{end}}", container]);
  return result.stdout
    .split("\n")
    .some((line) => line === "FORCE_LEGACY_SERVER_KEY=true");
};

const assertActiveSafeguard = async () => {
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const prefix = operator === 0 ? "coprocessor-" : `coprocessor${operator}-`;
    for (const worker of KEY_WORKERS) {
      const container = `${prefix}${worker}`;
      if (!(await forcesLegacyServerKey(container))) {
        throw new Error(`${container} is not forced to use legacy material`);
      }
    }
  }
};

const assertActiveImageTag = async (tag: string) => {
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const prefix = operator === 0 ? "coprocessor-" : `coprocessor${operator}-`;
    for (const service of ACTIVE_MIGRATION_SERVICES) {
      const container = `${prefix}${service}`;
      const result = await run(["docker", "inspect", "--format", "{{.Config.Image}}|{{.Image}}", container]);
      const [configuredImage, imageId] = result.stdout.trim().split("|");
      if (!configuredImage?.endsWith(`:${tag}`) || !imageId) {
        throw new Error(`${container} is not running the exact ${tag} image`);
      }
    }
  }
};

const assertGreenSafeguard = async () => {
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const prefix = operator === 0 ? "coprocessor-" : `coprocessor${operator}-`;
    for (const worker of KEY_WORKERS) {
      const green = `${prefix}gcs-${worker}`;
      if (await forcesLegacyServerKey(green)) {
        throw new Error(`${green} must not receive the force-legacy safeguard`);
      }
    }
  }
};

const assertGreenAbsent = async () => {
  const result = await run(["docker", "ps", "-a", "--format", "{{.Names}}"]);
  const greenContainers = result.stdout
    .split("\n")
    .filter((name) => /^coprocessor\d*-gcs-/.test(name));
  if (greenContainers.length) {
    throw new Error(`Green containers started before material convergence: ${greenContainers.join(", ")}`);
  }
};

const restartActiveKeyWorkers = async () => {
  const workers = Array.from({ length: OPERATOR_COUNT }, (_, operator) => {
    const prefix = operator === 0 ? "coprocessor-" : `coprocessor${operator}-`;
    return KEY_WORKERS.map((worker) => `${prefix}${worker}`);
  }).flat();
  await runStreaming(["docker", "restart", ...workers]);
  for (const worker of workers) {
    await waitForContainer(worker, "running");
  }
};

const assertWorkerRepresentation = async (
  role: "Active" | "Green",
  representation: "legacy" | "compressed-xof",
  since: string,
) => {
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const prefix = operator === 0 ? "coprocessor-" : `coprocessor${operator}-`;
    for (const worker of KEY_WORKERS) {
      const container = `${prefix}${role === "Green" ? "gcs-" : ""}${worker}`;
      await waitUntil(`${container} loaded ${representation}`, async () => {
        const result = await run(["docker", "logs", "--since", since, container], { allowFailure: true });
        const logs = `${result.stdout}\n${result.stderr}`;
        return logs.split("\n").some((line) =>
          line.includes("server_key_representation") && line.includes(representation)
        );
      }, 300);
    }
  }
};

const assertGreenWorkersParked = async () => {
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const activated = await sqlScalar(
      coprocessorDatabaseName(operator),
      `SELECT EXISTS (
         SELECT 1
           FROM upgrade_state
          WHERE stack_role = 'GCS'
            AND (state = 'DryRunStarted' OR gw_dry_run_started)
       )::int;`,
    );
    if (activated !== "0") {
      throw new Error(`operator ${operator} activated Green before compressed-material readiness`);
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

type MigratedStack = {
  continuityContract: string;
  preMigrationHandles: string[];
};

/** Reconstructs the migrated 0.15 state proven by the preceding rollout PR. */
export const reconstructMigrated015Fixture = async (
  ctx: RolloutRunContext,
): Promise<MigratedStack> => {
  const versions = migrationVersions();
  const baselineLock = await ctx.resolveVersionLock("rfc029-00-baseline", {
    versions: versions.baseline,
    sources: versionSources,
  });
  const targetSnapshotLock = await ctx.resolveVersionLock("rfc029-target-snapshot", {
    versions: {},
    sources: versionSources,
  });
  const baselineSnapshot = (await Bun.file(baselineLock).json()) as { env: Record<string, string> };
  const targetSnapshot = (await Bun.file(targetSnapshotLock).json()) as { env: Record<string, string> };
  const phaseVersions = migrationPhaseVersions(baselineSnapshot.env, targetSnapshot.env, versions.blueTag);
  const contractLock = await ctx.writeVersionLock("rfc029-01-contract", {
    versions: phaseVersions.contract,
    sources: versionSources,
  });
  const connectorLock = await ctx.writeVersionLock("rfc029-02-kms", {
    versions: phaseVersions.connector,
    sources: versionSources,
  });
  const blueLock = await ctx.writeVersionLock("rfc029-03-blue", {
    versions: phaseVersions.blue,
    sources: versionSources,
  });
  const scenario = path.join(ctx.stateDir(), "rollout", "rfc029-adoption.yaml");
  await Bun.write(scenario, adoptionScenario(versions.baselineTag));

  logPhase("00 restore production-style legacy state before the 0.15 rollout");
  await ctx.up({
    lockFile: baselineLock,
    scenario,
    overrides: [{ group: "test-suite" }],
  });
  await assertGreenAbsent();
  await assertActiveImageTag(versions.baselineTag);
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
  const continuityOutput = await runKeyContinuity("prepare");
  const continuityContract = continuityOutput.match(/RFC029_KEY_CONTINUITY_CONTRACT=(0x[0-9a-fA-F]{40})/)?.[1];
  if (!continuityContract) {
    throw new Error("pre-migration key continuity probe did not return its contract address");
  }
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

  const originalKeyStates = await Promise.all(
    Array.from({ length: OPERATOR_COUNT }, (_, operator) =>
      sqlScalar(coprocessorDatabaseName(operator), activeKeyStateSql),
    ),
  );

  logPhase("01 upgrade KMSGeneration to 0.15 without invoking migration");
  await upgradeKmsGeneration(ctx, contractLock);
  const state = await ctx.readState();
  const hostKey = defaultHostChainKey(state.scenario.hostChains);
  const hostChain = state.scenario.hostChains.find((chain) => chain.key === hostKey);
  if (!hostChain) {
    throw new Error(`default host chain ${hostKey} is missing from the rollout scenario`);
  }
  const hostChainId = hostChain.chainId;
  const hostRpc = hostReachableRpcUrl(state.discovery!.endpoints.hosts[hostKey]!.http);
  const currentHostBlock = async () =>
    Number((await run(["cast", "block-number", "--rpc-url", hostRpc])).stdout.trim());
  const mineHostBlock = async () => {
    await run(["cast", "rpc", "--rpc-url", hostRpc, "evm_mine"]);
    return currentHostBlock();
  };
  const deploymentBoundary = await currentHostBlock();

  logPhase("02 upgrade one KMS operator to 0.15 and prove a mixed fleet blocks the request");
  const baselineConnectorImages = (await connectorObservation(1)).images;
  await ctx.upgradeKmsOperators([1], {
    lockFile: connectorLock,
    overrides: [{ group: "kms-connector" }],
  });
  await assertKmsCoreVersions([1], phaseVersions.connector.CORE_VERSION!);
  const mixedConnectorImages = (await connectorObservation(1)).images;
  assertLocalConnectorUpgrade(baselineConnectorImages, mixedConnectorImages);
  const firstOperatorBoundary = await mineHostBlock();
  await waitForConnectorBoundary(1, firstOperatorBoundary, mixedConnectorImages);
  let mixedBlocked = false;
  try {
    assertConnectorMigrationReady(await connectorObservations(), deploymentBoundary, mixedConnectorImages);
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

  logPhase("03 upgrade every remaining KMS operator to 0.15 and establish listener boundaries");
  for (const operator of [2, 3, 4]) {
    await ctx.upgradeKmsOperators([operator], {
      lockFile: connectorLock,
      overrides: [{ group: "kms-connector" }],
    });
    const operatorBoundary = await mineHostBlock();
    await waitForConnectorBoundary(operator, operatorBoundary, mixedConnectorImages);
  }
  await assertKmsCoreVersions([1, 2, 3, 4], phaseVersions.connector.CORE_VERSION!);
  const expectedConnectorImages = (await connectorObservation(1)).images;
  assertLocalConnectorUpgrade(baselineConnectorImages, expectedConnectorImages);
  const migrationBoundary = await mineHostBlock();
  await waitUntil("all connector listeners reached the migration boundary", async () => {
    try {
      assertConnectorMigrationReady(await connectorObservations(), migrationBoundary, expectedConnectorImages);
      return true;
    } catch {
      return false;
    }
  }, 300);

  logPhase("04 reconstruct active 0.15 workers and pin them to legacy material");
  await ctx.upgradeRuntimeGroup("coprocessor", {
    lockFile: blueLock,
    bcsTag: versions.blueTag,
    bcsCompatTag: "v0.15.0",
  });
  await assertActiveSafeguard();
  await assertActiveImageTag(versions.blueTag);
  await assertGreenAbsent();
  await ctx.test("input-proof-compute-decrypt", { parallel: false });

  logPhase("05 request compressed material for the existing active Test key");
  const keyIdHex = await sqlScalar(
    coprocessorDatabaseName(0),
    "SELECT encode(key_id, 'hex') FROM keys ORDER BY sequence_number DESC LIMIT 1;",
  );
  const keyId = BigInt(`0x${keyIdHex}`).toString();
  const legacyKmsDigests = await kmsPublicMaterialDigests("ServerKey", keyIdHex);
  const migrationRequestBoundary = await currentHostBlock();
  const paramsType = state.scenario.kms.fheParams === "Test" ? 1 : 0;
  await ctx.runHostContractTask(
    `npx hardhat task:triggerKeygen --params-type ${paramsType} --existing-key-id ${keyId} --use-internal-proxy-address true`,
  );

  logPhase("06 wait until KMS and every active operator expose identical material");
  await waitUntil(
    "every KMS operator serves both representations under the original key ID",
    async () => {
      try {
        const [legacy, compressed] = await Promise.all([
          kmsPublicMaterialDigests("ServerKey", keyIdHex),
          kmsPublicMaterialDigests("CompressedXofKeySet", keyIdHex),
        ]);
        return (
          legacy.every((digest, index) => digest === legacyKmsDigests[index]) &&
          new Set(compressed).size === 1
        );
      } catch {
        return false;
      }
    },
  );
  let materialRows: OperatorMaterial[] = [];
  await waitUntil("all operators applied identical compressed material", async () => {
    const rows = await Promise.all(
      Array.from({ length: OPERATOR_COUNT }, (_, operator) =>
        operatorMaterial(operator, keyIdHex, hostChainId, migrationRequestBoundary),
      ),
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
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const migrated = await sqlScalar(coprocessorDatabaseName(operator), activeKeyStateSql);
    if (migrated !== originalKeyStates[operator]) {
      throw new Error(`operator ${operator} changed existing key identity or legacy bytes: ${migrated}`);
    }
    if (materialRows[operator]?.keyId !== migrated.split("|")[0]) {
      throw new Error(`operator ${operator} applied material to a different key ID`);
    }
  }
  await assertActiveSafeguard();
  await assertGreenAbsent();

  // Publication is passive: the active workers must still compute successfully after every
  // database contains the compressed representation, including after restarts.
  const activeRestartedAt = new Date().toISOString();
  await restartActiveKeyWorkers();
  await assertActiveSafeguard();
  await ctx.test("input-proof-compute-decrypt", { parallel: false });
  await assertWorkerRepresentation("Active", "legacy", activeRestartedAt);

  return { continuityContract, preMigrationHandles };
};

export default async function runMigrationAndAdoption(ctx: RolloutRunContext) {
  const { continuityContract, preMigrationHandles } = await reconstructMigrated015Fixture(ctx);

  logPhase("07 deploy 0.15.1 Green only after every Blue database has applied material");
  const greenStartedAt = new Date().toISOString();
  await ctx.startDeferredGreen();
  await assertActiveSafeguard();
  await assertGreenSafeguard();
  await assertGreenWorkersParked();
  await ctx.test("input-proof-compute-decrypt", { parallel: false });

  logPhase("08 run the existing blue-green failure, retry, unanimity, and cutover battery");
  await ctx.test("blue-green", { parallel: false });
  await assertWorkerRepresentation("Green", "compressed-xof", greenStartedAt);

  logPhase("09 verify preserved history and post-cutover protocol paths");
  await runKeyContinuity("reuse", continuityContract);
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
