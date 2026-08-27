import * as fs from "fs";

import type { InterfaceAbi } from "ethers";

import type { HostContractName } from "./addresses";

/**
 * ABIs for the placed contracts, read straight from the package's `./abi/*.json` subpath export.
 *
 * ABIs only — no bytecode, no templates, no patching. Placing the stack is
 * `@fhevm/host-contracts-cleartext`'s job (`deployAt`); the plugin reads it back afterwards, and for that
 * it needs nothing but the interfaces.
 */
const CLEARTEXT_PACKAGE = "@fhevm/host-contracts-cleartext";

/** The contract actually placed at each host address. Only the ones that must observe or expose cleartext
 * have a `Cleartext*` variant; the rest are the plain host contracts. */
const CONTRACT_NAMES: Record<HostContractName, string> = {
  ACL: "ACL",
  FHEVMExecutor: "CleartextFHEVMExecutor",
  KMSVerifier: "CleartextKMSVerifier",
  InputVerifier: "CleartextInputVerifier",
  HCULimit: "HCULimit",
  ProtocolConfig: "ProtocolConfig",
  KMSGeneration: "KMSGeneration",
  PauserSet: "PauserSet",
  CleartextArithmetic: "CleartextArithmetic",
  CleartextDB: "CleartextDB",
};

const cache = new Map<HostContractName, InterfaceAbi>();

export function getAbi(name: HostContractName): InterfaceAbi {
  const cached = cache.get(name);
  if (cached) {
    return cached;
  }

  const contractName = CONTRACT_NAMES[name];
  let resolved: string;
  try {
    resolved = require.resolve(`${CLEARTEXT_PACKAGE}/abi/${contractName}.json`);
  } catch (e) {
    throw new Error(`Unable to resolve the ABI for '${contractName}' from ${CLEARTEXT_PACKAGE}.\n${e}`);
  }

  const abi = JSON.parse(fs.readFileSync(resolved, "utf8")) as InterfaceAbi;
  cache.set(name, abi);
  return abi;
}
