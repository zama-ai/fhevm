import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";
import { CommandRunner, type RunResult } from "./CommandRunner";
import { GitHubClient } from "./GitHubClient";

const commandLayer = (responses: Record<string, unknown>) =>
  Layer.succeed(CommandRunner, {
    run: (argv: string[]) => {
      const payload = responses[argv.join(" ")];
      if (payload === undefined) {
        throw new Error(`Missing fake response for ${argv.join(" ")}`);
      }
      return Effect.succeed({
        stdout: JSON.stringify(payload),
        stderr: "",
        code: 0,
      } as RunResult);
    },
    runLive: () => Effect.succeed(0),
    runWithHeartbeat: () => Effect.void,
  });

describe("GitHubClient", () => {
  test("latestStableRelease skips prereleases and drafts", async () => {
    const tag = await Effect.runPromise(
      Effect.gen(function* () {
        const client = yield* GitHubClient;
        return yield* client.latestStableRelease();
      }).pipe(
        Effect.provide(GitHubClient.Live),
        Effect.provide(
          commandLayer({
            "gh api repos/zama-ai/fhevm/releases?per_page=100&page=1": [
              { tag_name: "v0.12.0-rc1", prerelease: true, draft: false },
              { tag_name: "v0.11.1", prerelease: false, draft: true },
              { tag_name: "v0.11.0", prerelease: false, draft: false },
            ],
          }),
        ),
      ),
    );
    expect(tag).toBe("v0.11.0");
  });

  test("mainCommits paginates", async () => {
    const commits = await Effect.runPromise(
      Effect.gen(function* () {
        const client = yield* GitHubClient;
        return yield* client.mainCommits(102);
      }).pipe(
        Effect.provide(GitHubClient.Live),
        Effect.provide(
          commandLayer({
            "gh api repos/zama-ai/fhevm/commits?sha=main&per_page=100&page=1": Array.from(
              { length: 100 },
              (_, index) => ({ sha: `sha-${index}` }),
            ),
            "gh api repos/zama-ai/fhevm/commits?sha=main&per_page=100&page=2": [
              { sha: "sha-100" },
              { sha: "sha-101" },
            ],
          }),
        ),
      ),
    );
    expect(commits).toHaveLength(102);
    expect(commits.at(0)).toBe("sha-0");
    expect(commits.at(-1)).toBe("sha-101");
  });
});
