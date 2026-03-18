import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";

import { GitHubApiError } from "./errors";
import {
  applyVersionEnvOverrides,
  bundleFromFiles,
  describeBundle,
  extractTag,
  missingRepoPackages,
  PACKAGE_TO_REPOSITORY,
  presetBundle,
  REPO_KEYS,
  REPO_PACKAGES,
  REPO_TAG,
  resolveTarget,
  SHA_REF,
  shortSha,
  simpleAclFloor,
  SIMPLE_ACL_MIN_SHA,
  walkImages,
} from "./resolve";
import { GitHubClient } from "./services/GitHubClient";
import type { VersionBundle } from "./types";

// ---------------------------------------------------------------------------
// YAML fixtures
// ---------------------------------------------------------------------------

const gatewayYaml = `
containers:
  - image: "ghcr.io/zama-ai/fhevm/gateway-contracts:v0.11.0"
`;

const hostYaml = `
containers:
  - image: "ghcr.io/zama-ai/fhevm/host-contracts:v0.11.0"
`;

const coprocessorDbYaml = `
containers:
  - image: "ghcr.io/zama-ai/fhevm/coprocessor/db-migration:v0.11.0"
`;

const coprocessorHostYaml = `
containers:
  - image: "ghcr.io/zama-ai/fhevm/coprocessor/host-listener:v0.11.0"
`;

const coprocessorGatewayYaml = `
containers:
  - image: "ghcr.io/zama-ai/fhevm/coprocessor/gw-listener:v0.11.0"
  - image: "ghcr.io/zama-ai/fhevm/coprocessor/tx-sender:v0.11.0"
`;

const coprocessorWorkersYaml = `
containers:
  - image: "ghcr.io/zama-ai/fhevm/coprocessor/tfhe-worker:v0.11.0"
  - image: "ghcr.io/zama-ai/fhevm/coprocessor/zkproof-worker:v0.11.0"
  - image: "ghcr.io/zama-ai/fhevm/coprocessor/sns-worker:v0.11.0"
`;

const connectorYaml = `
containers:
  - image: "ghcr.io/zama-ai/fhevm/kms-connector/db-migration:v0.11.0"
  - image: "ghcr.io/zama-ai/fhevm/kms-connector/gw-listener:v0.11.0"
  - image: "ghcr.io/zama-ai/fhevm/kms-connector/kms-worker:v0.11.0"
  - image: "ghcr.io/zama-ai/fhevm/kms-connector/tx-sender:v0.11.0"
`;

const kmsCoreYaml = `
containers:
  - image: "ghcr.io/zama-ai/kms/core-service-enclave:v0.13.0"
`;

const relayerYaml = `
containers:
  - image: "ghcr.io/zama-ai/console/relayer:v0.10.0"
  - image: "ghcr.io/zama-ai/console/relayer-migrate:v0.10.0"
`;

const testSuiteYaml = `
containers:
  - image: "ghcr.io/zama-ai/fhevm/test-suite/e2e:v0.11.0"
`;

const yamlByFile: Record<string, string> = {};
const addFile = (pattern: string, yaml: string) => {
  // The fixture maps a substring pattern to yaml content
  yamlByFile[pattern] = yaml;
};
addFile("gw-sc-deploy", gatewayYaml);
addFile("eth-sc-deploy", hostYaml);
addFile("coproc-infra-db-mig", coprocessorDbYaml);
addFile("eth-coproc-listener", coprocessorHostYaml);
addFile("gw-coprocessor", coprocessorGatewayYaml);
addFile("coproc-workers", coprocessorWorkersYaml);
addFile("kms-connector", connectorYaml);
addFile("kms-service", kmsCoreYaml);
addFile("relayer/relayer", relayerYaml);
addFile("fhevm-test-suite", testSuiteYaml);

const gitopsFileStub = (file: string): Effect.Effect<string, GitHubApiError> => {
  for (const [pattern, yaml] of Object.entries(yamlByFile)) {
    if (file.includes(pattern)) {
      return Effect.succeed(yaml);
    }
  }
  return Effect.fail(new GitHubApiError({ message: `No fixture for ${file}` }));
};

// A short SHA that will be present in our fake main commits
const KNOWN_SHA_SHORT = "abc1234";
const KNOWN_SHA_FULL = "abc1234000000000000000000000000000000dead";

