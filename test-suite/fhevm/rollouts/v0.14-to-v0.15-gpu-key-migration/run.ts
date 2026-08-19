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
} from "./checks";
import { migrationPhaseVersions, migrationVersions, versionSources } from "./versions";

const CONNECTOR_PARTIES = 4;
const OPERATOR_COUNT = 2;
const CONNECTOR_SERVICES = ["db-migration", "gw-listener", "kms-worker", "tx-sender"] as const;
const KEY_WORKERS = ["tfhe-worker", "zkproof-worker", "sns-worker"] as const;
const EAGER_KEY_WORKERS = ["zkproof-worker", "sns-worker"] as const;
const ACTIVE_MIGRATION_SERVICES = ["host-listener", "host-listener-poller", "host-listener-consumer", ...KEY_WORKERS];

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

export const migrationScenario = (baselineTag: string) => `version: 1
kind: blue-green
name: RFC 029 0.14 to 0.15 key migration
${hostChains}
topology:
  count: ${OPERATOR_COUNT}
  threshold: ${OPERATOR_COUNT}
bcs:
  source:
    mode: registry
    tag: ${JSON.stringify(baselineTag)}
gcs:
  source:
    mode: local
  stackVersion: "0.15.0"
  deferredStart: true
  env:
    FORCE_LEGACY_SERVER_KEY: "true"
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
        `${MINIO_EXTERNAL_URL}/kms-public/${kmsPublicPrefix(party)}/${type}/${keyId}`,
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

const contractUpgradeCommand = (task: string, contract: string) =>
  [
    `npx hardhat ${task}`,
    `--current-implementation previous-contracts/${contract}.sol:${contract}`,
    `--new-implementation contracts/${contract}.sol:${contract}`,
    "--verify-contract false",
    "--use-internal-proxy-address true",
  ].join(" ");

export const gatewayContractUpgradePlan = [
  ["task:upgradeDecryption", "Decryption"],
  ["task:upgradeCiphertextCommits", "CiphertextCommits"],
  ["task:upgradeInputVerification", "InputVerification"],
  ["task:upgradeGatewayConfig", "GatewayConfig"],
] as const;

export const hostContractUpgradePlan = [["task:upgradeKMSGeneration", "KMSGeneration"]] as const;

const upgradeContracts = async (ctx: RolloutRunContext, targetLock: string) => {
  await ctx.snapshotContracts("gateway");
  await ctx.snapshotContracts("host");
  await ctx.applyVersionLock("RFC 029 contract sources", {
    lockFile: targetLock,
    allowedVersionKeys: ["GATEWAY_VERSION", "HOST_VERSION"],
    overrides: [{ group: "gateway-contracts" }, { group: "host-contracts" }],
  });

  // Upgrade consumers of the removed priority-coprocessor getters before GatewayConfig removes them.
  for (const [task, contract] of gatewayContractUpgradePlan) {
    await ctx.runGatewayContractTask(contractUpgradeCommand(task, contract));
  }
  for (const [task, contract] of hostContractUpgradePlan) {
    await ctx.runHostContractTask(contractUpgradeCommand(task, contract));
  }
};

export default async function runMigration(ctx: RolloutRunContext) {
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
  const phaseVersions = migrationPhaseVersions(baselineSnapshot.env, targetSnapshot.env);
  const contractLock = await ctx.writeVersionLock("rfc029-01-contract", {
    versions: phaseVersions.contract,
    sources: versionSources,
  });
  const relayerLock = await ctx.writeVersionLock("rfc029-02-relayer", {
    versions: phaseVersions.relayer,
    sources: versionSources,
  });
  const connectorLock = await ctx.writeVersionLock("rfc029-03-kms", {
    versions: phaseVersions.connector,
    sources: versionSources,
  });
  const listenerCoreLock = await ctx.writeVersionLock("rfc029-04-listener-core", {
    versions: phaseVersions.listenerCore,
    sources: versionSources,
  });
  const scenario = path.join(ctx.stateDir(), "rollout", "rfc029-v014-to-v015.yaml");
  await Bun.write(scenario, migrationScenario(versions.baselineTag));

  logPhase("00 boot 0.14 and generate the legacy Test key");
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

  const originalKeyStates = await Promise.all(
    Array.from({ length: OPERATOR_COUNT }, (_, operator) =>
      sqlScalar(coprocessorDatabaseName(operator), activeKeyStateSql),
    ),
  );

  logPhase("01 upgrade every changed Gateway and Host contract without invoking migration");
  await upgradeContracts(ctx, contractLock);
  await ctx.test("input-proof-compute-decrypt", { parallel: false });
  await ctx.test("public-decryption", { parallel: false });
  await ctx.test("user-decryption", { parallel: false });

  logPhase("02 upgrade Relayer after contracts and before runtime consumers");
  await ctx.upgradeRuntimeGroup("relayer", { lockFile: relayerLock });
  await ctx.test("input-proof-compute-decrypt", { parallel: false });
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

  logPhase("03 upgrade one KMS Core and Connector pair and prove a mixed fleet blocks migration");
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

  logPhase("04 upgrade every remaining KMS operator to 0.15 and establish listener boundaries");
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
    } catch (error) {
      if (error instanceof Error && error.message.includes("listener cursor is before deployment block")) {
        return false;
      }
      throw error;
    }
  }, 300);
  await ctx.test("input-proof-compute-decrypt", { parallel: false });
  await ctx.test("public-decryption", { parallel: false });
  await ctx.test("user-decryption", { parallel: false });

  logPhase("05 upgrade listener-core before the 0.15 coprocessor fleet");
  await ctx.upgradeRuntimeGroup("listener-core", { lockFile: listenerCoreLock });

  logPhase("06 start 0.15 Green, still forced to use the legacy representation");
  await ctx.startDeferredGreen();
  await assertGreenForcedLegacy();
  await assertGreenWorkersParked();

  logPhase("07 cut over from 0.14 Blue to 0.15 Green through the existing Blue/Green flow");
  await ctx.test("blue-green", { parallel: false });

  logPhase("08 request compressed material for the existing active Test key");
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
  await ctx.runHostContractTask(
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
  await ctx.test("input-proof-compute-decrypt", { parallel: false });
  await assertWorkerRepresentation("Green", ["tfhe-worker"], "legacy", activeRestartedAt);

  logPhase("10 verify normal 0.15 encryption and decryption remain functional");
  await ctx.test("public-decryption", { parallel: false });
  await ctx.test("user-decryption", { parallel: false });
}
