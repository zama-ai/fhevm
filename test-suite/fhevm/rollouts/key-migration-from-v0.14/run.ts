/**
 * Upgrades a 0.14 stack to this branch one step at a time, with checks in between, then migrates
 * the existing key to compressed material and reshares it. Runs as the `key-migration-from-v0.14`
 * test profile on the `key-migration-from-v0.14` scenario, or its `-gpu` variant for the GPU phase.
 */
import { createHash } from "node:crypto";
import path from "node:path";
import YAML from "yaml";

import { runRetainedMaterial } from "../../src/consensus/retained-material-run";
import { withRolloutSupervisor } from "../../src/consensus/rollout-supervision";
import { type ContextAndEpoch, parseContextAndEpoch } from "../../src/commands/kms-context-switch";
import { resolveRolloutSolidityRef } from "../../src/commands/rollout-solidity";
import type { Stack } from "../../src/commands/stack";
import { castCall, resolveKmsGenerationTarget, waitForContainer } from "../../src/flow/readiness";
import {
  COPROCESSOR_DB_CONTAINER,
  DEFAULT_POSTGRES_PASSWORD,
  DEFAULT_POSTGRES_USER,
  coprocessorDatabaseName,
  defaultHostChainKey,
  OBJECT_STORE_EXTERNAL_URL,
  REPO_ROOT,
} from "../../src/layout";
import {
  kmsConnectorDbName,
  kmsConnectorPrefix,
  kmsPartyIds,
  kmsPublicPrefix,
} from "../../src/kms-party";
import { hostReachableRpcUrl, uint256ToId } from "../../src/utils/fs";
import { run, runStreaming } from "../../src/utils/process";
import {
  type ConnectorObservation,
  type OperatorMaterial,
  assertConnectorMigrationReady,
  assertConnectorUpgraded,
  assertOperatorMaterialAgreement,
} from "./checks";
import { withHeldRecipient } from "./held-recipient";

const CONNECTOR_PARTIES = 4;
const OPERATOR_COUNT = 2;
const CONNECTOR_SERVICES = ["db-migration", "gw-listener", "kms-worker", "tx-sender"] as const;
const KEY_WORKERS = ["tfhe-worker", "zkproof-worker", "sns-worker"] as const;
const EAGER_KEY_WORKERS = ["zkproof-worker", "sns-worker"] as const;
const ACTIVE_MIGRATION_SERVICES = ["host-listener", "host-listener-poller", "host-listener-consumer", ...KEY_WORKERS];

const logPhase = (label: string) => console.log(`\n[key migration] ${label}`);

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

