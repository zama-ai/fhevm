import * as fs from "fs";

import type { InterfaceAbi } from "ethers";

/**
 * The single source of truth for the cleartext engine (RFC-004). Artifacts are resolved through the
 * package's `./artifacts/*` subpath export, so a rebuild of the contracts
 * (`forge build && python3 scripts/gen-hardhat-artifacts.py`) is picked up immediately. There is no
 * vendored copy to keep in sync.
 */
export const CLEARTEXT_PACKAGE = "@fhevm/host-contracts-cleartext";

/**
 * The host-contract name the mock deploys -> the cleartext contract that stands in for it. The
 * cleartext contracts inherit their host counterparts, so each ABI is a superset of the host one
 * (plus the cleartext surface: `plaintext`/`plaintexts`, `*WithCleartext`, ...).
 */
export const CLEARTEXT_CONTRACT_NAMES = {
  ACL: "CleartextACL",
  FHEVMExecutor: "CleartextFHEVMExecutor",
  InputVerifier: "CleartextInputVerifier",
  KMSVerifier: "CleartextKMSVerifier",
  HCULimit: "CleartextHCULimit",
  ProtocolConfig: "CleartextProtocolConfig",
  KMSGeneration: "CleartextKMSGeneration",
  PauserSet: "PauserSet",
} as const;

export type HostContractName = keyof typeof CLEARTEXT_CONTRACT_NAMES;

export interface CleartextArtifact {
  readonly contractName: string;
  readonly abi: InterfaceAbi;
  readonly deployedBytecode: string;
  readonly path: string;
}

const cache = new Map<HostContractName, CleartextArtifact>();

export function getCleartextArtifact(name: HostContractName): CleartextArtifact {
  const cached = cache.get(name);
  if (cached) {
    return cached;
  }

  const cleartextName = CLEARTEXT_CONTRACT_NAMES[name];
  let path: string;
  try {
    path = require.resolve(`${CLEARTEXT_PACKAGE}/artifacts/${cleartextName}.json`);
  } catch (e) {
    throw new Error(
      `Unable to resolve the cleartext artifact '${cleartextName}' from ${CLEARTEXT_PACKAGE}. ` +
        `Is it installed, and has it been built ` +
        `(\`forge build && python3 scripts/gen-hardhat-artifacts.py\`)?\n${e}`,
    );
  }

  const json = JSON.parse(fs.readFileSync(path, "utf8"));
  if (typeof json.deployedBytecode !== "string" || json.deployedBytecode.length <= 2) {
    throw new Error(`Cleartext artifact '${cleartextName}' has no deployedBytecode (at ${path}).`);
  }

  const artifact: CleartextArtifact = {
    contractName: cleartextName,
    abi: json.abi,
    deployedBytecode: json.deployedBytecode,
    path,
  };
  cache.set(name, artifact);
  return artifact;
}
