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

export const HOST_CHAIN_TYPES = ["evm", "solana"] as const;
export const HOST_CHAIN_NODE_PROVISIONING = ["container", "external"] as const;

export type StepName = (typeof STEP_NAMES)[number];
export type VersionTarget = (typeof TARGETS)[number];
export type OverrideGroup = (typeof OVERRIDE_GROUPS)[number];
/** The kind of host chain. `evm` is the default; `solana` is the RFC-021 Solana host. */
export type HostChainType = (typeof HOST_CHAIN_TYPES)[number];
/**
 * How the host node is provisioned. `container` = fhevm-cli runs it (and deploys + registers it);
 * `external` = an outside process owns the node + deploy + registration (the Solana host-native
 * validator). (A future `host` value could run a fhevm-cli-managed host process.)
 */
export type HostChainNodeProvisioning = (typeof HOST_CHAIN_NODE_PROVISIONING)[number];

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
  /**
   * The host-chain kind. Omitted means `evm` (the historical default), so existing scenarios
   * are unchanged. `solana` selects the RFC-021 Solana host: a `solana-test-validator` node,
   * Anchor program deploy, and the Solana host-listener/finalized-account-fetcher.
   */
  type?: HostChainType;
  /**
   * Who provisions the host node. Omitted defaults by `type`: `evm` ⇒ `container` (fhevm-cli runs
   * the node + deploy + registration), `solana` ⇒ `external` (the host-native validator and the
   * solana-side bring-up own them). Set explicitly to override.
   */
  nodeProvisioning?: HostChainNodeProvisioning;
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
  fheParams?: KmsFheParams;
};

/** Fully-resolved KMS topology carried on the resolved scenario / StackSpec. */
export type ResolvedKmsTopology = {
  mode: KmsMode;
  parties: number;
  threshold: number;
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
