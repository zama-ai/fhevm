/**
 * Resolves version bundles from targets, lock files, GitHub metadata, support floors, and env overrides.
 */
import YAML from "yaml";

import {
  COMPAT_MATRIX,
  LEGACY_RELAYER_IMAGE_REPOSITORY,
  LEGACY_RELAYER_MIGRATE_IMAGE_REPOSITORY,
  MODERN_RELAYER_IMAGE_REPOSITORY,
  MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY,
} from "../compat/compat";
import { GitHubApiError } from "../errors";
import { commitsFrom, gitopsFile, mainCommits, packageTags } from "./github";
import { NON_NETWORK_COMPANIONS } from "./presets";
import { LATEST_SUPPORTED_PROFILE } from "../layout";
import type { VersionBundle, VersionTarget } from "../types";
import { normalizeRepository, readJson } from "../utils/fs";

const DEVNET_FILES = {
  gateway: "values/gw-blockchain/gw-sc-deploy-1-init/values-zws-dev.yaml",
  host: "values/eth-blockchain/eth-sc-deploy/values-zws-dev.yaml",
  coprocessorDb: "values/coproc/coproc-infra-db-mig/values-zws-dev.yaml",
  coprocessorHost: "values/eth-blockchain/eth-coproc-listener/values-zws-dev.yaml",
  coprocessorGateway: "values/gw-blockchain/gw-coprocessor/values-zws-dev.yaml",
  coprocessorWorkers: "values/coproc/coproc-workers/values-zws-dev.yaml",
  connector: "values/kms/kms-connector/values-zws-dev.yaml",
  kmsCore: "values/kms/kms-service/values-zws-dev.yaml",
  relayer: "values/relayer/relayer/values-relayer-dev.yaml",
  testSuite: "values/relayer/fhevm-test-suite/values-relayer-dev.yaml",
} as const;

const TESTNET_FILES = {
  gateway: "values/gw-blockchain/gw-sc-deploy-1-init/values-zws-testnet.yaml",
  host: "values/eth-blockchain/eth-sc-deploy/values-zws-testnet.yaml",
  coprocessorDb: "values/coproc/coproc-infra-db-mig/values-zws-testnet.yaml",
  coprocessorHost: "values/eth-blockchain/eth-coproc-listener/values-zws-testnet.yaml",
  coprocessorGateway: "values/gw-blockchain/gw-coprocessor/values-zws-testnet.yaml",
  coprocessorWorkers: "values/coproc/coproc-workers/values-zws-testnet.yaml",
  connector: "values/kms/kms-connector/values-kms-connector-mpc-testnet.yaml",
  kmsCore: "values/kms/kms-service/values-kms-core-mpc-testnet.yaml",
  relayer: "values/relayer/relayer/values-relayer-testnet.yaml",
  testSuite: "values/relayer/fhevm-test-suite/values-relayer-testnet.yaml",
} as const;

const MAINNET_FILES = {
  gateway: "values/gw-blockchain/gw-sc-deploy-1-init/values-zama-mainnet.yaml",
  host: "values/eth-blockchain/eth-sc-deploy/values-zama-mainnet.yaml",
  coprocessorDb: "values/coproc/coproc-infra-db-mig/values-coproc-mainnet.yaml",
  coprocessorHost: "values/eth-blockchain/eth-coproc-listener/values-coproc-mainnet.yaml",
  coprocessorGateway: "values/gw-blockchain/gw-coprocessor/values-coproc-mainnet.yaml",
  coprocessorWorkers: "values/coproc/coproc-workers/values-coproc-mainnet.yaml",
  connector: "values/kms/kms-connector/values-kms-connector-mpc-mainnet-1.yaml",
  kmsCore: "values/kms/kms-service/values-kms-core-mpc-mainnet-1.yaml",
  relayer: "values/relayer/relayer/values-relayer-mainnet.yaml",
  testSuite: "values/relayer/fhevm-test-suite/values-relayer-mainnet.yaml",
} as const;

