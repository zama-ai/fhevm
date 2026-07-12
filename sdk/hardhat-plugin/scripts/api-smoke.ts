/**
 * M4 gate — exercises the `FhevmApi` surface exactly as the template does, but without the full Hardhat
 * toolchain: the api is Hardhat-free (takes an EIP-1193 provider + ethers signers), so it can be driven
 * against a bare anvil. `index.ts`'s `extendEnvironment` glue is covered separately at M5 by the real
 * template run.
 *
 *   fhevm.createEncryptedInput(cut, user).add32(x).encrypt()  ->  CUT.add  ->  fhevm.userDecryptEuint(...)
 *
 * Run:  anvil --silent --port 8546 &   then   npm run api:smoke
 */
import * as fs from "fs";

import { Interface, JsonRpcProvider, Wallet, getAddress } from "ethers";

import { FhevmApi } from "../src/api";
import { FhevmType } from "../src/types";

const RPC_URL = process.env.RPC_URL ?? "http://127.0.0.1:8546";
const CUT_ARTIFACT =
  "/Users/aurora/Desktop/aurora/cleartext-mock/fhevm/host-contracts-cleartext/out/FhevmHarnessSmoke.t.sol/Adder.json";
const USER_PK = "0xac0974bec39a17e36ba4a6b4d238ff944bac47a8b3c62d5d24b6d0e3ba24aeb2";

let failures = 0;
const ok = (l: string, d = ""): void => void console.log(`  ✓ ${l}${d ? ` — ${d}` : ""}`);
const bad = (l: string, d = ""): void => void (failures++, console.log(`  ✗ ${l}${d ? ` — ${d}` : ""}`));

async function main(): Promise<void> {
  const provider = new JsonRpcProvider(RPC_URL);
  const eip1193 = { request: (a: { method: string; params?: unknown[] }) => provider.send(a.method, a.params ?? []) };

  const fhevm = new FhevmApi({ provider: eip1193, isMock: true });
  console.log(`\nM4 API smoke on ${RPC_URL} ...\n`);

  if (fhevm.isMock) ok("fhevm.isMock", "true");
  else bad("fhevm.isMock", "expected true");

  await fhevm.initializeCLIApi();
  ok("initializeCLIApi() (deploys engine)");

  // Impersonate + fund the user (not an anvil default account); the api signs the EIP-712 locally.
  const user = new Wallet(USER_PK);
  const userAddr = getAddress(user.address);
  await eip1193.request({ method: "anvil_impersonateAccount", params: [userAddr] });
  await eip1193.request({ method: "anvil_setBalance", params: [userAddr, "0x" + (10n ** 22n).toString(16)] });

  // Deploy the CUT (constructor sets coprocessor config).
  const artifact = JSON.parse(fs.readFileSync(CUT_ARTIFACT, "utf8"));
  const iface = new Interface(artifact.abi);
  const deployHash = (await eip1193.request({
    method: "eth_sendTransaction",
    params: [{ from: userAddr, data: artifact.bytecode.object }],
  })) as string;
  const deployReceipt = (await eip1193.request({ method: "eth_getTransactionReceipt", params: [deployHash] })) as {
    contractAddress: string;
  };
  const cut = getAddress(deployReceipt.contractAddress);
  ok("CUT deployed", cut);

  // createEncryptedInput(...).addN(x).encrypt() — the template's call shape. The Adder CUT consumes
  // euint64, so add64 here (the real template M5 uses a euint32 CUT with add32; the surface is the same).
  const enc = await fhevm.createEncryptedInput(cut, userAddr).add64(41).add64(1).encrypt();
  ok("createEncryptedInput().add64().encrypt()", `${enc.handles.length} handles, proof ${enc.inputProof.length} chars`);
  if (typeof enc.handles[0] !== "string" || !enc.handles[0].startsWith("0x")) {
    bad("handle is hex string", `got ${typeof enc.handles[0]}`);
  }

  // CUT.add(a, b, proof).
  const addData = iface.encodeFunctionData("add", [enc.handles[0], enc.handles[1], enc.inputProof]);
  const addHash = (await eip1193.request({
    method: "eth_sendTransaction",
    params: [{ from: userAddr, to: cut, data: addData }],
  })) as string;
  const addReceipt = (await eip1193.request({ method: "eth_getTransactionReceipt", params: [addHash] })) as {
    status: string;
  };
  if (addReceipt.status !== "0x1") {
    bad("CUT.add", `reverted (status ${addReceipt.status})`);
  } else {
    ok("CUT.add executed");
  }

  // Read the result handle, then userDecryptEuint(...) — the template's decrypt call shape.
  const resultHandle = iface.decodeFunctionResult(
    "result",
    await eip1193.request({ method: "eth_call", params: [{ to: cut, data: iface.encodeFunctionData("result", []) }, "latest"] }),
  )[0] as string;

  const clear = await fhevm.userDecryptEuint(FhevmType.euint64, resultHandle, cut, user);
  if (clear === 42n) ok("userDecryptEuint(result)", `${clear} == 41 + 1`);
  else bad("userDecryptEuint(result)", `got ${clear}, expected 42`);

  console.log(failures === 0 ? "\nM4 GATE PASSED\n" : `\nM4 GATE FAILED — ${failures} failure(s)\n`);
  if (failures > 0) process.exit(1);
}

main().catch((e) => {
  console.error("\nM4 GATE FAILED\n");
  console.error(e);
  process.exit(1);
});