// Build a fake commit list where the known SHA is before the simple-ACL floor
const fakeCommits = [
  KNOWN_SHA_FULL,
  "1111111000000000000000000000000000000000",
  SIMPLE_ACL_MIN_SHA,
  "0000000000000000000000000000000000000000",
];

const makeAllPackageTags = (tag: string) =>
  new Set([tag, "v0.11.0", "latest"]);

// ---------------------------------------------------------------------------
// Test layers
// ---------------------------------------------------------------------------

const TestGitHubClientForPreset = Layer.succeed(GitHubClient, {
  latestStableRelease: () => Effect.succeed("v0.11.0"),
  mainCommits: () => Effect.succeed(fakeCommits),
  packageTags: () => Effect.succeed(makeAllPackageTags(KNOWN_SHA_SHORT)),
  gitopsFile: gitopsFileStub,
});

const TestGitHubClientForGitOps = Layer.succeed(GitHubClient, {
  latestStableRelease: () => Effect.succeed("v0.11.0"),
  mainCommits: () => Effect.succeed(fakeCommits),
  packageTags: () => Effect.succeed(new Set<string>()),
  gitopsFile: gitopsFileStub,
});

// ---------------------------------------------------------------------------
// Tests: pure helpers
// ---------------------------------------------------------------------------

describe("walkImages", () => {
  test("extracts image from string format", () => {
    const out: Array<{ repository: string; tag: string }> = [];
    walkImages({ image: "ghcr.io/zama-ai/fhevm/gateway-contracts:v0.11.0" }, out);
    expect(out).toEqual([{ repository: "ghcr.io/zama-ai/fhevm/gateway-contracts", tag: "v0.11.0" }]);
  });

  test("extracts image from object format with name", () => {
    const out: Array<{ repository: string; tag: string }> = [];
    walkImages({ image: { name: "ghcr.io/zama-ai/fhevm/gateway-contracts", tag: "v0.11.0" } }, out);
    expect(out).toEqual([{ repository: "ghcr.io/zama-ai/fhevm/gateway-contracts", tag: "v0.11.0" }]);
  });

  test("extracts image from object format with repository field", () => {
    const out: Array<{ repository: string; tag: string }> = [];
    walkImages({ image: { repository: "ghcr.io/zama-ai/fhevm/gateway-contracts", tag: "v0.11.0" } }, out);
    expect(out).toEqual([{ repository: "ghcr.io/zama-ai/fhevm/gateway-contracts", tag: "v0.11.0" }]);
  });

  test("walks nested arrays", () => {
    const out: Array<{ repository: string; tag: string }> = [];
    walkImages([{ image: "ghcr.io/zama-ai/fhevm/gateway-contracts:v1" }], out);
    expect(out.length).toBe(1);
    expect(out[0].tag).toBe("v1");
  });

  test("walks nested objects", () => {
    const out: Array<{ repository: string; tag: string }> = [];
    walkImages({ containers: { first: { image: "ghcr.io/zama-ai/example:v2" } } }, out);
    expect(out.length).toBe(1);
    expect(out[0].tag).toBe("v2");
  });

  test("normalizes hub.zama.org repository names", () => {
    const out: Array<{ repository: string; tag: string }> = [];
    walkImages({ image: "hub.zama.org/ghcr/zama-ai/fhevm/gateway-contracts:v0.11.0" }, out);
    expect(out[0].repository).toBe("ghcr.io/zama-ai/fhevm/gateway-contracts");
  });

  test("ignores null and primitive nodes", () => {
    const out: Array<{ repository: string; tag: string }> = [];
    walkImages(null, out);
    walkImages(42, out);
    walkImages("hello", out);
    expect(out.length).toBe(0);
  });
});

describe("extractTag", () => {
  test("extracts tag from YAML with matching repository", () => {
    const yaml = 'containers:\n  - image: "ghcr.io/zama-ai/fhevm/gateway-contracts:v0.11.0"\n';
    expect(extractTag(yaml, "ghcr.io/zama-ai/fhevm/gateway-contracts")).toBe("v0.11.0");
  });

  test("throws when repository not found", () => {
    const yaml = 'containers:\n  - image: "ghcr.io/zama-ai/fhevm/gateway-contracts:v0.11.0"\n';
    expect(() => extractTag(yaml, "ghcr.io/zama-ai/fhevm/nonexistent")).toThrow("Missing");
  });
});