const REPO_PACKAGES = {
  GATEWAY_VERSION: "fhevm%2Fgateway-contracts",
  HOST_VERSION: "fhevm%2Fhost-contracts",
  COPROCESSOR_DB_MIGRATION_VERSION: "fhevm%2Fcoprocessor%2Fdb-migration",
  COPROCESSOR_HOST_LISTENER_VERSION: "fhevm%2Fcoprocessor%2Fhost-listener",
  COPROCESSOR_GW_LISTENER_VERSION: "fhevm%2Fcoprocessor%2Fgw-listener",
  COPROCESSOR_TX_SENDER_VERSION: "fhevm%2Fcoprocessor%2Ftx-sender",
  COPROCESSOR_TFHE_WORKER_VERSION: "fhevm%2Fcoprocessor%2Ftfhe-worker",
  COPROCESSOR_ZKPROOF_WORKER_VERSION: "fhevm%2Fcoprocessor%2Fzkproof-worker",
  COPROCESSOR_SNS_WORKER_VERSION: "fhevm%2Fcoprocessor%2Fsns-worker",
  COPROCESSOR_CONSENSUS_DETECTOR_VERSION: "fhevm%2Fcoprocessor%2Fconsensus-detector",
  COPROCESSOR_UPGRADE_CONTROLLER_VERSION: "fhevm%2Fcoprocessor%2Fupgrade-controller",
  LISTENER_CORE_VERSION: "fhevm%2Flistener%2Flistener-core",
  CONNECTOR_DB_MIGRATION_VERSION: "fhevm%2Fkms-connector%2Fdb-migration",
  CONNECTOR_GW_LISTENER_VERSION: "fhevm%2Fkms-connector%2Fgw-listener",
  CONNECTOR_KMS_WORKER_VERSION: "fhevm%2Fkms-connector%2Fkms-worker",
  CONNECTOR_TX_SENDER_VERSION: "fhevm%2Fkms-connector%2Ftx-sender",
  RELAYER_VERSION: "fhevm%2Frelayer",
  RELAYER_MIGRATE_VERSION: "fhevm%2Frelayer-migrate",
  TEST_SUITE_VERSION: "fhevm%2Ftest-suite%2Fe2e",
} as const;

export const REPO_KEYS = new Set(Object.keys(REPO_PACKAGES));

// Repo-owned images that may not be published at a resolved sha yet (brand-new images); pinned once published, omitted while unpublished so the compat layer gates their services out instead of docker failing to pull a missing manifest.
export const OPTIONAL_REPO_KEYS = new Set([
  "COPROCESSOR_CONSENSUS_DETECTOR_VERSION",
  "COPROCESSOR_UPGRADE_CONTROLLER_VERSION",
]);

