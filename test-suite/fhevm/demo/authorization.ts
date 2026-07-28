import { randomBytes, timingSafeEqual } from "node:crypto";
import fs from "node:fs/promises";
import path from "node:path";

export const DEMO_BOOT_ID_ENV = "DEMO_BOOT_ID";
export const DEMO_AUTH_TOKEN_FILE_ENV = "DEMO_AUTH_TOKEN_FILE";
export const DEMO_ALLOWED_ORIGIN_ENV = "DEMO_ALLOWED_ORIGIN";
export const DEMO_AUTH_TOKEN_FILENAME = "authorization-token";

const BOOT_ID_PATTERN = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
const TOKEN_PATTERN = /^[A-Za-z0-9_-]{43}$/;

export type DemoAuthorization = {
  readonly bootId: string;
  readonly token: string;
};

export type DemoAuthorizationDecision =
  | { readonly ok: true }
  | { readonly ok: false; readonly status: 401 | 409; readonly error: string };

const validatedBootId = (value: string | undefined): string => {
  if (value === undefined || !BOOT_ID_PATTERN.test(value)) {
    throw new Error(`${DEMO_BOOT_ID_ENV} must be a UUID`);
  }
  return value;
};

const validatedToken = (value: string): string => {
  if (!TOKEN_PATTERN.test(value)) throw new Error("demo authorization token must be 256-bit base64url");
  const decoded = Buffer.from(value, "base64url");
  if (decoded.length !== 32 || decoded.toString("base64url") !== value) {
    throw new Error("demo authorization token must be 256-bit base64url");
  }
  return value;
};

/** Creates or atomically rotates the mode-0600 capability file for one demo boot. */
export const createDemoAuthorizationFile = async (
  runtimeDir: string,
  bootId: string,
): Promise<{ readonly tokenFile: string; readonly authorization: DemoAuthorization }> => {
  const validatedId = validatedBootId(bootId);
  await fs.mkdir(runtimeDir, { recursive: true, mode: 0o700 });
  const tokenFile = path.join(runtimeDir, DEMO_AUTH_TOKEN_FILENAME);
  const temporary = `${tokenFile}.${process.pid}.${crypto.randomUUID()}.tmp`;
  const token = randomBytes(32).toString("base64url");
  try {
    const handle = await fs.open(temporary, "wx", 0o600);
    try {
      await handle.writeFile(`${token}\n`, "utf8");
      await handle.sync();
    } finally {
      await handle.close();
    }
    await fs.rename(temporary, tokenFile);
    await fs.chmod(tokenFile, 0o600);
  } catch (error) {
    await fs.rm(temporary, { force: true });
    throw error;
  }
  const authorization = { bootId: validatedId, token };
  return {
    tokenFile,
    authorization,
  };
};

/** Loads the boot capability from process environment and its protected file, failing closed. */
export const readDemoAuthorizationFromEnv = async (
  env: NodeJS.ProcessEnv = process.env,
): Promise<DemoAuthorization> => {
  const bootId = validatedBootId(env[DEMO_BOOT_ID_ENV]);
  const tokenFile = env[DEMO_AUTH_TOKEN_FILE_ENV];
  if (tokenFile === undefined || !path.isAbsolute(tokenFile)) {
    throw new Error(`${DEMO_AUTH_TOKEN_FILE_ENV} must be an absolute path`);
  }
  const stat = await fs.stat(tokenFile);
  if (!stat.isFile()) throw new Error(`${DEMO_AUTH_TOKEN_FILE_ENV} must identify a regular file`);
  if ((stat.mode & 0o077) !== 0) {
    throw new Error(`${DEMO_AUTH_TOKEN_FILE_ENV} must not be accessible by group or other users`);
  }
  return {
    bootId,
    token: validatedToken((await fs.readFile(tokenFile, "utf8")).trim()),
  };
};

/** Loads the one browser origin allowed to call the cross-origin local faucet. */
export const readDemoAllowedOriginFromEnv = (env: NodeJS.ProcessEnv = process.env): string => {
  const value = env[DEMO_ALLOWED_ORIGIN_ENV];
  if (value === undefined) throw new Error(`${DEMO_ALLOWED_ORIGIN_ENV} is required`);
  const url = new URL(value);
  if (
    url.origin !== value ||
    url.protocol !== "http:" ||
    url.hostname !== "127.0.0.1" ||
    url.username !== "" ||
    url.password !== ""
  ) {
    throw new Error(`${DEMO_ALLOWED_ORIGIN_ENV} must be an exact http://127.0.0.1 origin`);
  }
  return value;
};

/** Authorizes one request without revealing the current boot id or token. */
export const authorizeDemoHeaders = (
  getHeader: (name: string) => string | null | undefined,
  authorization: DemoAuthorization,
): DemoAuthorizationDecision => {
  const bootId = getHeader("x-fhevm-demo-boot-id");
  const header = getHeader("authorization");
  if (!bootId || !header) {
    return { ok: false, status: 401, error: "demo authorization required" };
  }
  if (bootId !== authorization.bootId) {
    return { ok: false, status: 409, error: "stale demo boot; reopen the launch URL" };
  }
  const candidate = header.startsWith("Bearer ") ? header.slice("Bearer ".length) : "";
  const expectedBytes = Buffer.from(authorization.token);
  const candidateBytes = Buffer.from(candidate);
  if (candidateBytes.length !== expectedBytes.length || !timingSafeEqual(candidateBytes, expectedBytes)) {
    return { ok: false, status: 401, error: "demo authorization required" };
  }
  return { ok: true };
};
