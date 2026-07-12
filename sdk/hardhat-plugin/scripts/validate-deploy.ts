/**
 * M1 gate — proves the deploy sequence is faithful, without Hardhat.
 *
 * The engine has zero `hardhat` imports and drives the chain purely through EIP-1193, so it can be run
 * against a bare anvil node with a minimal request adapter. Run:
 *
 *   anvil --silent --port 8546 &
 *   npm run validate:deploy
 *
 * (No --disable-code-size-limit: setCode bypasses EIP-170.)
 */
import { JsonRpcProvider } from "ethers";

import { ADDRESSES } from "../src/engine/addresses";
import { getCleartextArtifact } from "../src/engine/artifacts";
import { deployCleartextEngine } from "../src/engine/deploy";
import { ifaceFor, readCleartext } from "../src/engine/rpc";

const RPC_URL = process.env.RPC_URL ?? "http://127.0.0.1:8546";

function ok(label: string, detail = ""): void {
  console.log(`  ✓ ${label}${detail ? ` — ${detail}` : ""}`);
}

async function main(): Promise<void> {
  const provider = new JsonRpcProvider(RPC_URL);
  // JsonRpcProvider exposes .send(method, params); adapt it to the EIP-1193 shape the engine expects.
  const eip1193 = { request: (a: { method: string; params?: unknown[] }) => provider.send(a.method, a.params ?? []) };

  console.log(`\nDeploying the v0.14 cleartext engine to ${RPC_URL} ...\n`);
  const engine = await deployCleartextEngine(eip1193);
  ok("deploy + init completed", `node=${engine.node.kind} chainId=${engine.node.chainId}`);

  // 1. The deployed code is EXACTLY the package artifact — not a stale copy, not the real host contract.
  for (const name of ["FHEVMExecutor", "ACL", "KMSVerifier", "InputVerifier", "HCULimit"] as const) {
    const artifact = getCleartextArtifact(name);
    const onchain = await engine.node.getCode(ADDRESSES[name]);
    if (onchain !== artifact.deployedBytecode) {
      throw new Error(`${name}: on-chain code differs from ${artifact.contractName} artifact`);
    }
    ok(`${name} bytecode == ${artifact.contractName}`, `${(onchain.length - 2) / 2} B`);
  }

  // 2. Signer sets (proves ProtocolConfig was initialized before KMSVerifier) + EIP-712 domains.
  //    assertEngineReady already ran inside deployCleartextEngine; re-state the values for the log.
  const kmsSigners = (await readCleartext(engine.node, "KMSVerifier", "getKmsSigners"))[0] as string[];
  ok("KMSVerifier.getKmsSigners()", kmsSigners.join(", "));
  const coprocessorSigners = (await readCleartext(engine.node, "InputVerifier", "getCoprocessorSigners"))[0] as string[];
  ok("InputVerifier.getCoprocessorSigners()", coprocessorSigners.join(", "));

  const inputDomain = await readCleartext(engine.node, "InputVerifier", "eip712Domain");
  ok("InputVerifier EIP-712 domain", `${inputDomain.name} v${inputDomain.version} chainId=${inputDomain.chainId}`);
  const kmsDomain = await readCleartext(engine.node, "KMSVerifier", "eip712Domain");
  ok("KMSVerifier EIP-712 domain", `${kmsDomain.name} v${kmsDomain.version} chainId=${kmsDomain.chainId}`);

  // 3. The cleartext surface is wired: executor.plaintexts -> CleartextACL.plaintext. An unknown handle
  //    must REVERT with CleartextACLCleartextNotSaved, not return 0. That revert is the only signal
  //    distinguishing "no such plaintext" from "the plaintext is zero", so decrypt must never swallow it.
  const unknownHandle = "0x" + "00".repeat(32);
  const notSavedSelector = ifaceFor("ACL").getError("CleartextACLCleartextNotSaved")!.selector;
  let reverted = false;
  try {
    await readCleartext(engine.node, "FHEVMExecutor", "plaintexts", [unknownHandle]);
  } catch (e) {
    const blob = JSON.stringify(e, Object.getOwnPropertyNames(e as object)).toLowerCase();
    if (!blob.includes(notSavedSelector.slice(2))) {
      throw new Error(`plaintexts(unknown) reverted, but not with CleartextACLCleartextNotSaved (${blob.slice(0, 200)})`);
    }
    reverted = true;
  }
  if (!reverted) {
    throw new Error("plaintexts(unknown handle) returned a value instead of reverting — ACL wiring is wrong.");
  }
  ok("FHEVMExecutor.plaintexts -> CleartextACL", "unknown handle reverts with CleartextACLCleartextNotSaved");

  // 4. HCULimit really was initialized. Without the 3-arg init every FHE op reverts HCUBlockLimitExceeded.
  const cap = (await readCleartext(engine.node, "HCULimit", "getGlobalHCUCapPerBlock"))[0] as bigint;
  const maxPerTx = (await readCleartext(engine.node, "HCULimit", "getMaxHCUPerTx"))[0] as bigint;
  const maxDepth = (await readCleartext(engine.node, "HCULimit", "getMaxHCUDepthPerTx"))[0] as bigint;
  if (cap === 0n || maxPerTx === 0n || maxDepth === 0n) {
    throw new Error(`HCULimit was not initialized (cap=${cap} perTx=${maxPerTx} depth=${maxDepth})`);
  }
  ok("HCULimit initialized", `cap=${cap} perTx=${maxPerTx} depth=${maxDepth}`);

  console.log("\nM1 GATE PASSED\n");
}

main().catch((e) => {
  console.error("\nM1 GATE FAILED\n");
  console.error(e);
  process.exit(1);
});
