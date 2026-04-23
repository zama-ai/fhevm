/**
 * Uses local git history and checked-in defaults to resolve branch-aware sha baselines.
 */
import { GitHubApiError } from "../errors";
import { run } from "../utils/process";

const FHEVM_CLI_PATH = "test-suite/fhevm/fhevm-cli";
const LATEST_SUPPORTED_PROFILE_PATH = "test-suite/fhevm/profiles/latest-supported.json";
const SAFE_REF = /^[A-Za-z0-9._/-]+$/;

const remoteRef = (ref: string) => `origin/${ref}`;

const git = async (args: string[], allowFailure = false) => run(["git", ...args], { allowFailure });

const requireRef = (ref: string) => {
  if (!SAFE_REF.test(ref)) {
    throw new GitHubApiError(`Invalid git ref ${ref}`);
  }
  return ref;
};

const ensureRemoteRef = async (ref: string) => {
  const safeRef = requireRef(ref);
  const remote = remoteRef(safeRef);
  const probe = await git(["rev-parse", "--verify", remote], true);
  if (probe.code === 0) {
    return remote;
  }
  const fetched = await git(["fetch", "origin", safeRef], true);
  if (fetched.code === 0) {
    return remote;
  }
  throw new GitHubApiError(`Could not fetch git ref ${safeRef}`);
};

const gitShow = async (object: string) => {
  const shown = await git(["show", object], true);
  if (shown.code !== 0) {
    return undefined;
  }
  return shown.stdout;
};

const parseLegacyExport = (text: string, key: string) =>
  text.match(new RegExp(`^export ${key}=\\$\\{${key}:-"([^"]+)"\\}`, "m"))?.[1];

/** Returns recent commits for one git ref, fetching the ref first when needed. */
export const refCommits = async (ref: string, limit = 5000) => {
  const remote = await ensureRemoteRef(ref);
  const listed = await git(["rev-list", `--max-count=${String(limit)}`, remote], true);
  if (listed.code !== 0) {
    throw new GitHubApiError(`Could not read git history for ${ref}`);
  }
  return listed.stdout.split("\n").map((line) => line.trim()).filter(Boolean);
};

/** Ensures a commit belongs to the selected git ref. */
export const assertCommitOnRef = async (ref: string, commit: string) => {
  const remote = await ensureRemoteRef(ref);
  const checked = await git(["merge-base", "--is-ancestor", commit, remote], true);
  if (checked.code !== 0) {
    throw new GitHubApiError(`sha target ${commit.toLowerCase()} is not contained in ${ref}`);
  }
};

/** Reads branch-default version pins from the selected commit. */
export const baselineDefaults = async (commit: string) => {
  const profileText = await gitShow(`${commit}:${LATEST_SUPPORTED_PROFILE_PATH}`);
  if (profileText) {
    try {
      return JSON.parse(profileText).env as Record<string, string>;
    } catch (error) {
      throw new GitHubApiError(`Failed to parse ${LATEST_SUPPORTED_PROFILE_PATH} at ${commit}: ${error}`);
    }
  }

  const cliText = await gitShow(`${commit}:${FHEVM_CLI_PATH}`);
  if (!cliText) {
    throw new GitHubApiError(`Could not read branch defaults from ${commit}`);
  }
  const defaults = Object.fromEntries(
    ["CORE_VERSION", "RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"]
      .map((key) => [key, parseLegacyExport(cliText, key)] as const)
      .filter(([, value]) => value),
  ) as Record<string, string>;
  if (!Object.keys(defaults).length) {
    throw new GitHubApiError(`Could not parse branch defaults from ${FHEVM_CLI_PATH} at ${commit}`);
  }
  return defaults;
};
