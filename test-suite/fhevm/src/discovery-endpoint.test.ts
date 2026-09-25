import { expect, test } from "bun:test";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

const inspect = (address: string, gateway = "172.18.0.1", host = "0.0.0.0") => [{
  NetworkSettings: {
    Networks: { local: { IPAddress: address, Gateway: gateway } },
    Ports: { "9000/tcp": [{ HostIp: host, HostPort: "9000" }] },
  },
}];

const discover = (value: unknown) => {
  const directory = mkdtempSync(path.join(tmpdir(), "object-store-discovery-"));
  try {
    const fixture = path.join(directory, "inspect.json");
    writeFileSync(fixture, JSON.stringify(value));
    writeFileSync(path.join(directory, "docker"), '#!/bin/sh\n[ "$1" = inspect ] && [ "$2" = fhevm-object-store ] || exit 1\ncat "$OBJECT_STORE_INSPECT_FIXTURE"\n', { mode: 0o755 });
    return Bun.spawnSync(["bun", "-e", `import { defaultEndpoints } from ${JSON.stringify(path.join(import.meta.dir, "flow/discovery.ts"))}; console.log(JSON.stringify(await defaultEndpoints()));`], {
      env: { ...process.env, PATH: `${directory}:${process.env.PATH}`, OBJECT_STORE_INSPECT_FIXTURE: fixture },
      timeout: 10_000,
    });
  } finally { rmSync(directory, { recursive: true, force: true }); }
};

test("Object store discovery survives its container IP being reassigned to a restarted worker", () => {
  for (const address of ["172.18.0.2", "172.18.0.37"]) {
    const result = discover(inspect(address));
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    expect(JSON.parse(result.stdout.toString()).objectStoreExternal).toBe("http://172.18.0.1:9000");
  }
});

test("Object store discovery rejects an absent or ambiguous bridge gateway", () => {
  const ambiguous = inspect("172.18.0.2");
  Object.assign(ambiguous[0].NetworkSettings.Networks, { other: { IPAddress: "172.19.0.2", Gateway: "172.19.0.1" } });
  for (const value of [inspect("172.18.0.2", ""), ambiguous]) {
    const result = discover(value);
    expect(result.exitCode).not.toBe(0);
    expect(result.stderr.toString()).toContain("unambiguous IPv4 bridge gateway");
  }
});

test("Object store discovery refuses a host-loopback-only or unpublished S3 port", () => {
  const unpublished = inspect("172.18.0.2");
  unpublished[0].NetworkSettings.Ports["9000/tcp"] = [];
  for (const value of [inspect("172.18.0.2", "172.18.0.1", "127.0.0.1"), unpublished]) {
    const result = discover(value);
    expect(result.exitCode).not.toBe(0);
    expect(result.stderr.toString()).toContain("published IPv4 port");
  }
});