export const PACKAGE_TO_REPOSITORY = {
  GATEWAY_VERSION: "ghcr.io/zama-ai/fhevm/gateway-contracts",
  HOST_VERSION: "ghcr.io/zama-ai/fhevm/host-contracts",
  COPROCESSOR_DB_MIGRATION_VERSION: "ghcr.io/zama-ai/fhevm/coprocessor/db-migration",
  COPROCESSOR_HOST_LISTENER_VERSION: "ghcr.io/zama-ai/fhevm/coprocessor/host-listener",
  COPROCESSOR_GW_LISTENER_VERSION: "ghcr.io/zama-ai/fhevm/coprocessor/gw-listener",
  COPROCESSOR_TX_SENDER_VERSION: "ghcr.io/zama-ai/fhevm/coprocessor/tx-sender",
  COPROCESSOR_TFHE_WORKER_VERSION: "ghcr.io/zama-ai/fhevm/coprocessor/tfhe-worker",
  COPROCESSOR_ZKPROOF_WORKER_VERSION: "ghcr.io/zama-ai/fhevm/coprocessor/zkproof-worker",
  COPROCESSOR_SNS_WORKER_VERSION: "ghcr.io/zama-ai/fhevm/coprocessor/sns-worker",
  COPROCESSOR_CONSENSUS_DETECTOR_VERSION: "ghcr.io/zama-ai/fhevm/coprocessor/consensus-detector",
  COPROCESSOR_UPGRADE_CONTROLLER_VERSION: "ghcr.io/zama-ai/fhevm/coprocessor/upgrade-controller",
  LISTENER_CORE_VERSION: "ghcr.io/zama-ai/fhevm/listener/listener-core",
  CONNECTOR_DB_MIGRATION_VERSION: "ghcr.io/zama-ai/fhevm/kms-connector/db-migration",
  CONNECTOR_GW_LISTENER_VERSION: "ghcr.io/zama-ai/fhevm/kms-connector/gw-listener",
  CONNECTOR_KMS_WORKER_VERSION: "ghcr.io/zama-ai/fhevm/kms-connector/kms-worker",
  CONNECTOR_TX_SENDER_VERSION: "ghcr.io/zama-ai/fhevm/kms-connector/tx-sender",
  CORE_VERSION: "ghcr.io/zama-ai/kms/core-service-enclave",
  RELAYER_VERSION: "ghcr.io/zama-ai/console/relayer",
  RELAYER_MIGRATE_VERSION: "ghcr.io/zama-ai/console/relayer-migrate",
  TEST_SUITE_VERSION: "ghcr.io/zama-ai/fhevm/test-suite/e2e",
} as const;

const PACKAGE_REPOSITORY_CANDIDATES: Partial<Record<keyof typeof PACKAGE_TO_REPOSITORY, string[]>> = {
  RELAYER_VERSION: [MODERN_RELAYER_IMAGE_REPOSITORY, LEGACY_RELAYER_IMAGE_REPOSITORY],
  RELAYER_MIGRATE_VERSION: [MODERN_RELAYER_MIGRATE_IMAGE_REPOSITORY, LEGACY_RELAYER_MIGRATE_IMAGE_REPOSITORY],
};

const SHA_FALLBACK_COMMIT_WINDOW = 500;
// A missing tag from a flaked re-tag, an in-flight push pipeline, or an is-latest-commit skip
// sits a handful of commits behind the requested sha (a merge train at most). A published image
// lagging further than this means the component's publishing is genuinely broken, which a
// fallback must not paper over.
export const MAX_FALLBACK_COMMIT_DEPTH = 50;

export const REPO_TAG = /^[0-9a-f]{7}$/;
export const SHA_REF = /^(?:[0-9a-f]{7}|[0-9a-f]{40})$/i;
export const SIMPLE_ACL_MIN_SHA = COMPAT_MATRIX.anchors.SIMPLE_ACL_MIN_SHA;
export const SHA_RUNTIME_COMPAT_MIN_SHA = "1272b10b308b064e7477ca3272712b90b50280d9";

/** Recursively collects image references from loosely structured YAML documents. */
const walkImages = (node: unknown, out: Array<{ repository: string; tag: string }>) => {
  if (Array.isArray(node)) {
    for (const item of node) {
      walkImages(item, out);
    }
    return;
  }
  if (!node || typeof node !== "object") {
    return;
  }
  const record = node as Record<string, unknown>;
  if (typeof record.image === "string" && record.image.includes(":")) {
    const [repository, tag] = record.image.split(/:(?=[^:]+$)/);
    out.push({ repository: normalizeRepository(repository), tag });
  }
  if (record.image && typeof record.image === "object") {
    const image = record.image as Record<string, unknown>;
    const repository =
      typeof image.name === "string" ? image.name : typeof image.repository === "string" ? image.repository : undefined;
    const tag = typeof image.tag === "string" ? image.tag : undefined;
    if (repository && tag) {
      out.push({ repository: normalizeRepository(repository), tag });
    }
  }
  for (const value of Object.values(record)) {
    walkImages(value, out);
  }
};

/** Extracts normalized image tags from a GitOps YAML document. */
const extractTagsFromYaml = (text: string) => {
  const images: Array<{ repository: string; tag: string }> = [];
  walkImages(YAML.parse(text), images);
  return images;
};

