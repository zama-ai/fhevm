export interface CIEnvironment {
  isCI: boolean;
  isGitHubActions: boolean;
  cacheType: "gha" | "local" | "none";
}

type EnvReader = (name: string) => string | undefined;

const DEFAULT_ENV_READER: EnvReader = (name) => process.env[name];

let readEnv: EnvReader = DEFAULT_ENV_READER;

export function detectCI(options: { noCache?: boolean } = {}): CIEnvironment {
  const isGitHubActions = readEnv("GITHUB_ACTIONS") === "true";
  const isCI = readEnv("CI") === "true" || isGitHubActions;

  return {
    isCI,
    isGitHubActions,
    cacheType: options.noCache ? "none" : isCI ? "gha" : "local",
  };
}

export const __internal = {
  resetEnvReaderForTests(): void {
    readEnv = DEFAULT_ENV_READER;
  },
  setEnvReaderForTests(next: EnvReader): void {
    readEnv = next;
  },
};
