import { Context, Effect, Layer } from "effect";
import { CommandRunner } from "./CommandRunner";
import { GitHubApiError } from "../errors";

const FHEVM_REPO = "zama-ai/fhevm";
const GITOPS_REPO = "zama-zws/gitops";
const GH_OWNER = "zama-ai";

const explainGitHubCliError = (message: string): string => {
  const lower = message.toLowerCase();
  if (lower.includes("enoent") || lower.includes("not found")) {
    return "GitHub CLI `gh` is required. Install `gh`, authenticate with `gh auth login` or GH_TOKEN, or use --lock-file.";
  }
  if (lower.includes("401") || lower.includes("authentication")) {
    return "GitHub API not authenticated. Run `gh auth login`, export GH_TOKEN, or use --lock-file.";
  }
  if (lower.includes("rate limit") || lower.includes("429")) {
    return "GitHub API rate limit hit. Retry with authenticated GH_TOKEN or use --lock-file.";
  }
  return message;
};

export class GitHubClient extends Context.Tag("GitHubClient")<
  GitHubClient,
  {
    readonly latestStableRelease: () => Effect.Effect<string, GitHubApiError>;
    readonly mainCommits: (limit?: number) => Effect.Effect<string[], GitHubApiError>;
    readonly packageTags: (pkg: string) => Effect.Effect<Set<string>, GitHubApiError>;
    readonly gitopsFile: (file: string) => Effect.Effect<string, GitHubApiError>;
  }
>() {
  static Live = Layer.effect(
    GitHubClient,
    Effect.gen(function* () {
      const cmd = yield* CommandRunner;

      const runGhApi = (apiPath: string) =>
        cmd.run(["gh", "api", apiPath]).pipe(
          Effect.map((r) => JSON.parse(r.stdout)),
          Effect.mapError(
            (e) => new GitHubApiError({ message: explainGitHubCliError(e.stderr || String(e)) }),
          ),
        );

      const ghPages = <T>(apiPath: string, limit = 1000): Effect.Effect<T[], GitHubApiError> =>
        Effect.gen(function* () {
          const items: T[] = [];
          let page = 1;
          while (items.length < limit) {
            const join = apiPath.includes("?") ? "&" : "?";
            const payload: T[] = yield* runGhApi(
              `${apiPath}${join}per_page=100&page=${page}`,
            );
            if (!Array.isArray(payload) || !payload.length) break;
            items.push(...payload);
            if (payload.length < 100) break;
            page += 1;
          }
          return items.slice(0, limit);
        });

      return {
        latestStableRelease: () =>
          Effect.gen(function* () {
            const releases = yield* ghPages<{
              tag_name: string;
              prerelease: boolean;
              draft: boolean;
            }>(`repos/${FHEVM_REPO}/releases`, 200);
            const release = releases.find((r) => !r.prerelease && !r.draft);
            if (!release) {
              return yield* Effect.fail(
                new GitHubApiError({ message: "No stable fhevm release found" }),
              );
            }
            return release.tag_name;
          }),

        mainCommits: (limit = 200) =>
          Effect.gen(function* () {
            const commits = yield* ghPages<{ sha: string }>(
              `repos/${FHEVM_REPO}/commits?sha=main`,
              limit,
            );
            return commits.map((c) => c.sha);
          }),

        packageTags: (pkg) =>
          Effect.gen(function* () {
            const versions = yield* ghPages<{
              metadata?: { container?: { tags?: string[] } };
            }>(`/orgs/${GH_OWNER}/packages/container/${pkg}/versions`, 1000);
            return new Set(
              versions.flatMap((v) => v.metadata?.container?.tags ?? []).filter(Boolean),
            );
          }),

        gitopsFile: (file) =>
          Effect.gen(function* () {
            const payload: { content: string } = yield* runGhApi(
              `repos/${GITOPS_REPO}/contents/${file}?ref=main`,
            );
            return Buffer.from(payload.content.replace(/\n/g, ""), "base64").toString("utf8");
          }),
      };
    }),
  );
}
