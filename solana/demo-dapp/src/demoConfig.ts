import { address, type Address } from "@solana/kit";

export type DemoConfig = {
  readonly source: "demo-config";
  readonly demoBootId: string;
  readonly chainId: string;
  readonly rpcUrl: string;
  readonly wsUrl: string;
  readonly relayerUrl: string;
  readonly proofServiceUrl: string;
  readonly aclProgram: `0x${string}`;
  readonly userDecryptContextId: string;
  readonly authorityFundingLamports: string;
  readonly hostConfig: Address;
  readonly kmsContext: Address;
  readonly vault: Address;
  readonly programs: {
    readonly batcher: Address;
    readonly token: Address;
    readonly vault: Address;
    readonly host: Address;
  };
  readonly mints: {
    readonly joinUnderlying: Address;
    readonly payoutUnderlying: Address;
    readonly joinConfidential: Address;
    readonly payoutConfidential: Address;
  };
  readonly batchers: {
    readonly deposit: { readonly batcher: Address; readonly lookupTable: Address };
    readonly redeem: { readonly batcher: Address; readonly lookupTable: Address };
  };
  readonly personas: {
    readonly keeper: Address;
    readonly alice: Address;
  };
};

const object = (value: unknown, name: string): Record<string, unknown> => {
  if (typeof value !== "object" || value === null) throw new Error(`${name} must be an object`);
  return value as Record<string, unknown>;
};

const string = (value: unknown, name: string): string => {
  if (typeof value !== "string" || value.length === 0) throw new Error(`${name} must be a non-empty string`);
  return value;
};

const localUrl = (value: unknown, name: string, protocol: "http:" | "ws:"): string => {
  const parsed = new URL(string(value, name));
  if (parsed.protocol !== protocol || parsed.hostname !== "127.0.0.1") {
    throw new Error(`${name} must use ${protocol}//127.0.0.1`);
  }
  return parsed.toString().replace(/\/$/, "");
};

export const parseDemoConfig = (value: unknown): DemoConfig => {
  const raw = object(value, "demo config");
  const personas = object(raw.personas, "demo config.personas");
  const programs = object(raw.programs, "demo config.programs");
  const mints = object(raw.mints, "demo config.mints");
  const batchers = object(raw.batchers, "demo config.batchers");
  const deposit = object(batchers.deposit, "demo config.batchers.deposit");
  const redeem = object(batchers.redeem, "demo config.batchers.redeem");
  if (raw.source !== "demo-config") throw new Error("demo config.source must be demo-config");
  const rpcUrl = localUrl(raw.rpcUrl, "demo config.rpcUrl", "http:");
  if (rpcUrl !== "http://127.0.0.1:8899") throw new Error(`demo refuses non-local RPC ${rpcUrl}`);
  return {
    source: "demo-config",
    demoBootId: string(raw.demoBootId, "demo config.demoBootId"),
    chainId: string(raw.chainId, "demo config.chainId"),
    rpcUrl,
    wsUrl: localUrl(raw.wsUrl, "demo config.wsUrl", "ws:"),
    relayerUrl: localUrl(raw.relayerUrl, "demo config.relayerUrl", "http:"),
    proofServiceUrl: localUrl(raw.proofServiceUrl, "demo config.proofServiceUrl", "http:"),
    aclProgram: string(raw.aclProgram, "demo config.aclProgram") as `0x${string}`,
    userDecryptContextId: string(raw.userDecryptContextId, "demo config.userDecryptContextId"),
    authorityFundingLamports: string(raw.authorityFundingLamports, "demo config.authorityFundingLamports"),
    hostConfig: address(string(raw.hostConfig, "demo config.hostConfig")),
    kmsContext: address(string(raw.kmsContext, "demo config.kmsContext")),
    vault: address(string(raw.vault, "demo config.vault")),
    programs: {
      batcher: address(string(programs.batcher, "demo config.programs.batcher")),
      token: address(string(programs.token, "demo config.programs.token")),
      vault: address(string(programs.vault, "demo config.programs.vault")),
      host: address(string(programs.host, "demo config.programs.host")),
    },
    mints: {
      joinUnderlying: address(string(mints.joinUnderlying, "demo config.mints.joinUnderlying")),
      payoutUnderlying: address(string(mints.payoutUnderlying, "demo config.mints.payoutUnderlying")),
      joinConfidential: address(string(mints.joinConfidential, "demo config.mints.joinConfidential")),
      payoutConfidential: address(string(mints.payoutConfidential, "demo config.mints.payoutConfidential")),
    },
    batchers: {
      deposit: {
        batcher: address(string(deposit.batcher, "demo config.batchers.deposit.batcher")),
        lookupTable: address(string(deposit.lookupTable, "demo config.batchers.deposit.lookupTable")),
      },
      redeem: {
        batcher: address(string(redeem.batcher, "demo config.batchers.redeem.batcher")),
        lookupTable: address(string(redeem.lookupTable, "demo config.batchers.redeem.lookupTable")),
      },
    },
    personas: {
      keeper: address(string(personas.keeper, "demo config.personas.keeper")),
      alice: address(string(personas.alice, "demo config.personas.alice")),
    },
  };
};

export const parseDemoConfigResponse = (value: unknown): DemoConfig =>
  parseDemoConfig(object(value, "demo config response").config);