const findImageTag = (
  parsed: Record<string, Array<{ repository: string; tag: string }>>,
  docName: string,
  key: keyof typeof PACKAGE_TO_REPOSITORY,
) => {
  const repositories = PACKAGE_REPOSITORY_CANDIDATES[key] ?? [PACKAGE_TO_REPOSITORY[key]];
  for (const repository of repositories) {
    const hit = parsed[docName].find((img) => img.repository === repository);
    if (hit) {
      return hit.tag;
    }
  }
  throw new Error(`Missing ${repositories.join(" or ")} in GitOps file`);
};

/** Normalizes a full SHA into the short tag form used by repo-owned images. */
export const shortSha = (value: string) => value.toLowerCase().slice(0, 7);

/** Finds the position of the newest commit in an ancestry walk (newest first) that has a
 * published image tag; -1 when none does. The position is the commit distance behind the walk's
 * starting sha. */
export const findPublishedAncestorIndex = (commitShas: string[], publishedTags: Set<string>) =>
  commitShas.findIndex((sha) => publishedTags.has(shortSha(sha)));

/**
 * Resolves per-component fallback tags for repo-owned images with no published image at the
 * requested sha.
 *
 * A missing tag is an exceptional state: on every push the docker-build workflows either build a
 * changed component or re-tag the previous image with the new sha, so a gap means one of those
 * jobs failed, has not finished, or was skipped for a superseded branch tip. Each affected
 * component independently falls back to its newest published tag on the requested sha's
 * ancestry, and the substitution is recorded in `sources`. A fallback deeper than
 * MAX_FALLBACK_COMMIT_DEPTH fails resolution instead — that lag means the component's publishing
 * is broken, not flaked. Components with no published tag in the walked window (brand-new or
 * feature-branch-only packages) keep the requested sha, matching how latest-main skips gating
 * on such packages.
 */
export const resolveMissingRepoTagFallbacks = (options: {
  requestedTag: string;
  missingKeys: string[];
  commitShas: string[];
  packageTagsMap: Record<string, Set<string>>;
}): { overrides: Record<string, string>; sources: string[] } => {
  const overrides: Record<string, string> = {};
  const sources: string[] = [];
  for (const key of options.missingKeys) {
    const ancestorIndex = findPublishedAncestorIndex(options.commitShas, options.packageTagsMap[key] ?? new Set());
    if (ancestorIndex < 0) {
      sources.push(
        `${key}=${options.requestedTag} (unverified: no published tag within ${options.commitShas.length} commits)`,
      );
      continue;
    }
    if (ancestorIndex > MAX_FALLBACK_COMMIT_DEPTH) {
      throw new GitHubApiError(
        `${key} has no published image for ${options.requestedTag}, and its newest published image is ` +
          `${ancestorIndex} commits behind — beyond the ${MAX_FALLBACK_COMMIT_DEPTH}-commit fallback limit. ` +
          `Its docker builds look broken rather than flaked; fix or re-run them before resolving this baseline.`,
      );
    }
    const ancestor = shortSha(options.commitShas[ancestorIndex]);
    overrides[key] = ancestor;
    sources.push(`${key}=${ancestor} (fallback: ${options.requestedTag} unpublished)`);
  }
  return { overrides, sources };
};

/** Locates the simple-ACL support floor in fetched main history. */
export const simpleAclFloor = (commits: string[]) => {
  const floor = commits.indexOf(SIMPLE_ACL_MIN_SHA);
  if (floor < 0) {
    throw new Error(
      `simple-acl floor ${SIMPLE_ACL_MIN_SHA} was not found in fetched main history; increase the main history fetch window`,
    );
  }
  return floor;
};

/** Locates the runtime-compat support floor in fetched main history. */
export const shaRuntimeCompatFloor = (commits: string[]) => {
  const floor = commits.indexOf(SHA_RUNTIME_COMPAT_MIN_SHA);
  if (floor < 0) {
    throw new Error(
      `sha runtime compat floor ${SHA_RUNTIME_COMPAT_MIN_SHA} was not found in fetched main history; increase the main history fetch window`,
    );
  }
  return floor;
};

