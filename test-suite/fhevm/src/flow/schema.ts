import { SchemaGuardError } from "../errors";
import { REPO_ROOT, SCHEMA_COUPLED_GROUPS } from "../layout";
import { effectiveOverrides } from "../scenario/resolve";
import type { LocalOverride, OverrideGroup, State, VersionBundle } from "../types";
import { run } from "../utils/process";

const SCHEMA_GUARDS = {
  coprocessor: {
    versionKey: "COPROCESSOR_DB_MIGRATION_VERSION",
    repoPath: "coprocessor/fhevm-engine/db-migration/migrations",
  },
  "kms-connector": {
    versionKey: "CONNECTOR_DB_MIGRATION_VERSION",
    repoPath: "kms-connector/connector-db/migrations",
  },
} as const satisfies Partial<Record<OverrideGroup, { versionKey: string; repoPath: string }>>;

const SCHEMA_GUARD_TARGETS = new Set<VersionBundle["target"]>(["latest-supported", "latest-main", "sha"]);

const partialSchemaOverrides = (overrides: LocalOverride[]) =>
  overrides.filter(
    (item): item is LocalOverride & { services: string[] } =>
      !!item.services?.length && SCHEMA_COUPLED_GROUPS.includes(item.group),
  );

const assertSchemaRepoStable = async (
  group: OverrideGroup,
  bundle: VersionBundle,
  missingRefMessage: (ref: string) => string,
  mismatchMessage: (ref: string) => string,
) => {
  const guard = SCHEMA_GUARDS[group as keyof typeof SCHEMA_GUARDS];
  if (!guard || !SCHEMA_GUARD_TARGETS.has(bundle.target)) {
    return;
  }
  const ref = bundle.env[guard.versionKey];
  if (!ref) {
    return;
  }
  const verified = await run(["git", "rev-parse", "-q", "--verify", `${ref}^{commit}`], {
    cwd: REPO_ROOT,
    allowFailure: true,
  });
  if (verified.code !== 0) {
    throw new SchemaGuardError(group, missingRefMessage(ref));
  }
  const untracked = await run(
    ["git", "ls-files", "--others", "--exclude-standard", "--", guard.repoPath],
    { cwd: REPO_ROOT, allowFailure: true },
  );
  if (untracked.code !== 0) {
    throw new SchemaGuardError(group, `Failed to inspect local ${group} migrations`);
  }
  if (untracked.stdout.trim()) {
    throw new SchemaGuardError(group, mismatchMessage(ref));
  }
  const diff = await run(["git", "diff", "--quiet", "--exit-code", ref, "--", guard.repoPath], {
    cwd: REPO_ROOT,
    allowFailure: true,
  });
  if (diff.code === 1) {
    throw new SchemaGuardError(group, mismatchMessage(ref));
  }
  if (diff.code !== 0 && diff.code !== 1) {
    throw new SchemaGuardError(group, `Failed to compare local ${group} migrations against ${ref}`);
  }
};

export const assertSchemaCompatibility = async (
  bundle: VersionBundle,
  overrides: LocalOverride[],
  scenario: State["scenario"],
  allowSchemaMismatch: boolean,
) => {
  if (allowSchemaMismatch || !SCHEMA_GUARD_TARGETS.has(bundle.target)) {
    return;
  }
  for (const item of partialSchemaOverrides(effectiveOverrides(overrides, scenario))) {
    await assertSchemaRepoStable(
      item.group,
      bundle,
      (ref) => `Cannot compare local ${item.group} migrations against ${ref}; local git ref is missing. Run \`git fetch --tags\` or pass --allow-schema-mismatch.`,
      (ref) =>
        `${item.group}: local DB migrations diverge from ${ref}. Use --override ${item.group} or pass --allow-schema-mismatch if you know this service remains compatible.`,
    );
  }
};
