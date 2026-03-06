import { describe, expect, test } from "bun:test";

import {
  __internal,
  containerExists,
  getContainerExitCode,
  getContainerIp,
  getContainerLogs,
  getContainerState,
  listProjectContainers,
  parseComposePsOutput,
} from "./containers";

describe("docker containers", () => {
  test("validates docker network names", () => {
    expect(__internal.isValidNetworkName("fhevm_default")).toBe(true);
    expect(__internal.isValidNetworkName("fhevm.default-1")).toBe(true);
    expect(__internal.isValidNetworkName("bad\"name")).toBe(false);
    expect(__internal.isValidNetworkName("bad name")).toBe(false);
  });

  test("parses compose ps array output", () => {
    const rows = parseComposePsOutput(
      JSON.stringify([
        {
          Name: "fhevm-minio",
          Service: "fhevm-minio",
          State: "running",
          RunningFor: "2 minutes",
          Health: "healthy",
          ExitCode: 0,
        },
      ]),
    );

    expect(rows).toHaveLength(1);
    expect(rows[0]?.state).toBe("running");
    expect(rows[0]?.health).toBe("healthy");
    expect(rows[0]?.uptime).toBe("2 minutes");
  });

  test("parses compose ps line-delimited output", () => {
    const rows = parseComposePsOutput([
      JSON.stringify({ Name: "host-node", Service: "host-node", State: "running", Status: "Up 6 minutes" }),
      JSON.stringify({ Name: "gateway-node", Service: "gateway-node", State: "exited", ExitCode: 1 }),
    ].join("\n"));

    expect(rows).toHaveLength(2);
    expect(rows[0]?.name).toBe("host-node");
    expect(rows[1]?.state).toBe("exited");
    expect(rows[1]?.exitCode).toBe(1);
    expect(rows[0]?.uptime).toBe("6 minutes");
  });

  test("returns not-found state for missing container", async () => {
    const state = await getContainerState("fhevm-cli-non-existent-container");
    expect(state).toBe("not-found");
  });

  test("handles missing container metadata safely", async () => {
    expect(await getContainerExitCode("fhevm-cli-non-existent-container")).toBeUndefined();
    expect(await getContainerIp("fhevm-cli-non-existent-container")).toBeUndefined();
    expect(await getContainerLogs("fhevm-cli-non-existent-container")).toBe("");
    expect(await containerExists("fhevm-cli-non-existent-container")).toBe(false);
  });

  test("returns empty list when project has no compose state", async () => {
    const rows = await listProjectContainers("fhevm-cli-empty-project");
    expect(Array.isArray(rows)).toBe(true);
  });
});