describe("shortSha", () => {
  test("truncates to 7 lowercase chars", () => {
    expect(shortSha("ABCDEF1234567890")).toBe("abcdef1");
  });
});

describe("simpleAclFloor", () => {
  test("returns index of SIMPLE_ACL_MIN_SHA", () => {
    const commits = ["aaaa", SIMPLE_ACL_MIN_SHA, "bbbb"];
    expect(simpleAclFloor(commits)).toBe(1);
  });

  test("throws when floor SHA not found", () => {
    expect(() => simpleAclFloor(["aaaa", "bbbb"])).toThrow("simple-acl floor");
  });
});

describe("missingRepoPackages", () => {
  test("returns empty when all packages have the tag", () => {
    const tags: Record<string, Set<string>> = {};
    for (const key of Object.keys(REPO_PACKAGES)) {
      tags[key] = new Set(["abc1234"]);
    }
    expect(missingRepoPackages(tags, "abc1234")).toEqual([]);
  });

  test("returns decoded package names for missing tags", () => {
    const tags: Record<string, Set<string>> = {};
    for (const key of Object.keys(REPO_PACKAGES)) {
      tags[key] = new Set(["abc1234"]);
    }
    // Remove one key
    tags.GATEWAY_VERSION = new Set();
    const result = missingRepoPackages(tags, "abc1234");
    expect(result).toContain("fhevm/gateway-contracts");
  });
});

describe("presetBundle", () => {
  test("builds a bundle with repo-owned version for REPO_KEYS", () => {
    const bundle = presetBundle("latest-main", "v0.11.0", "latest-main-v0.11.0.json");
    for (const key of REPO_KEYS) {
      expect(bundle.env[key]).toBe("v0.11.0");
    }
  });

  test("uses NON_NETWORK_COMPANIONS for non-repo keys", () => {
    const bundle = presetBundle("latest-main", "v0.11.0", "test.json");
    expect(bundle.env.CORE_VERSION).toBeTruthy();
    expect(bundle.env.RELAYER_VERSION).toBeTruthy();
    expect(bundle.env.RELAYER_MIGRATE_VERSION).toBeTruthy();
  });

  test("includes sources", () => {
    const bundle = presetBundle("sha", "abc1234", "sha-abc1234.json", ["extra=source"]);
    expect(bundle.sources).toContain("preset=sha");
    expect(bundle.sources).toContain("repo-owned=abc1234");
    expect(bundle.sources).toContain("extra=source");
  });
});

describe("SHA_REF", () => {
  test("matches 7-char hex", () => {
    expect(SHA_REF.test("abc1234")).toBe(true);
  });

  test("matches 40-char hex", () => {
    expect(SHA_REF.test("abc1234567890abc1234567890abc123456789a0")).toBe(true);
  });

  test("rejects non-hex", () => {
    expect(SHA_REF.test("xyz1234")).toBe(false);
  });

  test("rejects wrong length", () => {
    expect(SHA_REF.test("abc12")).toBe(false);
    expect(SHA_REF.test("abc12345")).toBe(false);
  });
});

describe("REPO_TAG", () => {
  test("matches 7-char lowercase hex", () => {
    expect(REPO_TAG.test("abc1234")).toBe(true);
  });

  test("rejects uppercase", () => {
    expect(REPO_TAG.test("ABC1234")).toBe(false);
  });
});

// ---------------------------------------------------------------------------
// Tests: applyVersionEnvOverrides (pure)
// ---------------------------------------------------------------------------

