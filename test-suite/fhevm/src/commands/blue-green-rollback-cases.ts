/**
 * Rollback cases for the blue-green profile.
 *
 * Each case perturbs one operator, lets the attempt end, and then asserts that
 * every operator ended on the same version. They run before the successful
 * cutover, because the cutover moves `versioning` and merges the GCS schema
 * away. Each rollback leaves its rows at `failed`, which is what the listener
 * needs before it accepts the next proposal.
 *
 * The cases run with synthetic ops on, which the profile re-enables before it
 * calls them. The GCS host-listener then injects non-trivial work into each
 * window, so a healthy operator anchors and the rollback comes from the
 * perturbation. A quiet window would roll back on its own and prove nothing.
 *
 * They drive no client traffic of their own. Case 2 takes a whole operator out,
 * and on a two-of-two fleet that alone fails every input proof, so traffic
 * there would fail the case for a reason the case is not about.
 */
import { PreflightError } from "../errors";
import { OBJECT_STORE_EXTERNAL_URL, PROJECT, composePath, objectStoreOperatorHost } from "../layout";
import { composeEnv, run } from "../utils/process";

export type GatewayTips = { tipBefore: number; tipAfter: number };

export type RollbackCaseContext = {
  operatorDatabases: string[];
  hostChainCount: number;
  gcsStackVersion: string;
  psqlQuery: (database: string, query: string) => Promise<string>;
  waitUntil: (opts: {
    label: string;
    timeoutSecs: number;
    intervalMs?: number;
    check: () => Promise<boolean>;
  }) => Promise<void>;
  /** Fires the proposal. Returns once every operator holds the GCS rows. */
  propose: (proposalId: number, duration: string) => Promise<GatewayTips>;
  /** Waits for `DryRunStarted` and the Gateway gate, on these operators only. */
  waitDryRunStarted: (databases: string[], tips: GatewayTips) => Promise<void>;
  /** Runs `body` with background ERC-20 traffic on every host chain. */
  withTraffic: <T>(body: () => Promise<T>) => Promise<T>;
};

type RollbackCase = {
  label: string;
  proposalId: number;
  /**
   * True when the perturbation takes the operator out before the window opens.
   * That operator then never reaches `DryRunStarted`, so the profile waits for
   * the others, and no early hash of its own can let a peer anchor.
   */
  beforeWindow: boolean;
  perturb: () => Promise<void>;
  restore: () => Promise<void>;
  extraAssertions?: (attempt: Attempt) => Promise<void>;
};

type Attempt = { proposalHex: string; proposalBlock: string };

/** How long every operator must hold the old version after the rollback. */
const SPLIT_WATCH_SECONDS = 120;
/** The operator the cases perturb. Operator 0 stays healthy throughout. */
const PERTURBED = 1;
/**
 * Window length for these cases. Long enough that the injected work lands and
 * every healthy operator hashes it before the deadline, which is what case 4
 * needs for its `quorum`.
 */
const CASE_WINDOW = "3m";
/** Blocks the perturbed operator may still lag by once it catches up. */
const CATCH_UP_TOLERANCE = 5;

const SHARED_CONTAINERS = /^coprocessor-and-kms-db$/;

const containerNames = async (pattern: RegExp) => {
  const result = await run(["docker", "ps", "-a", "--format", "{{.Names}}"], { allowFailure: true });
  if (result.code !== 0) throw new PreflightError(result.stderr.trim() || "docker ps failed");
  return result.stdout
    .split(/\r?\n/)
    .map(line => line.trim())
    .filter(line => pattern.test(line) && !SHARED_CONTAINERS.test(line))
    .sort();
};

const operatorPrefix = (operator: number) => `coprocessor${operator === 0 ? "" : operator}-`;

/** Every container of one operator, Blue and Green. The shared database stays up. */
const operatorContainers = (operator: number) =>
  containerNames(new RegExp(`^${operatorPrefix(operator)}`));

/** The Blue host-chain listeners of one operator, without the Green ones. */
const blueHostListeners = (operator: number) =>
  containerNames(new RegExp(`^${operatorPrefix(operator)}host-listener(-|$)`));

/** The Green detector of one operator. */
const greenDetector = (operator: number) =>
  containerNames(new RegExp(`^${operatorPrefix(operator)}gcs-consensus-detector$`));

const stopContainers = async (containers: string[]) => {
  if (containers.length === 0) throw new PreflightError("found no containers to stop");
  await run(["docker", "stop", ...containers]);
  console.log(`OK:   stopped ${containers.length} container(s): ${containers.join(", ")}`);
};

const startContainers = async (containers: string[]) => {
  await run(["docker", "start", ...containers]);
  console.log(`OK:   started ${containers.length} container(s) again`);
};

/**
 * Recreate containers, with an optional compose override. Same shape as the
 * synthetic-ops recreate in the profile. `patch` of `undefined` recreates them
 * from the base files alone, which is how a case takes an override away.
 */
