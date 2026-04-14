export const STEP_NAMES = [
  "preflight",
  "resolve",
  "generate",
  "base",
  "kms-signer",
  "gateway-deploy",
  "host-deploy",
  "discover",
  "regenerate",
  "validate",
  "coprocessor",
  "kms-connector",
  "bootstrap",
  "relayer",
  "test-suite",
] as const;

export const TARGETS = ["latest-main", "latest-supported", "sha", "devnet", "testnet", "mainnet"] as const;
export const OVERRIDE_GROUPS = [
  "coprocessor",
  "kms-connector",
  "relayer",
  "gateway-contracts",
  "host-contracts",
  "test-suite",
] as const;

export type StepName = (typeof STEP_NAMES)[number];
export type VersionTarget = (typeof TARGETS)[number];
export type OverrideGroup = (typeof OVERRIDE_GROUPS)[number];

export type CoprocessorInstanceSource =
  | { mode: "inherit" }
  | { mode: "local" }
  | { mode: "registry"; tag: string };

export type CoprocessorScenarioInstance = {
  index: number;
  source?: CoprocessorInstanceSource;
  env?: Record<string, string>;
  args?: Record<string, string[]>;
  localServices?: string[];
};

export type HostChainScenario = {
  key: string;
  chainId: string;
  rpcPort: number;
  name?: string;
};

export type CoprocessorScenario = {
  version: 1;
  kind: "coprocessor-consensus";
  name?: string;
  description?: string;
  hostChains?: HostChainScenario[];
  topology: {
    count: number;
    threshold: number;
  };
  instances?: CoprocessorScenarioInstance[];
};

export type ResolvedCoprocessorScenarioInstance = {
  index: number;
  source: CoprocessorInstanceSource;
  env: Record<string, string>;
  args: Record<string, string[]>;
  localServices?: string[];
};

export type ResolvedCoprocessorScenario = {
  version: 1;
  kind: "coprocessor-consensus";
  origin: "default" | "file" | "override-shorthand";
  name?: string;
  description?: string;
  hostChains: HostChainScenario[];
  sourcePath?: string;
  topology: {
    count: number;
    threshold: number;
  };
  instances: ResolvedCoprocessorScenarioInstance[];
};

export type ScenarioSummary = {
  key: string;
  filePath: string;
  name?: string;
  description?: string;
};

export type LocalOverride = {
  group: OverrideGroup;
  services?: string[];
};

export type Topology = {
  count: number;
  threshold: number;
};

export type RpcEndpoints = {
  http: string;
  ws: string;
};

export type Discovery = {
  gateway: Record<string, string>;
  hosts: Record<string, Record<string, string>>;
  kmsSigner: string;
  fheKeyId: string;
  crsKeyId: string;
  actualFheKeyId?: string;
  actualCrsKeyId?: string;
  minioKeyPrefix?: string;
  endpoints: {
    gateway: RpcEndpoints;
    hosts: Record<string, RpcEndpoints>;
    minioInternal: string;
    minioExternal: string;
  };
};

export type VersionBundle = {
  target: VersionTarget;
  lockName: string;
  env: Record<string, string>;
  sources: string[];
};

export type BuiltImage = {
  ref: string;
  id: string;
  group: OverrideGroup;
  instanceIndex?: number;
};

export type State = {
  target: VersionTarget;
  lockPath: string;
  requiresGitHub?: boolean;
  versions: VersionBundle;
  overrides: LocalOverride[];
  scenario: ResolvedCoprocessorScenario;
  scenarioSourcePath?: string;
  discovery?: Discovery;
  builtImages?: BuiltImage[];
  completedSteps: StepName[];
  updatedAt: string;
};

export type UpOptions = {
  target: VersionTarget;
  requestedTarget?: VersionTarget;
  sha?: string;
  overrides: LocalOverride[];
  scenarioPath?: string;
  fromStep?: StepName;
  lockFile?: string;
  allowSchemaMismatch: boolean;
  resume: boolean;
  dryRun: boolean;
  reset: boolean;
};

export type CleanOptions = {
  keepImages: boolean;
};

export type TestOptions = {
  grep?: string;
  network: string;
  verbose: boolean;
  noHardhatCompile: boolean;
  parallel?: boolean;
};
