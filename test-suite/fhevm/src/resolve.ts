import { Effect } from "effect";
import YAML from "yaml";

import { COMPAT_MATRIX } from "./compat";
import { GitHubApiError } from "./errors";
import { NON_NETWORK_COMPANIONS } from "./presets";
import { LATEST_SUPPORTED_PROFILE } from "./layout";
import { GitHubClient } from "./services/GitHubClient";
import type { VersionBundle, VersionTarget } from "./types";
import { normalizeRepository, readJson } from "./utils";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

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

export const REPO_PACKAGES = {
  GATEWAY_VERSION: "fhevm%2Fgateway-contracts",
  HOST_VERSION: "fhevm%2Fhost-contracts",
  COPROCESSOR_DB_MIGRATION_VERSION: "fhevm%2Fcoprocessor%2Fdb-migration",
  COPROCESSOR_HOST_LISTENER_VERSION: "fhevm%2Fcoprocessor%2Fhost-listener",
  COPROCESSOR_GW_LISTENER_VERSION: "fhevm%2Fcoprocessor%2Fgw-listener",
  COPROCESSOR_TX_SENDER_VERSION: "fhevm%2Fcoprocessor%2Ftx-sender",
  COPROCESSOR_TFHE_WORKER_VERSION: "fhevm%2Fcoprocessor%2Ftfhe-worker",
  COPROCESSOR_ZKPROOF_WORKER_VERSION: "fhevm%2Fcoprocessor%2Fzkproof-worker",
  COPROCESSOR_SNS_WORKER_VERSION: "fhevm%2Fcoprocessor%2Fsns-worker",
  CONNECTOR_DB_MIGRATION_VERSION: "fhevm%2Fkms-connector%2Fdb-migration",
  CONNECTOR_GW_LISTENER_VERSION: "fhevm%2Fkms-connector%2Fgw-listener",
  CONNECTOR_KMS_WORKER_VERSION: "fhevm%2Fkms-connector%2Fkms-worker",
  CONNECTOR_TX_SENDER_VERSION: "fhevm%2Fkms-connector%2Ftx-sender",
  TEST_SUITE_VERSION: "fhevm%2Ftest-suite%2Fe2e",
} as const;

export const REPO_KEYS = new Set(Object.keys(REPO_PACKAGES));

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
  CONNECTOR_DB_MIGRATION_VERSION: "ghcr.io/zama-ai/fhevm/kms-connector/db-migration",
  CONNECTOR_GW_LISTENER_VERSION: "ghcr.io/zama-ai/fhevm/kms-connector/gw-listener",
  CONNECTOR_KMS_WORKER_VERSION: "ghcr.io/zama-ai/fhevm/kms-connector/kms-worker",
  CONNECTOR_TX_SENDER_VERSION: "ghcr.io/zama-ai/fhevm/kms-connector/tx-sender",
  CORE_VERSION: "ghcr.io/zama-ai/kms/core-service-enclave",
  RELAYER_VERSION: "ghcr.io/zama-ai/console/relayer",
  RELAYER_MIGRATE_VERSION: "ghcr.io/zama-ai/console/relayer-migrate",
  TEST_SUITE_VERSION: "ghcr.io/zama-ai/fhevm/test-suite/e2e",
} as const;

export const REPO_TAG = /^[0-9a-f]{7}$/;
export const SHA_REF = /^(?:[0-9a-f]{7}|[0-9a-f]{40})$/i;
export const SIMPLE_ACL_MIN_SHA = COMPAT_MATRIX.anchors.SIMPLE_ACL_MIN_SHA;
export const SHA_RUNTIME_COMPAT_MIN_SHA =
  "1272b10b308b064e7477ca3272712b90b50280d9";

// ---------------------------------------------------------------------------
// Pure helper functions
// ---------------------------------------------------------------------------

export const walkImages = (node: unknown, out: Array<{ repository: string; tag: string }>) => {
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
      typeof image.name === "string"
        ? image.name
        : typeof image.repository === "string"
          ? image.repository
          : undefined;
    const tag = typeof image.tag === "string" ? image.tag : undefined;
    if (repository && tag) {
      out.push({ repository: normalizeRepository(repository), tag });
    }
  }
  for (const value of Object.values(record)) {
    walkImages(value, out);
  }
};

