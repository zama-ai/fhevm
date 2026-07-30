import fs from "node:fs/promises";
import path from "node:path";

const MAX_MESSAGE_BYTES = 8_192;
const REQUEST_TIMEOUT_MS = 10 * 60 * 1_000;

export type SupervisorReseedRequest = {
  readonly version: 1;
  readonly action: "reseed";
  readonly bootId: string;
  readonly token: string;
};

export type SupervisorReseedResult = {
  readonly bootId: string;
  readonly launchUrl: string;
};

type SupervisorReply =
  | {
      readonly version: 1;
      readonly ok: true;
      readonly result: SupervisorReseedResult;
    }
  | { readonly version: 1; readonly ok: false; readonly error: string };

export type SupervisorProcessOwner = {
  readonly pid: number;
  readonly identity: string;
};

type SupervisorSocketOwner = SupervisorProcessOwner & {
  readonly version: 1;
  readonly bootId: string;
  readonly socketDev: number;
  readonly socketIno: number;
};

const currentUserId = (): number => {
  const userId = process.getuid?.();
  if (userId === undefined) {
    throw new Error("demo supervisor control requires a Unix user id");
  }
  return userId;
};

const ensurePrivateSocketDirectory = async (
  socketDirectory: string,
): Promise<void> => {
  await fs.mkdir(socketDirectory, { recursive: true, mode: 0o700 });
  const directory = await fs.lstat(socketDirectory);
  if (!directory.isDirectory() || directory.isSymbolicLink()) {
    throw new Error(
      `supervisor socket parent must be a real directory: ${socketDirectory}`,
    );
  }
  if (directory.uid !== currentUserId()) {
    throw new Error(
      `supervisor socket parent is not owned by the current user: ${socketDirectory}`,
    );
  }
  await fs.chmod(socketDirectory, 0o700);
};

export const readBoundedRequestBody = async (
  request: Request,
  maxBytes = MAX_MESSAGE_BYTES,
): Promise<string> => {
  const declaredLength = request.headers.get("content-length");
  if (declaredLength !== null) {
    if (!/^\d+$/.test(declaredLength) || Number(declaredLength) > maxBytes) {
      throw new Error("invalid supervisor request size");
    }
  }
  if (request.body === null) return "";
  const reader = request.body.getReader();
  const chunks: Uint8Array[] = [];
  let size = 0;
  for (;;) {
    const { done, value } = await reader.read();
    if (done) break;
    size += value.byteLength;
    if (size > maxBytes) {
      await reader.cancel();
      throw new Error("supervisor request is too large");
    }
    chunks.push(value);
  }
  return Buffer.concat(chunks).toString("utf8");
};

const socketOwnerPath = (socketPath: string): string =>
  `${socketPath}.owner.json`;

const readSocketOwner = async (
  socketPath: string,
): Promise<SupervisorSocketOwner | null> => {
  try {
    const owner = JSON.parse(
      await fs.readFile(socketOwnerPath(socketPath), "utf8"),
    ) as SupervisorSocketOwner;
    return owner.version === 1 &&
      typeof owner.bootId === "string" &&
      Number.isSafeInteger(owner.pid) &&
      owner.pid > 0 &&
      typeof owner.identity === "string" &&
      owner.identity.length > 0 &&
      Number.isSafeInteger(owner.socketDev) &&
      Number.isSafeInteger(owner.socketIno)
      ? owner
      : null;
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
    return null;
  }
};

const writeSocketOwner = async (
  socketPath: string,
  owner: SupervisorSocketOwner,
): Promise<void> => {
  const target = socketOwnerPath(socketPath);
  const temporary = `${target}.${process.pid}.${crypto.randomUUID()}.tmp`;
  try {
    const handle = await fs.open(temporary, "wx", 0o600);
    try {
      await handle.writeFile(`${JSON.stringify(owner, null, 2)}\n`, "utf8");
      await handle.sync();
    } finally {
      await handle.close();
    }
    await fs.rename(temporary, target);
  } catch (error) {
    await fs.rm(temporary, { force: true });
    throw error;
  }
};

const prepareSocketPath = async (
  socketPath: string,
  bootId: string,
  isExactOwner: (owner: SupervisorProcessOwner) => Promise<boolean>,
): Promise<void> => {
  let existing: Awaited<ReturnType<typeof fs.lstat>> | null;
  try {
    existing = await fs.lstat(socketPath);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") existing = null;
    else throw error;
  }
  const owner = await readSocketOwner(socketPath);
  if (existing === null && owner === null) return;
  if (owner === null) {
    throw new Error(
      `supervisor control ownership is missing or invalid at ${socketOwnerPath(socketPath)}`,
    );
  }
  if (owner.bootId !== bootId) {
    throw new Error(
      `supervisor control path belongs to unexpected boot ${owner.bootId}`,
    );
  }
  if (await isExactOwner(owner)) {
    throw new Error(
      `an active supervisor owned by pid ${owner.pid} already controls boot ${bootId}`,
    );
  }
  if (existing !== null) {
    if (
      !existing.isSocket() ||
      existing.uid !== currentUserId() ||
      existing.dev !== owner.socketDev ||
      existing.ino !== owner.socketIno
    ) {
      throw new Error(
        `refusing changed supervisor control socket at ${socketPath}`,
      );
    }
    await fs.rm(socketPath);
  }
  await fs.rm(socketOwnerPath(socketPath));
};

