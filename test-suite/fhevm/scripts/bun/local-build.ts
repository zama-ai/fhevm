import fs from "node:fs";
import path from "node:path";
import type { RunCommandFn } from "./process";
import type { DeployOptions } from "./types";

type LocalBuildHandlersDeps = {
  FHEVM_DIR: string;
  PROJECT: string;
  WORKTREE_BUILD_CONTEXT_CACHE_RELATIVE_DIR: string;
  WORKTREE_BUILD_CONTEXT_ENV: string;
  LOCAL_CACHE_SERVICES: string[];
  runCommand: RunCommandFn;
  logInfo: (message: string) => void;
  logWarn: (message: string) => void;
};

export function createLocalBuildHandlers(deps: LocalBuildHandlersDeps) {
  const {
    FHEVM_DIR,
    PROJECT,
    WORKTREE_BUILD_CONTEXT_CACHE_RELATIVE_DIR,
    WORKTREE_BUILD_CONTEXT_ENV,
    LOCAL_CACHE_SERVICES,
    runCommand,
    logInfo,
    logWarn,
  } = deps;

  const WORKTREE_BUILD_CONTEXT_CACHE_DIR = path.resolve(FHEVM_DIR, WORKTREE_BUILD_CONTEXT_CACHE_RELATIVE_DIR);
  const buildComposePathCache = new Map<string, string>();
  const generatedBuildComposeFiles = new Set<string>();

  function resolveLocalBuildxCacheRoot(): string {
    const configured = process.env.FHEVM_BUILDX_CACHE_DIR ?? ".buildx-cache";
    if (path.isAbsolute(configured)) {
      return configured;
    }
    return path.resolve(FHEVM_DIR, configured);
  }

  function resolveBuildId(): string {
    const explicit = process.env.FHEVM_BUILD_ID?.trim();
    if (explicit) {
      return explicit;
    }

    const gitHead = runCommand(["git", "rev-parse", "HEAD"], {
      cwd: FHEVM_DIR,
      capture: true,
      check: false,
      allowFailure: true,
    });
    const candidate = gitHead.stdout.trim();
    if (gitHead.status === 0 && /^[0-9a-f]{40}$/i.test(candidate)) {
      return candidate;
    }

    return "unknown";
  }

  function configureBuildId(): void {
    const buildId = resolveBuildId();
    process.env.FHEVM_BUILD_ID = buildId;

    if (buildId === "unknown") {
      logWarn("Could not resolve git commit SHA. Using BUILD_ID=unknown for local image builds.");
      return;
    }

    logInfo(`Using BUILD_ID=${buildId.slice(0, 12)} for local image builds.`);
  }

  function isGitWorktreeCheckout(): boolean {
    const dotGitPath = path.resolve(FHEVM_DIR, ".git");
    if (!fs.existsSync(dotGitPath) || !fs.statSync(dotGitPath).isFile()) {
      return false;
    }

    const dotGit = fs.readFileSync(dotGitPath, "utf8").trim();
    if (!dotGit.startsWith("gitdir:")) {
      return false;
    }

    return /[\\/]worktrees[\\/]/.test(dotGit);
  }

  function prepareWorktreeBuildContextIfNeeded(options: DeployOptions): boolean {
    if (!options.localBuild || !isGitWorktreeCheckout()) {
      return false;
    }

    const existing = process.env[WORKTREE_BUILD_CONTEXT_ENV]?.trim();
    if (existing) {
      return false;
    }

    fs.mkdirSync(WORKTREE_BUILD_CONTEXT_CACHE_DIR, { recursive: true });
    const contextRoot = fs.mkdtempSync(path.join(WORKTREE_BUILD_CONTEXT_CACHE_DIR, `${PROJECT}-`));

    logWarn("Detected git worktree + --local. Preparing temporary Docker build context.");

    try {
      runCommand(
        [
          "rsync",
          "-a",
          "--delete",
          "--exclude",
          ".git",
          "--exclude",
          WORKTREE_BUILD_CONTEXT_CACHE_RELATIVE_DIR,
          `${FHEVM_DIR}/`,
          `${contextRoot}/`,
        ],
        { check: true },
      );

      const commonGitDir = runCommand(["git", "rev-parse", "--git-common-dir"], {
        cwd: FHEVM_DIR,
        capture: true,
        check: true,
      }).stdout.trim();
      const worktreeGitDir = runCommand(["git", "rev-parse", "--git-dir"], {
        cwd: FHEVM_DIR,
        capture: true,
        check: true,
      }).stdout.trim();

      const commonGitDirAbs = path.resolve(FHEVM_DIR, commonGitDir);
      const worktreeGitDirAbs = path.resolve(FHEVM_DIR, worktreeGitDir);
      const contextGitDir = path.join(contextRoot, ".git");
      fs.mkdirSync(contextGitDir, { recursive: true });

      runCommand(["rsync", "-a", "--delete", `${commonGitDirAbs}/`, `${contextGitDir}/`], { check: true });

      const headSource = path.join(worktreeGitDirAbs, "HEAD");
      const headTarget = path.join(contextGitDir, "HEAD");
      fs.copyFileSync(headSource, headTarget);

      process.env[WORKTREE_BUILD_CONTEXT_ENV] = contextRoot;
      logInfo(`Using temporary build context: ${contextRoot}`);
      return true;
    } catch (error) {
      fs.rmSync(contextRoot, { recursive: true, force: true });
      throw error;
    }
  }

  function resolveComposeForBuild(composePath: string, useBuild: boolean): string {
    if (!useBuild) {
      return composePath;
    }

    const contextRoot = process.env[WORKTREE_BUILD_CONTEXT_ENV]?.trim();
    if (!contextRoot) {
      return composePath;
    }

    const cached = buildComposePathCache.get(composePath);
    if (cached) {
      return cached;
    }

    const source = fs.readFileSync(composePath, "utf8");
    const rewritten = source.replace(
      /^(\s*context:\s*)\.\.\/\.\.\/\.\.(\s*(#.*)?)?$/gm,
      (_match, prefix: string, suffix = "") => `${prefix}${contextRoot}${suffix}`,
    );

    if (rewritten === source) {
      buildComposePathCache.set(composePath, composePath);
      return composePath;
    }

    const overridePath = `${composePath}.worktree.local.yml`;
    fs.writeFileSync(overridePath, rewritten, "utf8");
    buildComposePathCache.set(composePath, overridePath);
    generatedBuildComposeFiles.add(overridePath);
    return overridePath;
  }

  function cleanupWorktreeBuildArtifacts(): void {
    for (const overridePath of generatedBuildComposeFiles) {
      if (fs.existsSync(overridePath)) {
        fs.rmSync(overridePath, { force: true });
      }
    }
    generatedBuildComposeFiles.clear();
    buildComposePathCache.clear();

    const contextRoot = process.env[WORKTREE_BUILD_CONTEXT_ENV]?.trim();
    if (contextRoot) {
      fs.rmSync(contextRoot, { recursive: true, force: true });
      delete process.env[WORKTREE_BUILD_CONTEXT_ENV];
    }
  }

  function configureLocalBuild(): void {
    logInfo("Enabling local BuildKit cache and disabling provenance attestations.");
    process.env.DOCKER_BUILDKIT = "1";
    process.env.COMPOSE_DOCKER_CLI_BUILD = "1";
    process.env.BUILDX_NO_DEFAULT_ATTESTATIONS = "1";
    process.env.DOCKER_BUILD_PROVENANCE = "false";
    process.env.FHEVM_CARGO_PROFILE = "local";

    const cacheRoot = resolveLocalBuildxCacheRoot();
    process.env.FHEVM_BUILDX_CACHE_DIR = cacheRoot;
    fs.mkdirSync(cacheRoot, { recursive: true });

    const setLocalCache = (serviceName: string): void => {
      const serviceKey = serviceName.replace(/-/g, "_").toUpperCase();
      const cacheDir = path.resolve(cacheRoot, serviceName);
      fs.mkdirSync(cacheDir, { recursive: true });
      process.env[`FHEVM_CACHE_FROM_${serviceKey}`] = `type=local,src=${cacheDir}`;
      process.env[`FHEVM_CACHE_TO_${serviceKey}`] = `type=local,dest=${cacheDir},mode=max`;
    };

    const coprocessorCacheDir = path.resolve(cacheRoot, "coprocessor");
    fs.mkdirSync(coprocessorCacheDir, { recursive: true });
    process.env.FHEVM_CACHE_FROM_COPROCESSOR = `type=local,src=${coprocessorCacheDir}`;
    process.env.FHEVM_CACHE_TO_COPROCESSOR = `type=local,dest=${coprocessorCacheDir},mode=max`;

    const kmsConnectorCacheDir = path.resolve(cacheRoot, "kms-connector");
    fs.mkdirSync(kmsConnectorCacheDir, { recursive: true });
    process.env.FHEVM_CACHE_FROM_KMS_CONNECTOR = `type=local,src=${kmsConnectorCacheDir}`;
    process.env.FHEVM_CACHE_TO_KMS_CONNECTOR = `type=local,dest=${kmsConnectorCacheDir},mode=max`;

    for (const service of LOCAL_CACHE_SERVICES) {
      setLocalCache(service);
    }
  }

  return {
    resolveLocalBuildxCacheRoot,
    configureBuildId,
    prepareWorktreeBuildContextIfNeeded,
    resolveComposeForBuild,
    cleanupWorktreeBuildArtifacts,
    configureLocalBuild,
  };
}
