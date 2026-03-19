import { GitHubApiError } from "./errors";
import { run } from "./shell";

const FHEVM_REPO = "zama-ai/fhevm";
const GITOPS_REPO = "zama-zws/gitops";
const GH_OWNER = "zama-ai";

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
  return message;
};

const runGhApi = async <T>(apiPath: string): Promise<T> => {
  try {
    const result = await run(["gh", "api", apiPath]);
    return JSON.parse(result.stdout) as T;
  } catch (error) {
    const message =
      error instanceof Error ? explainGitHubCliError(error.message) : String(error);
    throw new GitHubApiError(message);
  }
};

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

export const mainCommits = async (limit = 200) => {
  const commits = await ghPages<{ sha: string }>(`repos/${FHEVM_REPO}/commits?sha=main`, limit);
  return commits.map((item) => item.sha);
};

export const packageTags = async (pkg: string) => {
  const versions = await ghPages<{
    metadata?: { container?: { tags?: string[] } };
  }>(`/orgs/${GH_OWNER}/packages/container/${pkg}/versions`, 1000);
  return new Set(
    versions.flatMap((item) => item.metadata?.container?.tags ?? []).filter(Boolean),
  );
};

export const gitopsFile = async (file: string) => {
  const payload = await runGhApi<{ content: string }>(
    `repos/${GITOPS_REPO}/contents/${file}?ref=main`,
  );
  return Buffer.from(payload.content.replace(/\n/g, ""), "base64").toString("utf8");
};