/** Builds a preset bundle for floating local targets and their companion pins. */
export const presetBundle = (
  target: "latest-main" | "sha",
  repoVersion: string,
  lockName: string,
  sources: string[] = [],
  publishedOptionalKeys?: ReadonlySet<string>,
  repoKeyOverrides?: Record<string, string>,
): VersionBundle => ({
  target,
  lockName,
  env: Object.fromEntries(
    Object.keys(PACKAGE_TO_REPOSITORY).flatMap((key) => {
      // Omit optional repo-owned images the resolved sha has not published (per `publishedOptionalKeys`) so the compat layer gates the service out instead of docker failing with `manifest unknown`.
      if (OPTIONAL_REPO_KEYS.has(key) && publishedOptionalKeys && !publishedOptionalKeys.has(key)) {
        return [];
      }
      const version = REPO_KEYS.has(key)
        ? repoKeyOverrides?.[key] ?? repoVersion
        : NON_NETWORK_COMPANIONS[target][key as keyof (typeof NON_NETWORK_COMPANIONS)[typeof target]];
      if (!version) {
        throw new Error(`Missing ${target} preset for ${key}`);
      }
      return [[key, version]];
    }),
  ),
  sources: [`preset=${target}`, `repo-owned=${repoVersion}`, ...sources],
});

/** Applies explicit version env overrides on top of a resolved bundle. */
export const applyVersionEnvOverrides = (
  bundle: VersionBundle,
  env: Record<string, string | undefined>,
): VersionBundle => {
  // Optional repo-owned images may be absent from the resolved bundle when unpublished; still honor an explicit env pin so callers can force-test a specific published tag.
  const overrideKeys = new Set([
    ...Object.keys(bundle.env),
    ...[...OPTIONAL_REPO_KEYS].filter((key) => env[key]?.length),
  ]);
  const overrides = Object.fromEntries(
    [...overrideKeys].filter((key) => env[key]?.length).map((key) => [key, env[key] as string]),
  );
  if (!Object.keys(overrides).length) {
    return bundle;
  }
  return {
    ...bundle,
    env: { ...bundle.env, ...overrides },
    sources: [...bundle.sources, `env=${Object.keys(overrides).sort().join(",")}`],
  };
};

/** Formats a resolved bundle as key=value lines for logging and previews. */
export const describeBundle = (bundle: VersionBundle) =>
  Object.entries(bundle.env)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n");

