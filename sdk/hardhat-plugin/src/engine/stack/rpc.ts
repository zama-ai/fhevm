import { Interface, type Result } from "ethers";

import { ADDRESSES, type HostContractName } from "./addresses";
import { getAbi } from "./abis";
import { FhevmNode } from "../node";

/**
 * ABI-aware read/write helpers over the provider-neutral {@link FhevmNode}.
 *
 * `Interface` is used purely as an ABI codec (encode calldata / decode results) — it is not bound to any
 * provider or signer, so the engine stays provider-neutral: all chain interaction still flows through the
 * node's EIP-1193 `request`.
 */
const ifaceCache = new Map<HostContractName, Interface>();

export function ifaceFor(name: HostContractName): Interface {
  let iface = ifaceCache.get(name);
  if (!iface) {
    iface = new Interface(getAbi(name));
    ifaceCache.set(name, iface);
  }
  return iface;
}

/** `eth_call` a view function on a host contract and return the decoded result. */
export async function readCleartext(
  node: FhevmNode,
  name: HostContractName,
  fn: string,
  args: unknown[] = [],
): Promise<Result> {
  const iface = ifaceFor(name);
  const raw = await node.call(ADDRESSES[name], iface.encodeFunctionData(fn, args));
  return iface.decodeFunctionResult(fn, raw);
}

/** Send a state-changing call from `from` (must be impersonated) to a host contract. */
export async function sendCleartext(
  node: FhevmNode,
  from: string,
  name: HostContractName,
  fn: string,
  args: unknown[] = [],
): Promise<void> {
  const iface = ifaceFor(name);
  await node.sendTransaction({ from, to: ADDRESSES[name], data: iface.encodeFunctionData(fn, args) });
}
