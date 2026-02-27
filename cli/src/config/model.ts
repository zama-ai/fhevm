export interface KeyPair {
  privateKey: string;
  address: string;
}

export interface CoprocessorKeySet {
  txSender: KeyPair;
  signer: KeyPair;
  s3BucketUrl: string;
}

export interface KmsNodeKeySet {
  txSender: KeyPair;
  signer: KeyPair;
  ipAddress: string;
  storageUrl: string;
}

export interface CustodianKeySet {
  txSender: KeyPair;
  signer: KeyPair;
  encryptionKey: string;
}

export interface DerivedKeys {
  deployer: KeyPair;
  newOwner: KeyPair;
  txSender: KeyPair;
  coprocessors: CoprocessorKeySet[];
  kmsNodes: KmsNodeKeySet[];
  custodians: CustodianKeySet[];
  pausers: KeyPair[];
}

export interface ContractAddresses {
  gatewayConfig?: string;
  kmsGeneration?: string;
  inputVerification?: string;
  decryption?: string;
  acl?: string;
  fhevmExecutor?: string;
  pauserSet?: string;
  hostPauserSet?: string;
  multichainAcl?: string;
  ciphertextCommits?: string;
  protocolPayment?: string;
  zamaOft?: string;
  feesSenderToBurner?: string;
  kmsVerifier?: string;
  inputVerifier?: string;
}

export interface FhevmConfig {
  chainIds: {
    host: number;
    gateway: number;
  };
  mnemonic: string;
  ports: {
    postgres: number;
    relayerPostgres: number;
    hostRpc: number;
    gatewayRpc: number;
    minioApi: number;
    minioConsole: number;
    kmsCore: number;
    relayerHttp: number;
  };
  db: {
    user: string;
    password: string;
    host: string;
    port: number;
    relayerHost: string;
    relayerPort: number;
    coprocessorDb: string;
    kmsConnectorDb: string;
    relayerDb: string;
  };
  minio: {
    endpoint: string;
    rootUser: string;
    rootPassword: string;
    accessKey: string;
    secretKey: string;
    region: string;
    buckets: {
      public: string;
      ct64: string;
      ct128: string;
    };
  };
  rpc: {
    hostHttp: string;
    hostWs: string;
    gatewayHttp: string;
    gatewayWs: string;
    kmsCore: string;
    relayerHttp: string;
  };
  thresholds: {
    publicDecryption: number;
    userDecryption: number;
    kmsGeneration: number;
    coprocessor: number;
    mpc: number;
  };
  topology: {
    numKmsNodes: number;
    numCoprocessors: number;
    numCustodians: number;
    numPausers: number;
    numHostChains: number;
  };
  protocol: {
    name: string;
    website: string;
    inputVerificationPrice: string;
    publicDecryptionPrice: string;
    userDecryptionPrice: string;
  };
  keys: DerivedKeys;
  contracts: ContractAddresses;
  runtime: {
    minioIp?: string;
    kmsSigner?: string;
    fheKeyId?: string;
    crsKeyId?: string;
  };
}

export const DEFAULT_MNEMONIC =
  "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer";

export const DEFAULT_HOST_CHAIN_ID = 12345;
export const DEFAULT_GATEWAY_CHAIN_ID = 54321;
export const MAX_COPROCESSORS = 5;

export const HD_INDICES = {
  deployer: 9, // Must match legacy host-sc deployer to produce deterministic contract addresses
  newOwner: 2,
  coprocessorTxSender: [5, 8, 10, 11, 12],
  kmsTxSender: [3],
  custodians: [17, 19, 21],
  pausers: [23, 24],
  txSender: 25,
} as const;

export const CUSTODIAN_ENCRYPTION_KEYS = [
  "0xea8b8b710d770493a41b588808ea8e09d986561f73d523227718233f3b4742de793f18a9885136a9e7054b00ba0050a17f0c7d1bf180aaff5ece0fa3343afb1b",
  "0x753c623ada1ad141eb01f99196ad7b69cce63c7fd3e0fbbc2b6c46ea007103cbb71c8b018bd372fe4de2429d614aeaaaf2409736ad0d7c01de26cbe82c9d9195",
  "0xcb7d351548eadf0041c683e97e46f70f1a269062a94e8c5c69c6b07e6ce0369970f431024d9868537ffe502c1d8d9c740b75597014305070e86fcfaa56333f8e",
] as const;

export function createDefaultConfig(
  keys: DerivedKeys,
  overrides: Partial<FhevmConfig> = {},
): FhevmConfig {
  const base: FhevmConfig = {
    chainIds: {
      host: DEFAULT_HOST_CHAIN_ID,
      gateway: DEFAULT_GATEWAY_CHAIN_ID,
    },
    mnemonic: DEFAULT_MNEMONIC,
    ports: {
      postgres: 5432,
      relayerPostgres: 5433,
      hostRpc: 8545,
      gatewayRpc: 8546,
      minioApi: 9000,
      minioConsole: 9001,
      kmsCore: 50051,
      relayerHttp: 3000,
    },
    db: {
      user: "postgres",
      password: "postgres",
      host: "coprocessor-and-kms-db",
      port: 5432,
      relayerHost: "fhevm-relayer-db",
      relayerPort: 5432,
      coprocessorDb: "coprocessor",
      kmsConnectorDb: "kms-connector",
      relayerDb: "relayer_db",
    },
    minio: {
      endpoint: "http://minio:9000",
      rootUser: "minioadmin",
      rootPassword: "minioadmin",
      accessKey: "fhevm-access-key",
      secretKey: "fhevm-access-secret-key",
      region: "eu-west-1",
      buckets: {
        public: "kms-public",
        ct64: "ct64",
        ct128: "ct128",
      },
    },
    rpc: {
      hostHttp: "http://host-node:8545",
      hostWs: "ws://host-node:8545",
      gatewayHttp: "http://gateway-node:8546",
      gatewayWs: "ws://gateway-node:8546",
      kmsCore: "http://kms-core:50051",
      relayerHttp: "http://fhevm-relayer:3000",
    },
    thresholds: {
      publicDecryption: 1,
      userDecryption: 1,
      kmsGeneration: 1,
      coprocessor: 1,
      mpc: 1,
    },
    topology: {
      numKmsNodes: 1,
      numCoprocessors: 1,
      numCustodians: 3,
      numPausers: 2,
      numHostChains: 1,
    },
    protocol: {
      name: "Protocol",
      website: "https://protocol.com",
      inputVerificationPrice: "10000000000000000000",
      publicDecryptionPrice: "1000000000000000000",
      userDecryptionPrice: "1000000000000000000",
    },
    keys,
    contracts: {
      // FeesSenderToBurner can be any non-zero address in mocked environments.
      // See gateway-contracts/.env.example for reference.
      feesSenderToBurner: "0x0000111122223333444455556666777788889999",
    },
    runtime: {},
  };

  return {
    ...base,
    ...overrides,
  };
}
