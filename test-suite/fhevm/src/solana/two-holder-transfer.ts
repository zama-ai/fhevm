import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { address, createKeyPairSignerFromBytes, getAddressEncoder } from "@solana/kit";

import { COPROCESSOR_DB_CONTAINER, REPO_ROOT } from "../layout";
import { runSolanaCurrentUserDecrypt } from "./current-user-decrypt";
import { run } from "../utils/process";

export const SOLANA_TWO_HOLDER_TRANSFER_PROFILE = "solana-two-holder-transfer";
export const SOLANA_TWO_HOLDER_TRANSFER_DESCRIPTION =
  "Transfer an SDK-encrypted euint64 between two real Solana holders and decrypt both latest balances.";

const RPC_URL = "http://127.0.0.1:8899";
const WS_URL = "ws://127.0.0.1:8900";
const RELAYER_URL = "http://127.0.0.1:3000";
const ACL_PROGRAM = "0x4cd3022dff504a675caf2d9b4f4014d0b3dc3ea17ffb97ba355cec5a933a30ee";
const DEFAULT_USER_DECRYPT_CONTEXT =
  "3166189940082864718613269121331309980362851143201109172953918312716374638593";
const LIVE_CLIENT = path.join(REPO_ROOT, "solana/scripts/e2e/live-client/target/debug/poc-live-client");
const LIVE_CLIENT_DIR = path.join(REPO_ROOT, "solana/scripts/e2e/live-client");
const SDK_WORKER = path.join(REPO_ROOT, "test-suite/fhevm/solana-two-holder-transfer.ts");
const CLI_DIR = path.join(REPO_ROOT, "test-suite/fhevm");
const BASE58 = /^[1-9A-HJ-NP-Za-km-z]{32,44}$/;
const SIGNATURE = /^[1-9A-HJ-NP-Za-km-z]{87,88}$/;
const BYTES32 = /^0x[0-9a-f]{64}$/i;

export type BalanceState = {
  version: 1;
  mint: string;
  owner: string;
  tokenAccount: string;
  encryptedValueAccount: string;
  aclValueKey: string;
  currentHandle: string;
  chainId: string;
};

export type Holder = { owner: string; keypairPath: string; secretKey: string };
export type TwoHolderScenario = {
  mint: string;
  computeSigner: string;
  alice: Holder;
  bob: Holder;
};

export type TwoHolderDependencies = {
  provision(): Promise<TwoHolderScenario>;
  readBalance(scenario: TwoHolderScenario, holder: Holder): Promise<BalanceState>;
  waitForHandle(handle: string): Promise<void>;
  transfer(scenario: TwoHolderScenario, alice: BalanceState, bob: BalanceState): Promise<void>;
  decrypt(scenario: TwoHolderScenario, holder: Holder, state: BalanceState, expected: bigint): Promise<bigint>;
  cleanup(scenario: TwoHolderScenario | undefined): Promise<void>;
};

const parseJsonLine = (output: string): unknown => {
  const line = output.trim().split(/\r?\n/).at(-1);
  if (!line) throw new Error("command did not emit final-line JSON");
  return JSON.parse(line) as unknown;
};

const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === "object" && value !== null && !Array.isArray(value);

const hasExactKeys = (value: Record<string, unknown>, keys: readonly string[]) =>
  Object.keys(value).sort().join(",") === [...keys].sort().join(",");

export const parseBalanceState = (output: string, expectedMint: string, expectedOwner: string): BalanceState => {
  const value = parseJsonLine(output);
  if (
    !isRecord(value) ||
    !hasExactKeys(value, [
      "version",
      "mint",
      "owner",
      "tokenAccount",
      "encryptedValueAccount",
      "aclValueKey",
      "currentHandle",
      "chainId",
    ]) ||
    value.version !== 1 ||
    value.mint !== expectedMint ||
    value.owner !== expectedOwner
  ) {
    throw new Error("balance-state probe identity or version mismatch");
  }
  for (const name of ["tokenAccount", "encryptedValueAccount"] as const) {
    if (typeof value[name] !== "string" || !BASE58.test(value[name])) {
      throw new Error(`balance-state probe has invalid ${name}`);
    }
  }
  for (const name of ["aclValueKey", "currentHandle"] as const) {
    if (typeof value[name] !== "string" || !BYTES32.test(value[name])) {
      throw new Error(`balance-state probe has invalid ${name}`);
    }
  }
  if (typeof value.chainId !== "string" || !/^\d+$/.test(value.chainId) || (BigInt(value.chainId) & (1n << 63n)) === 0n) {
    throw new Error("balance-state probe has invalid Solana chainId");
  }
  return {
    version: 1,
    mint: value.mint,
    owner: value.owner,
    tokenAccount: value.tokenAccount as string,
    encryptedValueAccount: value.encryptedValueAccount as string,
    aclValueKey: value.aclValueKey as string,
    currentHandle: value.currentHandle as string,
    chainId: value.chainId,
  };
};