export const extractTag = (text: string, repository: string) => {
  const parsed = extractTagsFromYaml(text);
  const hit = parsed.find((image) => image.repository === repository);
  if (!hit) {
    throw new Error(`Missing ${repository} in GitOps file`);
  }
  return hit.tag;
};

/** Parse a YAML document once and return all image references. */
const extractTagsFromYaml = (text: string) => {
  const images: Array<{ repository: string; tag: string }> = [];
  walkImages(YAML.parse(text), images);
  return images;
};

export const shortSha = (value: string) => value.toLowerCase().slice(0, 7);

export const simpleAclFloor = (commits: string[]) => {
  const floor = commits.indexOf(SIMPLE_ACL_MIN_SHA);
  if (floor < 0) {
    throw new Error(
      `simple-acl floor ${SIMPLE_ACL_MIN_SHA} was not found in fetched main history; increase the main history fetch window`,
    );
  }
  return floor;
};

export const shaRuntimeCompatFloor = (commits: string[]) => {
  const floor = commits.indexOf(SHA_RUNTIME_COMPAT_MIN_SHA);
  if (floor < 0) {
    throw new Error(
      `sha runtime compat floor ${SHA_RUNTIME_COMPAT_MIN_SHA} was not found in fetched main history; increase the main history fetch window`,
    );
  }
  return floor;
};

export const missingRepoPackages = (packageTags: Record<string, Set<string>>, tag: string) =>
  Object.entries(REPO_PACKAGES)
    .filter(([key]) => !packageTags[key]?.has(tag))
    .map(([, pkg]) => decodeURIComponent(pkg));

export const presetBundle = (
  target: "latest-main" | "sha",
  repoVersion: string,
  lockName: string,
  sources: string[] = [],
): VersionBundle => ({
  target,
  lockName,
  env: Object.fromEntries(
    Object.keys(PACKAGE_TO_REPOSITORY).map((key) => {
      const version = REPO_KEYS.has(key)
        ? repoVersion
        : NON_NETWORK_COMPANIONS[target][key as keyof (typeof NON_NETWORK_COMPANIONS)[typeof target]];
      if (!version) {
        throw new Error(`Missing ${target} preset for ${key}`);
      }
      return [key, version];
    }),
  ),
  sources: [`preset=${target}`, `repo-owned=${repoVersion}`, ...sources],
});