const recreateServices = async (
  containers: string[],
  name: string,
  patch?: (service: string) => Record<string, unknown>,
) => {
  if (containers.length === 0) throw new PreflightError(`found no containers for ${name}`);
  const format = [
    '{{index .Config.Labels "com.docker.compose.project.config_files"}}',
    '{{index .Config.Labels "com.docker.compose.service"}}',
  ].join("\t");
  const meta = await run(["docker", "inspect", ...containers, "--format", format], { allowFailure: true });
  if (meta.code !== 0) throw new PreflightError(`docker inspect failed: ${meta.stderr.trim()}`);

  const groups = new Map<string, string[]>();
  for (const line of meta.stdout.split(/\r?\n/).map(entry => entry.trim()).filter(Boolean)) {
    const [files, service] = line.split("\t");
    if (!files || !service) throw new PreflightError(`compose labels are missing on "${containers.join(", ")}"`);
    groups.set(files, [...(groups.get(files) ?? []), service]);
  }

  const env = await composeEnv("coprocessor");
  const recreated: string[] = [];
  for (const [index, [files, services]] of [...groups].entries()) {
    const overrideArgs: string[] = [];
    if (patch) {
      const override = composePath(`${name}-${index}`);
      await Bun.write(
        override,
        JSON.stringify({ services: Object.fromEntries(services.map(service => [service, patch(service)])) }),
      );
      overrideArgs.push("-f", override);
    }
    const up = await run(
      [
        "docker", "compose", "-p", PROJECT,
        ...files.split(",").flatMap(file => ["-f", file]),
        ...overrideArgs,
        "up", "-d", "--force-recreate", "--no-deps",
        ...services,
      ],
      { env, allowFailure: true },
    );
    if (up.code !== 0) throw new PreflightError(`recreating ${name} failed: ${up.stderr.trim()}`);
    recreated.push(...services);
  }
  return recreated.sort();
};

/**
 * Read one operator's decision record, or `undefined` when it wrote none. The
 * store publishes on the host port, so this reads the object a peer reads.
 */
const decisionRecord = async (operator: number, attempt: Attempt) => {
  const key = `decision/proposal=${attempt.proposalHex}/attempt=${attempt.proposalBlock}/outcome.json`;
  const response = await fetch(`${OBJECT_STORE_EXTERNAL_URL}/coproc-${operator}/${key}`);
  if (!response.ok) return undefined;
  const record = (await response.json()) as { decision?: string };
  return record.decision;
};

const attemptOf = async (ctx: RollbackCaseContext): Promise<Attempt> => {
  const row = await ctx.psqlQuery(
    ctx.operatorDatabases[0]!,
    "SELECT encode(proposal_id,'hex')||'|'||proposal_block FROM upgrade_state WHERE stack_role='GCS' LIMIT 1;",
  );
  const [proposalHex, proposalBlock] = row.split("|");
  if (!proposalHex || !proposalBlock) throw new Error(`cannot read the active attempt: "${row}"`);
  return { proposalHex, proposalBlock };
};

const waitForRollback = async (ctx: RollbackCaseContext, database: string, timeoutSecs: number) =>
  ctx.waitUntil({
    label: `${database}  GCS rolled back to PAUSED/failed`,
    timeoutSecs,
    check: async () =>
      (await ctx.psqlQuery(
        database,
        `SELECT CASE WHEN COUNT(*)=${ctx.hostChainCount}
                            AND BOOL_AND(state='PAUSED' AND status='failed')
                     THEN 'rolled-back' ELSE 'waiting' END
           FROM upgrade_state WHERE stack_role='GCS';`,
      )) === "rolled-back",
  });

const highestIndexedBlock = async (ctx: RollbackCaseContext, database: string) =>
  Number(await ctx.psqlQuery(database, "SELECT COALESCE(MAX(block_number), 0) FROM host_chain_blocks_valid;"));

/**
 * Wait until the perturbed operator indexes what the healthy one holds.
 *
 * The split appears only once the operator that was away works through its
 * backlog. Watching its version before that watches the wrong minutes.
 */
const waitForCatchUp = async (ctx: RollbackCaseContext) => {
  const perturbed = ctx.operatorDatabases[PERTURBED]!;
  const healthy = ctx.operatorDatabases[0]!;
  await ctx.waitUntil({
    label: `${perturbed}  caught up with ${healthy}`,
    timeoutSecs: 600,
    check: async () =>
      (await highestIndexedBlock(ctx, perturbed)) + CATCH_UP_TOLERANCE >=
      (await highestIndexedBlock(ctx, healthy)),
  });
};

/**
 * The assertions every case makes.
 *
 * The version watch is the regression guard. A split appears minutes late, so
 * one reading proves nothing. This watches every operator for
 * [`SPLIT_WATCH_SECONDS`] once the perturbed one catches up.
 */
