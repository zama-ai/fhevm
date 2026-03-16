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

export const TARGETS = ["latest-main", "latest-release", "sha", "devnet", "testnet", "mainnet"] as const;
export const OVERRIDE_GROUPS = [
  "coprocessor",
  "kms-connector",
  "gateway-contracts",
  "host-contracts",
  "test-suite",
] as const;

export type StepName = (typeof STEP_NAMES)[number];
export type VersionTarget = (typeof TARGETS)[number];
export type OverrideGroup = (typeof OVERRIDE_GROUPS)[number];

export type InstanceOverride = {
  env: Record<string, string>;
  args: Record<string, string[]>;
};

export type LocalOverride = {
  group: OverrideGroup;
  services?: string[];
};

export type Topology = {
  count: number;
  threshold: number;
  instances: Record<string, InstanceOverride>;
};

export type Discovery = {
  gateway: Record<string, string>;
  host: Record<string, string>;
  kmsSigner: string;
  fheKeyId: string;
  crsKeyId: string;
  actualFheKeyId?: string;
  actualCrsKeyId?: string;
  minioKeyPrefix?: string;
  endpoints: {
    gatewayHttp: string;
    gatewayWs: string;
    hostHttp: string;
    hostWs: string;
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
};

export type State = {
  target: VersionTarget;
  lockPath: string;
  versions: VersionBundle;
  overrides: LocalOverride[];
  topology: Topology;
  discovery?: Discovery;
  builtImages?: BuiltImage[];
  completedSteps: StepName[];
  updatedAt: string;
};

export type UpOptions = {
  target: VersionTarget;
  sha?: string;
  overrides: LocalOverride[];
  topology: Topology;
  fromStep?: StepName;
  lockFile?: string;
  allowSchemaMismatch: boolean;
  resume: boolean;
  dryRun: boolean;
  reset: boolean;
};

export type CleanOptions = {
  images: boolean;
};

export type TestOptions = {
  grep?: string;
  network: string;
  verbose: boolean;
  parallel?: boolean;
};