export const applyVersionEnvOverrides = (
  bundle: VersionBundle,
  env: Record<string, string | undefined>,
): VersionBundle => {
  const overrides = Object.fromEntries(
    Object.keys(bundle.env)
      .filter((key) => env[key]?.length)
      .map((key) => [key, env[key] as string]),
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

export const describeBundle = (bundle: VersionBundle) =>
  Object.entries(bundle.env)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n");

// ---------------------------------------------------------------------------
// Effect-based functions (use GitHubClient from context)
// ---------------------------------------------------------------------------

/**
 * Fetch all GitOps value files in parallel and extract image tags into a
 * VersionBundle. Equivalent to the old `bundleFromFiles(client, target, files)`.
 */
export const bundleFromFiles = (
  target: VersionTarget,
  files: Record<string, string>,
): Effect.Effect<VersionBundle, GitHubApiError, GitHubClient> =>
  Effect.gen(function* () {
    const gh = yield* GitHubClient;
    const entries = Object.entries(files);
    const contents = yield* Effect.all(
      entries.map(([, file]) => gh.gitopsFile(file)),
      { concurrency: "unbounded" },
    );
    const docs = Object.fromEntries(entries.map(([name], index) => [name, contents[index]]));

    try {
      // Parse each YAML document once, then look up repositories by pre-parsed images
      const parsed = Object.fromEntries(
        Object.entries(docs).map(([name, text]) => [name, extractTagsFromYaml(text)]),
      );
      const findTag = (docName: string, repository: string) => {
        const hit = parsed[docName].find((img) => img.repository === repository);
        if (!hit) throw new Error(`Missing ${repository} in GitOps file`);
        return hit.tag;
      };
      const env = {
        GATEWAY_VERSION: findTag("gateway", PACKAGE_TO_REPOSITORY.GATEWAY_VERSION),
        HOST_VERSION: findTag("host", PACKAGE_TO_REPOSITORY.HOST_VERSION),
        COPROCESSOR_DB_MIGRATION_VERSION: findTag("coprocessorDb", PACKAGE_TO_REPOSITORY.COPROCESSOR_DB_MIGRATION_VERSION),
        COPROCESSOR_HOST_LISTENER_VERSION: findTag("coprocessorHost", PACKAGE_TO_REPOSITORY.COPROCESSOR_HOST_LISTENER_VERSION),
        COPROCESSOR_GW_LISTENER_VERSION: findTag("coprocessorGateway", PACKAGE_TO_REPOSITORY.COPROCESSOR_GW_LISTENER_VERSION),
        COPROCESSOR_TX_SENDER_VERSION: findTag("coprocessorGateway", PACKAGE_TO_REPOSITORY.COPROCESSOR_TX_SENDER_VERSION),
        COPROCESSOR_TFHE_WORKER_VERSION: findTag("coprocessorWorkers", PACKAGE_TO_REPOSITORY.COPROCESSOR_TFHE_WORKER_VERSION),
        COPROCESSOR_ZKPROOF_WORKER_VERSION: findTag("coprocessorWorkers", PACKAGE_TO_REPOSITORY.COPROCESSOR_ZKPROOF_WORKER_VERSION),
        COPROCESSOR_SNS_WORKER_VERSION: findTag("coprocessorWorkers", PACKAGE_TO_REPOSITORY.COPROCESSOR_SNS_WORKER_VERSION),
        CONNECTOR_DB_MIGRATION_VERSION: findTag("connector", PACKAGE_TO_REPOSITORY.CONNECTOR_DB_MIGRATION_VERSION),
        CONNECTOR_GW_LISTENER_VERSION: findTag("connector", PACKAGE_TO_REPOSITORY.CONNECTOR_GW_LISTENER_VERSION),
        CONNECTOR_KMS_WORKER_VERSION: findTag("connector", PACKAGE_TO_REPOSITORY.CONNECTOR_KMS_WORKER_VERSION),
        CONNECTOR_TX_SENDER_VERSION: findTag("connector", PACKAGE_TO_REPOSITORY.CONNECTOR_TX_SENDER_VERSION),
        CORE_VERSION: findTag("kmsCore", PACKAGE_TO_REPOSITORY.CORE_VERSION),
        RELAYER_VERSION: findTag("relayer", PACKAGE_TO_REPOSITORY.RELAYER_VERSION),
        RELAYER_MIGRATE_VERSION: findTag("relayer", PACKAGE_TO_REPOSITORY.RELAYER_MIGRATE_VERSION),
        TEST_SUITE_VERSION: findTag("testSuite", PACKAGE_TO_REPOSITORY.TEST_SUITE_VERSION),
      };
      return {
        target,
        lockName: `${target}.json`,
        env,
        sources: Object.values(files),
      } satisfies VersionBundle;
    } catch (error) {
      return yield* Effect.fail(
        new GitHubApiError({ message: error instanceof Error ? error.message : String(error) }),
      );
    }
  });

/**
 * Fetch GHCR package tags for all repo-owned packages in parallel.
 * Equivalent to the old `repoPackageTags(client)`.
 */
const repoPackageTags = Effect.gen(function* () {
  const gh = yield* GitHubClient;
  const entries = Object.entries(REPO_PACKAGES);
  const tagSets = yield* Effect.all(
    entries.map(([, pkg]) => gh.packageTags(pkg)),
    { concurrency: "unbounded" },
  );
  return Object.fromEntries(entries.map(([key], index) => [key, tagSets[index]])) as Record<string, Set<string>>;
});

/**
 * Resolve a VersionTarget to a VersionBundle using the GitHubClient from context.
 * Equivalent to the old `resolveTarget(target, client, options)`.
 */
export const resolveTarget = (
  target: VersionTarget,
  options: { sha?: string } = {},
): Effect.Effect<VersionBundle, GitHubApiError, GitHubClient> =>
  Effect.gen(function* () {
    if (target === "latest-supported") {
      return yield* Effect.tryPromise({
        try: () => readJson<VersionBundle>(LATEST_SUPPORTED_PROFILE),
        catch: (error) => new GitHubApiError({ message: `Failed to read latest-supported profile: ${error}` }),
      }).pipe(
        Effect.map((bundle) => ({
          ...bundle,
          target,
          lockName: "latest-supported.json",
          sources: ["profile=latest-supported", ...bundle.sources.filter((source) => source !== "profile=latest-supported")],
        })),
      );
    }
    if (target === "devnet") {
      return yield* bundleFromFiles(target, DEVNET_FILES);
    }
    if (target === "testnet") {
      return yield* bundleFromFiles(target, TESTNET_FILES);
    }
    if (target === "mainnet") {
      return yield* bundleFromFiles(target, MAINNET_FILES);
    }

    const gh = yield* GitHubClient;

    if (target === "sha") {
      const requested = options.sha?.trim();
      if (!requested) {
        return yield* Effect.fail(new GitHubApiError({ message: "--target sha requires --sha" }));
      }
      if (!SHA_REF.test(requested)) {
        return yield* Effect.fail(
          new GitHubApiError({ message: `Invalid sha ${requested}; expected 7 or 40 hex characters` }),
        );
      }
      const tag = shortSha(requested);
      const [packageTagsMap, commits] = yield* Effect.all(
        [repoPackageTags, gh.mainCommits(5000)],
        { concurrency: 2 },
      );
      const missing = missingRepoPackages(packageTagsMap, tag);
      if (missing.length) {
        return yield* Effect.fail(
          new GitHubApiError({
            message: `Could not find a complete sha image set for ${tag}; missing: ${missing.join(", ")}`,
          }),
        );
      }
      let floor: number;
      let compatFloor: number;
      try {
        floor = simpleAclFloor(commits);
        compatFloor = shaRuntimeCompatFloor(commits);
      } catch (error) {
        return yield* Effect.fail(
          new GitHubApiError({ message: error instanceof Error ? error.message : String(error) }),
        );
      }
      const index = commits.findIndex((sha) =>
        requested.length === 40
          ? sha.toLowerCase() === requested.toLowerCase()
          : sha.startsWith(tag),
      );
      if (index < 0) {
        return yield* Effect.fail(
          new GitHubApiError({
            message: `sha target ${requested.length === 40 ? requested.toLowerCase() : tag} is unsupported; only main commits at or after ${SIMPLE_ACL_MIN_SHA.slice(0, 7)} are supported`,
          }),
        );
      }
      if (index > floor) {
        return yield* Effect.fail(
          new GitHubApiError({ message: `sha target ${tag} predates the simple-ACL cutover and is unsupported` }),
        );
      }
      if (index > compatFloor) {
        return yield* Effect.fail(
          new GitHubApiError({
            message: `sha target ${tag} predates the modern gw-listener drift-address cutover (${SHA_RUNTIME_COMPAT_MIN_SHA.slice(0, 7)}) and is unsupported by the current CLI; use latest-supported or a newer sha`,
          }),
        );
      }
      return presetBundle(target, tag, `sha-${tag}.json`, [`requested-sha=${requested.toLowerCase()}`]);
    }

    // target === "latest-main"
    const [packageTagsMap, commits] = yield* Effect.all(
      [repoPackageTags, gh.mainCommits(5000)],
      { concurrency: 2 },
    );
    let floor: number;
    let compatFloor: number;
    try {
      floor = simpleAclFloor(commits);
      compatFloor = shaRuntimeCompatFloor(commits);
    } catch (error) {
      return yield* Effect.fail(
        new GitHubApiError({ message: error instanceof Error ? error.message : String(error) }),
      );
    }
    const short = commits
      .slice(0, Math.min(floor, compatFloor) + 1)
      .map((sha) => sha.slice(0, 7))
      .find((sha) => Object.values(packageTagsMap).every((set) => set.has(sha) && REPO_TAG.test(sha)));
    if (!short) {
      return yield* Effect.fail(
        new GitHubApiError({ message: "Could not find a supported modern latest-main image set" }),
      );
    }
    return presetBundle(target, short, `latest-main-${short}.json`);
  });
