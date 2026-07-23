import { execFileSync, spawn } from "child_process";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";

import { ContractFactory, type InterfaceAbi, JsonRpcProvider, Wallet } from "ethers";

import { FhevmApi } from "../src/api";
import { ADDRESSES } from "../src/engine/stack/addresses";
import { publicDecrypt } from "../src/engine/fhe/decrypt";
import { readCleartext } from "../src/engine/stack/rpc";
import { FhevmType } from "../src/types";

const PORT = 8899;
const RPC_URL = `http://127.0.0.1:${PORT}`;
// anvil's default mnemonic, account 0.
const DEPLOYER_PK = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const USER_PK = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

const INPUT_VALUE = 42n;
const ADDEND = 8n;
const EXPECTED = INPUT_VALUE + ADDEND; // 50

let failures = 0;

function check(label: string, actual: unknown, expected: unknown): void {
  const ok = actual === expected;
  console.log(`${ok ? "  ok  " : "  FAIL"} ${label}${ok ? "" : ` — got ${actual}, expected ${expected}`}`);
  if (!ok) {
    failures++;
  }
}

/** Compiles Harness.sol with forge in a scratch dir and returns its creation bytecode + ABI. */
function compileHarness(): { abi: InterfaceAbi; bytecode: string } {
  // __dirname is dist/test once compiled; the .sol lives in the source tree.
  const source = path.resolve(__dirname, "..", "..", "test", "Harness.sol");
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "fhevm-harness-"));
  fs.mkdirSync(path.join(root, "src"));
  fs.copyFileSync(source, path.join(root, "src", "Harness.sol"));
  fs.writeFileSync(path.join(root, "foundry.toml"), '[profile.default]\nsrc = "src"\nout = "out"\n');

  execFileSync("forge", ["build", "--root", root], { stdio: "pipe" });
  const artifact = JSON.parse(fs.readFileSync(path.join(root, "out", "Harness.sol", "Harness.json"), "utf8"));
  return { abi: artifact.abi, bytecode: artifact.bytecode.object };
}

async function main(): Promise<void> {
  console.log("compiling Harness.sol...");
  const harnessArtifact = compileHarness();

  console.log(`starting anvil on ${PORT}...`);
  const anvil = spawn("anvil", ["--port", String(PORT), "--silent"], { stdio: "ignore" });

  try {
    // cacheTimeout -1 disables ethers' short-lived RPC cache; otherwise eth_getTransactionCount is served
    // stale right after a deploy and the next tx is signed with an already-used nonce.
    const provider = new JsonRpcProvider(RPC_URL, undefined, { cacheTimeout: -1, staticNetwork: true });
    for (let i = 0; ; i++) {
      try {
        await provider.getBlockNumber();
        break;
      } catch (e) {
        if (i > 100) throw e;
        await new Promise((r) => setTimeout(r, 100));
      }
    }

    const eip1193 = { request: (a: { method: string; params?: unknown[] }) => provider.send(a.method, (a.params ?? []) as unknown[]) };
    const deployer = new Wallet(DEPLOYER_PK, provider);
    const user = new Wallet(USER_PK, provider);

    // 1. Deploy the cleartext engine.
    console.log("\ndeploying the cleartext engine...");
    const api = new FhevmApi({ provider: eip1193, isMock: true });
    const engine = await api.ensureDeployed();
    console.log(`  engine ready on chainId ${engine.node.chainId} (${engine.node.kind})`);

    // Every contract is placed and carries code.
    for (const [name, address] of Object.entries(ADDRESSES)) {
      const code = await provider.getCode(address);
      check(`${name} placed at ${address}`, code.length > 2, true);
    }

    // 2. Deploy the user-contract stand-in.
    console.log("\ndeploying Harness...");
    const harness = await new ContractFactory(harnessArtifact.abi, harnessArtifact.bytecode, deployer).deploy();
    await harness.waitForDeployment();
    const harnessAddress = await harness.getAddress();
    console.log(`  Harness at ${harnessAddress}`);

    // 3. Encrypt an input bound to (harness, user).
    console.log("\nencrypting an input...");
    const input = await api.createEncryptedInput(harnessAddress, user.address).add64(INPUT_VALUE).encrypt();
    console.log(`  handle     ${input.handles[0]}`);
    console.log(`  inputProof ${input.inputProof.length / 2 - 1} bytes`);

    // 4. The real path: verifyInput (coprocessor EIP-712 checked on-chain by InputVerifier) -> fheAdd ->
    //    ACL grants. If the input proof or the cleartext channel were wrong, this reverts or records nothing.
    console.log("\nrunning verifyInput + fheAdd + ACL grants...");
    const tx = await (harness.connect(deployer) as any).ingestAndAdd(
      input.handles[0],
      input.inputProof,
      user.address,
      ADDEND,
    );
    await tx.wait();
    const resultHandle: string = await (harness as any).result();
    console.log(`  result handle ${resultHandle}`);

    // 5. The cleartext actually landed in CleartextDB, via the executor -> CleartextArithmetic -> DB chain.
    const [viaExecutor] = await readCleartext(engine.node, "FHEVMExecutor", "plaintexts", [resultHandle]);
    check(`executor.plaintexts(result) == ${EXPECTED}`, BigInt(viaExecutor as bigint), EXPECTED);
    const [viaDb] = await readCleartext(engine.node, "CleartextDB", "get", [resultHandle]);
    check(`cleartextDb.get(result) == ${EXPECTED}`, BigInt(viaDb as bigint), EXPECTED);

    // 6. User decryption: EIP-712 signed here, verified ON-CHAIN, result XOR-masked and unmasked here.
    console.log("\nuser-decrypting...");
    const userValue = await api.userDecryptEuint(FhevmType.euint64, resultHandle, harnessAddress, user);
    check(`userDecryptEuint == ${EXPECTED}`, userValue, EXPECTED);

    // 7. Public decryption (the handle was marked publicly decryptable by the harness).
    console.log("\npublic-decrypting...");
    const [publicValue] = await publicDecrypt(engine.node, [resultHandle]);
    check(`publicDecrypt == ${EXPECTED}`, publicValue, EXPECTED);

    // 8. ACL is really enforced: a stranger who was never granted the handle must be rejected.
    console.log("\nchecking ACL rejects an unauthorized user...");
    const stranger = Wallet.createRandom().connect(provider);
    let rejected = false;
    try {
      await api.userDecryptEuint(FhevmType.euint64, resultHandle, harnessAddress, stranger);
    } catch {
      rejected = true;
    }
    check("unauthorized user-decrypt reverts", rejected, true);
  } finally {
    anvil.kill();
  }

  console.log(failures === 0 ? "\nAll checks passed." : `\n${failures} check(s) FAILED.`);
  process.exit(failures === 0 ? 0 : 1);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
