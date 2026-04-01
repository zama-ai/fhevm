/**
 * Fetches live GitHub and GHCR metadata needed to resolve floating fhevm version targets.
 */
import { GitHubApiError } from "../errors";
import { run } from "../utils/process";

const FHEVM_REPO = "zama-ai/fhevm";
const GITOPS_REPO = "zama-zws/gitops";
const GH_OWNER = "zama-ai";
const GH_API_TIMEOUT_MS = 20_000;
const GH_API_RETRIES = 3;
const GH_API_RETRY_DELAY_MS = 500;

/** Rewrites raw `gh` failures into actionable user-facing guidance. */
const explainGitHubCliError = (message: string): string => {
  const lower = message.toLowerCase();
  if (lower.includes("enoent") || lower.includes("not found")) {
    return "GitHub CLI `gh` is required. Install `gh`, authenticate with `gh auth login` or GH_TOKEN, or use `--lock-file` / `--target latest-supported` to avoid GitHub resolution.";
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
const shouldRetryGitHubCliError = (message: string) => {
  const lower = message.toLowerCase();
  return (
    lower.includes("connection refused") ||
    lower.includes("timed out") ||
    lower.includes("tls handshake timeout") ||
    lower.includes("temporary failure") ||
    lower.includes("connection reset") ||
    lower.includes("econnreset")
  );
};

const runGhApi = async <T>(apiPath: string): Promise<T> => {
  for (let attempt = 1; attempt <= GH_API_RETRIES; attempt += 1) {
    try {
      const result = await run(["gh", "api", apiPath], { timeoutMs: GH_API_TIMEOUT_MS });
      return JSON.parse(result.stdout) as T;
    } catch (error) {
      const raw = error instanceof Error ? error.message : String(error);
      if (attempt < GH_API_RETRIES && shouldRetryGitHubCliError(raw)) {
        await Bun.sleep(GH_API_RETRY_DELAY_MS * attempt);
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

/** Returns recent main-branch commit SHAs for fhevm. */
export const mainCommits = async (limit = 200) => {
  const commits = await ghPages<{ sha: string }>(`repos/${FHEVM_REPO}/commits?sha=main`, limit);
  return commits.map((item) => item.sha);
};

/** Returns the published tag set for one GHCR package. */
export const packageTags = async (pkg: string) => {
  const versions = await ghPages<{
    metadata?: { container?: { tags?: string[] } };
  }>(`/orgs/${GH_OWNER}/packages/container/${pkg}/versions`, 1000);
  return new Set(
    versions.flatMap((item) => item.metadata?.container?.tags ?? []).filter(Boolean),
  );
};

/** Fetches and decodes a GitOps file from the main branch. */
export const gitopsFile = async (file: string) => {
  const payload = await runGhApi<{ content: string }>(
    `repos/${GITOPS_REPO}/contents/${file}?ref=main`,
  );
  return Buffer.from(payload.content.replace(/\n/g, ""), "base64").toString("utf8");
};