const assertEveryOperatorRolledBack = async (ctx: RollbackCaseContext) => {
  for (const database of ctx.operatorDatabases) {
    await waitForRollback(ctx, database, 600);
    const flags = await ctx.psqlQuery(
      database,
      "SELECT MIN(last_error)||'|'||BOOL_OR(host_consensus_reached)||'|'||BOOL_OR(gw_consensus_reached) " +
        "FROM upgrade_state WHERE stack_role='GCS';",
    );
    if (flags !== "unanimity_consensus_timeout|false|false") {
      throw new Error(`${database} rollback left unexpected flags "${flags}"`);
    }
    const residue = await ctx.psqlQuery(
      database,
      `SELECT (SELECT count(*) FROM "gcs-${ctx.gcsStackVersion}".computations) + ` +
        `(SELECT count(*) FROM "gcs-${ctx.gcsStackVersion}".state_hash);`,
    );
    if (residue !== "0") throw new Error(`${database} gcs schema not reset: ${residue} residual row(s)`);
  }

  await waitForCatchUp(ctx);
  const deadline = Date.now() + SPLIT_WATCH_SECONDS * 1000;
  while (Date.now() < deadline) {
    for (const database of ctx.operatorDatabases) {
      const version = await ctx.psqlQuery(database, "SELECT stack_version FROM versioning;");
      if (version !== "v0.14") {
        throw new Error(`${database}.versioning = "${version}" after the rollback: the fleet split`);
      }
    }
    await new Promise(resolve => setTimeout(resolve, 5000));
  }
  console.log(`OK:   every operator kept v0.14 for ${SPLIT_WATCH_SECONDS}s after it caught up`);
};

/** The traffic moves the Gateway tip, so each attempt pins its own block. */
const runRollbackCase = async (ctx: RollbackCaseContext, item: RollbackCase) => {
  // Assert after the traffic stops, so the catch-up wait sees a still chain.
  const attempt = await ctx.withTraffic(() => runRollbackCaseBody(ctx, item));
  await assertEveryOperatorRolledBack(ctx);
  await item.extraAssertions?.(attempt);
  console.log(`OK:   rollback case ${item.proposalId} passed`);
};

const runRollbackCaseBody = async (ctx: RollbackCaseContext, item: RollbackCase) => {
  console.log(`\n==== rollback case ${item.proposalId}: ${item.label} ====`);
  const tips = await ctx.propose(item.proposalId, CASE_WINDOW);
  const attempt = await attemptOf(ctx);

  // An operator that goes before the window opens never reaches DryRunStarted,
  // so this asserts the Gateway gate on the operators that stay.
  if (item.beforeWindow) await item.perturb();
  const watched = item.beforeWindow
    ? ctx.operatorDatabases.filter((_, index) => index !== PERTURBED)
    : ctx.operatorDatabases;
  await ctx.waitDryRunStarted(watched, tips);
  if (!item.beforeWindow) await item.perturb();

  await waitForRollback(ctx, ctx.operatorDatabases[0]!, 600);
  console.log("OK:   the healthy operator cancelled the upgrade");

  await item.restore();
  return attempt;
};

/**
 * Run the three rollback cases, in order, before the successful cutover.
 *
 * Case 4 blocks the reads of the perturbed operator from the healthy
 * operator's bucket. Each operator publishes its bucket under its own hostname
 * for the one store, so one blocked host entry on one container cuts that one
 * path and leaves every other read working.
 */
export const runRollbackCases = async (ctx: RollbackCaseContext) => {
  if (ctx.operatorDatabases.length < 2) {
    throw new PreflightError("the rollback cases need at least two operators");
  }

  await runRollbackCase(ctx, {
    label: "a node stops before the window, and starts again after the abort",
    proposalId: 2,
    beforeWindow: true,
    perturb: async () => stopContainers(await operatorContainers(PERTURBED)),
    restore: async () => startContainers(await operatorContainers(PERTURBED)),
    extraAssertions: async attempt => {
      const record = await decisionRecord(PERTURBED, attempt);
      if (record === "quorum") {
        throw new Error("the operator that was down recorded quorum after it came back");
      }
    },
  });

  await runRollbackCase(ctx, {
    label: "Blue falls behind before the window, and starts again after the abort",
    proposalId: 3,
    beforeWindow: true,
    perturb: async () => stopContainers(await blueHostListeners(PERTURBED)),
    restore: async () => startContainers(await blueHostListeners(PERTURBED)),
  });

  await runRollbackCase(ctx, {
    label: "one operator records quorum, and the other reaches its deadline first",
    proposalId: 4,
    beforeWindow: false,
    perturb: async () => {
      const blocked = objectStoreOperatorHost(0);
      const services = await recreateServices(
        await greenDetector(PERTURBED),
        "rollback-case-isolate",
        () => ({ extra_hosts: [`${blocked}:127.0.0.1`] }),
      );
      console.log(`OK:   ${services.join(", ")} cannot reach ${blocked}`);
    },
    restore: async () => {
      await recreateServices(await greenDetector(PERTURBED), "rollback-case-restore");
      console.log("OK:   the detector reads every bucket again");
    },
    extraAssertions: async attempt => {
      const healthy = await decisionRecord(0, attempt);
      const perturbed = await decisionRecord(PERTURBED, attempt);
      if (healthy !== "quorum" || perturbed !== "abort") {
        throw new Error(
          `expected quorum from the healthy operator and abort from the isolated one, read "${healthy}" and "${perturbed}"`,
        );
      }
      console.log("OK:   abort beat quorum, and both operators cancelled the upgrade");
    },
  });
};
