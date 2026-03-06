import { describe, expect, it } from "bun:test";
import {
  CUSTODIAN_ENCRYPTION_KEYS,
  DEFAULT_GATEWAY_CHAIN_ID,
  DEFAULT_HOST_CHAIN_ID,
  createDefaultConfig,
  type DerivedKeys,
} from "./model";

const DUMMY_KEY = {
  privateKey: "0x01",
  address: "0x0000000000000000000000000000000000000001",
};

const DUMMY_KEYS: DerivedKeys = {
  deployer: DUMMY_KEY,
  newOwner: DUMMY_KEY,
  txSender: DUMMY_KEY,
  coprocessors: [{ txSender: DUMMY_KEY, signer: DUMMY_KEY, s3BucketUrl: "s3://ct128" }],
  kmsNodes: [{ txSender: DUMMY_KEY, signer: DUMMY_KEY, ipAddress: "127.0.0.1", storageUrl: "s3://kms-public" }],
  custodians: CUSTODIAN_ENCRYPTION_KEYS.map((key) => ({
    txSender: DUMMY_KEY,
    signer: DUMMY_KEY,
    encryptionKey: key,
  })),
  pausers: [DUMMY_KEY, DUMMY_KEY],
};

describe("config model", () => {
  it("creates the default config with expected defaults", () => {
    const config = createDefaultConfig(DUMMY_KEYS);

    expect(config.chainIds.host).toBe(DEFAULT_HOST_CHAIN_ID);
    expect(config.chainIds.gateway).toBe(DEFAULT_GATEWAY_CHAIN_ID);
    expect(config.db.host).toBe("coprocessor-and-kms-db");
    expect(config.minio.endpoint).toBe("http://minio:9000");
    expect(config.thresholds.publicDecryption).toBe(1);
    expect(config.topology.numCoprocessors).toBe(1);
    expect(config.protocol.name).toBe("Protocol");
    expect(config.keys).toBe(DUMMY_KEYS);
  });

  it("supports top-level overrides", () => {
    const config = createDefaultConfig(DUMMY_KEYS, {
      chainIds: { host: 100, gateway: 200 },
      runtime: { kmsSigner: "0xabc" },
    });

    expect(config.chainIds.host).toBe(100);
    expect(config.chainIds.gateway).toBe(200);
    expect(config.runtime.kmsSigner).toBe("0xabc");
  });

  it("is JSON serializable", () => {
    const config = createDefaultConfig(DUMMY_KEYS);
    const parsed = JSON.parse(JSON.stringify(config));

    expect(parsed.chainIds.gateway).toBe(DEFAULT_GATEWAY_CHAIN_ID);
    expect(parsed.db.user).toBe("postgres");
    expect(parsed.minio.buckets.ct64).toBe("ct64");
  });
});
