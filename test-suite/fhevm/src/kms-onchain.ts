/**
 * Shared on-chain helpers for the KMS acceptance profiles: load the deployer (the ACL owner on
 * devnet / the test-suite), broadcast an owner transaction to any contract, assert a custom-error
 * revert, and pull uint / event values out of cast output and transaction receipts.
 *
 * Both the KMS-generation abort profile (targets KMSGeneration) and the KMS context-switch /
 * context-epoch-destroy profile (targets ProtocolConfig) drive owner transactions the same way, so
 * the plumbing lives here and each profile passes its own contract address.
 */
import { PreflightError } from "./errors";
import { envPath } from "./layout";
import { readEnvFile, uint256ToId, withHexPrefix } from "./utils/fs";
import { run } from "./utils/process";

export type Receipt = { status: string; logs: { address: string; topics: string[]; data: string }[] };
export type Owner = { key: string; address: string };

/** The little-endian byte order the connector stores U256 ids in (alloy `as_le_slice`). */
export const uint256LeHex = (id: bigint) => {
  const be = uint256ToId(id);
  let le = "";
  for (let i = be.length - 2; i >= 0; i -= 2) {
    le += be.slice(i, i + 2);
  }
  return le;
};

/** Parses the first integer token of cast's `<decimal> [<sci-notation>]` output. */
export const parseUintOutput = (raw: string): bigint => {
  const token = raw
    .replace(/\[[^\]]*\]/g, " ")
    .split(/\s+/)
    .find((candidate) => /^(0x[0-9a-fA-F]+|\d+)$/.test(candidate));
  if (token === undefined) {
    throw new PreflightError(`could not parse a uint from cast output: ${JSON.stringify(raw)}`);
  }
  return BigInt(token);
};

/** First 32-byte word of a log's `data` field (non-indexed leading argument). */
export const firstDataWord = (data: string): bigint => {
  const hex = data.replace(/^0x/, "");
  if (hex.length < 64) {
    throw new PreflightError(`event data too short for a uint256 word: ${data}`);
  }
  return BigInt(`0x${hex.slice(0, 64)}`);
};

/** Extracts the leading non-indexed uint256 of the receipt event matching `topic0`, or throws. */
export const eventLogWord = (receipt: Receipt, topic0: string, eventName: string): bigint => {
  const log = receipt.logs.find((entry) => entry.topics[0]?.toLowerCase() === topic0.toLowerCase());
  if (!log) {
    throw new PreflightError(
      `transaction receipt has no ${eventName} event (topics seen: ${receipt.logs.map((entry) => entry.topics[0]).join(", ") || "none"})`,
    );
  }
  return firstDataWord(log.data);
};

/** Reads an indexed topic of the receipt event whose topic0 matches `targetTopic`, or throws.
 * Indexed uint256 args (e.g. ProtocolConfig's `KmsContextDestroyed(uint256 indexed)`) live in
 * `topics`, not `data`. */
export const getEventTopic = (receipt: Receipt, targetTopic: string, topicIndex: number): bigint => {
  const log = receipt.logs.find((entry) => entry.topics[0]?.toLowerCase() === targetTopic.toLowerCase());
  if (!log) {
    throw new PreflightError(
      `transaction receipt has no event with topic ${targetTopic} (topics seen: ${receipt.logs.map((entry) => entry.topics[0]).join(", ") || "none"})`,
    );
  }
  const topic = log.topics[topicIndex];
  if (topic === undefined) {
    throw new PreflightError(`event with topic ${targetTopic} has no indexed topic at position ${topicIndex}`);
  }
  return BigInt(topic);
};

const keccakCache = new Map<string, string>();

/** `cast keccak`, memoized — the same event/error signatures are hashed several times per run. */
export const keccakTopic = async (signature: string) => {
  let hash = keccakCache.get(signature);
  if (hash === undefined) {
    hash = (await run(["cast", "keccak", signature])).stdout.trim();
    keccakCache.set(signature, hash);
  }
  return hash;
};

/** Loads the host-chain owner (the deployer) from the generated host-sc env. */
export const loadHostOwner = async (): Promise<Owner> => {
  const env = await readEnvFile(envPath("host-sc"));
  const rawKey = env.DEPLOYER_PRIVATE_KEY;
  if (!rawKey) {
    throw new PreflightError(
      `no DEPLOYER_PRIVATE_KEY in ${envPath("host-sc")} — cannot act as the ProtocolConfig / KMSGeneration owner`,
    );
  }
  const key = withHexPrefix(rawKey);
  // allowFailure everywhere the key is on the command line: CommandError echoes the full argv.
  const result = await run(["cast", "wallet", "address", "--private-key", key], { allowFailure: true });
  if (result.code !== 0) {
    throw new PreflightError(
      `could not derive the owner address from DEPLOYER_PRIVATE_KEY: ${(result.stderr || result.stdout).trim()}`,
    );
  }
  return { key, address: result.stdout.trim() };
};

/** Sends an owner transaction to `address` and returns the parsed receipt. Throws a PreflightError
 * without echoing the command line (it carries the private key). */
export const castSend = async (
  rpcUrl: string,
  address: string,
  owner: Owner,
  signature: string,
  ...args: string[]
): Promise<Receipt> => {
  const result = await run(
    ["cast", "send", address, signature, ...args, "--rpc-url", rpcUrl, "--private-key", owner.key, "--json"],
    { allowFailure: true },
  );
  if (result.code !== 0) {
    throw new PreflightError(
      `cast send ${signature} [${args.join(", ")}] failed: ${(result.stderr || result.stdout).trim().slice(0, 400)}`,
    );
  }
  try {
    return JSON.parse(result.stdout) as Receipt;
  } catch {
    throw new PreflightError(`cast send ${signature} returned a non-JSON receipt: ${result.stdout.trim().slice(0, 200)}`);
  }
};

/** Asserts an eth_call from the owner to `address` reverts with the given custom error. */
export const callContractAndExpectRevert = async (
  rpcUrl: string,
  address: string,
  owner: Owner,
  label: string,
  errorSignature: string,
  callSignature: string,
  ...args: string[]
) => {
  const result = await run(
    ["cast", "call", address, callSignature, ...args, "--from", owner.address, "--rpc-url", rpcUrl],
    { allowFailure: true },
  );
  if (result.code === 0) {
    throw new PreflightError(`${label}: expected ${errorSignature} revert, but the call succeeded`);
  }
  // Match the 4-byte selector in the revert data; also accept cast decoding the error by name.
  const selector = (await keccakTopic(errorSignature)).slice(2, 10);
  const errorName = errorSignature.split("(")[0];
  const output = `${result.stdout}\n${result.stderr}`;
  if (!output.includes(selector) && !output.includes(errorName)) {
    throw new PreflightError(
      `${label}: reverted, but not with ${errorSignature} (selector ${selector}): ${output.trim().slice(0, 300)}`,
    );
  }
};
