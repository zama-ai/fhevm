import { describe, expect, it } from "bun:test";
import {
  COPROCESSOR_TEMPLATES,
  ENV_FILE_NAMES,
  LOCAL_COMPONENT_MAP,
  SERVICE_MAP,
  getComposeFilesForServices,
  getServiceByName,
  getServicesByEnvFile,
  getServicesByGroup,
} from "./service-map";

describe("service map", () => {
  it("contains expected topology size", () => {
    expect(SERVICE_MAP.length).toBe(35);
  });

  it("finds services by name", () => {
    const service = getServiceByName("coprocessor-tfhe-worker");
    expect(service?.group).toBe("coprocessor");
    expect(service?.versionVar).toBe("COPROCESSOR_TFHE_WORKER_VERSION");
  });

  it("returns coprocessor group services", () => {
    const services = getServicesByGroup("coprocessor");
    expect(services).toHaveLength(8);
    expect(COPROCESSOR_TEMPLATES).toHaveLength(8);
  });

  it("returns services by env file", () => {
    const services = getServicesByEnvFile("coprocessor");
    expect(services).toHaveLength(8);
  });

  it("uses docker-state for gw-listener readiness", () => {
    expect(getServiceByName("coprocessor-gw-listener")?.healthCheck).toBe("docker-state");
  });

  it("stores rpc endpoints on node service definitions", () => {
    expect(getServiceByName("host-node")?.healthEndpoint).toBe("http://localhost:8545");
    expect(getServiceByName("gateway-node")?.healthEndpoint).toBe("http://localhost:8546");
  });

  it("maps local coprocessor component to all related services", () => {
    expect(LOCAL_COMPONENT_MAP.coprocessor).toHaveLength(8);
    expect(LOCAL_COMPONENT_MAP.coprocessor).toContain("coprocessor-db-migration");
    expect(LOCAL_COMPONENT_MAP.coprocessor).toContain("coprocessor-transaction-sender");
  });

  it("collects unique compose files for selected services", () => {
    const services = [
      getServiceByName("coprocessor-db-migration"),
      getServiceByName("coprocessor-tfhe-worker"),
      getServiceByName("kms-core"),
    ].filter((value): value is NonNullable<typeof value> => Boolean(value));

    const composeFiles = getComposeFilesForServices(services);
    expect(composeFiles).toContain("test-suite/fhevm/docker-compose/coprocessor-docker-compose.yml");
    expect(composeFiles).toContain("test-suite/fhevm/docker-compose/core-docker-compose.yml");
    expect(composeFiles).toHaveLength(2);
  });

  it("uses valid env file keys and version vars when present", () => {
    for (const service of SERVICE_MAP) {
      expect(ENV_FILE_NAMES.includes(service.envFile as (typeof ENV_FILE_NAMES)[number])).toBe(true);
      if (service.versionVar) {
        expect(service.versionVar.length).toBeGreaterThan(0);
      }
      expect(service.composeFile.endsWith(".yml")).toBe(true);
    }
  });

  it("includes pause and unpause contract tasks", () => {
    expect(getServiceByName("gateway-sc-pause")?.group).toBe("contracts");
    expect(getServiceByName("gateway-sc-pause")?.composeFile).toContain(
      "gateway-pause-docker-compose.yml",
    );
    expect(getServiceByName("gateway-sc-unpause")?.group).toBe("contracts");
    expect(getServiceByName("gateway-sc-unpause")?.composeFile).toContain(
      "gateway-unpause-docker-compose.yml",
    );
    expect(getServiceByName("host-sc-pause")?.group).toBe("contracts");
    expect(getServiceByName("host-sc-pause")?.composeFile).toContain("host-pause-docker-compose.yml");
    expect(getServiceByName("host-sc-unpause")?.group).toBe("contracts");
    expect(getServiceByName("host-sc-unpause")?.composeFile).toContain(
      "host-unpause-docker-compose.yml",
    );
    expect(LOCAL_COMPONENT_MAP["gateway-contracts"]).toContain("gateway-sc-pause");
    expect(LOCAL_COMPONENT_MAP["host-contracts"]).toContain("host-sc-unpause");
  });
});