// Hash large-object pages in order: Default SNS keys can exceed PostgreSQL's
// single-bytea read limit. Include page numbers to detect sparse-page changes.
// The zero-length read validates the OID, preserving errors for missing objects.
export const activeKeyStateSql =
  "SELECT encode(key_id, 'hex') || '|' || encode(key_id_gw, 'hex') || '|' || md5(sks_key) || '|' || " +
  "md5(pks_key) || '|' || COALESCE(md5(cks_key), 'NULL') || '|' || " +
  "COALESCE(md5(encode(lo_get(sns_pk, 0, 0), 'hex') || COALESCE((SELECT " +
  "string_agg(pageno::text || ':' || md5(data), ',' ORDER BY pageno) " +
  "FROM pg_largeobject WHERE loid = sns_pk), '')), 'NULL') || '|' || " +
  "sequence_number || '|' || COALESCE(chain_id::text, 'NULL') || '|' || " +
  "COALESCE(encode(block_hash, 'hex'), 'NULL') || '|' || count(*) OVER () " +
  "FROM keys ORDER BY sequence_number DESC LIMIT 1;";

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
    } catch (error) {
      if (error instanceof Error && error.message.includes("listener cursor is before deployment block")) {
        return false;
      }
      throw error;
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

function kmsPublicMaterialDigests(
  type: "ServerKey" | "CompressedXofKeySet",
  keyId: string,
): Promise<string[]>;
function kmsPublicMaterialDigests(
  type: "CompressedXofKeySet",
  keyId: string,
  missingIsPending: true,
): Promise<string[] | undefined>;
async function kmsPublicMaterialDigests(
  type: "ServerKey" | "CompressedXofKeySet",
  keyId: string,
  missingIsPending = false,
): Promise<string[] | undefined> {
  const responses = await Promise.all(
    kmsPartyIds(CONNECTOR_PARTIES).map(async (party) => {
      const response = await fetch(
        `${OBJECT_STORE_EXTERNAL_URL}/kms-public/${kmsPublicPrefix(party)}/${type}/${keyId}`,
      );
      return { party, response };
    }),
  );
  const failures = responses.filter(({ response }) => !response.ok);
  if (failures.length) {
    if (missingIsPending && failures.every(({ response }) => response.status === 404)) {
      return undefined;
    }
    throw new Error(
      failures
        .map(({ party, response }) => `KMS operator ${party} does not serve ${type}/${keyId}: HTTP ${response.status}`)
        .join("; "),
    );
  }
  return Promise.all(
    responses.map(async ({ response }) =>
      createHash("sha256").update(Buffer.from(await response.arrayBuffer())).digest("hex")
    ),
  );
}

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

const assertGreenForcedLegacy = async () => {
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const prefix = operator === 0 ? "coprocessor-" : `coprocessor${operator}-`;
    for (const worker of KEY_WORKERS) {
      const green = `${prefix}gcs-${worker}`;
      if (!(await forcesLegacyServerKey(green))) {
        throw new Error(`${green} must receive the force-legacy safeguard`);
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

const restartActiveKeyWorkers = async (role: "Active" | "Green") => {
  const workers = Array.from({ length: OPERATOR_COUNT }, (_, operator) => {
    const prefix = operator === 0 ? "coprocessor-" : `coprocessor${operator}-`;
    return KEY_WORKERS.map((worker) => `${prefix}${role === "Green" ? "gcs-" : ""}${worker}`);
  }).flat();
  await runStreaming(["docker", "restart", ...workers]);
  for (const worker of workers) {
    await waitForContainer(worker, "running");
  }
};

const assertWorkerRepresentation = async (
  role: "Active" | "Green",
  workers: readonly (typeof KEY_WORKERS)[number][],
  representation: "legacy" | "compressed-xof",
  since: string,
) => {
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const prefix = operator === 0 ? "coprocessor-" : `coprocessor${operator}-`;
    for (const worker of workers) {
      const container = `${prefix}${role === "Green" ? "gcs-" : ""}${worker}`;
      await waitUntil(`${container} loaded ${representation}`, async () => {
        const result = await run(["docker", "logs", "--since", since, container]);
        const logs = `${result.stdout}\n${result.stderr}`;
        const representationLines = logs
          .split("\n")
          .filter((line) => line.includes("server_key_representation"));
        if (representationLines.length === 0) return false;
        if (representationLines.some((line) => !line.includes(representation))) {
          throw new Error(`${container} loaded the wrong server key representation`);
        }
        return true;
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

/** Fails fast on a stack this test cannot drive, and returns what it needs from the scenario. */
const keyMigrationScenario = async (stack: Stack) => {
  const state = await stack.readState();
  const scenario = state.scenario;
  if (
    scenario.kind !== "blue-green" ||
    scenario.bootstrap?.upgrade !== "deferred" ||
    scenario.bcs.source.mode !== "registry" ||
    scenario.topology.count !== OPERATOR_COUNT ||
    scenario.kms.parties !== CONNECTOR_PARTIES ||
    !state.bootstrapPending ||
    state.bootstrapPending.completed?.length
  ) {
    throw new Error("key-migration-from-v0.14 needs a fresh `up --scenario key-migration-from-v0.14` (or its -gpu variant)");
  }
  // Green runs with the production chart's serving mode: legacy material until the operator flips it.
  const chart = YAML.parse(await Bun.file(path.join(REPO_ROOT, "charts/coprocessor/values.yaml")).text());
  if (chart?.commonConfig?.forceLegacyServerKey !== true || scenario.gcs.env.FORCE_LEGACY_SERVER_KEY !== "true") {
    throw new Error("Green must mirror the coprocessor chart's commonConfig.forceLegacyServerKey, which this test expects to be true");
  }
  return {
    state,
    baselineTag: scenario.bootstrap.tag,
    blueTag: scenario.bcs.source.tag,
    targetCoreVersion: state.bootstrapPending.target.env.CORE_VERSION!,
  };
};

export default async function keyMigration(stack: Stack) {
  const migrationFault = process.env.RFC029_MIGRATION_FAULT ?? "none";
  if (!["none", "lagging-recipient", "application-interruption", "download-interrupt", "download-wrong-digest", "download-malformed", "download-wrong-key"].includes(migrationFault)) throw new Error(`unsupported migration fault ${migrationFault}`);
  const { state, baselineTag, blueTag, targetCoreVersion } = await keyMigrationScenario(stack);
  // Threshold Test keys use drift noise reduction, which CUDA does not support, so the GPU
  // scenario generates the baseline key with the Default parameters.
  const gpuContinuation = state.scenario.kms.fheParams === "Default";
  if (gpuContinuation && !process.env.RFC029_GPU_IMAGES) throw new Error("the GPU phase requires the package-migration image receipt in RFC029_GPU_IMAGES");
  // Run the same baseline-compatible dApps throughout the migration. Current
  // Solidity can call executor methods that did not exist before the upgrade.
  const solidityLibraryRevision = await resolveRolloutSolidityRef(baselineTag);
  const migrationTest = (profile: string) => stack.test(profile, { solidityLibraryRevision });

  logPhase(`00 check the 0.14 stack and its legacy ${gpuContinuation ? "Default" : "Test"} key`);
  await assertGreenAbsent();
  await assertActiveImageTag(blueTag);
  for (let operator = 0; operator < OPERATOR_COUNT; operator += 1) {
    const keyState = await sqlScalar(
      coprocessorDatabaseName(operator),
      "SELECT (sks_key IS NOT NULL)::int || '|' || " +
        "(compressed_xof_keyset IS NOT NULL)::int FROM keys ORDER BY sequence_number DESC LIMIT 1;",
    );
    if (keyState !== "1|0") {
      throw new Error(`operator ${operator} did not start with legacy-only key material: ${keyState}`);
    }
  }
  await migrationTest("input-proof-compute-decrypt");

  await runRetainedMaterial(await stack.readState(), "seed", "release-migration");

  const originalKeyStates = await Promise.all(
    Array.from({ length: OPERATOR_COUNT }, (_, operator) =>
      sqlScalar(coprocessorDatabaseName(operator), activeKeyStateSql),
    ),
  );

  logPhase("01 upgrade every Gateway and Host contract without invoking migration");
  await stack.upgrade("contracts");
  await migrationTest("input-proof-compute-decrypt");
  await migrationTest("public-decryption");
  await migrationTest("user-decryption");

  logPhase("02 upgrade Relayer after contracts and before runtime consumers");
  await stack.upgrade("relayer");
  await migrationTest("input-proof-compute-decrypt");
  const hostKey = defaultHostChainKey(state.scenario.hostChains);
  const hostChain = state.scenario.hostChains.find((chain) => chain.key === hostKey);
  if (!hostChain) {
    throw new Error(`default host chain ${hostKey} is missing from the scenario`);
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

  logPhase("03 upgrade one KMS Core and Connector pair and prove a mixed fleet blocks migration");
  // This scenario creates one context/epoch and performs no rotation before the upgrade, so every
  // party must hold the material of the active one, which the upgrade hands to the 0.15 core.
  const target = resolveKmsGenerationTarget(state);
  if (!target.configAddress) throw new Error("KMS migration requires ProtocolConfig discovery");
  const active = parseContextAndEpoch(await castCall(target.rpcUrl, target.configAddress, "getCurrentKmsContextAndEpoch()(uint256,uint256)"));
  for (const party of kmsPartyIds(CONNECTOR_PARTIES)) {
    for (const [type, id] of [["Context", uint256ToId(active.contextId)], ["PrssSetupCombined", uint256ToId(active.epochId)]]) {
      const response = await fetch(`${OBJECT_STORE_EXTERNAL_URL}/kms-public/PRIV-p${party}/${type}/${id}`, { method: "HEAD", signal: AbortSignal.timeout(30_000) });
      if (!response.ok) throw new Error(`KMS party ${party} lacks the on-chain ${type} ${id}: HTTP ${response.status}`);
    }
  }
  const baselineConnectorImages = (await connectorObservation(1)).images;
  await stack.upgrade("kms-operator:1");
  await assertKmsCoreVersions([1], targetCoreVersion);
  const mixedConnectorImages = (await connectorObservation(1)).images;
  assertConnectorUpgraded(baselineConnectorImages, mixedConnectorImages);
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

  logPhase("04 upgrade every remaining KMS operator to 0.15 and establish listener boundaries");
  for (const operator of [2, 3, 4] as const) {
    await stack.upgrade(`kms-operator:${operator}`);
    const operatorBoundary = await mineHostBlock();
    await waitForConnectorBoundary(operator, operatorBoundary, mixedConnectorImages);
  }
  await assertKmsCoreVersions([1, 2, 3, 4], targetCoreVersion);
  const expectedConnectorImages = (await connectorObservation(1)).images;
  assertConnectorUpgraded(baselineConnectorImages, expectedConnectorImages);
  const migrationBoundary = await mineHostBlock();
  await waitUntil("all connector listeners reached the migration boundary", async () => {
    try {
      assertConnectorMigrationReady(await connectorObservations(), migrationBoundary, expectedConnectorImages);
      return true;
    } catch (error) {
      if (error instanceof Error && error.message.includes("listener cursor is before deployment block")) {
        return false;
      }
      throw error;
    }
  }, 300);
  await migrationTest("input-proof-compute-decrypt");
  await migrationTest("public-decryption");
  await migrationTest("user-decryption");

  logPhase("05 upgrade listener-core before the 0.15 coprocessor fleet");
  await stack.upgrade("listener-core");

  logPhase("06 start 0.15 Green, still forced to use the legacy representation");
  await stack.upgrade("green");
  await assertGreenForcedLegacy();
  await assertGreenWorkersParked();

  logPhase("07 cut over from 0.14 Blue to 0.15 Green through the existing Blue/Green flow");
  await migrationTest("blue-green");

  if (process.env.RFC029_RETIRED_WRITERS === "1") {
    logPhase("07b reject retired compute and publication with original prepared work");
    const env = { RETIRED_OPERATOR_COUNT: String(OPERATOR_COUNT) };
    await withRolloutSupervisor(stack.stateDir(), "hold-retired-writers.sh", ["sns-worker"], env, async () => {
      await runRetainedMaterial(await stack.readState(), "retired-sns", "release-migration");
      await withRolloutSupervisor(stack.stateDir(), "hold-retired-writers.sh", ["tfhe-worker"], env, async () => {
        await runRetainedMaterial(await stack.readState(), "retired-tfhe", "release-migration");
        for (let checkpoint = 0; checkpoint < 3; checkpoint++) {
          await runRetainedMaterial(await stack.readState(), "retired-check", "release-migration");
          if (checkpoint < 2) await Bun.sleep(15_000);
        }
      });
    });
    await runRetainedMaterial(await stack.readState(), "retired-recover", "release-migration");
  }

  logPhase("08 request compressed material for the existing active key");
  const keyIdHex = await sqlScalar(
    coprocessorDatabaseName(0),
    "SELECT encode(key_id, 'hex') FROM keys ORDER BY sequence_number DESC LIMIT 1;",
  );
  const keyId = BigInt(`0x${keyIdHex}`).toString();
  const legacyKmsDigests = await kmsPublicMaterialDigests("ServerKey", keyIdHex);
  if (new Set(legacyKmsDigests).size !== 1) {
    throw new Error("KMS operators disagree on the existing ServerKey material before migration");
  }
  const migrationRequestBoundary = await currentHostBlock();
  const paramsType = state.scenario.kms.fheParams === "Test" ? 1 : 0;
  const generateMaterial = async () => {
    await stack.runHostContractTask(
      `npx hardhat task:triggerKeygen --params-type ${paramsType} --existing-key-id ${keyId} --use-internal-proxy-address true`,
    );

    logPhase("09 wait until KMS and every 0.15 operator expose identical material");
    await waitUntil(
      "every KMS operator serves both representations under the original key ID",
      async () => {
        const legacy = await kmsPublicMaterialDigests("ServerKey", keyIdHex);
        if (new Set(legacy).size !== 1) {
          throw new Error("KMS operators serve different ServerKey material");
        }
        if (!legacy.every((digest, index) => digest === legacyKmsDigests[index])) {
          throw new Error("KMS migration changed the existing ServerKey material");
        }
        const compressed = await kmsPublicMaterialDigests("CompressedXofKeySet", keyIdHex, true);
        if (!compressed) return false;
        if (new Set(compressed).size !== 1) {
          throw new Error("KMS operators serve different CompressedXofKeySet material");
        }
        return true;
      },
    );
    if (migrationFault === "lagging-recipient") {
      await waitUntil("uninterrupted operator applied the selected migration", async () =>
        Boolean(await operatorMaterial(0, keyIdHex, hostChainId, migrationRequestBoundary)));
      if (await operatorMaterial(1, keyIdHex, hostChainId, migrationRequestBoundary)) {
        throw new Error("held recipient claimed activation before its listener resumed");
      }
      const lagging = await sqlScalar(coprocessorDatabaseName(1),
        "SELECT (compressed_xof_keyset IS NOT NULL)::int FROM keys ORDER BY sequence_number DESC LIMIT 1;");
      if (lagging !== "0") throw new Error("lagging recipient acquired compressed material during the observed outage");
      console.log("[KEY-02 lagging arm] generation and operator 0 activation completed while operator 1 retained legacy-only material");
    }
  };
  if (migrationFault !== "none") {
    const names = (await run(["docker", "ps", "-a", "--format", "{{.Names}}"])).stdout.trim().split(/\r?\n/)
      .filter(name => /^coprocessor1-gcs-host-listener(?:-[a-zA-Z0-9-]+)?$/.test(name));
    if (names.length < 3) throw new Error("cannot identify every live listener role for the held recipient");
    if (migrationFault === "lagging-recipient") await withHeldRecipient(stack.stateDir(), names, generateMaterial);
    else if (migrationFault.startsWith("download-")) await withRolloutSupervisor(stack.stateDir(), "hold-key-download.sh", names, {
      MIGRATION_KEY_HEX: keyIdHex, MIGRATION_DATABASE: coprocessorDatabaseName(1),
      MIGRATION_DOWNLOAD_MODE: migrationFault.slice("download-".length),
      POSTGRES_PASSWORD: DEFAULT_POSTGRES_PASSWORD,
    }, generateMaterial);
    else await withRolloutSupervisor(stack.stateDir(), "interrupt-key-application.sh", names, {
      MIGRATION_KEY_HEX: keyIdHex, MIGRATION_DATABASE: coprocessorDatabaseName(1),
      POSTGRES_PASSWORD: DEFAULT_POSTGRES_PASSWORD,
    }, generateMaterial);
  } else await generateMaterial();
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
    if (materialRows[operator]?.existingKeyId !== migrated.split("|")[0]) {
      throw new Error(`operator ${operator} applied material to a different key ID`);
    }
  }
  await assertGreenForcedLegacy();

  // The 0.15 release publishes compressed material but keeps serving with the legacy representation.
  const activeRestartedAt = new Date().toISOString();
  await restartActiveKeyWorkers("Green");
  await assertGreenForcedLegacy();
  await assertWorkerRepresentation("Green", EAGER_KEY_WORKERS, "legacy", activeRestartedAt);
  await migrationTest("input-proof-compute-decrypt");
  await assertWorkerRepresentation("Green", ["tfhe-worker"], "legacy", activeRestartedAt);

  logPhase("10 verify normal 0.15 encryption and decryption remain functional");
  await migrationTest("public-decryption");
  await migrationTest("user-decryption");
  await runRetainedMaterial(await stack.readState(), "verify", "release-migration");
  if (gpuContinuation) {
    logPhase("11 consume migrated compressed keys on GPU, then repeat after a worker restart");
    const since = new Date().toISOString();
    await withRolloutSupervisor(stack.stateDir(), "hold-migration-gpu.sh", [], {
      RFC029_GPU_IMAGES: process.env.RFC029_GPU_IMAGES!,
    }, async () => {
      for (const checkpoint of ["initial load", "restart"]) {
        const boundary = checkpoint === "initial load" ? since : new Date().toISOString();
        if (checkpoint === "restart") await restartActiveKeyWorkers("Green");
        await runRetainedMaterial(await stack.readState(), "verify", "release-migration", true);
        await assertWorkerRepresentation("Green", KEY_WORKERS, "compressed-xof", boundary);
        for (let operator = 0; operator < OPERATOR_COUNT; operator++) {
          if (await sqlScalar(coprocessorDatabaseName(operator), activeKeyStateSql) !== originalKeyStates[operator]) {
            throw new Error("GPU serving changed original key identity or legacy material");
          }
        }
      }
    });
    await assertGreenForcedLegacy();
  }

  logPhase("12 reshare the migrated key through an epoch rotation, then a context switch");
  // Each KMS node checks the getKeyInfo record the connector supplies against the migrated record it holds.
  const readKmsEpoch = async () =>
    parseContextAndEpoch(await castCall(target.rpcUrl, target.configAddress!, "getCurrentKmsContextAndEpoch()(uint256,uint256)"));
  const reshare = async (
    label: string,
    task: string,
    activated: (before: ContextAndEpoch, current: ContextAndEpoch) => boolean,
    afterBroadcast?: (before: ContextAndEpoch) => Promise<void>,
  ) => {
    const before = await readKmsEpoch();
    await stack.runHostContractTask(`npx hardhat ${task} --use-internal-proxy-address true`);
    await afterBroadcast?.(before);
    await waitUntil(`the ${label} reshared the migrated key and activated on chain`, async () =>
      activated(before, await readKmsEpoch()), 600);
    await migrationTest("public-decryption");
    await migrationTest("user-decryption");
    await runRetainedMaterial(await stack.readState(), "verify", "release-migration");
  };
  await reshare("epoch rotation", "task:defineNewEpochForCurrentKmsContext", (before, current) =>
    current.contextId === before.contextId && current.epochId !== before.epochId);
  await reshare(
    "context switch",
    "task:defineNewKmsContextAndEpoch",
    (before, current) => current.contextId !== before.contextId,
    // The gateway only accepts decryptions for KMS contexts registered on GatewayConfig.
    (before) =>
      stack.runGatewayContractTask("npx hardhat task:updateKmsContext --use-internal-proxy-address true", {
        env: { KMS_CONTEXT_ID: (before.contextId + 1n).toString() },
      }),
  );
}