const removeExactSocket = async (
  socketPath: string,
  owner: SupervisorSocketOwner,
): Promise<void> => {
  try {
    const socket = await fs.lstat(socketPath);
    if (
      socket.isSocket() &&
      socket.uid === currentUserId() &&
      socket.dev === owner.socketDev &&
      socket.ino === owner.socketIno
    ) {
      await fs.rm(socketPath);
    }
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
  }
  const recorded = await readSocketOwner(socketPath);
  if (
    recorded?.bootId === owner.bootId &&
    recorded.pid === owner.pid &&
    recorded.identity === owner.identity &&
    recorded.socketDev === owner.socketDev &&
    recorded.socketIno === owner.socketIno
  ) {
    await fs.rm(socketOwnerPath(socketPath));
  }
};

export const startSupervisorControl = async ({
  socketPath,
  bootId,
  owner,
  isExactOwner,
  onReseed,
}: {
  readonly socketPath: string;
  readonly bootId: string;
  readonly owner: SupervisorProcessOwner;
  readonly isExactOwner: (
    owner: SupervisorProcessOwner,
  ) => Promise<boolean>;
  readonly onReseed: (
    request: SupervisorReseedRequest,
  ) => Promise<SupervisorReseedResult>;
}): Promise<() => Promise<void>> => {
  const socketDirectory = path.dirname(socketPath);
  await ensurePrivateSocketDirectory(socketDirectory);
  await prepareSocketPath(socketPath, bootId, isExactOwner);

  const server = Bun.serve({
    unix: socketPath,
    async fetch(request) {
      if (
        request.method === "GET" &&
        new URL(request.url).pathname === "/health"
      ) {
        return Response.json({ bootId });
      }
      if (
        request.method !== "POST" ||
        new URL(request.url).pathname !== "/reseed"
      ) {
        return Response.json(
          { version: 1, ok: false, error: "not found" } satisfies SupervisorReply,
          { status: 404 },
        );
      }
      try {
        const body = await readBoundedRequestBody(request);
        const message = JSON.parse(body) as SupervisorReseedRequest;
        if (
          message.version !== 1 ||
          message.action !== "reseed" ||
          typeof message.bootId !== "string" ||
          typeof message.token !== "string"
        ) {
          throw new Error("invalid supervisor request");
        }
        return Response.json({
          version: 1,
          ok: true,
          result: await onReseed(message),
        } satisfies SupervisorReply);
      } catch (error) {
        return Response.json(
          {
            version: 1,
            ok: false,
            error: error instanceof Error ? error.message : String(error),
          } satisfies SupervisorReply,
          { status: 400 },
        );
      }
    },
  });

  let socketOwner: SupervisorSocketOwner | undefined;
  try {
    const socket = await fs.lstat(socketPath);
    if (!socket.isSocket() || socket.uid !== currentUserId()) {
      throw new Error(`supervisor control path is not an owned socket`);
    }
    socketOwner = {
      version: 1,
      bootId,
      pid: owner.pid,
      identity: owner.identity,
      socketDev: socket.dev,
      socketIno: socket.ino,
    };
    await fs.chmod(socketPath, 0o600);
    await writeSocketOwner(socketPath, socketOwner);
  } catch (error) {
    await server.stop(true);
    if (socketOwner !== undefined) {
      await removeExactSocket(socketPath, socketOwner);
    }
    throw error;
  }

  return async () => {
    await server.stop(true);
    await removeExactSocket(socketPath, socketOwner);
  };
};

export const requestSupervisorReseed = async (
  socketPath: string,
  request: SupervisorReseedRequest,
): Promise<SupervisorReseedResult> => {
  const response = await fetch("http://localhost/reseed", {
    unix: socketPath,
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(request),
    signal: AbortSignal.timeout(REQUEST_TIMEOUT_MS),
  });
  const reply = (await response.json()) as SupervisorReply;
  if (reply.version !== 1) throw new Error("invalid supervisor response");
  if (!reply.ok) throw new Error(reply.error);
  if (
    typeof reply.result.bootId !== "string" ||
    typeof reply.result.launchUrl !== "string"
  ) {
    throw new Error("invalid supervisor response");
  }
  return reply.result;
};
