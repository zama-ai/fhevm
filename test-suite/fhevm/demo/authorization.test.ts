import { afterEach, describe, expect, test } from "bun:test";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import {
  authorizeDemoHeaders,
  createDemoAuthorizationFile,
  DEMO_ALLOWED_ORIGIN_ENV,
  DEMO_AUTH_TOKEN_FILE_ENV,
  DEMO_BOOT_ID_ENV,
  readDemoAllowedOriginFromEnv,
  readDemoAuthorizationFromEnv,
} from "./authorization";

const BOOT_ID = "1848f8e6-b670-4b1d-97a7-99ac20780ada";
const temporaryDirectories: string[] = [];

afterEach(async () => {
  for (const directory of temporaryDirectories.splice(0)) {
    await fs.rm(directory, { recursive: true, force: true });
  }
});

const temporaryDirectory = async (): Promise<string> => {
  const directory = await fs.mkdtemp(path.join(os.tmpdir(), "demo-authorization-"));
  temporaryDirectories.push(directory);
  return directory;
};

describe("demo boot authorization", () => {
  test("creates and atomically rotates a mode-0600 256-bit token", async () => {
    const directory = await temporaryDirectory();
    const first = await createDemoAuthorizationFile(directory, BOOT_ID);
    const firstToken = (await fs.readFile(first.tokenFile, "utf8")).trim();
    expect(Buffer.from(firstToken, "base64url")).toHaveLength(32);
    expect((await fs.stat(first.tokenFile)).mode & 0o777).toBe(0o600);
    expect(first.launchFragment).toBe(`#boot=${BOOT_ID}&token=${firstToken}`);

    const second = await createDemoAuthorizationFile(directory, BOOT_ID);
    expect(second.tokenFile).toBe(first.tokenFile);
    expect(second.authorization.token).not.toBe(first.authorization.token);
    expect((await fs.readFile(second.tokenFile, "utf8")).trim()).toBe(second.authorization.token);
  });

  test("loads only an absolute protected token file and exact loopback origin", async () => {
    const directory = await temporaryDirectory();
    const created = await createDemoAuthorizationFile(directory, BOOT_ID);
    await expect(
      readDemoAuthorizationFromEnv({
        [DEMO_BOOT_ID_ENV]: BOOT_ID,
        [DEMO_AUTH_TOKEN_FILE_ENV]: created.tokenFile,
      }),
    ).resolves.toEqual(created.authorization);
    expect(
      readDemoAllowedOriginFromEnv({
        [DEMO_ALLOWED_ORIGIN_ENV]: "http://127.0.0.1:5173",
      }),
    ).toBe("http://127.0.0.1:5173");

    await fs.chmod(created.tokenFile, 0o644);
    await expect(
      readDemoAuthorizationFromEnv({
        [DEMO_BOOT_ID_ENV]: BOOT_ID,
        [DEMO_AUTH_TOKEN_FILE_ENV]: created.tokenFile,
      }),
    ).rejects.toThrow("group or other users");
    expect(() =>
      readDemoAllowedOriginFromEnv({
        [DEMO_ALLOWED_ORIGIN_ENV]: "http://localhost:5173",
      }),
    ).toThrow("exact http://127.0.0.1 origin");
  });

  test("distinguishes stale boot from missing or incorrect bearer credentials", async () => {
    const directory = await temporaryDirectory();
    const { authorization } = await createDemoAuthorizationFile(directory, BOOT_ID);
    const headers = new Headers({
      authorization: `Bearer ${authorization.token}`,
      "x-fhevm-demo-boot-id": authorization.bootId,
    });
    expect(authorizeDemoHeaders((name) => headers.get(name), authorization)).toEqual({ ok: true });

    headers.set("x-fhevm-demo-boot-id", "c4ef95ed-2ca7-4d83-8d00-b547023ac9e2");
    expect(authorizeDemoHeaders((name) => headers.get(name), authorization)).toEqual({
      ok: false,
      status: 409,
      error: "stale demo boot; reopen the launch URL",
    });
    headers.set("x-fhevm-demo-boot-id", authorization.bootId);
    headers.set("authorization", "Bearer short");
    expect(authorizeDemoHeaders((name) => headers.get(name), authorization)).toEqual({
      ok: false,
      status: 401,
      error: "demo authorization required",
    });
  });

  test("fails closed when required authorization environment is absent or malformed", async () => {
    await expect(readDemoAuthorizationFromEnv({})).rejects.toThrow(DEMO_BOOT_ID_ENV);
    await expect(
      readDemoAuthorizationFromEnv({
        [DEMO_BOOT_ID_ENV]: BOOT_ID,
        [DEMO_AUTH_TOKEN_FILE_ENV]: "relative-token",
      }),
    ).rejects.toThrow("absolute path");
    expect(() => readDemoAllowedOriginFromEnv({})).toThrow(DEMO_ALLOWED_ORIGIN_ENV);
  });
});
