/**
 * Fetches live GitHub and GHCR metadata needed to resolve floating fhevm version targets.
 */
import { GitHubApiError } from "../errors";
import { run } from "../utils/process";

const FHEVM_REPO = "zama-ai/fhevm";
const GITOPS_REPO = "zama-zws/gitops";
const GH_OWNER = "zama-ai";
const GH_API_TIMEOUT_MS = 20_000;
const GH_API_RETRIES = 7;
const GH_API_RETRY_DELAY_MS = 1_000;
const GH_API_RETRY_MAX_DELAY_MS = 16_000;
const GH_API_RATE_LIMIT_RETRY_DELAY_MS = 60_000;
const GH_API_RATE_LIMIT_RETRY_MAX_DELAY_MS = 300_000;
const GH_API_RETRY_JITTER_RATIO = 0.2;
const GH_PACKAGE_VERSION_LIMIT = 5_000;

/** Rewrites raw `gh` failures into actionable user-facing guidance. */
export const explainGitHubCliError = (message: string): string => {
  const lower = message.toLowerCase();
  if (lower.includes("enoent") || lower.includes("not found")) {
    return "GitHub CLI `gh` is required. Install `gh`, authenticate with `gh auth login` or GH_TOKEN, or use `--lock-file` / `--target latest-supported` to avoid GitHub resolution.";
  }
  if (lower.includes("read:packages") || lower.includes("scope to get a package") || (lower.includes("http 403") && lower.includes("package"))) {
    return "GitHub API is missing package-read scope. Run `gh auth refresh -s read:packages`, export GH_TOKEN with `read:packages`, or use `--lock-file` / `--target latest-supported` to avoid GitHub resolution.";
  }
  if (lower.includes("401") || lower.includes("authentication")) {
    return "GitHub API not authenticated. Run `gh auth login`, export GH_TOKEN, or use `--lock-file` / `--target latest-supported` to avoid GitHub resolution.";
  }
  if (lower.includes("rate limit") || lower.includes("429")) {
    return "GitHub API rate limit hit. Retry with authenticated GH_TOKEN or use `--lock-file` / `--target latest-supported` to avoid GitHub resolution.";
  }
  if (lower.includes("timed out")) {
    return "GitHub metadata lookup timed out. Retry, use `--lock-file`, or use `--target latest-supported` to avoid GitHub resolution.";
  }
  return message;
};

/** Runs `gh api` and parses its JSON payload with CLI-specific error handling. */
export const isRateLimitGitHubCliError = (message: string) => {
  const lower = message.toLowerCase();
  return lower.includes("secondary rate limit") || lower.includes("rate limit") || /\bhttp 429\b/.test(lower);
};

export const shouldRetryGitHubCliError = (message: string) => {
  const lower = message.toLowerCase();
  return (
    isRateLimitGitHubCliError(message) ||
    lower.includes("connection refused") ||
    lower.includes("timed out") ||
    lower.includes("tls handshake timeout") ||
    lower.includes("temporary failure") ||
    lower.includes("connection reset") ||
    lower.includes("econnreset") ||
    /\bhttp 5\d\d\b/.test(lower) ||
    lower.includes("service unavailable") ||
    lower.includes("no server is currently available to service your request") ||
    lower.includes("bad gateway") ||
    lower.includes("gateway timeout")
  );
};

export const retryDelayMs = (attempt: number, rateLimited = false) => {
  const initialDelay = rateLimited ? GH_API_RATE_LIMIT_RETRY_DELAY_MS : GH_API_RETRY_DELAY_MS;
  const maxDelay = rateLimited ? GH_API_RATE_LIMIT_RETRY_MAX_DELAY_MS : GH_API_RETRY_MAX_DELAY_MS;
  const base = Math.min(initialDelay * 2 ** (attempt - 1), maxDelay);
  return base + Math.floor(base * GH_API_RETRY_JITTER_RATIO * Math.random());
};

