import YAML from "yaml";

import { NON_NETWORK_COMPANIONS } from "./presets";
import type { Runner, RunResult } from "./utils";
import { normalizeRepository, toError } from "./utils";
import type { VersionBundle, VersionTarget } from "./types";

type GitHubClient = {
  latestStableRelease(): Promise<string>;
  mainCommits(limit?: number): Promise<string[]>;
  packageTags(pkg: string): Promise<Set<string>>;
  gitopsFile(file: string): Promise<string>;
};

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
};

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
};

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
};

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
  CONNECTOR_DB_MIGRATION_VERSION: "fhevm%2Fkms-connector%2Fdb-migration",
  CONNECTOR_GW_LISTENER_VERSION: "fhevm%2Fkms-connector%2Fgw-listener",
  CONNECTOR_KMS_WORKER_VERSION: "fhevm%2Fkms-connector%2Fkms-worker",
  CONNECTOR_TX_SENDER_VERSION: "fhevm%2Fkms-connector%2Ftx-sender",
  TEST_SUITE_VERSION: "fhevm%2Ftest-suite%2Fe2e",
} as const;

const REPO_KEYS = new Set(Object.keys(REPO_PACKAGES));

const PACKAGE_TO_REPOSITORY = {
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

const GH_OWNER = "zama-ai";
const FHEVM_REPO = "zama-ai/fhevm";
const GITOPS_REPO = "zama-zws/gitops";

const parseJson = <T>(value: RunResult) => JSON.parse(value.stdout) as T;

const explainGitHubCliError = (error: unknown) => {
  const message = toError(error).message;
  const lower = message.toLowerCase();
  if (
    message.includes("which gh failed") ||
    lower.includes("spawn gh") ||
    lower.includes("enoent") ||
    lower.includes("gh: command not found")
  ) {
    return new Error(
      "GitHub CLI `gh` is required for target resolution. Install `gh`, authenticate with `gh auth login` or GH_TOKEN, or use --lock-file to skip GitHub resolution.",
    );
  }
  if (
    lower.includes("authentication failed") ||
    lower.includes("authentication required") ||
    lower.includes("gh auth login") ||
    lower.includes("http 401") ||
    lower.includes("requires authentication")
  ) {
    return new Error(
      "GitHub API access is not authenticated. Run `gh auth login`, export GH_TOKEN, or use --lock-file to skip GitHub resolution.",
    );
  }
  if (
    lower.includes("rate limit") ||
    lower.includes("secondary rate limit") ||
    lower.includes("api rate limit exceeded") ||
    lower.includes("http 429")
  ) {
    return new Error(
      "GitHub API rate limit hit while resolving versions. Retry with an authenticated GH_TOKEN or use --lock-file to run with a pinned bundle.",
    );
  }
  return error;
};

const runGhApi = async (runner: Runner, path: string) => {
  try {
    return await runner(["gh", "api", path]);
  } catch (error) {
    throw explainGitHubCliError(error);
  }
};

const ghPages = async <T>(runner: Runner, path: string, limit = 1000) => {
  const items: T[] = [];
  let page = 1;
  while (items.length < limit) {
    const join = path.includes("?") ? "&" : "?";
    const payload = parseJson<T[]>(await runGhApi(runner, `${path}${join}per_page=100&page=${page}`));
    if (!payload.length) {
      break;
    }
    items.push(...payload);
    if (payload.length < 100) {
      break;
    }
    page += 1;
  }
  return items.slice(0, limit);
};

export const createGitHubClient = (runner: Runner): GitHubClient => ({
  latestStableRelease: async () => {
    const releases = await ghPages<{ tag_name: string; prerelease: boolean; draft: boolean }>(
      runner,
      `repos/${FHEVM_REPO}/releases`,
      200,
    );
    const release = releases.find((item) => !item.prerelease && !item.draft);
    if (!release) {
      throw new Error("No stable fhevm release found");
    }
    return release.tag_name;
  },
  mainCommits: async (limit = 200) =>
    (await ghPages<{ sha: string }>(runner, `repos/${FHEVM_REPO}/commits?sha=main`, limit)).map(
      (item) => item.sha,
    ),
  packageTags: async (pkg) => {
    const versions = await ghPages<{ metadata?: { container?: { tags?: string[] } } }>(
      runner,
      `/orgs/${GH_OWNER}/packages/container/${pkg}/versions`,
      1000,
    );
    return new Set(
      versions.flatMap((item) => item.metadata?.container?.tags ?? []).filter(Boolean),
    );
  },
  gitopsFile: async (file) => {
    const payload = parseJson<{ content: string }>(
      await runGhApi(runner, `repos/${GITOPS_REPO}/contents/${file}?ref=main`),
    );
    return Buffer.from(payload.content.replace(/\n/g, ""), "base64").toString("utf8");
  },
});

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

const extractTag = (text: string, repository: string) => {
  const images: Array<{ repository: string; tag: string }> = [];
  walkImages(YAML.parse(text), images);
  const hit = images.find((image) => image.repository === repository);
  if (!hit) {
    throw new Error(`Missing ${repository} in GitOps file`);
  }
  return hit.tag;
};

const bundleFromFiles = async (client: GitHubClient, target: VersionTarget, files: Record<string, string>) => {
  const payload = await Promise.all(
    Object.entries(files).map(async ([name, file]) => [name, await client.gitopsFile(file)] as const),
  );
  const docs = Object.fromEntries(payload);
  const env = {
    GATEWAY_VERSION: extractTag(docs.gateway, PACKAGE_TO_REPOSITORY.GATEWAY_VERSION),
    HOST_VERSION: extractTag(docs.host, PACKAGE_TO_REPOSITORY.HOST_VERSION),
    COPROCESSOR_DB_MIGRATION_VERSION: extractTag(
      docs.coprocessorDb,
      PACKAGE_TO_REPOSITORY.COPROCESSOR_DB_MIGRATION_VERSION,
    ),
    COPROCESSOR_HOST_LISTENER_VERSION: extractTag(
      docs.coprocessorHost,
      PACKAGE_TO_REPOSITORY.COPROCESSOR_HOST_LISTENER_VERSION,
    ),
    COPROCESSOR_GW_LISTENER_VERSION: extractTag(
      docs.coprocessorGateway,
      PACKAGE_TO_REPOSITORY.COPROCESSOR_GW_LISTENER_VERSION,
    ),
    COPROCESSOR_TX_SENDER_VERSION: extractTag(
      docs.coprocessorGateway,
      PACKAGE_TO_REPOSITORY.COPROCESSOR_TX_SENDER_VERSION,
    ),
    COPROCESSOR_TFHE_WORKER_VERSION: extractTag(
      docs.coprocessorWorkers,
      PACKAGE_TO_REPOSITORY.COPROCESSOR_TFHE_WORKER_VERSION,
    ),
    COPROCESSOR_ZKPROOF_WORKER_VERSION: extractTag(
      docs.coprocessorWorkers,
      PACKAGE_TO_REPOSITORY.COPROCESSOR_ZKPROOF_WORKER_VERSION,
    ),
    COPROCESSOR_SNS_WORKER_VERSION: extractTag(
      docs.coprocessorWorkers,
      PACKAGE_TO_REPOSITORY.COPROCESSOR_SNS_WORKER_VERSION,
    ),
    CONNECTOR_DB_MIGRATION_VERSION: extractTag(
      docs.connector,
      PACKAGE_TO_REPOSITORY.CONNECTOR_DB_MIGRATION_VERSION,
    ),
    CONNECTOR_GW_LISTENER_VERSION: extractTag(
      docs.connector,
      PACKAGE_TO_REPOSITORY.CONNECTOR_GW_LISTENER_VERSION,
    ),
    CONNECTOR_KMS_WORKER_VERSION: extractTag(
      docs.connector,
      PACKAGE_TO_REPOSITORY.CONNECTOR_KMS_WORKER_VERSION,
    ),
    CONNECTOR_TX_SENDER_VERSION: extractTag(
      docs.connector,
      PACKAGE_TO_REPOSITORY.CONNECTOR_TX_SENDER_VERSION,
    ),
    CORE_VERSION: extractTag(docs.kmsCore, PACKAGE_TO_REPOSITORY.CORE_VERSION),
    RELAYER_VERSION: extractTag(docs.relayer, PACKAGE_TO_REPOSITORY.RELAYER_VERSION),
    RELAYER_MIGRATE_VERSION: extractTag(docs.relayer, PACKAGE_TO_REPOSITORY.RELAYER_MIGRATE_VERSION),
    TEST_SUITE_VERSION: extractTag(docs.testSuite, PACKAGE_TO_REPOSITORY.TEST_SUITE_VERSION),
  };
  return {
    target,
    lockName: `${target}.json`,
    env,
    sources: Object.values(files),
  } satisfies VersionBundle;
};

const REPO_TAG = /^[0-9a-f]{7}$/;
const SHA_REF = /^(?:[0-9a-f]{7}|[0-9a-f]{40})$/i;
const SIMPLE_ACL_MIN_SHA = "803f1048727eabf6d8b3df618203e3c7dda77890";

const repoPackageName = (pkg: string) => decodeURIComponent(pkg);

const repoPackageTags = async (client: GitHubClient) =>
  Object.fromEntries(
    await Promise.all(
      Object.entries(REPO_PACKAGES).map(async ([key, pkg]) => [key, await client.packageTags(pkg)] as const),
    ),
  );

const missingRepoPackages = (packageTags: Record<string, Set<string>>, tag: string) =>
  Object.entries(REPO_PACKAGES)
    .filter(([key]) => !packageTags[key]?.has(tag))
    .map(([, pkg]) => repoPackageName(pkg));

const shortSha = (value: string) => value.toLowerCase().slice(0, 7);

const simpleAclFloor = (commits: string[]) => {
  const floor = commits.indexOf(SIMPLE_ACL_MIN_SHA);
  if (floor < 0) {
    throw new Error(
      `simple-acl floor ${SIMPLE_ACL_MIN_SHA} was not found in fetched main history; increase the main history fetch window`,
    );
  }
  return floor;
};

const presetBundle = (
  target: "latest-release" | "latest-main" | "sha",
  repoVersion: string,
  lockName: string,
  sources: string[] = [],
) => ({
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
}) satisfies VersionBundle;

export const resolveTarget = async (
  target: VersionTarget,
  client: GitHubClient,
  options: { sha?: string } = {},
): Promise<VersionBundle> => {
  if (target === "devnet") {
    return bundleFromFiles(client, target, DEVNET_FILES);
  }
  if (target === "testnet") {
    return bundleFromFiles(client, target, TESTNET_FILES);
  }
  if (target === "mainnet") {
    return bundleFromFiles(client, target, MAINNET_FILES);
  }

  if (target === "latest-release") {
    const release = await client.latestStableRelease();
    return presetBundle(target, release, `latest-release-${release}.json`);
  }

  if (target === "sha") {
    const requested = options.sha?.trim();
    if (!requested) {
      throw new Error("--target sha requires --sha");
    }
    if (!SHA_REF.test(requested)) {
      throw new Error(`Invalid sha ${requested}; expected 7 or 40 hex characters`);
    }
    const tag = shortSha(requested);
    const packageTags = await repoPackageTags(client);
    const missing = missingRepoPackages(packageTags, tag);
    if (missing.length) {
      throw new Error(`Could not find a complete sha image set for ${tag}; missing: ${missing.join(", ")}`);
    }
    const commits = await client.mainCommits(5000);
    const floor = simpleAclFloor(commits);
    const index = commits.findIndex((sha) => sha.startsWith(tag));
    if (index < 0) {
      throw new Error(`sha target ${tag} is unsupported; only main commits at or after ${SIMPLE_ACL_MIN_SHA.slice(0, 7)} are supported`);
    }
    if (index > floor) {
      throw new Error(`sha target ${tag} predates the simple-ACL cutover and is unsupported`);
    }
    return presetBundle(target, tag, `sha-${tag}.json`, [`requested-sha=${requested.toLowerCase()}`]);
  }
  const packageTags = await repoPackageTags(client);
  const commits = await client.mainCommits(5000);
  const floor = simpleAclFloor(commits);
  const short = commits.slice(0, floor + 1).map((sha) => sha.slice(0, 7)).find((sha) =>
    Object.values(packageTags).every((set) => set.has(sha) && REPO_TAG.test(sha)),
  );
  if (!short) {
    throw new Error("Could not find a supported modern latest-main image set");
  }
  return presetBundle(target, short, `latest-main-${short}.json`);
};

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
