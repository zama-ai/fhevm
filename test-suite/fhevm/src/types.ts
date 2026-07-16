export const STEP_NAMES = [
  "preflight",
  "resolve",
  "generate",
  "base",
  "listener-core",
  "kms-signer",
  "gateway-deploy",
  "host-deploy",
  "discover",
  "bridge-deploy",
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
  "listener-core",
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

/** KMS deployment mode for a scenario. */
export type KmsMode = "centralized" | "threshold";

/** FHE parameter set: Test = small/fast for CI, Default = prod-size. */
export type KmsFheParams = "Test" | "Default";

/** Raw `kms` block as written in a scenario YAML. */
export type KmsScenarioBlock = {
  mode?: KmsMode;
  parties?: number;
  threshold?: number;
  /** Initial on-chain committee size; defaults to `parties`. When `< parties` the extra cores boot
   *  as spares (peers=None) so a context switch can rotate one in (e.g. a node swap). */
  committeeSize?: number;
  fheParams?: KmsFheParams;
};

/** Fully-resolved KMS topology carried on the resolved scenario / StackSpec. */
export type ResolvedKmsTopology = {
  mode: KmsMode;
  /** Total cores provisioned in the cluster. */
  parties: number;
  threshold: number;
  /** Initial on-chain committee (and the `3t+1` MPC group); `<= parties`. Cores beyond it are spares. */
  committeeSize: number;
  fheParams: KmsFheParams;
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
  kms?: KmsScenarioBlock;
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
  kms: ResolvedKmsTopology;
};

// Raw `blue-green` scenario as written in YAML.
export type BlueGreenScenario = {
  version: 1;
  kind: "blue-green";
  name?: string;
  description?: string;
  hostChains?: HostChainScenario[];
  topology?: {
    count: number;
    threshold: number;
  };
  // Blue (previous release) fleet.
  bcs?: {
    source?: CoprocessorInstanceSource;
    env?: Record<string, string>;
    args?: Record<string, string[]>;
  };
  gcs: {
    source?: CoprocessorInstanceSource;
    stackVersion: string;
    env?: Record<string, string>;
    args?: Record<string, string[]>;
  };
  kms?: KmsScenarioBlock;
};

export type ResolvedBlueGreenScenarioFleet = {
  source: CoprocessorInstanceSource;
  env: Record<string, string>;
  args: Record<string, string[]>;
};

export type ResolvedBlueGreenScenario = {
  version: 1;
  kind: "blue-green";
  origin: "default" | "file";
  name?: string;
  description?: string;
  hostChains: HostChainScenario[];
  sourcePath?: string;
  topology: {
    count: number;
    threshold: number;
  };
  bcs: ResolvedBlueGreenScenarioFleet;
  gcs: ResolvedBlueGreenScenarioFleet & { stackVersion: string };
  kms: ResolvedKmsTopology;
};

// Union of every scenario shape the runtime accepts. Narrow on `kind`.
export type ResolvedScenario = ResolvedCoprocessorScenario | ResolvedBlueGreenScenario;

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
  kmsSigners: string[];
  // Per-party serialized CA certificate (hex `0x…`), discovered alongside the signers. Optional
  // like minioKeyPrefix: seeded to [] by createDiscovery and filled at the `kms-signer` step.
  kmsCaCerts?: string[];
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
  scenario: ResolvedScenario;
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
  // True when overrides were expanded from --build (all groups) rather than explicit --override flags.
  build?: boolean;
  scenarioPath?: string;
  // Blue-green only: override `bcs.source` to `{mode: registry, tag: bcsTag}`.
  bcsTag?: string;
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