/** Resolves a bundle directly from GitOps-owned files for network targets. */
const bundleFromFiles = async (
  target: VersionTarget,
  files: Record<string, string>,
): Promise<VersionBundle> => {
  try {
    const entries = Object.entries(files);
    const contents = await Promise.all(entries.map(([, file]) => gitopsFile(file)));
    const docs = Object.fromEntries(entries.map(([name], index) => [name, contents[index]]));
    const parsed = Object.fromEntries(
      Object.entries(docs).map(([name, text]) => [name, extractTagsFromYaml(text)]),
    ) as Record<string, Array<{ repository: string; tag: string }>>;
    const coprocessorHostListenerVersion = findImageTag(parsed, "coprocessorHost", "COPROCESSOR_HOST_LISTENER_VERSION");
    return {
      target,
      lockName: `${target}.json`,
      env: {
        GATEWAY_VERSION: findImageTag(parsed, "gateway", "GATEWAY_VERSION"),
        HOST_VERSION: findImageTag(parsed, "host", "HOST_VERSION"),
        COPROCESSOR_DB_MIGRATION_VERSION: findImageTag(parsed, "coprocessorDb", "COPROCESSOR_DB_MIGRATION_VERSION"),
        COPROCESSOR_HOST_LISTENER_VERSION: coprocessorHostListenerVersion,
        COPROCESSOR_GW_LISTENER_VERSION: findImageTag(parsed, "coprocessorGateway", "COPROCESSOR_GW_LISTENER_VERSION"),
        COPROCESSOR_TX_SENDER_VERSION: findImageTag(parsed, "coprocessorGateway", "COPROCESSOR_TX_SENDER_VERSION"),
        COPROCESSOR_TFHE_WORKER_VERSION: findImageTag(parsed, "coprocessorWorkers", "COPROCESSOR_TFHE_WORKER_VERSION"),
        COPROCESSOR_ZKPROOF_WORKER_VERSION: findImageTag(parsed, "coprocessorWorkers", "COPROCESSOR_ZKPROOF_WORKER_VERSION"),
        COPROCESSOR_SNS_WORKER_VERSION: findImageTag(parsed, "coprocessorWorkers", "COPROCESSOR_SNS_WORKER_VERSION"),
        // GitOps YAMLs don't carry consensus-detector / upgrade-controller image refs yet; pin
        // them to the host-listener tag (same release cadence).
        COPROCESSOR_CONSENSUS_DETECTOR_VERSION: coprocessorHostListenerVersion,
        COPROCESSOR_UPGRADE_CONTROLLER_VERSION: coprocessorHostListenerVersion,
        LISTENER_CORE_VERSION: coprocessorHostListenerVersion,
        CONNECTOR_DB_MIGRATION_VERSION: findImageTag(parsed, "connector", "CONNECTOR_DB_MIGRATION_VERSION"),
        CONNECTOR_GW_LISTENER_VERSION: findImageTag(parsed, "connector", "CONNECTOR_GW_LISTENER_VERSION"),
        CONNECTOR_KMS_WORKER_VERSION: findImageTag(parsed, "connector", "CONNECTOR_KMS_WORKER_VERSION"),
        CONNECTOR_TX_SENDER_VERSION: findImageTag(parsed, "connector", "CONNECTOR_TX_SENDER_VERSION"),
        CORE_VERSION: findImageTag(parsed, "kmsCore", "CORE_VERSION"),
        RELAYER_VERSION: findImageTag(parsed, "relayer", "RELAYER_VERSION"),
        RELAYER_MIGRATE_VERSION: findImageTag(parsed, "relayer", "RELAYER_MIGRATE_VERSION"),
        TEST_SUITE_VERSION: findImageTag(parsed, "testSuite", "TEST_SUITE_VERSION"),
      },
      sources: Object.values(files),
    };
  } catch (error) {
    throw new GitHubApiError(error instanceof Error ? error.message : String(error));
  }
};

/**
 * Selects the newest short main-commit SHA published by every gating repo-owned package.
 *
 * A package gates resolution only when its published tag set intersects the candidate
 * window. Two kinds of package are treated as "don't gate on this":
 *  - an empty set: the image hasn't been published yet (typically a brand-new image
 *    whose CI build hasn't landed);
 *  - a non-empty set whose tags are all feature-branch builds not yet on main: it would
 *    otherwise reject every main commit and stall resolution entirely.
 * Both are pinned to the resolved sha by presetBundle (host-listener release cadence, as
 * bundleFromFiles does for network targets) until CI publishes them for main commits, at
 * which point they start gating again automatically.
 */
export const selectSupportedMainSha = (
  candidateShas: string[],
  packageTagsMap: Record<string, Set<string>>,
): string | undefined => {
  const gatingSets = Object.values(packageTagsMap).filter(
    (set) => set.size > 0 && candidateShas.some((sha) => set.has(sha)),
  );
  return candidateShas.find((sha) => gatingSets.every((set) => set.has(sha)));
};

/** Fetches the available tag sets for all repo-owned packages. */
const repoPackageTags = async (targetTag?: string) =>
  Object.fromEntries(
    await Promise.all(
      Object.entries(REPO_PACKAGES).map(async ([key, pkg]) => [key, await packageTags(pkg, targetTag)] as const),
    ),
  ) as Record<string, Set<string>>;