describe("applyVersionEnvOverrides", () => {
  const baseBundle: VersionBundle = {
    target: "latest-main",
    lockName: "test.json",
    env: {
      GATEWAY_VERSION: "v0.11.0",
      HOST_VERSION: "v0.11.0",
    },
    sources: ["preset=latest-main"],
  };

  test("returns original bundle when no overrides match", () => {
    const result = applyVersionEnvOverrides(baseBundle, { UNRELATED_KEY: "value" });
    expect(result).toBe(baseBundle);
  });

  test("returns original bundle when override values are empty strings", () => {
    const result = applyVersionEnvOverrides(baseBundle, { GATEWAY_VERSION: "" });
    expect(result).toBe(baseBundle);
  });

  test("applies matching overrides", () => {
    const result = applyVersionEnvOverrides(baseBundle, { GATEWAY_VERSION: "custom-v1" });
    expect(result.env.GATEWAY_VERSION).toBe("custom-v1");
    expect(result.env.HOST_VERSION).toBe("v0.11.0");
  });

  test("adds env source entry with sorted keys", () => {
    const result = applyVersionEnvOverrides(baseBundle, {
      HOST_VERSION: "custom-host",
      GATEWAY_VERSION: "custom-gw",
    });
    expect(result.sources).toContain("env=GATEWAY_VERSION,HOST_VERSION");
  });

  test("does not mutate original bundle", () => {
    applyVersionEnvOverrides(baseBundle, { GATEWAY_VERSION: "custom-v1" });
    expect(baseBundle.env.GATEWAY_VERSION).toBe("v0.11.0");
  });
});

// ---------------------------------------------------------------------------
// Tests: describeBundle (pure)
// ---------------------------------------------------------------------------

describe("describeBundle", () => {
  test("formats env entries as KEY=VALUE lines", () => {
    const bundle: VersionBundle = {
      target: "latest-main",
      lockName: "test.json",
      env: { A: "1", B: "2" },
      sources: [],
    };
    expect(describeBundle(bundle)).toBe("A=1\nB=2");
  });
});

// ---------------------------------------------------------------------------
// Tests: resolveTarget (Effect-based)
// ---------------------------------------------------------------------------

describe("resolveTarget", () => {
  test("latest-supported returns the tracked baseline profile", async () => {
    const program = resolveTarget("latest-supported");
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClientForPreset)));
    expect(result.target).toBe("latest-supported");
    expect(result.env.GATEWAY_VERSION).toBe("v0.11.0");
    expect(result.lockName).toBe("latest-supported.json");
    expect(result.sources).toContain("profile=latest-supported");
  });

  test("latest-main finds newest commit with full image coverage", async () => {
    const program = resolveTarget("latest-main");
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClientForPreset)));
    expect(result.target).toBe("latest-main");
    expect(result.env.GATEWAY_VERSION).toBe(KNOWN_SHA_SHORT);
    expect(result.lockName).toBe(`latest-main-${KNOWN_SHA_SHORT}.json`);
  });

  test("sha target resolves with valid SHA", async () => {
    const program = resolveTarget("sha", { sha: KNOWN_SHA_SHORT });
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClientForPreset)));
    expect(result.target).toBe("sha");
    expect(result.env.GATEWAY_VERSION).toBe(KNOWN_SHA_SHORT);
    expect(result.lockName).toBe(`sha-${KNOWN_SHA_SHORT}.json`);
    expect(result.sources).toContain(`requested-sha=${KNOWN_SHA_SHORT}`);
  });

  test("sha target rejects missing SHA", async () => {
    const program = resolveTarget("sha");
    const result = await Effect.runPromise(
      program.pipe(Effect.provide(TestGitHubClientForPreset), Effect.either),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left.message).toContain("--target sha requires --sha");
    }
  });

  test("sha target rejects invalid format", async () => {
    const program = resolveTarget("sha", { sha: "xyz" });
    const result = await Effect.runPromise(
      program.pipe(Effect.provide(TestGitHubClientForPreset), Effect.either),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left.message).toContain("Invalid sha");
    }
  });

  test("sha target rejects when packages are missing", async () => {
    const MissingPackageClient = Layer.succeed(GitHubClient, {
      latestStableRelease: () => Effect.succeed("v0.11.0"),
      mainCommits: () => Effect.succeed(fakeCommits),
      packageTags: () => Effect.succeed(new Set<string>()), // no tags
      gitopsFile: gitopsFileStub,
    });
    const program = resolveTarget("sha", { sha: KNOWN_SHA_SHORT });
    const result = await Effect.runPromise(
      program.pipe(Effect.provide(MissingPackageClient), Effect.either),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left.message).toContain("Could not find a complete sha image set");
    }
  });

  test("sha target rejects when commit not found in main history", async () => {
    const NotOnMainClient = Layer.succeed(GitHubClient, {
      latestStableRelease: () => Effect.succeed("v0.11.0"),
      mainCommits: () => Effect.succeed([SIMPLE_ACL_MIN_SHA]), // known SHA not present
      packageTags: () => Effect.succeed(makeAllPackageTags("dead000")),
      gitopsFile: gitopsFileStub,
    });
    const program = resolveTarget("sha", { sha: "dead000" });
    const result = await Effect.runPromise(
      program.pipe(Effect.provide(NotOnMainClient), Effect.either),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left.message).toContain("unsupported");
    }
  });

  test("sha target rejects commits older than simple-ACL floor", async () => {
    // Put our commit AFTER the floor SHA
    const commitsWithOldSha = [
      SIMPLE_ACL_MIN_SHA,
      "dead000000000000000000000000000000000000",
    ];
    const OldShaClient = Layer.succeed(GitHubClient, {
      latestStableRelease: () => Effect.succeed("v0.11.0"),
      mainCommits: () => Effect.succeed(commitsWithOldSha),
      packageTags: () => Effect.succeed(makeAllPackageTags("dead000")),
      gitopsFile: gitopsFileStub,
    });
    const program = resolveTarget("sha", { sha: "dead000" });
    const result = await Effect.runPromise(
      program.pipe(Effect.provide(OldShaClient), Effect.either),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left.message).toContain("predates the simple-ACL cutover");
    }
  });

  test("devnet resolves from GitOps files", async () => {
    const program = resolveTarget("devnet");
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClientForGitOps)));
    expect(result.target).toBe("devnet");
    expect(result.lockName).toBe("devnet.json");
    expect(result.env.GATEWAY_VERSION).toBe("v0.11.0");
    expect(result.env.CORE_VERSION).toBe("v0.13.0");
  });

  test("testnet resolves from GitOps files", async () => {
    const program = resolveTarget("testnet");
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClientForGitOps)));
    expect(result.target).toBe("testnet");
    expect(result.lockName).toBe("testnet.json");
    expect(result.env.GATEWAY_VERSION).toBe("v0.11.0");
  });

  test("mainnet resolves from GitOps files", async () => {
    const program = resolveTarget("mainnet");
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClientForGitOps)));
    expect(result.target).toBe("mainnet");
    expect(result.lockName).toBe("mainnet.json");
    expect(result.env.GATEWAY_VERSION).toBe("v0.11.0");
  });

  test("latest-main fails when no matching commit found", async () => {
    const NoMatchClient = Layer.succeed(GitHubClient, {
      latestStableRelease: () => Effect.succeed("v0.11.0"),
      mainCommits: () => Effect.succeed([SIMPLE_ACL_MIN_SHA]),
      packageTags: () => Effect.succeed(new Set<string>()), // no tags match anything
      gitopsFile: gitopsFileStub,
    });
    const program = resolveTarget("latest-main");
    const result = await Effect.runPromise(
      program.pipe(Effect.provide(NoMatchClient), Effect.either),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left.message).toContain("Could not find a supported modern latest-main image set");
    }
  });
});