export const parseTransferWorkerResult = (output: string): void => {
  const value = parseJsonLine(output);
  if (
    !isRecord(value) ||
    !hasExactKeys(value, ["version", "signature", "inputHandle"]) ||
    value.version !== 1 ||
    typeof value.signature !== "string" ||
    !SIGNATURE.test(value.signature) ||
    typeof value.inputHandle !== "string" ||
    !BYTES32.test(value.inputHandle)
  ) {
    throw new Error("SDK transfer worker returned malformed versioned JSON");
  }
};

const keypair = async (file: string, expectedOwner: string): Promise<{ secretKey: string }> => {
  const bytes = JSON.parse(await fs.readFile(file, "utf8")) as unknown;
  if (!Array.isArray(bytes) || bytes.length !== 64 || bytes.some((byte) => !Number.isInteger(byte) || byte < 0 || byte > 255)) {
    throw new Error(`invalid Solana keypair ${file}`);
  }
  const signer = await createKeyPairSignerFromBytes(Uint8Array.from(bytes));
  if (signer.address !== expectedOwner) throw new Error(`keypair ${file} does not match owner ${expectedOwner}`);
  return { secretKey: `0x${Buffer.from(bytes.slice(0, 32)).toString("hex")}` };
};

export const solanaUserDecryptContext = (
  decimal = process.env.SOLANA_UD_CONTEXT_ID ?? DEFAULT_USER_DECRYPT_CONTEXT,
): string => {
  if (!/^\d+$/.test(decimal)) throw new Error("SOLANA_UD_CONTEXT_ID must be an unsigned decimal integer");
  const value = BigInt(decimal);
  if (value >= 1n << 256n) throw new Error("SOLANA_UD_CONTEXT_ID must fit in 32 bytes");
  return `0x${value.toString(16).padStart(64, "0")}`;
};

const runLiveClient = (environment: Record<string, string>, home?: string) =>
  run([LIVE_CLIENT], { cwd: LIVE_CLIENT_DIR, env: { ...environment, ...(home ? { HOME: home } : {}) } });

const captureAddress = (output: string, label: string): string => {
  const match = output.match(new RegExp(`${label}\\s+([1-9A-HJ-NP-Za-km-z]{32,44})`, "i"));
  if (!match?.[1]) throw new Error(`could not parse ${label}`);
  return match[1];
};