/** Resolves a user-facing version target into a concrete version bundle. */
export const resolveTarget = async (
  target: VersionTarget,
  options: { sha?: string } = {},
): Promise<VersionBundle> => {
  if (target === "latest-supported") {
    try {
      const bundle = await readJson<VersionBundle>(LATEST_SUPPORTED_PROFILE);
      return {
        ...bundle,
        target,
        lockName: "latest-supported.json",
        sources: ["profile=latest-supported", ...bundle.sources.filter((source) => source !== "profile=latest-supported")],
      };
    } catch (error) {
      throw new GitHubApiError(`Failed to read latest-supported profile: ${error}`);
    }
  }
  if (target === "devnet") return bundleFromFiles(target, DEVNET_FILES);
  if (target === "testnet") return bundleFromFiles(target, TESTNET_FILES);
  if (target === "mainnet") return bundleFromFiles(target, MAINNET_FILES);

  if (target === "sha") {
    const requested = options.sha?.trim();
    if (!requested) {
      throw new GitHubApiError("--target sha requires --sha");
    }
    if (!SHA_REF.test(requested)) {
      throw new GitHubApiError(`Invalid sha ${requested}; expected 7 or 40 hex characters`);
    }
    const tag = shortSha(requested);
    const lockName = `sha-${tag}.json`;
    const baseSources = [`requested-sha=${requested.toLowerCase()}`];

    // The docker-build workflows re-tag unchanged components on every push, so normally every
    // repo-owned image is published at the requested sha. Verify that instead of trusting it: a
    // flaked re-tag or failed build otherwise only surfaces as `manifest unknown` at pull time.
    let packageTagsMap: Record<string, Set<string>>;
    try {
      packageTagsMap = await repoPackageTags(tag);
    } catch (error) {
      // GitHub metadata is unavailable (offline, missing scopes): keep the historical unverified
      // pin so `--target sha` still resolves, and record that the check was skipped.
      const reason = error instanceof Error ? error.message.split("\n")[0] : String(error);
      console.log(`[resolve] sha ${tag}: skipping published-image check (${reason})`);
      return presetBundle(target, tag, lockName, [...baseSources, "published-image-check=skipped"], new Set());
    }
    const missingKeys = Object.keys(REPO_PACKAGES).filter(
      (key) => !OPTIONAL_REPO_KEYS.has(key) && !packageTagsMap[key]?.has(tag),
    );
    if (!missingKeys.length) {
      return presetBundle(target, tag, lockName, baseSources, new Set());
    }
    console.log(
      `[resolve] sha ${tag}: no published image for ${missingKeys.join(", ")}; looking for published ancestor tags`,
    );
    const commitShas = await commitsFrom(requested, SHA_FALLBACK_COMMIT_WINDOW);
    const { overrides, sources } = resolveMissingRepoTagFallbacks({
      requestedTag: tag,
      missingKeys,
      commitShas,
      packageTagsMap,
    });
    for (const source of sources) {
      console.log(`[resolve] ${source}`);
    }
    return presetBundle(target, tag, lockName, [...baseSources, ...sources], new Set(), overrides);
  }

  const [packageTagsMap, commits] = await Promise.all([repoPackageTags(), mainCommits(5000)]);
  let floor: number;
  let compatFloor: number;
  try {
    floor = simpleAclFloor(commits);
    compatFloor = shaRuntimeCompatFloor(commits);
  } catch (error) {
    throw new GitHubApiError(error instanceof Error ? error.message : String(error));
  }
  const candidateShas = commits
    .slice(0, Math.min(floor, compatFloor) + 1)
    .map((sha) => sha.slice(0, 7))
    .filter((sha) => REPO_TAG.test(sha));
  const short = selectSupportedMainSha(candidateShas, packageTagsMap);
  if (!short) {
    throw new GitHubApiError("Could not find a supported modern latest-main image set");
  }
  const publishedOptionalKeys = new Set(
    [...OPTIONAL_REPO_KEYS].filter((key) => packageTagsMap[key]?.has(short)),
  );
  return presetBundle(target, short, `latest-main-${short}.json`, [], publishedOptionalKeys);
};
