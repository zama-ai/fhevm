export type DeployOptions = {
  forceBuild: boolean;
  localBuild: boolean;
  autoTracing: boolean;
  telemetrySmoke: boolean;
  strictOtel: boolean;
  coprocessorCount: number;
  coprocessorThresholdOverride?: number;
  networkProfile?: "testnet" | "mainnet";
  resumeStep?: string;
  onlyStep?: string;
};

export type CleanOptions = {
  purgeImages: boolean;
  purgeBuildCache: boolean;
  purgeNetworks: boolean;
  allFhevmProjects: boolean;
};

export type TestOptions = {
  verbose: boolean;
  network: string;
  grep?: string;
  noRelayer: boolean;
  noHardhatCompile: boolean;
};

export type HostPortSpec = {
  envVar: string;
  defaultPort: number;
  protocol: "tcp" | "udp";
};
