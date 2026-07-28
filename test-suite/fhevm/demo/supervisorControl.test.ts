import { afterEach, describe, expect, test } from "bun:test";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import {
  readBoundedRequestBody,
  requestSupervisorReseed,
  startSupervisorControl,
  type SupervisorReseedRequest,
} from "./supervisorControl";

const temporaryDirectories: string[] = [];
afterEach(async () => {
  for (const directory of temporaryDirectories.splice(0)) {
    await fs.rm(directory, { recursive: true, force: true });
  }
});

describe("demo supervisor control", () => {
  test("runs reseed through one mode-0600 local socket", async () => {
    const directory = await fs.mkdtemp(
      path.join(os.tmpdir(), "demo-supervisor-control-"),
    );
    temporaryDirectories.push(directory);
    const socketPath = path.join(directory, "supervisor.sock");
    let received: SupervisorReseedRequest | undefined;
    const stop = await startSupervisorControl({
      socketPath,
      bootId: "123e4567-e89b-42d3-a456-426614174000",
      owner: { pid: 42, identity: "supervisor:42" },
      isExactOwner: async (owner) => owner.identity === "supervisor:42",
      onReseed: async (request) => {
        received = request;
        return {
          bootId: request.bootId,
          launchUrl: "http://127.0.0.1:5173/#redacted",
        };
      },
    });
    try {
      expect((await fs.stat(directory)).mode & 0o777).toBe(0o700);
      expect((await fs.stat(socketPath)).mode & 0o777).toBe(0o600);
      const request = {
        version: 1 as const,
        action: "reseed" as const,
        bootId: "123e4567-e89b-42d3-a456-426614174000",
        token: "A".repeat(43),
      };
      expect(await requestSupervisorReseed(socketPath, request)).toEqual({
        bootId: request.bootId,
        launchUrl: "http://127.0.0.1:5173/#redacted",
      });
      expect(received).toEqual(request);
      await expect(
        startSupervisorControl({
          socketPath,
          bootId: request.bootId,
          owner: { pid: 43, identity: "supervisor:43" },
          isExactOwner: async (owner) => owner.identity === "supervisor:42",
          onReseed: async () => {
            throw new Error("must not run");
          },
        }),
      ).rejects.toThrow("active supervisor");
    } finally {
      await stop();
    }
    await expect(fs.access(socketPath)).rejects.toHaveProperty(
      "code",
      "ENOENT",
    );
  });

  test("returns reseed failures to the requesting CLI", async () => {
    const directory = await fs.mkdtemp(
      path.join(os.tmpdir(), "demo-supervisor-control-"),
    );
    temporaryDirectories.push(directory);
    const socketPath = path.join(directory, "supervisor.sock");
    const stop = await startSupervisorControl({
      socketPath,
      bootId: "123e4567-e89b-42d3-a456-426614174000",
      owner: { pid: 42, identity: "supervisor:42" },
      isExactOwner: async () => false,
      onReseed: async () => {
        throw new Error("reseed failed safely");
      },
    });
    try {
      await expect(
        requestSupervisorReseed(socketPath, {
          version: 1,
          action: "reseed",
          bootId: "123e4567-e89b-42d3-a456-426614174000",
          token: "A".repeat(43),
        }),
      ).rejects.toThrow("reseed failed safely");
    } finally {
      await stop();
    }
  });

  test("recovers an unchanged same-user socket left by a stopped supervisor", async () => {
    const directory = await fs.mkdtemp(
      path.join(os.tmpdir(), "demo-supervisor-control-"),
    );
    temporaryDirectories.push(directory);
    const socketPath = path.join(directory, "supervisor.sock");
    const stale = Bun.serve({
      unix: socketPath,
      fetch: () => Response.json({ bootId: "stale" }),
    });
    await stale.stop(true);
    const bootId = "123e4567-e89b-42d3-a456-426614174000";
    const staleSocket = await fs.lstat(socketPath);
    expect(staleSocket.isSocket()).toBe(true);
    await fs.writeFile(
      `${socketPath}.owner.json`,
      JSON.stringify({
        version: 1,
        bootId,
        pid: 99,
        identity: "dead:99",
        socketDev: staleSocket.dev,
        socketIno: staleSocket.ino,
      }),
      { mode: 0o600 },
    );
    const stop = await startSupervisorControl({
      socketPath,
      bootId,
      owner: { pid: 42, identity: "supervisor:42" },
      isExactOwner: async () => false,
      onReseed: async () => ({
        bootId,
        launchUrl: "http://127.0.0.1:5173/#redacted",
      }),
    });
    try {
      expect((await fs.lstat(socketPath)).isSocket()).toBe(true);
    } finally {
      await stop();
    }
  });

  test("cleanup never unlinks a replacement socket inode", async () => {
    const directory = await fs.mkdtemp(
      path.join(os.tmpdir(), "demo-supervisor-control-"),
    );
    temporaryDirectories.push(directory);
    const socketPath = path.join(directory, "supervisor.sock");
    const bootId = "123e4567-e89b-42d3-a456-426614174000";
    const stop = await startSupervisorControl({
      socketPath,
      bootId,
      owner: { pid: 42, identity: "supervisor:42" },
      isExactOwner: async () => false,
      onReseed: async () => ({
        bootId,
        launchUrl: "http://127.0.0.1:5173/#redacted",
      }),
    });
    await fs.rm(socketPath);
    const replacement = Bun.serve({
      unix: socketPath,
      fetch: () => new Response("replacement"),
    });
    const replacementInode = (await fs.lstat(socketPath)).ino;

    await stop();
    expect((await fs.lstat(socketPath)).ino).toBe(replacementInode);

    await replacement.stop(true);
    await fs.rm(socketPath, { force: true });
  });

  test("bounds declared and streamed request bodies before parsing", async () => {
    await expect(
      readBoundedRequestBody(
        new Request("http://localhost/reseed", {
          method: "POST",
          headers: { "content-length": "9000" },
          body: "{}",
        }),
      ),
    ).rejects.toThrow("invalid supervisor request size");

    await expect(
      readBoundedRequestBody(
        new Request("http://localhost/reseed", {
          method: "POST",
          body: new Blob(["A".repeat(9_000)]),
        }),
      ),
    ).rejects.toThrow("too large");
  });
});
