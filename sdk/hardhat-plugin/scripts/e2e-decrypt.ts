/**
 * M3 gate — full end-to-end against a live anvil, with a real contract-under-test.
 *
 *   encrypt (M2) -> CUT.add (FHE.fromExternal + FHE.add + FHE.allow) -> userDecrypt (M3)
 *
 * The CUT is the package's own `Adder` (from FhevmHarnessSmoke.t.sol), CREATE-deployed so its
 * constructor wires the coprocessor config. This is the first test that exercises the whole pipeline
 * on the real on-chain cleartext engine. Run:
 *
 *   anvil --silent --port 8546 &
 *   npm run e2e:decrypt
 */
import * as fs from "fs";

import { Interface, JsonRpcProvider, Wallet, getAddress, hexlify } from "ethers";

import { FheType } from "../src/engine/fhetype";
import { deployCleartextEngine } from "../src/engine/deploy";
import { encryptInput } from "../src/engine/encrypt";
import { generateKeypair, userDecrypt } from "../src/engine/decrypt";
import { FhevmNode } from "../src/engine/node";

const RPC_URL = process.env.RPC_URL ?? "http://127.0.0.1:8546";
const CUT_ARTIFACT =
  "/Users/aurora/Desktop/aurora/cleartext-mock/fhevm/host-contracts-cleartext/out/FhevmHarnessSmoke.t.sol/Adder.json";

// anvil default account #0 — unlocked, well-known key. Acts as the CUT caller and the decrypting user.
const USER_PK = "0xac0974bec39a17e36ba4a6b4d238ff944bac47a8b3c62d5d24b6d0e3ba24aeb2";

let failures = 0;
function ok(label: string, detail = ""): void {
  console.log(`  ✓ ${label}${detail ? ` — ${detail}` : ""}`);
}
function bad(label: string, detail = ""): void {
  failures++;
  console.log(`  ✗ ${label}${detail ? ` — ${detail}` : ""}`);
}

async function deployCut(node: FhevmNode, from: string): Promise<{ address: string; iface: Interface }> {
  const artifact = JSON.parse(fs.readFileSync(CUT_ARTIFACT, "utf8"));
  const hash = (await node.provider.request({
    method: "eth_sendTransaction",
    params: [{ from, data: artifact.bytecode.object }],
  })) as string;
  const receipt = (await node.provider.request({
    method: "eth_getTransactionReceipt",
    params: [hash],
  })) as { contractAddress?: string; status?: string };
  if (receipt?.status !== "0x1" || !receipt.contractAddress) {
    throw new Error(`CUT deploy failed (status ${receipt?.status})`);
  }
  return { address: getAddress(receipt.contractAddress), iface: new Interface(artifact.abi) };
}

async function main(): Promise<void> {
  const provider = new JsonRpcProvider(RPC_URL);
  const eip1193 = { request: (a: { method: string; params?: unknown[] }) => provider.send(a.method, a.params ?? []) };

  console.log(`\nE2E on ${RPC_URL} ...\n`);
  const engine = await deployCleartextEngine(eip1193);
  const node = engine.node;
  const user = new Wallet(USER_PK);
  const userAddr = getAddress(user.address);
  ok("engine deployed", `chainId=${node.chainId}`);

  // Impersonate + fund the user so the node accepts `eth_sendTransaction` from it (it need not be an
  // anvil default account). The EIP-712 is still signed locally by the Wallet.
  await node.impersonate(userAddr);
  await node.setBalance(userAddr, 10_000n * 10n ** 18n);

  const cut = await deployCut(node, userAddr);
  ok("CUT (Adder) deployed", cut.address);

  // encrypt two euint64s targeting the CUT, owned by the user.
  const a = 40n;
  const b = 2n;
  const enc = await encryptInput({
    values: [
      { type: FheType.euint64, value: a },
      { type: FheType.euint64, value: b },
    ],
    aclAddress: engine.addresses.ACL,
    contractAddress: cut.address,
    userAddress: userAddr,
    hostChainId: node.chainId,
    handleVersion: 0,
    coprocessorSigners: [engine.signers.coprocessor],
    coprocessorThreshold: 1,
  });
  ok("encrypted inputs", `${enc.handles.length} handles, proof ${enc.inputProof.length} B`);

  // CUT.add(a, b, proof) — runs FHE.fromExternal + FHE.add + FHE.allow(result, msg.sender).
  const addData = cut.iface.encodeFunctionData("add", [hexlify(enc.handles[0]), hexlify(enc.handles[1]), hexlify(enc.inputProof)]);
  await node.sendTransaction({ from: userAddr, to: cut.address, data: addData });
  ok("CUT.add executed on-chain");

  // Read the result handle.
  const resultRaw = await node.call(cut.address, cut.iface.encodeFunctionData("result", []));
  const resultHandle = cut.iface.decodeFunctionResult("result", resultRaw)[0] as string;
  ok("result handle", resultHandle);

  // 1. Authorized user-decrypt -> a + b.
  const [clear] = await userDecrypt(node, {
    pairs: [{ handle: resultHandle, contractAddress: cut.address }],
    user,
    keypair: generateKeypair(),
  });
  if (clear === a + b) {
    ok("userDecrypt(result)", `${clear} == ${a} + ${b}`);
  } else {
    bad("userDecrypt(result)", `got ${clear}, expected ${a + b}`);
  }

  // 2. Unauthorized user -> the on-chain ACL read must REVERT (not return 0).
  const stranger = Wallet.createRandom();
  try {
    const [leaked] = await userDecrypt(node, {
      pairs: [{ handle: resultHandle, contractAddress: cut.address }],
      user: stranger,
      keypair: generateKeypair(),
    });
    bad("unauthorized user-decrypt", `did NOT throw — leaked ${leaked} (ACL bypassed!)`);
  } catch {
    ok("unauthorized user-decrypt reverts", "ACL enforced on-chain, not swallowed");
  }

  // 3. Unknown handle -> revert, never a silent 0.
  const unknownHandle = "0x" + "11".repeat(31) + "05"; // euint64 type byte, never persisted
  try {
    const [leaked] = await userDecrypt(node, {
      pairs: [{ handle: unknownHandle, contractAddress: cut.address }],
      user,
      keypair: generateKeypair(),
    });
    bad("unknown-handle decrypt", `did NOT throw — returned ${leaked}`);
  } catch {
    ok("unknown-handle decrypt reverts", "no silent zero");
  }

  console.log(failures === 0 ? "\nM3 GATE PASSED\n" : `\nM3 GATE FAILED — ${failures} failure(s)\n`);
  if (failures > 0) process.exit(1);
}

main().catch((e) => {
  console.error("\nM3 GATE FAILED\n");
  console.error(e);
  process.exit(1);
});