// ---------------------------------------------------------------------------
// Tests: bundleFromFiles (Effect-based)
// ---------------------------------------------------------------------------

describe("bundleFromFiles", () => {
  const files = {
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

  test("produces VersionBundle from all value files", async () => {
    const program = bundleFromFiles("devnet", files);
    const result = await Effect.runPromise(program.pipe(Effect.provide(TestGitHubClientForGitOps)));
    expect(result.target).toBe("devnet");
    expect(result.lockName).toBe("devnet.json");
    // All PACKAGE_TO_REPOSITORY keys should be present
    for (const key of Object.keys(PACKAGE_TO_REPOSITORY)) {
      expect(result.env[key]).toBeTruthy();
    }
  });

  test("fails with GitHubApiError when tag extraction fails", async () => {
    const BadYamlClient = Layer.succeed(GitHubClient, {
      latestStableRelease: () => Effect.succeed("v0.11.0"),
      mainCommits: () => Effect.succeed([]),
      packageTags: () => Effect.succeed(new Set<string>()),
      gitopsFile: () => Effect.succeed("empty: true"), // no image entries
    });
    const program = bundleFromFiles("devnet", files);
    const result = await Effect.runPromise(
      program.pipe(Effect.provide(BadYamlClient), Effect.either),
    );
    expect(result._tag).toBe("Left");
    if (result._tag === "Left") {
      expect(result.left).toBeInstanceOf(GitHubApiError);
      expect(result.left.message).toContain("Missing");
    }
  });
});