const retryReason = (message: string) => {
  const lower = message.toLowerCase();
  if (lower.includes("secondary rate limit")) return "secondary rate limit";
  if (lower.includes("rate limit") || /\bhttp 429\b/.test(lower)) return "rate limit";
  if (/\bhttp 5\d\d\b/.test(lower)) return "GitHub API 5xx";
  if (lower.includes("timed out") || lower.includes("tls handshake timeout")) return "timeout";
  if (lower.includes("connection reset") || lower.includes("econnreset")) return "connection reset";
  if (lower.includes("connection refused")) return "connection refused";
  if (lower.includes("temporary failure")) return "temporary failure";
  return "transient GitHub API error";
};

const runGhApi = async <T>(apiPath: string): Promise<T> => {
  for (let attempt = 1; attempt <= GH_API_RETRIES; attempt += 1) {
    try {
      const result = await run(["gh", "api", apiPath], { timeoutMs: GH_API_TIMEOUT_MS });
      return JSON.parse(result.stdout) as T;
    } catch (error) {
      const raw = error instanceof Error ? error.message : String(error);
      if (attempt < GH_API_RETRIES && shouldRetryGitHubCliError(raw)) {
        const delay = retryDelayMs(attempt, isRateLimitGitHubCliError(raw));
        console.log(
          `[resolve] gh api retry ${attempt}/${GH_API_RETRIES - 1} after ${retryReason(raw)}; waiting ${(delay / 1000).toFixed(1)}s`,
        );
        await Bun.sleep(delay);
        continue;
      }
      throw new GitHubApiError(explainGitHubCliError(raw));
    }
  }
  throw new GitHubApiError("GitHub metadata lookup failed");
};

/** Fetches paginated GitHub API items until exhausted or the limit is reached. */
const ghPages = async <T>(apiPath: string, limit = 1000): Promise<T[]> => {
  const items: T[] = [];
  let page = 1;
  while (items.length < limit) {
    const join = apiPath.includes("?") ? "&" : "?";
    const payload = await runGhApi<T[]>(`${apiPath}${join}per_page=100&page=${page}`);
    if (!Array.isArray(payload) || payload.length === 0) {
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

type PackageVersion = {
  metadata?: { container?: { tags?: string[] } };
};

const packageVersionTags = (versions: PackageVersion[]) =>
  versions.flatMap((item) => item.metadata?.container?.tags ?? []).filter(Boolean);

export const shouldStopPackageTagScan = (
  tags: Set<string>,
  payload: PackageVersion[],
  targetTag?: string,
) => (targetTag && tags.has(targetTag)) || payload.length < 100;

/** Returns recent main-branch commit SHAs for fhevm. */
export const mainCommits = async (limit = 200) => {
  const commits = await ghPages<{ sha: string }>(`repos/${FHEVM_REPO}/commits?sha=main`, limit);
  return commits.map((item) => item.sha);
};

/** Returns the published tag set for one GHCR package. */
export const packageTags = async (pkg: string, targetTag?: string) => {
  const tags = new Set<string>();
  let page = 1;
  while (tags.size < GH_PACKAGE_VERSION_LIMIT) {
    const payload = await runGhApi<PackageVersion[]>(
      `/orgs/${GH_OWNER}/packages/container/${pkg}/versions?per_page=100&page=${page}`,
    );
    if (!Array.isArray(payload) || payload.length === 0) {
      break;
    }
    for (const tag of packageVersionTags(payload)) {
      tags.add(tag);
    }
    if (shouldStopPackageTagScan(tags, payload, targetTag)) {
      break;
    }
    page += 1;
  }
  return tags;
};

/** Fetches and decodes a GitOps file from the main branch. */
export const gitopsFile = async (file: string) => {
  const payload = await runGhApi<{ content: string }>(
    `repos/${GITOPS_REPO}/contents/${file}?ref=main`,
  );
  return Buffer.from(payload.content.replace(/\n/g, ""), "base64").toString("utf8");
};