const realDependencies = (): TwoHolderDependencies => {
  let bobHome: string | undefined;
  return {
    async provision() {
      await fs.access(LIVE_CLIENT);
      const aliceKeypair = path.join(os.homedir(), ".config/solana/id.json");
      const aliceOwner = (await run(["solana", "address", "-k", aliceKeypair])).stdout.trim();
      if (!BASE58.test(aliceOwner)) throw new Error("invalid Alice address");
      bobHome = await fs.mkdtemp(path.join(os.tmpdir(), "fhevm-solana-bob-"));
      const bobKeypair = path.join(bobHome, ".config/solana/id.json");
      await fs.mkdir(path.dirname(bobKeypair), { recursive: true });
      await run(["solana-keygen", "new", "--no-bip39-passphrase", "--silent", "--force", "--outfile", bobKeypair]);
      const bobOwner = (await run(["solana", "address", "-k", bobKeypair])).stdout.trim();
      if (!BASE58.test(bobOwner) || bobOwner === aliceOwner) throw new Error("Bob must have a distinct valid address");
      await run(["solana", "airdrop", "5", bobOwner, "--url", RPC_URL]);

      const underlyingOutput = await run(["spl-token", "create-token", "--decimals", "9", "--url", RPC_URL]);
      const underlying = captureAddress(`${underlyingOutput.stdout}\n${underlyingOutput.stderr}`, "Creating token");
      await run(["spl-token", "create-account", underlying, "--url", RPC_URL]);
      await run(["spl-token", "mint", underlying, "1000000", "--url", RPC_URL]);
      const mintOutput = await runLiveClient({ UNDERLYING_MINT: underlying });
      const mintText = `${mintOutput.stdout}\n${mintOutput.stderr}`;
      const mint = captureAddress(mintText, "confidential mint");
      const computeSigner = captureAddress(mintText, "compute_signer");
      await runLiveClient({ CONSUME_WRAP: "1", MINT: mint, UNDERLYING_MINT: underlying, WRAP_AMOUNT: "1000" });
      await runLiveClient({ INITIALIZE_TOKEN_ACCOUNT: "1", MINT: mint }, bobHome);
      const alice = { owner: aliceOwner, keypairPath: aliceKeypair, ...(await keypair(aliceKeypair, aliceOwner)) };
      const bob = { owner: bobOwner, keypairPath: bobKeypair, ...(await keypair(bobKeypair, bobOwner)) };
      return { mint, computeSigner, alice, bob };
    },
    async readBalance(scenario, holder) {
      const result = await runLiveClient({ TOKEN_BALANCE_STATE: "1", MINT: scenario.mint, TOKEN_OWNER: holder.owner });
      return parseBalanceState(result.stdout, scenario.mint, holder.owner);
    },
    async waitForHandle(handle) {
      if (!BYTES32.test(handle)) throw new Error("invalid handle before ciphertext wait");
      const hex = handle.slice(2);
      for (let attempt = 0; attempt < 40; attempt += 1) {
        const result = await run(
          [
            "docker",
            "exec",
            COPROCESSOR_DB_CONTAINER,
            "psql",
            "-U",
            "postgres",
            "-d",
            "coprocessor",
            "-tAc",
            `SELECT ciphertext IS NOT NULL AND ciphertext128 IS NOT NULL FROM ciphertext_digest WHERE handle=decode('${hex}','hex')`,
          ],
          { allowFailure: true },
        );
        if (result.code === 0 && result.stdout.trim() === "t") return;
        await Bun.sleep(3_000);
      }
      throw new Error(`ciphertext materialization timed out for ${handle}`);
    },
    async transfer(scenario, alice, bob) {
      if (alice.chainId !== bob.chainId) throw new Error("Alice and Bob balance handles disagree on chain id");
      const result = await run(["node", SDK_WORKER], {
        cwd: CLI_DIR,
        env: {
          TRANSFER_RPC_URL: RPC_URL,
          TRANSFER_WS_URL: WS_URL,
          TRANSFER_RELAYER_URL: RELAYER_URL,
          TRANSFER_ACL_PROGRAM: ACL_PROGRAM,
          TRANSFER_CHAIN_ID: alice.chainId,
          TRANSFER_OWNER_KEYPAIR: scenario.alice.keypairPath,
          TRANSFER_OWNER: scenario.alice.owner,
          TRANSFER_RECIPIENT: scenario.bob.owner,
          TRANSFER_MINT: scenario.mint,
          TRANSFER_COMPUTE_SIGNER: scenario.computeSigner,
          TRANSFER_FROM_ACCOUNT: alice.tokenAccount,
          TRANSFER_TO_ACCOUNT: bob.tokenAccount,
          TRANSFER_FROM_BALANCE: alice.encryptedValueAccount,
          TRANSFER_TO_BALANCE: bob.encryptedValueAccount,
        },
      });
      parseTransferWorkerResult(result.stdout);
    },
    decrypt: (scenario, holder, state, expected) =>
      runSolanaCurrentUserDecrypt({
        UD_RELAYER_URL: RELAYER_URL,
        UD_CONTRACTS_CHAIN_ID: state.chainId,
        UD_HANDLE: state.currentHandle,
        UD_SECRET_KEY: holder.secretKey,
        UD_CONTEXT_ID: solanaUserDecryptContext(),
        UD_ALLOWED_DOMAIN_KEYS: `0x${Buffer.from(getAddressEncoder().encode(address(scenario.mint))).toString("hex")}`,
        UD_ACL_VALUE_KEY: state.aclValueKey,
        UD_EXPECTED: expected.toString(),
      }),
    async cleanup() {
      if (bobHome) await fs.rm(bobHome, { recursive: true, force: true });
      bobHome = undefined;
    },
  };
};

/** Runs one real two-holder transfer and proves both current balances through independent SDK decrypts. */
export const runSolanaTwoHolderTransfer = async (dependencies: TwoHolderDependencies = realDependencies()) => {
  let scenario: TwoHolderScenario | undefined;
  try {
    scenario = await dependencies.provision();
    const initialAlice = await dependencies.readBalance(scenario, scenario.alice);
    const initialBob = await dependencies.readBalance(scenario, scenario.bob);
    await dependencies.waitForHandle(initialAlice.currentHandle);
    await dependencies.waitForHandle(initialBob.currentHandle);
    await dependencies.decrypt(scenario, scenario.alice, initialAlice, 1000n);
    await dependencies.decrypt(scenario, scenario.bob, initialBob, 0n);

    await dependencies.transfer(scenario, initialAlice, initialBob);
    const finalAlice = await dependencies.readBalance(scenario, scenario.alice);
    const finalBob = await dependencies.readBalance(scenario, scenario.bob);
    if (finalAlice.currentHandle === initialAlice.currentHandle || finalBob.currentHandle === initialBob.currentHandle) {
      throw new Error("confidential transfer did not rotate both current balance handles");
    }
    await dependencies.waitForHandle(finalAlice.currentHandle);
    await dependencies.waitForHandle(finalBob.currentHandle);
    await dependencies.decrypt(scenario, scenario.alice, finalAlice, 600n);
    await dependencies.decrypt(scenario, scenario.bob, finalBob, 400n);
    console.log("[solana-two-holder-transfer] Alice=600 Bob=400");
  } finally {
    await dependencies.cleanup(scenario);
  }
};
